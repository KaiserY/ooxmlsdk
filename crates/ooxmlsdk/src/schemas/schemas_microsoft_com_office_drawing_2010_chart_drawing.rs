//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the ContentPart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cdr14:contentPart")]
pub struct ContentPart {
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub relationship_id: crate::simple_type::StringValue,
  /// bwMode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(kind = "token"))]
  pub black_white_mode: Option<crate::schemas::a::BlackWhiteModeValues>,
  /// Defines the NonVisualContentPartProperties Class.
  #[sdk(child(qname = "cdr14:nvContentPartPr"))]
  pub non_visual_content_part_properties: Option<std::boxed::Box<NonVisualContentPartProperties>>,
  /// Defines the ApplicationNonVisualDrawingProperties Class.
  #[sdk(child(qname = "cdr14:nvPr"))]
  pub application_non_visual_drawing_properties: Option<ApplicationNonVisualDrawingProperties>,
  /// Defines the Transform2D Class.
  #[sdk(child(qname = "cdr14:xfrm"))]
  pub transform2_d: Option<std::boxed::Box<Transform2D>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cdr14:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the NonVisualDrawingProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cdr14:cNvPr")]
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
#[sdk(qname = "cdr14:cNvContentPartPr")]
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
/// Defines the NonVisualContentPartProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cdr14:nvContentPartPr")]
pub struct NonVisualContentPartProperties {
  /// Defines the NonVisualDrawingProperties Class.
  #[sdk(child(qname = "cdr14:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Defines the NonVisualInkContentPartProperties Class.
  #[sdk(child(qname = "cdr14:cNvContentPartPr"))]
  pub non_visual_ink_content_part_properties:
    Option<std::boxed::Box<NonVisualInkContentPartProperties>>,
}
/// Defines the ApplicationNonVisualDrawingProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cdr14:nvPr")]
pub struct ApplicationNonVisualDrawingProperties {
  /// macro
  #[sdk(attr(qname = ":macro"))]
  pub r#macro: Option<crate::simple_type::StringValue>,
  /// fPublished
  #[sdk(attr(qname = ":fPublished"))]
  pub published: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Transform2D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cdr14:xfrm")]
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
/// Defines the OfficeArtExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cdr14:extLst")]
pub struct OfficeArtExtensionList {
  /// Extension.
  #[sdk(child(qname = "a:ext"))]
  pub extension: Vec<crate::schemas::a::Extension>,
}
