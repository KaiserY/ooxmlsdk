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
#[sdk(qname = "p14:nvContentPartPr")]
pub struct NonVisualContentPartProperties {
  /// Defines the NonVisualDrawingProperties Class.
  #[sdk(child(qname = "p14:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Defines the NonVisualInkContentPartProperties Class.
  #[sdk(child(qname = "p14:cNvContentPartPr"))]
  pub non_visual_ink_content_part_properties:
    Option<std::boxed::Box<NonVisualInkContentPartProperties>>,
  /// Defines the ApplicationNonVisualDrawingProperties Class.
  #[sdk(child(qname = "p14:nvPr"))]
  pub application_non_visual_drawing_properties:
    std::boxed::Box<ApplicationNonVisualDrawingProperties>,
}
/// Defines the Transform2D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:xfrm")]
pub struct Transform2D {
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
  #[sdk(child(qname = "a:off"))]
  pub offset: Option<crate::schemas::a::Offset>,
  /// Extents
  #[sdk(child(qname = "a:ext"))]
  pub extents: Option<crate::schemas::a::Extents>,
}
/// Defines the ExtensionListModify Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:extLst")]
pub struct ExtensionListModify {
  /// Modify
  #[sdk(attr(qname = ":mod"))]
  pub modify: Option<crate::simple_type::BooleanValue>,
  /// Extension.
  #[sdk(child(qname = "p:ext"))]
  pub extension: Vec<crate::schemas::p::Extension>,
}
/// Defines the Media Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:media")]
pub struct Media {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Embedded Picture Reference
  #[sdk(attr(qname = "r:embed"))]
  pub embed: Option<crate::simple_type::StringValue>,
  /// Linked Picture Reference
  #[sdk(attr(qname = "r:link"))]
  pub link: Option<crate::simple_type::StringValue>,
  /// Defines the MediaTrim Class.
  #[sdk(child(qname = "p14:trim"))]
  pub media_trim: Option<MediaTrim>,
  /// Defines the MediaFade Class.
  #[sdk(child(qname = "p14:fade"))]
  pub media_fade: Option<MediaFade>,
  /// Defines the MediaBookmarkList Class.
  #[sdk(child(qname = "p14:bmkLst"))]
  pub media_bookmark_list: Option<MediaBookmarkList>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the VortexTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:vortex")]
pub struct VortexTransition {
  /// Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<crate::schemas::p::TransitionSlideDirectionValues>,
}
/// Defines the PanTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:pan")]
pub struct PanTransition {
  /// Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<crate::schemas::p::TransitionSlideDirectionValues>,
}
/// Defines the SwitchTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:switch")]
pub struct SwitchTransition {
  /// dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<TransitionLeftRightDirectionTypeValues>,
}
/// Defines the FlipTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:flip")]
pub struct FlipTransition {
  /// dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<TransitionLeftRightDirectionTypeValues>,
}
/// Defines the FerrisTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:ferris")]
pub struct FerrisTransition {
  /// dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<TransitionLeftRightDirectionTypeValues>,
}
/// Defines the GalleryTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:gallery")]
pub struct GalleryTransition {
  /// dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<TransitionLeftRightDirectionTypeValues>,
}
/// Defines the ConveyorTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:conveyor")]
pub struct ConveyorTransition {
  /// dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<TransitionLeftRightDirectionTypeValues>,
}
/// Defines the RippleTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:ripple")]
pub struct RippleTransition {
  /// dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_set(source = 0u32, union = 0u64, values = &["lu", "ru", "ld", "rd"]))]
  #[sdk(string_set(source = 1u32, union = 0u64, values = &["center"]))]
  pub direction: Option<crate::simple_type::StringValue>,
}
/// Defines the PrismTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:prism")]
pub struct PrismTransition {
  /// dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<crate::schemas::p::TransitionSlideDirectionValues>,
  /// isContent
  #[sdk(attr(qname = ":isContent"))]
  pub is_content: Option<crate::simple_type::BooleanValue>,
  /// isInverted
  #[sdk(attr(qname = ":isInverted"))]
  pub is_inverted: Option<crate::simple_type::BooleanValue>,
}
/// Defines the DoorsTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:doors")]
pub struct DoorsTransition {
  /// Transition Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<crate::schemas::p::DirectionValues>,
}
/// Defines the WindowTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:window")]
pub struct WindowTransition {
  /// Transition Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<crate::schemas::p::DirectionValues>,
}
/// Defines the GlitterTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:glitter")]
pub struct GlitterTransition {
  /// dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<crate::schemas::p::TransitionSlideDirectionValues>,
  /// pattern
  #[sdk(attr(qname = ":pattern"))]
  #[sdk(string_format(kind = "token"))]
  pub pattern: Option<TransitionPatternValues>,
}
/// Defines the WarpTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:warp")]
pub struct WarpTransition {
  /// Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<crate::schemas::p::TransitionInOutDirectionValues>,
}
/// Defines the FlythroughTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:flythrough")]
pub struct FlythroughTransition {
  /// dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<crate::schemas::p::TransitionInOutDirectionValues>,
  /// hasBounce
  #[sdk(attr(qname = ":hasBounce"))]
  pub has_bounce: Option<crate::simple_type::BooleanValue>,
}
/// Defines the ShredTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:shred")]
pub struct ShredTransition {
  /// pattern
  #[sdk(attr(qname = ":pattern"))]
  #[sdk(string_format(kind = "token"))]
  pub pattern: Option<TransitionShredPatternValues>,
  /// dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<crate::schemas::p::TransitionInOutDirectionValues>,
}
/// Defines the RevealTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:reveal")]
pub struct RevealTransition {
  /// thruBlk
  #[sdk(attr(qname = ":thruBlk"))]
  pub through_black: Option<crate::simple_type::BooleanValue>,
  /// dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<TransitionLeftRightDirectionTypeValues>,
}
/// Defines the WheelReverseTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:wheelReverse")]
pub struct WheelReverseTransition {
  /// Spokes
  #[sdk(attr(qname = ":spokes"))]
  pub spokes: Option<crate::simple_type::UInt32Value>,
}
/// Defines the BookmarkTarget Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:bmkTgt")]
pub struct BookmarkTarget {
  /// spid
  #[sdk(attr(qname = ":spid"))]
  pub shape_id: crate::simple_type::UInt32Value,
  /// bmkName
  #[sdk(attr(qname = ":bmkName"))]
  pub bookmark_name: crate::simple_type::StringValue,
}
/// Defines the SectionProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:sectionPr")]
pub struct SectionProperties {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Defines the SectionOld Class.
  #[sdk(child(qname = "p14:section"))]
  pub section_old: Vec<SectionOld>,
}
/// Defines the SectionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:sectionLst")]
pub struct SectionList {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Defines the Section Class.
  #[sdk(child(qname = "p14:section"))]
  pub section: Vec<Section>,
}
/// Defines the BrowseMode Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:browseMode")]
pub struct BrowseMode {
  /// showStatus
  #[sdk(attr(qname = ":showStatus"))]
  pub show_status: Option<crate::simple_type::BooleanValue>,
}
/// Defines the LaserColor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:laserClr")]
pub struct LaserColor {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, boxed, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub laser_color_choice: Option<LaserColorChoice>,
}
/// Defines the DefaultImageDpi Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:defaultImageDpi")]
pub struct DefaultImageDpi {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Defines the DiscardImageEditData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:discardImageEditData")]
pub struct DiscardImageEditData {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::BooleanValue,
}
/// Defines the ShowMediaControls Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:showMediaCtrls")]
pub struct ShowMediaControls {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::BooleanValue,
}
/// Defines the LaserTraceList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:laserTraceLst")]
pub struct LaserTraceList {
  /// Defines the TracePointList Class.
  #[sdk(child(qname = "p14:tracePtLst"))]
  pub trace_point_list: Vec<TracePointList>,
}
/// Defines the CreationId Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:creationId")]
pub struct CreationId {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Defines the ModificationId Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:modId")]
pub struct ModificationId {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Defines the ShowEventRecordList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:showEvtLst")]
pub struct ShowEventRecordList {
  #[sdk(
        choice(
            child(variant = TriggerEventRecord, qname = "p14:triggerEvt"),
            child(variant = PlayEventRecord, qname = "p14:playEvt"),
            child(variant = StopEventRecord, qname = "p14:stopEvt"),
            child(variant = PauseEventRecord, qname = "p14:pauseEvt"),
            child(variant = ResumeEventRecord, qname = "p14:resumeEvt"),
            child(variant = SeekEventRecord, qname = "p14:seekEvt"),
            child(variant = NullEventRecord, qname = "p14:nullEvt")
        )
    )]
  pub show_event_record_list_choice: Vec<ShowEventRecordListChoice>,
}
/// Defines the NonVisualDrawingProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:cNvPr")]
pub struct NonVisualDrawingProperties {
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
  #[sdk(child(qname = "a:hlinkClick"))]
  pub hyperlink_on_click: Option<std::boxed::Box<crate::schemas::a::HyperlinkOnClick>>,
  /// Hyperlink associated with hovering over the element.
  #[sdk(child(qname = "a:hlinkHover"))]
  pub hyperlink_on_hover: Option<std::boxed::Box<crate::schemas::a::HyperlinkOnHover>>,
  /// Future extension
  #[sdk(child(qname = "a:extLst"))]
  pub non_visual_drawing_properties_extension_list:
    Option<crate::schemas::a::NonVisualDrawingPropertiesExtensionList>,
}
/// Defines the NonVisualInkContentPartProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:cNvContentPartPr")]
pub struct NonVisualInkContentPartProperties {
  /// isComment
  #[sdk(attr(qname = ":isComment"))]
  pub is_comment: Option<crate::simple_type::BooleanValue>,
  /// Defines the ContentPartLocks Class.
  #[sdk(child(qname = "a14:cpLocks"))]
  pub content_part_locks: Option<std::boxed::Box<crate::schemas::a14::ContentPartLocks>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "a14:extLst"))]
  pub office_art_extension_list: Option<crate::schemas::a14::OfficeArtExtensionList>,
}
/// Defines the ApplicationNonVisualDrawingProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:nvPr")]
pub struct ApplicationNonVisualDrawingProperties {
  /// Is a Photo Album
  #[sdk(attr(qname = ":isPhoto"))]
  pub is_photo: Option<crate::simple_type::BooleanValue>,
  /// Is User Drawn
  #[sdk(attr(qname = ":userDrawn"))]
  pub user_drawn: Option<crate::simple_type::BooleanValue>,
  /// Placeholder Shape
  #[sdk(child(qname = "p:ph"))]
  pub placeholder_shape: Option<std::boxed::Box<crate::schemas::p::PlaceholderShape>>,
  #[sdk(
        choice(
            child(variant = AudioFromCd, boxed, qname = "a:audioCd"),
            child(variant = WaveAudioFile, qname = "a:wavAudioFile"),
            child(variant = AudioFromFile, boxed, qname = "a:audioFile"),
            child(variant = VideoFromFile, boxed, qname = "a:videoFile"),
            child(variant = QuickTimeFromFile, boxed, qname = "a:quickTimeFile")
        )
    )]
  pub application_non_visual_drawing_properties_choice:
    Option<ApplicationNonVisualDrawingPropertiesChoice>,
  /// Customer Data List.
  #[sdk(child(qname = "p:custDataLst"))]
  pub customer_data_list: Option<std::boxed::Box<crate::schemas::p::CustomerDataList>>,
  /// Defines the ApplicationNonVisualDrawingPropertiesExtensionList Class.
  #[sdk(child(qname = "p:extLst"))]
  pub application_non_visual_drawing_properties_extension_list:
    Option<crate::schemas::p::ApplicationNonVisualDrawingPropertiesExtensionList>,
}
/// Defines the MediaBookmark Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:bmk")]
pub struct MediaBookmark {
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// time
  #[sdk(attr(qname = ":time"))]
  pub time: Option<crate::simple_type::StringValue>,
}
/// Defines the MediaTrim Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:trim")]
pub struct MediaTrim {
  /// st
  #[sdk(attr(qname = ":st"))]
  pub start: Option<crate::simple_type::StringValue>,
  /// end
  #[sdk(attr(qname = ":end"))]
  pub end: Option<crate::simple_type::StringValue>,
}
/// Defines the MediaFade Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:fade")]
pub struct MediaFade {
  /// in
  #[sdk(attr(qname = ":in"))]
  pub in_duration: Option<crate::simple_type::StringValue>,
  /// out
  #[sdk(attr(qname = ":out"))]
  pub out_duration: Option<crate::simple_type::StringValue>,
}
/// Defines the MediaBookmarkList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:bmkLst")]
pub struct MediaBookmarkList {
  /// Defines the MediaBookmark Class.
  #[sdk(child(qname = "p14:bmk"))]
  pub media_bookmark: Vec<MediaBookmark>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:extLst")]
pub struct ExtensionList {
  /// Extension.
  #[sdk(child(qname = "p:ext"))]
  pub extension: Vec<crate::schemas::p::Extension>,
}
/// Defines the SectionOld Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:section")]
pub struct SectionOld {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// slideIdLst
  #[sdk(attr(list, qname = ":slideIdLst"))]
  #[sdk(number_range(range = 256..2147483648))]
  pub slide_id_list: Option<Vec<crate::simple_type::UInt32Value>>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the SectionSlideIdListEntry Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:sldId")]
pub struct SectionSlideIdListEntry {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(number_range(range = 256..2147483648))]
  pub id: crate::simple_type::UInt32Value,
}
/// Defines the SectionSlideIdList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:sldIdLst")]
pub struct SectionSlideIdList {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Defines the SectionSlideIdListEntry Class.
  #[sdk(child(qname = "p14:sldId"))]
  pub section_slide_id_list_entry: Vec<SectionSlideIdListEntry>,
}
/// Defines the Section Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:section")]
pub struct Section {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Defines the SectionSlideIdList Class.
  #[sdk(child(qname = "p14:sldIdLst"))]
  pub section_slide_id_list: SectionSlideIdList,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the TracePoint Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:tracePt")]
pub struct TracePoint {
  /// t
  #[sdk(attr(qname = ":t"))]
  pub time: crate::simple_type::StringValue,
  /// x
  #[sdk(attr(qname = ":x"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub x_coordinate: crate::simple_type::Int64Value,
  /// y
  #[sdk(attr(qname = ":y"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub y_coordinate: crate::simple_type::Int64Value,
}
/// Defines the TracePointList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:tracePtLst")]
pub struct TracePointList {
  /// Defines the TracePoint Class.
  #[sdk(child(qname = "p14:tracePt"))]
  pub trace_point: Vec<TracePoint>,
}
/// Defines the TriggerEventRecord Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:triggerEvt")]
pub struct TriggerEventRecord {
  /// type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(kind = "token"))]
  pub r#type: crate::schemas::p::TriggerEventValues,
  /// time
  #[sdk(attr(qname = ":time"))]
  pub time: crate::simple_type::StringValue,
  /// objId
  #[sdk(attr(qname = ":objId"))]
  pub object_id: crate::simple_type::UInt32Value,
}
/// Defines the PlayEventRecord Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:playEvt")]
pub struct PlayEventRecord {
  /// time
  #[sdk(attr(qname = ":time"))]
  pub time: crate::simple_type::StringValue,
  /// objId
  #[sdk(attr(qname = ":objId"))]
  pub object_id: crate::simple_type::UInt32Value,
}
/// Defines the StopEventRecord Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:stopEvt")]
pub struct StopEventRecord {
  /// time
  #[sdk(attr(qname = ":time"))]
  pub time: crate::simple_type::StringValue,
  /// objId
  #[sdk(attr(qname = ":objId"))]
  pub object_id: crate::simple_type::UInt32Value,
}
/// Defines the PauseEventRecord Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:pauseEvt")]
pub struct PauseEventRecord {
  /// time
  #[sdk(attr(qname = ":time"))]
  pub time: crate::simple_type::StringValue,
  /// objId
  #[sdk(attr(qname = ":objId"))]
  pub object_id: crate::simple_type::UInt32Value,
}
/// Defines the ResumeEventRecord Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:resumeEvt")]
pub struct ResumeEventRecord {
  /// time
  #[sdk(attr(qname = ":time"))]
  pub time: crate::simple_type::StringValue,
  /// objId
  #[sdk(attr(qname = ":objId"))]
  pub object_id: crate::simple_type::UInt32Value,
}
/// Defines the SeekEventRecord Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:seekEvt")]
pub struct SeekEventRecord {
  /// time
  #[sdk(attr(qname = ":time"))]
  pub time: crate::simple_type::StringValue,
  /// objId
  #[sdk(attr(qname = ":objId"))]
  pub object_id: crate::simple_type::UInt32Value,
  /// seek
  #[sdk(attr(qname = ":seek"))]
  pub seek: crate::simple_type::StringValue,
}
/// Defines the NullEventRecord Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p14:nullEvt")]
pub struct NullEventRecord {
  /// time
  #[sdk(attr(qname = ":time"))]
  pub time: crate::simple_type::StringValue,
  /// objId
  #[sdk(attr(qname = ":objId"))]
  pub object_id: crate::simple_type::UInt32Value,
}
#[derive(Clone, Debug, PartialEq)]
pub enum LaserColorChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(crate::schemas::a::RgbColorModelPercentage),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<crate::schemas::a::RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(crate::schemas::a::HslColor),
  /// System Color.
  SystemColor(crate::schemas::a::SystemColor),
  /// Scheme Color.
  SchemeColor(crate::schemas::a::SchemeColor),
  /// Preset Color.
  PresetColor(crate::schemas::a::PresetColor),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ShowEventRecordListChoice {
  /// Defines the TriggerEventRecord Class.
  TriggerEventRecord(TriggerEventRecord),
  /// Defines the PlayEventRecord Class.
  PlayEventRecord(PlayEventRecord),
  /// Defines the StopEventRecord Class.
  StopEventRecord(StopEventRecord),
  /// Defines the PauseEventRecord Class.
  PauseEventRecord(PauseEventRecord),
  /// Defines the ResumeEventRecord Class.
  ResumeEventRecord(ResumeEventRecord),
  /// Defines the SeekEventRecord Class.
  SeekEventRecord(SeekEventRecord),
  /// Defines the NullEventRecord Class.
  NullEventRecord(NullEventRecord),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ApplicationNonVisualDrawingPropertiesChoice {
  AudioFromCd(std::boxed::Box<crate::schemas::a::AudioFromCd>),
  WaveAudioFile(crate::schemas::a::WaveAudioFile),
  AudioFromFile(std::boxed::Box<crate::schemas::a::AudioFromFile>),
  VideoFromFile(std::boxed::Box<crate::schemas::a::VideoFromFile>),
  QuickTimeFromFile(std::boxed::Box<crate::schemas::a::QuickTimeFromFile>),
}
