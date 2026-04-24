//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the ContentPart Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr14:contentPart.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr14:CT_ContentPart/xdr14:contentPart")]
pub struct ContentPart {
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub relationship_id: crate::simple_type::StringValue,
  /// bwMode
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :bwMode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues>,
  /// _
  #[sdk(child(qname = "xdr14:CT_ContentPartNonVisual/xdr14:nvContentPartPr"))]
  pub excel_non_visual_content_part_shape_properties:
    Option<std::boxed::Box<ExcelNonVisualContentPartShapeProperties>>,
  /// _
  #[sdk(child(qname = "xdr14:CT_ApplicationNonVisualDrawingProps/xdr14:nvPr"))]
  pub application_non_visual_drawing_properties: Option<ApplicationNonVisualDrawingProperties>,
  /// _
  #[sdk(child(qname = "a:CT_Transform2D/xdr14:xfrm"))]
  pub transform2_d: Option<std::boxed::Box<Transform2D>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/xdr14:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the NonVisualDrawingProperties Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr14:cNvPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualDrawingProps/xdr14:cNvPr")]
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
/// When the object is serialized out as xml, it's qualified name is xdr14:cNvContentPartPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_NonVisualInkContentPartProperties/xdr14:cNvContentPartPr")]
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
/// Defines the ExcelNonVisualContentPartShapeProperties Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr14:nvContentPartPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr14:CT_ContentPartNonVisual/xdr14:nvContentPartPr")]
pub struct ExcelNonVisualContentPartShapeProperties {
  /// _
  #[sdk(child(qname = "a:CT_NonVisualDrawingProps/xdr14:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// _
  #[sdk(child(qname = "a14:CT_NonVisualInkContentPartProperties/xdr14:cNvContentPartPr"))]
  pub non_visual_ink_content_part_properties:
    Option<std::boxed::Box<NonVisualInkContentPartProperties>>,
}
/// Defines the ApplicationNonVisualDrawingProperties Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr14:nvPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr14:CT_ApplicationNonVisualDrawingProps/xdr14:nvPr")]
pub struct ApplicationNonVisualDrawingProperties {
  /// macro
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :macro
  #[sdk(attr(qname = ":macro"))]
  pub r#macro: Option<crate::simple_type::StringValue>,
  /// fPublished
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :fPublished
  #[sdk(attr(qname = ":fPublished"))]
  pub published_flag: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Transform2D Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr14:xfrm.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Transform2D/xdr14:xfrm")]
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
/// Defines the OfficeArtExtensionList Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr14:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_OfficeArtExtensionList/xdr14:extLst")]
pub struct OfficeArtExtensionList {
  ///Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension>,
}
