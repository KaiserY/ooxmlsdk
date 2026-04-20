//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum TransitionPatternValues {
  #[sdk(rename = "diamond")]
  #[default]
  Diamond,
  #[sdk(rename = "hexagon")]
  Hexagon,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum TransitionCenterDirectionTypeValues {
  #[sdk(rename = "center")]
  #[default]
  Center,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum TransitionShredPatternValues {
  #[sdk(rename = "strip")]
  #[default]
  Strip,
  #[sdk(rename = "rectangle")]
  Rectangle,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum TransitionLeftRightDirectionTypeValues {
  #[sdk(rename = "l")]
  #[default]
  Left,
  #[sdk(rename = "r")]
  Right,
}
/// Defines the NonVisualContentPartProperties Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:nvContentPartPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_ContentPartNonVisual/p14:nvContentPartPr")]
pub struct NonVisualContentPartProperties {
  /// _
  #[sdk(child(qname = "a:CT_NonVisualDrawingProps/p14:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// _
  #[sdk(child(qname = "a14:CT_NonVisualInkContentPartProperties/p14:cNvContentPartPr"))]
  pub non_visual_ink_content_part_properties:
    Option<std::boxed::Box<NonVisualInkContentPartProperties>>,
  /// _
  #[sdk(child(qname = "p:CT_ApplicationNonVisualDrawingProps/p14:nvPr"))]
  pub application_non_visual_drawing_properties:
    std::boxed::Box<ApplicationNonVisualDrawingProperties>,
}
/// Defines the Transform2D Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:xfrm.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Transform2D/p14:xfrm")]
pub struct Transform2D {
  /// Rotation
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rot
  #[sdk(attr(qname = ":rot"))]
  pub rotation: Option<crate::simple_type::Int32Value>,
  /// Horizontal Flip
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :flipH
  #[sdk(attr(qname = ":flipH"))]
  pub horizontal_flip: Option<crate::simple_type::BooleanValue>,
  /// Vertical Flip
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :flipV
  #[sdk(attr(qname = ":flipV"))]
  pub vertical_flip: Option<crate::simple_type::BooleanValue>,
  ///Offset
  #[sdk(child(qname = "a:CT_Point2D/a:off"))]
  pub offset: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Offset>,
  ///Extents
  #[sdk(child(qname = "a:CT_PositiveSize2D/a:ext"))]
  pub extents: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extents>,
}
/// Defines the ExtensionListModify Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ExtensionListModify/p14:extLst")]
pub struct ExtensionListModify {
  /// Modify
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :mod
  #[sdk(attr(qname = ":mod"))]
  pub modify: Option<crate::simple_type::BooleanValue>,
  ///Extension.
  #[sdk(child(qname = "p:CT_Extension/p:ext"))]
  pub extension:
    Vec<crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Extension>,
}
/// Defines the Media Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:media.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_Media/p14:media")]
pub struct Media {
  /// Embedded Picture Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:embed
  #[sdk(attr(qname = "r:embed"))]
  pub embed: Option<crate::simple_type::StringValue>,
  /// Linked Picture Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:link
  #[sdk(attr(qname = "r:link"))]
  pub link: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "p14:CT_MediaTrim/p14:trim"))]
  pub media_trim: Option<MediaTrim>,
  /// _
  #[sdk(child(qname = "p14:CT_MediaFade/p14:fade"))]
  pub media_fade: Option<MediaFade>,
  /// _
  #[sdk(child(qname = "p14:CT_MediaBookmarkList/p14:bmkLst"))]
  pub media_bookmark_list: Option<MediaBookmarkList>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the VortexTransition Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:vortex.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SideDirectionTransition/p14:vortex")]
pub struct VortexTransition {
    /// Direction
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :dir
    #[sdk(attr(qname = ":dir"))]
    #[sdk(string_format(source = 0u32, kind = "token"))]
    pub direction: Option<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TransitionSlideDirectionValues,
    >,
}
/// Defines the PanTransition Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:pan.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SideDirectionTransition/p14:pan")]
pub struct PanTransition {
    /// Direction
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :dir
    #[sdk(attr(qname = ":dir"))]
    #[sdk(string_format(source = 0u32, kind = "token"))]
    pub direction: Option<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TransitionSlideDirectionValues,
    >,
}
/// Defines the SideDirectionTransitionType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SideDirectionTransition/")]
pub struct SideDirectionTransitionType {
    /// Direction
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :dir
    #[sdk(attr(qname = ":dir"))]
    #[sdk(string_format(source = 0u32, kind = "token"))]
    pub direction: Option<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TransitionSlideDirectionValues,
    >,
}
/// Defines the SwitchTransition Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:switch.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_LeftRightDirectionTransition/p14:switch")]
pub struct SwitchTransition {
  /// dir
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub direction: Option<TransitionLeftRightDirectionTypeValues>,
}
/// Defines the FlipTransition Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:flip.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_LeftRightDirectionTransition/p14:flip")]
pub struct FlipTransition {
  /// dir
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub direction: Option<TransitionLeftRightDirectionTypeValues>,
}
/// Defines the FerrisTransition Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:ferris.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_LeftRightDirectionTransition/p14:ferris")]
pub struct FerrisTransition {
  /// dir
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub direction: Option<TransitionLeftRightDirectionTypeValues>,
}
/// Defines the GalleryTransition Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:gallery.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_LeftRightDirectionTransition/p14:gallery")]
pub struct GalleryTransition {
  /// dir
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub direction: Option<TransitionLeftRightDirectionTypeValues>,
}
/// Defines the ConveyorTransition Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:conveyor.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_LeftRightDirectionTransition/p14:conveyor")]
pub struct ConveyorTransition {
  /// dir
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub direction: Option<TransitionLeftRightDirectionTypeValues>,
}
/// Defines the LeftRightDirectionTransitionType Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_LeftRightDirectionTransition/")]
pub struct LeftRightDirectionTransitionType {
  /// dir
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub direction: Option<TransitionLeftRightDirectionTypeValues>,
}
/// Defines the RippleTransition Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:ripple.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_RippleTransition/p14:ripple")]
pub struct RippleTransition {
  /// dir
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_set(source = 0u32, union = 0u64, values = &["lu", "ru", "ld", "rd"]))]
  #[sdk(string_set(source = 1u32, union = 0u64, values = &["center"]))]
  pub direction: Option<crate::simple_type::StringValue>,
}
/// Defines the HoneycombTransition Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:honeycomb.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Empty/p14:honeycomb")]
pub struct HoneycombTransition {}
/// Defines the FlashTransition Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:flash.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Empty/p14:flash")]
pub struct FlashTransition {}
/// Defines the EmptyType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Empty/")]
pub struct EmptyType {}
/// Defines the PrismTransition Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:prism.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_PrismTransition/p14:prism")]
pub struct PrismTransition {
    /// dir
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :dir
    #[sdk(attr(qname = ":dir"))]
    #[sdk(string_format(source = 0u32, kind = "token"))]
    pub direction: Option<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TransitionSlideDirectionValues,
    >,
    /// isContent
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :isContent
    #[sdk(attr(qname = ":isContent"))]
    pub is_content: Option<crate::simple_type::BooleanValue>,
    /// isInverted
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :isInverted
    #[sdk(attr(qname = ":isInverted"))]
    pub is_inverted: Option<crate::simple_type::BooleanValue>,
}
/// Defines the DoorsTransition Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:doors.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_OrientationTransition/p14:doors")]
pub struct DoorsTransition {
  /// Transition Direction
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub direction:
    Option<crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::DirectionValues>,
}
/// Defines the WindowTransition Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:window.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_OrientationTransition/p14:window")]
pub struct WindowTransition {
  /// Transition Direction
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub direction:
    Option<crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::DirectionValues>,
}
/// Defines the OrientationTransitionType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_OrientationTransition/")]
pub struct OrientationTransitionType {
  /// Transition Direction
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub direction:
    Option<crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::DirectionValues>,
}
/// Defines the GlitterTransition Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:glitter.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_GlitterTransition/p14:glitter")]
pub struct GlitterTransition {
    /// dir
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :dir
    #[sdk(attr(qname = ":dir"))]
    #[sdk(string_format(source = 0u32, kind = "token"))]
    pub direction: Option<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TransitionSlideDirectionValues,
    >,
    /// pattern
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :pattern
    #[sdk(attr(qname = ":pattern"))]
    #[sdk(string_format(source = 0u32, kind = "token"))]
    pub pattern: Option<TransitionPatternValues>,
}
/// Defines the WarpTransition Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:warp.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_InOutTransition/p14:warp")]
pub struct WarpTransition {
    /// Direction
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :dir
    #[sdk(attr(qname = ":dir"))]
    #[sdk(string_format(source = 0u32, kind = "token"))]
    pub direction: Option<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TransitionInOutDirectionValues,
    >,
}
/// Defines the FlythroughTransition Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:flythrough.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_FlyThroughTransition/p14:flythrough")]
pub struct FlythroughTransition {
    /// dir
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :dir
    #[sdk(attr(qname = ":dir"))]
    #[sdk(string_format(source = 0u32, kind = "token"))]
    pub direction: Option<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TransitionInOutDirectionValues,
    >,
    /// hasBounce
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :hasBounce
    #[sdk(attr(qname = ":hasBounce"))]
    pub has_bounce: Option<crate::simple_type::BooleanValue>,
}
/// Defines the ShredTransition Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:shred.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_ShredTransition/p14:shred")]
pub struct ShredTransition {
    /// pattern
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :pattern
    #[sdk(attr(qname = ":pattern"))]
    #[sdk(string_format(source = 0u32, kind = "token"))]
    pub pattern: Option<TransitionShredPatternValues>,
    /// dir
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :dir
    #[sdk(attr(qname = ":dir"))]
    #[sdk(string_format(source = 0u32, kind = "token"))]
    pub direction: Option<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TransitionInOutDirectionValues,
    >,
}
/// Defines the RevealTransition Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:reveal.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_RevealTransition/p14:reveal")]
pub struct RevealTransition {
  /// thruBlk
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :thruBlk
  #[sdk(attr(qname = ":thruBlk"))]
  pub through_black: Option<crate::simple_type::BooleanValue>,
  /// dir
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub direction: Option<TransitionLeftRightDirectionTypeValues>,
}
/// Defines the WheelReverseTransition Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:wheelReverse.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_WheelTransition/p14:wheelReverse")]
pub struct WheelReverseTransition {
  /// Spokes
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :spokes
  #[sdk(attr(qname = ":spokes"))]
  pub spokes: Option<crate::simple_type::UInt32Value>,
}
/// Defines the BookmarkTarget Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:bmkTgt.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_MediaBookmarkTarget/p14:bmkTgt")]
pub struct BookmarkTarget {
  /// spid
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :spid
  #[sdk(attr(qname = ":spid"))]
  pub shape_id: crate::simple_type::UInt32Value,
  /// bmkName
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :bmkName
  #[sdk(attr(qname = ":bmkName"))]
  pub bookmark_name: crate::simple_type::StringValue,
}
/// Defines the SectionProperties Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:sectionPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_SectionProperties/p14:sectionPr")]
pub struct SectionProperties {
  /// _
  #[sdk(child(qname = "p14:CT_SectionOld/p14:section"))]
  pub p14_section: Vec<SectionOld>,
}
/// Defines the SectionList Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:sectionLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_SectionList/p14:sectionLst")]
pub struct SectionList {
  /// _
  #[sdk(child(qname = "p14:CT_Section/p14:section"))]
  pub p14_section: Vec<Section>,
}
/// Defines the BrowseMode Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:browseMode.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_BrowseMode/p14:browseMode")]
pub struct BrowseMode {
  /// showStatus
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showStatus
  #[sdk(attr(qname = ":showStatus"))]
  pub show_status: Option<crate::simple_type::BooleanValue>,
}
/// Defines the LaserColor Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:laserClr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color/p14:laserClr")]
pub struct LaserColor {
  #[sdk(choice)]
  pub xml_children: Option<LaserColorChoice>,
}
/// Defines the DefaultImageDpi Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:defaultImageDpi.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_DefaultImageDpi/p14:defaultImageDpi")]
pub struct DefaultImageDpi {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  /// val
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Defines the DiscardImageEditData Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:discardImageEditData.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_DiscardImageEditData/p14:discardImageEditData")]
pub struct DiscardImageEditData {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  /// val
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::BooleanValue,
}
/// Defines the ShowMediaControls Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:showMediaCtrls.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_ShowMediaControls/p14:showMediaCtrls")]
pub struct ShowMediaControls {
  /// val
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::BooleanValue,
}
/// Defines the LaserTraceList Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:laserTraceLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_LaserTraceList/p14:laserTraceLst")]
pub struct LaserTraceList {
  /// _
  #[sdk(child(qname = "p14:CT_LaserTrace/p14:tracePtLst"))]
  pub p14_trace_pt_lst: Vec<TracePointList>,
}
/// Defines the CreationId Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:creationId.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_RandomId/p14:creationId")]
pub struct CreationId {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  /// val
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Defines the ModificationId Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:modId.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_RandomId/p14:modId")]
pub struct ModificationId {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  /// val
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Defines the RandomIdType Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_RandomId/")]
pub struct RandomIdType {
  /// val
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Defines the ShowEventRecordList Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:showEvtLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_ShowEventRecordList/p14:showEvtLst")]
pub struct ShowEventRecordList {
  #[sdk(choice)]
  pub xml_children: Vec<ShowEventRecordListChoice>,
}
/// Defines the NonVisualDrawingProperties Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:cNvPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualDrawingProps/p14:cNvPr")]
pub struct NonVisualDrawingProperties {
    /// Application defined unique identifier.
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :id
    #[sdk(attr(qname = ":id"))]
    pub id: crate::simple_type::UInt32Value,
    /// Name compatible with Object Model (non-unique).
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :name
    #[sdk(attr(qname = ":name"))]
    pub name: crate::simple_type::StringValue,
    /// Description of the drawing element.
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :descr
    #[sdk(attr(qname = ":descr"))]
    pub description: Option<crate::simple_type::StringValue>,
    /// Flag determining to show or hide this element.
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :hidden
    #[sdk(attr(qname = ":hidden"))]
    pub hidden: Option<crate::simple_type::BooleanValue>,
    /// Title
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :title
    #[sdk(attr(qname = ":title"))]
    pub title: Option<crate::simple_type::StringValue>,
    ///Hyperlink associated with clicking or selecting the element.
    #[sdk(child(qname = "a:CT_Hyperlink/a:hlinkClick"))]
    pub hyperlink_on_click: Option<
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HyperlinkOnClick,
        >,
    >,
    ///Hyperlink associated with hovering over the element.
    #[sdk(child(qname = "a:CT_Hyperlink/a:hlinkHover"))]
    pub hyperlink_on_hover: Option<
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HyperlinkOnHover,
        >,
    >,
    ///Future extension
    #[sdk(child(qname = "a:CT_NonVisualDrawingPropsExtensionList/a:extLst"))]
    pub non_visual_drawing_properties_extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NonVisualDrawingPropertiesExtensionList,
    >,
}
/// Defines the NonVisualInkContentPartProperties Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:cNvContentPartPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_NonVisualInkContentPartProperties/p14:cNvContentPartPr")]
pub struct NonVisualInkContentPartProperties {
  /// isComment
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :isComment
  #[sdk(attr(qname = ":isComment"))]
  pub is_comment: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "a14:CT_ContentPartLocking/a14:cpLocks"))]
  pub content_part_locks: Option<
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2010_main::ContentPartLocks,
    >,
  >,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a14:extLst"))]
  pub office_art_extension_list:
    Option<crate::schemas::schemas_microsoft_com_office_drawing_2010_main::OfficeArtExtensionList>,
}
/// Defines the ApplicationNonVisualDrawingProperties Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:nvPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ApplicationNonVisualDrawingProps/p14:nvPr")]
pub struct ApplicationNonVisualDrawingProperties {
    /// Is a Photo Album
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :isPhoto
    #[sdk(attr(qname = ":isPhoto"))]
    pub is_photo: Option<crate::simple_type::BooleanValue>,
    /// Is User Drawn
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :userDrawn
    #[sdk(attr(qname = ":userDrawn"))]
    pub user_drawn: Option<crate::simple_type::BooleanValue>,
    ///Placeholder Shape
    #[sdk(child(qname = "p:CT_Placeholder/p:ph"))]
    pub placeholder_shape: Option<
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::PlaceholderShape,
        >,
    >,
    #[sdk(choice)]
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
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:bmk.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_MediaBookmark/p14:bmk")]
pub struct MediaBookmark {
  /// name
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// time
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :time
  #[sdk(attr(qname = ":time"))]
  pub time: Option<crate::simple_type::StringValue>,
}
/// Defines the MediaTrim Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:trim.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_MediaTrim/p14:trim")]
pub struct MediaTrim {
  /// st
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :st
  #[sdk(attr(qname = ":st"))]
  pub start: Option<crate::simple_type::StringValue>,
  /// end
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :end
  #[sdk(attr(qname = ":end"))]
  pub end: Option<crate::simple_type::StringValue>,
}
/// Defines the MediaFade Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:fade.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_MediaFade/p14:fade")]
pub struct MediaFade {
  /// in
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :in
  #[sdk(attr(qname = ":in"))]
  pub in_duration: Option<crate::simple_type::StringValue>,
  /// out
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :out
  #[sdk(attr(qname = ":out"))]
  pub out_duration: Option<crate::simple_type::StringValue>,
}
/// Defines the MediaBookmarkList Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:bmkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_MediaBookmarkList/p14:bmkLst")]
pub struct MediaBookmarkList {
  /// _
  #[sdk(child(qname = "p14:CT_MediaBookmark/p14:bmk"))]
  pub p14_bmk: Vec<MediaBookmark>,
}
/// Defines the ExtensionList Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ExtensionList/p14:extLst")]
pub struct ExtensionList {
  ///Extension.
  #[sdk(child(qname = "p:CT_Extension/p:ext"))]
  pub extension:
    Vec<crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Extension>,
}
/// Defines the SectionOld Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:section.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_SectionOld/p14:section")]
pub struct SectionOld {
  /// name
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// slideIdLst
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :slideIdLst
  #[sdk(attr(qname = ":slideIdLst"))]
  #[sdk(number_range(
    source = 0u32,
    min = "256",
    max = "2147483648",
    min_inclusive = true,
    max_inclusive = false
  ))]
  pub slide_id_list: Option<crate::simple_type::ListValue<crate::simple_type::UInt32Value>>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the SectionSlideIdListEntry Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:sldId.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_SectionSlideIdListEntry/p14:sldId")]
pub struct SectionSlideIdListEntry {
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
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
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:sldIdLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_SectionSlideIdList/p14:sldIdLst")]
pub struct SectionSlideIdList {
  /// _
  #[sdk(child(qname = "p14:CT_SectionSlideIdListEntry/p14:sldId"))]
  pub p14_sld_id: Vec<SectionSlideIdListEntry>,
}
/// Defines the Section Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:section.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_Section/p14:section")]
pub struct Section {
  /// name
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "p14:CT_SectionSlideIdList/p14:sldIdLst"))]
  pub section_slide_id_list: std::boxed::Box<SectionSlideIdList>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the TracePoint Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:tracePt.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_LaserTracePoint/p14:tracePt")]
pub struct TracePoint {
  /// t
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :t
  #[sdk(attr(qname = ":t"))]
  pub time: crate::simple_type::StringValue,
  /// x
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :x
  #[sdk(attr(qname = ":x"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub x_coordinate: crate::simple_type::Int64Value,
  /// y
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :y
  #[sdk(attr(qname = ":y"))]
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
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:tracePtLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_LaserTrace/p14:tracePtLst")]
pub struct TracePointList {
  /// _
  #[sdk(child(qname = "p14:CT_LaserTracePoint/p14:tracePt"))]
  pub p14_trace_pt: Vec<TracePoint>,
}
/// Defines the TriggerEventRecord Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:triggerEvt.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_TriggerEventRecord/p14:triggerEvt")]
pub struct TriggerEventRecord {
  /// type
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub r#type:
    crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TriggerEventValues,
  /// time
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :time
  #[sdk(attr(qname = ":time"))]
  pub time: crate::simple_type::StringValue,
  /// objId
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :objId
  #[sdk(attr(qname = ":objId"))]
  pub object_id: crate::simple_type::UInt32Value,
}
/// Defines the PlayEventRecord Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:playEvt.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_MediaPlaybackEventRecord/p14:playEvt")]
pub struct PlayEventRecord {
  /// time
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :time
  #[sdk(attr(qname = ":time"))]
  pub time: crate::simple_type::StringValue,
  /// objId
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :objId
  #[sdk(attr(qname = ":objId"))]
  pub object_id: crate::simple_type::UInt32Value,
}
/// Defines the StopEventRecord Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:stopEvt.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_MediaPlaybackEventRecord/p14:stopEvt")]
pub struct StopEventRecord {
  /// time
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :time
  #[sdk(attr(qname = ":time"))]
  pub time: crate::simple_type::StringValue,
  /// objId
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :objId
  #[sdk(attr(qname = ":objId"))]
  pub object_id: crate::simple_type::UInt32Value,
}
/// Defines the PauseEventRecord Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:pauseEvt.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_MediaPlaybackEventRecord/p14:pauseEvt")]
pub struct PauseEventRecord {
  /// time
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :time
  #[sdk(attr(qname = ":time"))]
  pub time: crate::simple_type::StringValue,
  /// objId
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :objId
  #[sdk(attr(qname = ":objId"))]
  pub object_id: crate::simple_type::UInt32Value,
}
/// Defines the ResumeEventRecord Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:resumeEvt.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_MediaPlaybackEventRecord/p14:resumeEvt")]
pub struct ResumeEventRecord {
  /// time
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :time
  #[sdk(attr(qname = ":time"))]
  pub time: crate::simple_type::StringValue,
  /// objId
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :objId
  #[sdk(attr(qname = ":objId"))]
  pub object_id: crate::simple_type::UInt32Value,
}
/// Defines the MediaPlaybackEventRecordType Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_MediaPlaybackEventRecord/")]
pub struct MediaPlaybackEventRecordType {
  /// time
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :time
  #[sdk(attr(qname = ":time"))]
  pub time: crate::simple_type::StringValue,
  /// objId
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :objId
  #[sdk(attr(qname = ":objId"))]
  pub object_id: crate::simple_type::UInt32Value,
}
/// Defines the SeekEventRecord Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:seekEvt.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_MediaSeekEventRecord/p14:seekEvt")]
pub struct SeekEventRecord {
  /// time
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :time
  #[sdk(attr(qname = ":time"))]
  pub time: crate::simple_type::StringValue,
  /// objId
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :objId
  #[sdk(attr(qname = ":objId"))]
  pub object_id: crate::simple_type::UInt32Value,
  /// seek
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :seek
  #[sdk(attr(qname = ":seek"))]
  pub seek: crate::simple_type::StringValue,
}
/// Defines the NullEventRecord Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p14:nullEvt.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:CT_NullEventRecord/p14:nullEvt")]
pub struct NullEventRecord {
  /// time
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :time
  #[sdk(attr(qname = ":time"))]
  pub time: crate::simple_type::StringValue,
  /// objId
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :objId
  #[sdk(attr(qname = ":objId"))]
  pub object_id: crate::simple_type::UInt32Value,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum LaserColorChoice {
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
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum ShowEventRecordListChoice {
  #[sdk(child(qname = "p14:CT_TriggerEventRecord/p14:triggerEvt"))]
  P14TriggerEvt(std::boxed::Box<TriggerEventRecord>),
  #[sdk(child(qname = "p14:CT_MediaPlaybackEventRecord/p14:playEvt"))]
  P14PlayEvt(std::boxed::Box<PlayEventRecord>),
  #[sdk(child(qname = "p14:CT_MediaPlaybackEventRecord/p14:stopEvt"))]
  P14StopEvt(std::boxed::Box<StopEventRecord>),
  #[sdk(child(qname = "p14:CT_MediaPlaybackEventRecord/p14:pauseEvt"))]
  P14PauseEvt(std::boxed::Box<PauseEventRecord>),
  #[sdk(child(qname = "p14:CT_MediaPlaybackEventRecord/p14:resumeEvt"))]
  P14ResumeEvt(std::boxed::Box<ResumeEventRecord>),
  #[sdk(child(qname = "p14:CT_MediaSeekEventRecord/p14:seekEvt"))]
  P14SeekEvt(std::boxed::Box<SeekEventRecord>),
  #[sdk(child(qname = "p14:CT_NullEventRecord/p14:nullEvt"))]
  P14NullEvt(std::boxed::Box<NullEventRecord>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
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
