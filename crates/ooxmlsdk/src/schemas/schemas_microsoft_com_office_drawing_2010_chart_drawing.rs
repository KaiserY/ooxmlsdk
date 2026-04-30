//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the ContentPart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "cdr14:CT_ContentPart/cdr14:contentPart")]
pub struct ContentPart {
  /// id
  #[sdk(attr(office2010, qname = "r:id"))]
  pub relationship_id: crate::simple_type::StringValue,
  /// bwMode
  #[sdk(attr(office2010, qname = ":bwMode"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues>,
  /// _
  #[sdk(child(
    office2010,
    qname = "cdr14:CT_ContentPartNonVisual/cdr14:nvContentPartPr"
  ))]
  pub non_visual_content_part_properties: Option<std::boxed::Box<NonVisualContentPartProperties>>,
  /// _
  #[sdk(child(
    office2010,
    qname = "cdr14:CT_ApplicationNonVisualDrawingProps/cdr14:nvPr"
  ))]
  pub application_non_visual_drawing_properties: Option<ApplicationNonVisualDrawingProperties>,
  /// _
  #[sdk(child(office2010, qname = "a:CT_Transform2D/cdr14:xfrm"))]
  pub transform2_d: Option<std::boxed::Box<Transform2D>>,
  /// _
  #[sdk(child(office2010, qname = "a:CT_OfficeArtExtensionList/cdr14:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the NonVisualDrawingProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_NonVisualDrawingProps/cdr14:cNvPr")]
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
  qname = "a14:CT_NonVisualInkContentPartProperties/cdr14:cNvContentPartPr"
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
/// Defines the NonVisualContentPartProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "cdr14:CT_ContentPartNonVisual/cdr14:nvContentPartPr"
)]
pub struct NonVisualContentPartProperties {
  /// _
  #[sdk(child(office2010, qname = "a:CT_NonVisualDrawingProps/cdr14:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// _
  #[sdk(child(
    office2010,
    qname = "a14:CT_NonVisualInkContentPartProperties/cdr14:cNvContentPartPr"
  ))]
  pub non_visual_ink_content_part_properties:
    Option<std::boxed::Box<NonVisualInkContentPartProperties>>,
}
/// Defines the ApplicationNonVisualDrawingProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "cdr14:CT_ApplicationNonVisualDrawingProps/cdr14:nvPr"
)]
pub struct ApplicationNonVisualDrawingProperties {
  /// macro
  #[sdk(attr(office2010, qname = ":macro"))]
  pub r#macro: Option<crate::simple_type::StringValue>,
  /// fPublished
  #[sdk(attr(office2010, qname = ":fPublished"))]
  pub published: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Transform2D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_Transform2D/cdr14:xfrm")]
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
/// Defines the OfficeArtExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_OfficeArtExtensionList/cdr14:extLst")]
pub struct OfficeArtExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension>,
}
