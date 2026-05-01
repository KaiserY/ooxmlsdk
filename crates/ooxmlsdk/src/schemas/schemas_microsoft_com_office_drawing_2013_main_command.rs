//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ResourceLinkage {
  #[sdk(rename = "embed")]
  #[default]
  Embed,
  #[sdk(rename = "link")]
  Link,
  #[sdk(rename = "linkAndEmbed")]
  LinkAndEmbed,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DetachConnection {
  #[sdk(rename = "start")]
  #[default]
  Start,
  #[sdk(rename = "end")]
  End,
  #[sdk(rename = "both")]
  Both,
}
/// Defines the ShapeMoniker Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_ShapeMoniker/oac:spMk")]
pub struct ShapeMoniker {
  /// id
  #[sdk(attr(office2016, qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// creationId
  #[sdk(attr(office2016, qname = ":creationId"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub creation_id: Option<crate::simple_type::StringValue>,
}
/// Defines the GroupShapeMoniker Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_GroupShapeMoniker/oac:grpSpMk")]
pub struct GroupShapeMoniker {
  /// id
  #[sdk(attr(office2016, qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// creationId
  #[sdk(attr(office2016, qname = ":creationId"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub creation_id: Option<crate::simple_type::StringValue>,
}
/// Defines the GraphicFrameMoniker Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_GraphicFrameMoniker/oac:graphicFrameMk")]
pub struct GraphicFrameMoniker {
  /// id
  #[sdk(attr(office2016, qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// creationId
  #[sdk(attr(office2016, qname = ":creationId"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub creation_id: Option<crate::simple_type::StringValue>,
}
/// Defines the ConnectorMoniker Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_ConnectorMoniker/oac:cxnSpMk")]
pub struct ConnectorMoniker {
  /// id
  #[sdk(attr(office2016, qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// creationId
  #[sdk(attr(office2016, qname = ":creationId"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub creation_id: Option<crate::simple_type::StringValue>,
}
/// Defines the PictureMoniker Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_PictureMoniker/oac:picMk")]
pub struct PictureMoniker {
  /// id
  #[sdk(attr(office2016, qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// creationId
  #[sdk(attr(office2016, qname = ":creationId"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub creation_id: Option<crate::simple_type::StringValue>,
}
/// Defines the InkMoniker Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_InkMoniker/oac:inkMk")]
pub struct InkMoniker {
  /// id
  #[sdk(attr(office2016, qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// creationId
  #[sdk(attr(office2016, qname = ":creationId"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub creation_id: Option<crate::simple_type::StringValue>,
}
/// Defines the DrawingMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_DrawingMonikerList/oac:dgMkLst")]
pub struct DrawingMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the Transform2D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "a:CT_Transform2D/oac:xfrm")]
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
/// Defines the GroupShapeMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_GroupShapeMonikerList/oac:grpSpMkLst")]
pub struct GroupShapeMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the DeMkLstDrawingElementMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_DrawingElementMonikerList/oac:deMkLst")]
pub struct DeMkLstDrawingElementMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the DeMasterMkLstDrawingElementMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "oac:CT_DrawingElementMonikerList/oac:deMasterMkLst"
)]
pub struct DeMasterMkLstDrawingElementMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the DeSrcMkLstDrawingElementMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_DrawingElementMonikerList/oac:deSrcMkLst")]
pub struct DeSrcMkLstDrawingElementMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the DeTgtMkLstDrawingElementMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_DrawingElementMonikerList/oac:deTgtMkLst")]
pub struct DeTgtMkLstDrawingElementMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the ImgDataImgData Class.
pub type ImgDataImgData = crate::simple_type::Base64BinaryValue;
/// Defines the OrigImgDataImgData Class.
pub type OrigImgDataImgData = crate::simple_type::Base64BinaryValue;
/// Defines the SndDataImgData Class.
pub type SndDataImgData = crate::simple_type::Base64BinaryValue;
/// Defines the ResourceUrl Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_ResourceUrl/oac:imgUrl")]
pub struct ResourceUrl {
  /// src
  #[sdk(attr(office2016, qname = ":src"))]
  #[sdk(string_format(source = 0u32, kind = "uri"))]
  pub src: Option<crate::simple_type::StringValue>,
  /// linkage
  #[sdk(attr(office2016, qname = ":linkage"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub linkage: Option<ResourceLinkage>,
}
/// Defines the GroupCommand Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_GroupCommand/oac:grpCmd")]
pub struct GroupCommand {
  /// verId
  #[sdk(attr(office2016, qname = ":verId"))]
  pub ver_id: Option<crate::simple_type::UInt32Value>,
  /// preventRegroup
  #[sdk(attr(office2016, qname = ":preventRegroup"))]
  pub prevent_regroup: Option<crate::simple_type::BooleanValue>,
  /// grpId
  #[sdk(attr(office2016, qname = ":grpId"))]
  pub grp_id: Option<crate::simple_type::UInt32Value>,
  /// Defines the DrawingMonikerList Class.
  #[sdk(child(office2016, qname = "oac:CT_DrawingMonikerList/oac:dgMkLst"))]
  pub drawing_moniker_list: Option<DrawingMonikerList>,
  #[sdk(choice(
    microsoft365,
    qname = "oac:CT_ShapeMoniker/oac:spMk",
    qname = "oac:CT_GroupShapeMoniker/oac:grpSpMk",
    qname = "oac:CT_GraphicFrameMoniker/oac:graphicFrameMk",
    qname = "oac:CT_ConnectorMoniker/oac:cxnSpMk",
    qname = "oac:CT_PictureMoniker/oac:picMk",
    qname = "oac:CT_InkMoniker/oac:inkMk"
  ))]
  pub group_command_choice: Vec<GroupCommandChoice>,
  /// Defines the GroupShapeProperties Class.
  #[sdk(child(office2016, qname = "a:CT_GroupShapeProperties/oac:grpSpPr"))]
  pub oac_grp_sp_pr: Option<std::boxed::Box<GroupShapeProperties>>,
  /// Defines the NonVisualDrawingProps Class.
  #[sdk(child(office2016, qname = "a:CT_NonVisualDrawingProps/oac:cNvPr"))]
  pub oac_c_nv_pr: Option<std::boxed::Box<NonVisualDrawingProps>>,
  /// Defines the NonVisualGroupDrawingShapeProps Class.
  #[sdk(child(
    office2016,
    qname = "a:CT_NonVisualGroupDrawingShapeProps/oac:cNvGrpSpPr"
  ))]
  pub oac_c_nv_grp_sp_pr: Option<std::boxed::Box<NonVisualGroupDrawingShapeProps>>,
}
/// Defines the ImgLink Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_ImgLink/oac:imgLink")]
pub struct ImgLink {
  /// tgt
  #[sdk(attr(office2016, qname = ":tgt"))]
  #[sdk(string_format(source = 1u32, kind = "uri"))]
  pub tgt: crate::simple_type::StringValue,
}
/// Defines the DocumentContextMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_DocumentContextMonikerList/oac:dcMkLst")]
pub struct DocumentContextMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the GraphicParentMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "oac:CT_GraphicParentMonikerList/oac:graphicParentMkLst"
)]
pub struct GraphicParentMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the ShapeMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_ShapeMonikerList/oac:spMkLst")]
pub struct ShapeMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the GraphicFrameMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "oac:CT_GraphicFrameMonikerList/oac:graphicFrameMkLst"
)]
pub struct GraphicFrameMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the ConnectorMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_ConnectorMonikerList/oac:cxnSpMkLst")]
pub struct ConnectorMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the PictureMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_PictureMonikerList/oac:picMkLst")]
pub struct PictureMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the InkMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_InkMonikerList/oac:inkMkLst")]
pub struct InkMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the TextBodyMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_TextBodyMonikerList/oac:txBodyMkLst")]
pub struct TextBodyMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the TextCharRangeMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_TextCharRangeMonikerList/oac:txMkLst")]
pub struct TextCharRangeMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the HyperlinkMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_HyperlinkMonikerList/oac:hlinkMkLst")]
pub struct HyperlinkMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the Model3DMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_Model3DMonikerList/oac:model3DMkLst")]
pub struct Model3DMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the ViewSelectionStgList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_ViewSelectionStgList/oac:viewSelLst")]
pub struct ViewSelectionStgList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the EditorSelectionStgList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_EditorSelectionStgList/oac:editorSelLst")]
pub struct EditorSelectionStgList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the DrawingSelectionStgList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_DrawingSelectionStgList/oac:drSelLst")]
pub struct DrawingSelectionStgList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the TableMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_TableMonikerList/oac:tblMkLst")]
pub struct TableMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the TableCellMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_TableCellMonikerList/oac:tcMkLst")]
pub struct TableCellMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the TableRowMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_TableRowMonikerList/oac:trMkLst")]
pub struct TableRowMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the TableColumnMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_TableColumnMonikerList/oac:gridColMkLst")]
pub struct TableColumnMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the ModifyNonVisualDrawingProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_ModifyNonVisualDrawingProps/oac:cNvPr")]
pub struct ModifyNonVisualDrawingProps {
  /// name
  #[sdk(attr(office2016, qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// descr
  #[sdk(attr(office2016, qname = ":descr"))]
  pub descr: Option<crate::simple_type::StringValue>,
  /// hidden
  #[sdk(attr(office2016, qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  /// title
  #[sdk(attr(office2016, qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// decor
  #[sdk(attr(office2016, qname = ":decor"))]
  pub decor: Option<crate::simple_type::BooleanValue>,
  /// scriptLink
  #[sdk(attr(office2016, qname = ":scriptLink"))]
  pub script_link: Option<crate::simple_type::StringValue>,
}
/// Defines the ModifyTransformProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_ModifyTransformProps/oac:xfrm")]
pub struct ModifyTransformProps {
  /// x
  #[sdk(attr(office2016, qname = ":x"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub x: Option<crate::simple_type::Int64Value>,
  /// y
  #[sdk(attr(office2016, qname = ":y"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub y: Option<crate::simple_type::Int64Value>,
  /// cx
  #[sdk(attr(office2016, qname = ":cx"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub cx: Option<crate::simple_type::Int64Value>,
  /// cy
  #[sdk(attr(office2016, qname = ":cy"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub cy: Option<crate::simple_type::Int64Value>,
  /// rot
  #[sdk(attr(office2016, qname = ":rot"))]
  pub rot: Option<crate::simple_type::Int32Value>,
  /// flipH
  #[sdk(attr(office2016, qname = ":flipH"))]
  pub flip_h: Option<crate::simple_type::BooleanValue>,
  /// flipV
  #[sdk(attr(office2016, qname = ":flipV"))]
  pub flip_v: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Point2DType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "a:CT_Point2D/oac:off")]
pub struct Point2DType {
  /// X-Axis Coordinate
  #[sdk(attr(qname = ":x"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub x: crate::simple_type::Int64Value,
  /// Y-Axis Coordinate
  #[sdk(attr(qname = ":y"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub y: crate::simple_type::Int64Value,
}
/// Defines the TextParagraphPropertiesType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "a:CT_TextParagraphProperties/oac:pPr")]
pub struct TextParagraphPropertiesType {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Left Margin
  #[sdk(attr(qname = ":marL"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub left_margin: Option<crate::simple_type::Int32Value>,
  /// Right Margin
  #[sdk(attr(qname = ":marR"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub right_margin: Option<crate::simple_type::Int32Value>,
  /// Level
  #[sdk(attr(qname = ":lvl"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "8",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub level: Option<crate::simple_type::Int32Value>,
  /// Indent
  #[sdk(attr(qname = ":indent"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-51206400",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub indent: Option<crate::simple_type::Int32Value>,
  /// Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub alignment:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextAlignmentTypeValues>,
  /// Default Tab Size
  #[sdk(attr(qname = ":defTabSz"))]
  pub default_tab_size: Option<crate::simple_type::Int32Value>,
  /// Right To Left
  #[sdk(attr(qname = ":rtl"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// East Asian Line Break
  #[sdk(attr(qname = ":eaLnBrk"))]
  pub east_asian_line_break: Option<crate::simple_type::BooleanValue>,
  /// Font Alignment
  #[sdk(attr(qname = ":fontAlgn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub font_alignment:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextFontAlignmentValues>,
  /// Latin Line Break
  #[sdk(attr(qname = ":latinLnBrk"))]
  pub latin_line_break: Option<crate::simple_type::BooleanValue>,
  /// Hanging Punctuation
  #[sdk(attr(qname = ":hangingPunct"))]
  pub height: Option<crate::simple_type::BooleanValue>,
  /// Line Spacing
  #[sdk(child(qname = "a:CT_TextSpacing/a:lnSpc"))]
  pub line_spacing: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LineSpacing>,
  >,
  /// Space Before
  #[sdk(child(qname = "a:CT_TextSpacing/a:spcBef"))]
  pub space_before: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SpaceBefore>,
  >,
  /// Space After
  #[sdk(child(qname = "a:CT_TextSpacing/a:spcAft"))]
  pub space_after: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SpaceAfter>,
  >,
  #[sdk(choice(
    qname = "a:CT_TextBulletColorFollowText/a:buClrTx",
    qname = "a:CT_Color/a:buClr"
  ))]
  pub text_paragraph_properties_type_choice1: Option<TextParagraphPropertiesTypeChoice>,
  #[sdk(choice(
    qname = "a:CT_TextBulletSizeFollowText/a:buSzTx",
    qname = "a:CT_TextBulletSizePercent/a:buSzPct",
    qname = "a:CT_TextBulletSizePoint/a:buSzPts"
  ))]
  pub text_paragraph_properties_type_choice2: Option<TextParagraphPropertiesTypeChoice2>,
  #[sdk(choice(
    qname = "a:CT_TextBulletTypefaceFollowText/a:buFontTx",
    qname = "a:CT_TextFont/a:buFont"
  ))]
  pub text_paragraph_properties_type_choice3: Option<TextParagraphPropertiesTypeChoice3>,
  #[sdk(choice(
    qname = "a:CT_TextNoBullet/a:buNone",
    qname = "a:CT_TextAutonumberBullet/a:buAutoNum",
    qname = "a:CT_TextCharBullet/a:buChar",
    qname = "a:CT_TextBlipBullet/a:buBlip"
  ))]
  pub text_paragraph_properties_type_choice4: Option<TextParagraphPropertiesTypeChoice4>,
  /// Tab List.
  #[sdk(child(qname = "a:CT_TextTabStopList/a:tabLst"))]
  pub a_tab_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TabStopList>,
  /// Default Text Run Properties.
  #[sdk(child(qname = "a:CT_TextCharacterProperties/a:defRPr"))]
  pub a_def_r_pr: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::DefaultRunProperties,
    >,
  >,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Defines the TextBodyProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "a:CT_TextBodyProperties/oac:bodyPr")]
pub struct TextBodyProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Rotation
  #[sdk(attr(qname = ":rot"))]
  pub rotation: Option<crate::simple_type::Int32Value>,
  /// Paragraph Spacing
  #[sdk(attr(qname = ":spcFirstLastPara"))]
  pub use_paragraph_spacing: Option<crate::simple_type::BooleanValue>,
  /// Text Vertical Overflow
  #[sdk(attr(qname = ":vertOverflow"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub vertical_overflow: Option<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextVerticalOverflowValues,
  >,
  /// Text Horizontal Overflow
  #[sdk(attr(qname = ":horzOverflow"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub horizontal_overflow: Option<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextHorizontalOverflowValues,
  >,
  /// Vertical Text
  #[sdk(attr(qname = ":vert"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub vertical:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextVerticalValues>,
  /// Text Wrapping Type
  #[sdk(attr(qname = ":wrap"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub wrap:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextWrappingValues>,
  /// Left Inset
  #[sdk(attr(qname = ":lIns"))]
  pub left_inset: Option<crate::simple_type::Int32Value>,
  /// Top Inset
  #[sdk(attr(qname = ":tIns"))]
  pub top_inset: Option<crate::simple_type::Int32Value>,
  /// Right Inset
  #[sdk(attr(qname = ":rIns"))]
  pub right_inset: Option<crate::simple_type::Int32Value>,
  /// Bottom Inset
  #[sdk(attr(qname = ":bIns"))]
  pub bottom_inset: Option<crate::simple_type::Int32Value>,
  /// Number of Columns
  #[sdk(attr(qname = ":numCol"))]
  #[sdk(number_range(
    source = 0u32,
    min = "1",
    max = "16",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub column_count: Option<crate::simple_type::Int32Value>,
  /// Space Between Columns
  #[sdk(attr(qname = ":spcCol"))]
  #[sdk(number_range(source = 0u32, min = "0", min_inclusive = true, max_inclusive = false))]
  pub column_spacing: Option<crate::simple_type::Int32Value>,
  /// Columns Right-To-Left
  #[sdk(attr(qname = ":rtlCol"))]
  pub right_to_left_columns: Option<crate::simple_type::BooleanValue>,
  /// From WordArt
  #[sdk(attr(qname = ":fromWordArt"))]
  pub from_word_art: Option<crate::simple_type::BooleanValue>,
  /// Anchor
  #[sdk(attr(qname = ":anchor"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub anchor:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextAnchoringTypeValues>,
  /// Anchor Center
  #[sdk(attr(qname = ":anchorCtr"))]
  pub anchor_center: Option<crate::simple_type::BooleanValue>,
  /// Force Anti-Alias
  #[sdk(attr(qname = ":forceAA"))]
  pub force_anti_alias: Option<crate::simple_type::BooleanValue>,
  /// Text Upright
  #[sdk(attr(qname = ":upright"))]
  pub up_right: Option<crate::simple_type::BooleanValue>,
  /// Compatible Line Spacing
  #[sdk(attr(qname = ":compatLnSpc"))]
  pub compatible_line_spacing: Option<crate::simple_type::BooleanValue>,
  /// Preset Text Shape
  #[sdk(child(qname = "a:CT_PresetTextShape/a:prstTxWarp"))]
  pub preset_text_warp: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetTextWarp>,
  >,
  #[sdk(choice(
    qname = "a:CT_TextNoAutofit/a:noAutofit",
    qname = "a:CT_TextNormalAutofit/a:normAutofit",
    qname = "a:CT_TextShapeAutofit/a:spAutoFit"
  ))]
  pub text_body_properties_choice1: Option<TextBodyPropertiesChoice>,
  /// 3D Scene Properties.
  #[sdk(child(qname = "a:CT_Scene3D/a:scene3d"))]
  pub a_scene3d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Scene3DType>,
  >,
  #[sdk(choice(qname = "a:CT_Shape3D/a:sp3d", qname = "a:CT_FlatText/a:flatTx"))]
  pub text_body_properties_choice2: Option<TextBodyPropertiesChoice2>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Defines the ModifyNonVisualDrawingShapeProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "oac:CT_ModifyNonVisualDrawingShapeProps/oac:cNvSpPr"
)]
pub struct ModifyNonVisualDrawingShapeProps {
  /// noGrp
  #[sdk(attr(office2016, qname = ":noGrp"))]
  pub no_grp: Option<crate::simple_type::BooleanValue>,
  /// noSelect
  #[sdk(attr(office2016, qname = ":noSelect"))]
  pub no_select: Option<crate::simple_type::BooleanValue>,
  /// noRot
  #[sdk(attr(office2016, qname = ":noRot"))]
  pub no_rot: Option<crate::simple_type::BooleanValue>,
  /// noChangeAspect
  #[sdk(attr(office2016, qname = ":noChangeAspect"))]
  pub no_change_aspect: Option<crate::simple_type::BooleanValue>,
  /// noMove
  #[sdk(attr(office2016, qname = ":noMove"))]
  pub no_move: Option<crate::simple_type::BooleanValue>,
  /// noResize
  #[sdk(attr(office2016, qname = ":noResize"))]
  pub no_resize: Option<crate::simple_type::BooleanValue>,
  /// noEditPoints
  #[sdk(attr(office2016, qname = ":noEditPoints"))]
  pub no_edit_points: Option<crate::simple_type::BooleanValue>,
  /// noAdjustHandles
  #[sdk(attr(office2016, qname = ":noAdjustHandles"))]
  pub no_adjust_handles: Option<crate::simple_type::BooleanValue>,
  /// noChangeArrowheads
  #[sdk(attr(office2016, qname = ":noChangeArrowheads"))]
  pub no_change_arrowheads: Option<crate::simple_type::BooleanValue>,
  /// noChangeShapeType
  #[sdk(attr(office2016, qname = ":noChangeShapeType"))]
  pub no_change_shape_type: Option<crate::simple_type::BooleanValue>,
  /// noTextEdit
  #[sdk(attr(office2016, qname = ":noTextEdit"))]
  pub no_text_edit: Option<crate::simple_type::BooleanValue>,
  /// txBox
  #[sdk(attr(office2016, qname = ":txBox"))]
  pub tx_box: Option<crate::simple_type::BooleanValue>,
}
/// Defines the ShapePropsMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_ShapePropsMonikerList/oac:spMkLst")]
pub struct ShapePropsMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the ShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "a:CT_ShapeProperties/oac:spPr")]
pub struct ShapeProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Black and White Mode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues>,
  /// 2D Transform for Individual Objects
  #[sdk(child(qname = "a:CT_Transform2D/a:xfrm"))]
  pub transform2_d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Transform2D>,
  >,
  #[sdk(choice(
    qname = "a:CT_CustomGeometry2D/a:custGeom",
    qname = "a:CT_PresetGeometry2D/a:prstGeom"
  ))]
  pub shape_properties_choice1: Option<ShapePropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_BlipFillProperties/a:blipFill",
    qname = "a:CT_PatternFillProperties/a:pattFill",
    qname = "a:CT_GroupFillProperties/a:grpFill"
  ))]
  pub shape_properties_choice2: Option<ShapePropertiesChoice2>,
  /// Defines the Outline Class.
  #[sdk(child(qname = "a:CT_LineProperties/a:ln"))]
  pub a_ln: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Outline>,
  >,
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub shape_properties_choice3: Option<ShapePropertiesChoice3>,
  /// 3D Scene Properties.
  #[sdk(child(qname = "a:CT_Scene3D/a:scene3d"))]
  pub a_scene3d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Scene3DType>,
  >,
  /// Apply 3D shape properties.
  #[sdk(child(qname = "a:CT_Shape3D/a:sp3d"))]
  pub a_sp3d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Shape3DType>,
  >,
  /// Defines the ShapePropertiesExtensionList Class.
  #[sdk(child(qname = "a:CT_ShapePropertiesExtensionList/a:extLst"))]
  pub a_ext_lst: Option<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ShapePropertiesExtensionList,
  >,
}
/// Defines the ResetShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_ResetShapeProperties/oac:spPr")]
pub struct ResetShapeProperties {
  /// Defines the XfrmEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:xfrm"))]
  pub xfrm_empty: Option<()>,
  /// Defines the GeomEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:geom"))]
  pub geom_empty: Option<()>,
  /// Defines the FillEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:fill"))]
  pub fill_empty: Option<()>,
  /// Defines the LnEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:ln"))]
  pub ln_empty: Option<()>,
  /// Defines the EffectEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:effect"))]
  pub effect_empty: Option<()>,
  /// Defines the Scene3dEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:scene3d"))]
  pub scene3d_empty: Option<()>,
  /// Defines the Sp3dEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:sp3d"))]
  pub sp3d_empty: Option<()>,
  /// Defines the ExtLstEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:extLst"))]
  pub ext_lst_empty: Option<()>,
  /// Defines the BwModeEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:bwMode"))]
  pub bw_mode_empty: Option<()>,
}
/// Defines the LnRefStyleMatrixReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "a:CT_StyleMatrixReference/oac:lnRef")]
pub struct LnRefStyleMatrixReference {
  /// Style Matrix Index
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub ln_ref_style_matrix_reference_choice: Option<LnRefStyleMatrixReferenceChoice>,
}
/// Defines the FillRefStyleMatrixReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "a:CT_StyleMatrixReference/oac:fillRef")]
pub struct FillRefStyleMatrixReference {
  /// Style Matrix Index
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub fill_ref_style_matrix_reference_choice: Option<FillRefStyleMatrixReferenceChoice>,
}
/// Defines the EffectRefStyleMatrixReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "a:CT_StyleMatrixReference/oac:effectRef")]
pub struct EffectRefStyleMatrixReference {
  /// Style Matrix Index
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub effect_ref_style_matrix_reference_choice: Option<EffectRefStyleMatrixReferenceChoice>,
}
/// Defines the FontReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "a:CT_FontReference/oac:fontRef")]
pub struct FontReference {
  /// Identifier
  #[sdk(attr(qname = ":idx"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub index:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FontCollectionIndexValues,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub font_reference_choice: Option<FontReferenceChoice>,
}
/// Defines the ModifyShapeStyleProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_ModifyShapeStyleProps/oac:style")]
pub struct ModifyShapeStyleProps {
  /// Defines the LnRefStyleMatrixReference Class.
  #[sdk(child(office2016, qname = "a:CT_StyleMatrixReference/oac:lnRef"))]
  pub ln_ref_style_matrix_reference: Option<std::boxed::Box<LnRefStyleMatrixReference>>,
  /// Defines the FillRefStyleMatrixReference Class.
  #[sdk(child(office2016, qname = "a:CT_StyleMatrixReference/oac:fillRef"))]
  pub fill_ref_style_matrix_reference: Option<std::boxed::Box<FillRefStyleMatrixReference>>,
  /// Defines the EffectRefStyleMatrixReference Class.
  #[sdk(child(office2016, qname = "a:CT_StyleMatrixReference/oac:effectRef"))]
  pub effect_ref_style_matrix_reference: Option<std::boxed::Box<EffectRefStyleMatrixReference>>,
  /// Defines the FontReference Class.
  #[sdk(child(office2016, qname = "a:CT_FontReference/oac:fontRef"))]
  pub font_reference: Option<std::boxed::Box<FontReference>>,
}
/// Defines the ResetXsdboolean Class.
pub type ResetXsdboolean = crate::simple_type::BooleanValue;
/// Defines the UseBoundsXsdboolean Class.
pub type UseBoundsXsdboolean = crate::simple_type::BooleanValue;
/// Defines the BlipFillProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "a:CT_BlipFillProperties/oac:blipFill")]
pub struct BlipFillProperties {
  /// DPI Setting
  #[sdk(attr(qname = ":dpi"))]
  pub dpi: Option<crate::simple_type::UInt32Value>,
  /// Rotate With Shape
  #[sdk(attr(qname = ":rotWithShape"))]
  pub rotate_with_shape: Option<crate::simple_type::BooleanValue>,
  /// Defines the Blip Class.
  #[sdk(child(qname = "a:CT_Blip/a:blip"))]
  pub blip:
    Option<std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Blip>>,
  /// Source Rectangle
  #[sdk(child(qname = "a:CT_RelativeRect/a:srcRect"))]
  pub source_rectangle:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SourceRectangle>,
  #[sdk(choice(
    qname = "a:CT_TileInfoProperties/a:tile",
    qname = "a:CT_StretchInfoProperties/a:stretch"
  ))]
  pub blip_fill_properties_choice: Option<BlipFillPropertiesChoice>,
}
/// Defines the FillRectRelativeRectProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_RelativeRectProps/oac:fillRect")]
pub struct FillRectRelativeRectProps {
  /// l
  #[sdk(attr(office2016, qname = ":l"))]
  pub l: Option<crate::simple_type::Int32Value>,
  /// t
  #[sdk(attr(office2016, qname = ":t"))]
  pub t: Option<crate::simple_type::Int32Value>,
  /// r
  #[sdk(attr(office2016, qname = ":r"))]
  pub r: Option<crate::simple_type::Int32Value>,
  /// b
  #[sdk(attr(office2016, qname = ":b"))]
  pub b: Option<crate::simple_type::Int32Value>,
}
/// Defines the SrcRectRelativeRectProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_RelativeRectProps/oac:srcRect")]
pub struct SrcRectRelativeRectProps {
  /// l
  #[sdk(attr(office2016, qname = ":l"))]
  pub l: Option<crate::simple_type::Int32Value>,
  /// t
  #[sdk(attr(office2016, qname = ":t"))]
  pub t: Option<crate::simple_type::Int32Value>,
  /// r
  #[sdk(attr(office2016, qname = ":r"))]
  pub r: Option<crate::simple_type::Int32Value>,
  /// b
  #[sdk(attr(office2016, qname = ":b"))]
  pub b: Option<crate::simple_type::Int32Value>,
}
/// Defines the ResetBlipFillProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_ResetBlipFillProperties/oac:blipFill")]
pub struct ResetBlipFillProperties {
  /// Defines the SrcRectEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:srcRect"))]
  pub src_rect_empty: Option<()>,
  /// Defines the FillModeEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:fillMode"))]
  pub fill_mode_empty: Option<()>,
  /// Defines the DpiEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:dpi"))]
  pub dpi_empty: Option<()>,
  /// Defines the RotWithShapeEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:rotWithShape"))]
  pub rot_with_shape_empty: Option<()>,
}
/// Defines the ModifyNonVisualGroupDrawingShapeProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "oac:CT_ModifyNonVisualGroupDrawingShapeProps/oac:cNvGrpSpPr"
)]
pub struct ModifyNonVisualGroupDrawingShapeProps {
  /// noGrp
  #[sdk(attr(office2016, qname = ":noGrp"))]
  pub no_grp: Option<crate::simple_type::BooleanValue>,
  /// noUngrp
  #[sdk(attr(office2016, qname = ":noUngrp"))]
  pub no_ungrp: Option<crate::simple_type::BooleanValue>,
  /// noSelect
  #[sdk(attr(office2016, qname = ":noSelect"))]
  pub no_select: Option<crate::simple_type::BooleanValue>,
  /// noRot
  #[sdk(attr(office2016, qname = ":noRot"))]
  pub no_rot: Option<crate::simple_type::BooleanValue>,
  /// noChangeAspect
  #[sdk(attr(office2016, qname = ":noChangeAspect"))]
  pub no_change_aspect: Option<crate::simple_type::BooleanValue>,
  /// noMove
  #[sdk(attr(office2016, qname = ":noMove"))]
  pub no_move: Option<crate::simple_type::BooleanValue>,
  /// noResize
  #[sdk(attr(office2016, qname = ":noResize"))]
  pub no_resize: Option<crate::simple_type::BooleanValue>,
}
/// Defines the GroupShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "a:CT_GroupShapeProperties/oac:grpSpPr")]
pub struct GroupShapeProperties {
  /// Black and White Mode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues>,
  /// 2D Transform for Grouped Objects
  #[sdk(child(qname = "a:CT_GroupTransform2D/a:xfrm"))]
  pub transform_group: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TransformGroup>,
  >,
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_BlipFillProperties/a:blipFill",
    qname = "a:CT_PatternFillProperties/a:pattFill",
    qname = "a:CT_GroupFillProperties/a:grpFill"
  ))]
  pub group_shape_properties_choice1: Option<GroupShapePropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub group_shape_properties_choice2: Option<GroupShapePropertiesChoice2>,
  /// 3D Scene Properties.
  #[sdk(child(qname = "a:CT_Scene3D/a:scene3d"))]
  pub a_scene3d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Scene3DType>,
  >,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Defines the ResetGroupShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_ResetGroupShapeProperties/oac:grpSpPr")]
pub struct ResetGroupShapeProperties {
  /// Defines the XfrmEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:xfrm"))]
  pub xfrm_empty: Option<()>,
  /// Defines the FillEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:fill"))]
  pub fill_empty: Option<()>,
  /// Defines the EffectEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:effect"))]
  pub effect_empty: Option<()>,
  /// Defines the Scene3dEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:scene3d"))]
  pub scene3d_empty: Option<()>,
  /// Defines the ExtLstEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:extLst"))]
  pub ext_lst_empty: Option<()>,
  /// Defines the BwModeEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:bwMode"))]
  pub bw_mode_empty: Option<()>,
}
/// Defines the NonVisualDrawingProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "a:CT_NonVisualDrawingProps/oac:cNvPr")]
pub struct NonVisualDrawingProps {
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
/// Defines the NonVisualGroupDrawingShapeProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "a:CT_NonVisualGroupDrawingShapeProps/oac:cNvGrpSpPr"
)]
pub struct NonVisualGroupDrawingShapeProps {
    /// Defines the GroupShapeLocks Class.
    #[sdk(child(qname = "a:CT_GroupLocking/a:grpSpLocks"))]
    pub group_shape_locks: Option<
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GroupShapeLocks,
        >,
    >,
    /// Defines the NonVisualGroupDrawingShapePropsExtensionList Class.
    #[sdk(child(qname = "a:CT_NonVisualGroupDrawingShapePropsExtensionList/a:extLst"))]
    pub non_visual_group_drawing_shape_props_extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NonVisualGroupDrawingShapePropsExtensionList,
    >,
}
/// Defines the ModifyNonVisualGraphicFrameProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "oac:CT_ModifyNonVisualGraphicFrameProps/oac:cNvGraphicFramePr"
)]
pub struct ModifyNonVisualGraphicFrameProps {
  /// noGrp
  #[sdk(attr(office2016, qname = ":noGrp"))]
  pub no_grp: Option<crate::simple_type::BooleanValue>,
  /// noDrilldown
  #[sdk(attr(office2016, qname = ":noDrilldown"))]
  pub no_drilldown: Option<crate::simple_type::BooleanValue>,
  /// noSelect
  #[sdk(attr(office2016, qname = ":noSelect"))]
  pub no_select: Option<crate::simple_type::BooleanValue>,
  /// noChangeAspect
  #[sdk(attr(office2016, qname = ":noChangeAspect"))]
  pub no_change_aspect: Option<crate::simple_type::BooleanValue>,
  /// noMove
  #[sdk(attr(office2016, qname = ":noMove"))]
  pub no_move: Option<crate::simple_type::BooleanValue>,
  /// noResize
  #[sdk(attr(office2016, qname = ":noResize"))]
  pub no_resize: Option<crate::simple_type::BooleanValue>,
}
/// Defines the StCxnConnection Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "a:CT_Connection/oac:stCxn")]
pub struct StCxnConnection {
  /// Identifier
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// Index
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
}
/// Defines the EndCxnConnection Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "a:CT_Connection/oac:endCxn")]
pub struct EndCxnConnection {
  /// Identifier
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// Index
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
}
/// Defines the ModifyNonVisualConnectorProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "oac:CT_ModifyNonVisualConnectorProps/oac:cNvCxnSpPr"
)]
pub struct ModifyNonVisualConnectorProps {
  /// noGrp
  #[sdk(attr(office2016, qname = ":noGrp"))]
  pub no_grp: Option<crate::simple_type::BooleanValue>,
  /// noSelect
  #[sdk(attr(office2016, qname = ":noSelect"))]
  pub no_select: Option<crate::simple_type::BooleanValue>,
  /// noRot
  #[sdk(attr(office2016, qname = ":noRot"))]
  pub no_rot: Option<crate::simple_type::BooleanValue>,
  /// noChangeAspect
  #[sdk(attr(office2016, qname = ":noChangeAspect"))]
  pub no_change_aspect: Option<crate::simple_type::BooleanValue>,
  /// noMove
  #[sdk(attr(office2016, qname = ":noMove"))]
  pub no_move: Option<crate::simple_type::BooleanValue>,
  /// noResize
  #[sdk(attr(office2016, qname = ":noResize"))]
  pub no_resize: Option<crate::simple_type::BooleanValue>,
  /// noEditPoints
  #[sdk(attr(office2016, qname = ":noEditPoints"))]
  pub no_edit_points: Option<crate::simple_type::BooleanValue>,
  /// noAdjustHandles
  #[sdk(attr(office2016, qname = ":noAdjustHandles"))]
  pub no_adjust_handles: Option<crate::simple_type::BooleanValue>,
  /// noChangeArrowheads
  #[sdk(attr(office2016, qname = ":noChangeArrowheads"))]
  pub no_change_arrowheads: Option<crate::simple_type::BooleanValue>,
  /// noChangeShapeType
  #[sdk(attr(office2016, qname = ":noChangeShapeType"))]
  pub no_change_shape_type: Option<crate::simple_type::BooleanValue>,
  /// Defines the StCxnConnection Class.
  #[sdk(child(office2016, qname = "a:CT_Connection/oac:stCxn"))]
  pub st_cxn_connection: Option<StCxnConnection>,
  /// Defines the EndCxnConnection Class.
  #[sdk(child(office2016, qname = "a:CT_Connection/oac:endCxn"))]
  pub end_cxn_connection: Option<EndCxnConnection>,
}
/// Defines the ResetNonVisualConnectorProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "oac:CT_ResetNonVisualConnectorProps/oac:cNvCxnSpPr"
)]
pub struct ResetNonVisualConnectorProps {
  /// Defines the StCxnEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:stCxn"))]
  pub st_cxn_empty: Option<()>,
  /// Defines the EndCxnEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:endCxn"))]
  pub end_cxn_empty: Option<()>,
  /// Defines the NoGrpEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:noGrp"))]
  pub no_grp_empty: Option<()>,
  /// Defines the NoSelectEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:noSelect"))]
  pub no_select_empty: Option<()>,
  /// Defines the NoRotEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:noRot"))]
  pub no_rot_empty: Option<()>,
  /// Defines the NoChangeAspectEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:noChangeAspect"))]
  pub no_change_aspect_empty: Option<()>,
  /// Defines the NoMoveEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:noMove"))]
  pub no_move_empty: Option<()>,
  /// Defines the NoResizeEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:noResize"))]
  pub no_resize_empty: Option<()>,
  /// Defines the NoEditPointsEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:noEditPoints"))]
  pub no_edit_points_empty: Option<()>,
  /// Defines the NoAdjustHandlesEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:noAdjustHandles"))]
  pub no_adjust_handles_empty: Option<()>,
  /// Defines the NoChangeArrowheadsEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:noChangeArrowheads"))]
  pub no_change_arrowheads_empty: Option<()>,
  /// Defines the NoChangeShapeTypeEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:noChangeShapeType"))]
  pub no_change_shape_type_empty: Option<()>,
}
/// Defines the CompressPictureProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_CompressPictureProps/oac:compressPicPr")]
pub struct CompressPictureProps {
  /// removeCrop
  #[sdk(attr(office2016, qname = ":removeCrop"))]
  pub remove_crop: Option<crate::simple_type::BooleanValue>,
  /// useLocalDpi
  #[sdk(attr(office2016, qname = ":useLocalDpi"))]
  pub use_local_dpi: Option<crate::simple_type::BooleanValue>,
  /// cstate
  #[sdk(attr(office2016, qname = ":cstate"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub cstate:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipCompressionValues>,
}
/// Defines the ModifyNonVisualPictureProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_ModifyNonVisualPictureProps/oac:cNvPicPr")]
pub struct ModifyNonVisualPictureProps {
  /// noGrp
  #[sdk(attr(office2016, qname = ":noGrp"))]
  pub no_grp: Option<crate::simple_type::BooleanValue>,
  /// noSelect
  #[sdk(attr(office2016, qname = ":noSelect"))]
  pub no_select: Option<crate::simple_type::BooleanValue>,
  /// noRot
  #[sdk(attr(office2016, qname = ":noRot"))]
  pub no_rot: Option<crate::simple_type::BooleanValue>,
  /// noChangeAspect
  #[sdk(attr(office2016, qname = ":noChangeAspect"))]
  pub no_change_aspect: Option<crate::simple_type::BooleanValue>,
  /// noMove
  #[sdk(attr(office2016, qname = ":noMove"))]
  pub no_move: Option<crate::simple_type::BooleanValue>,
  /// noResize
  #[sdk(attr(office2016, qname = ":noResize"))]
  pub no_resize: Option<crate::simple_type::BooleanValue>,
  /// noEditPoints
  #[sdk(attr(office2016, qname = ":noEditPoints"))]
  pub no_edit_points: Option<crate::simple_type::BooleanValue>,
  /// noAdjustHandles
  #[sdk(attr(office2016, qname = ":noAdjustHandles"))]
  pub no_adjust_handles: Option<crate::simple_type::BooleanValue>,
  /// noChangeArrowheads
  #[sdk(attr(office2016, qname = ":noChangeArrowheads"))]
  pub no_change_arrowheads: Option<crate::simple_type::BooleanValue>,
  /// noChangeShapeType
  #[sdk(attr(office2016, qname = ":noChangeShapeType"))]
  pub no_change_shape_type: Option<crate::simple_type::BooleanValue>,
  /// noCrop
  #[sdk(attr(office2016, qname = ":noCrop"))]
  pub no_crop: Option<crate::simple_type::BooleanValue>,
  /// preferRelativeResize
  #[sdk(attr(office2016, qname = ":preferRelativeResize"))]
  pub prefer_relative_resize: Option<crate::simple_type::BooleanValue>,
}
/// Defines the ResetNonVisualPictureProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_ResetNonVisualPictureProps/oac:cNvPicPr")]
pub struct ResetNonVisualPictureProps {
  /// Defines the LfPrEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:lfPr"))]
  pub lf_pr_empty: Option<()>,
}
/// Defines the BoundRect Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "a:CT_BoundRect/oac:bounds")]
pub struct BoundRect {
  /// l
  #[sdk(attr(qname = ":l"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub l: crate::simple_type::Int64Value,
  /// t
  #[sdk(attr(qname = ":t"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub t: crate::simple_type::Int64Value,
  /// r
  #[sdk(attr(qname = ":r"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub r: crate::simple_type::Int64Value,
  /// b
  #[sdk(attr(qname = ":b"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub b: crate::simple_type::Int64Value,
}
/// Defines the SVGBlipMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_SVGBlipMonikerList/oac:svgBlipMkLst")]
pub struct SvgBlipMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the LinePropertiesType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "a:CT_LineProperties/oac:lineProps")]
pub struct LinePropertiesType {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// line width
  #[sdk(attr(qname = ":w"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "20116800",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub width: Option<crate::simple_type::Int32Value>,
  /// line cap
  #[sdk(attr(qname = ":cap"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub cap_type:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LineCapValues>,
  /// compound line type
  #[sdk(attr(qname = ":cmpd"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub compound_line_type:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::CompoundLineValues>,
  /// pen alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub alignment:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PenAlignmentValues>,
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_PatternFillProperties/a:pattFill"
  ))]
  pub line_properties_type_choice1: Option<LinePropertiesTypeChoice>,
  #[sdk(choice(
    qname = "a:CT_PresetLineDashProperties/a:prstDash",
    qname = "a:CT_DashStopList/a:custDash"
  ))]
  pub line_properties_type_choice2: Option<LinePropertiesTypeChoice2>,
  #[sdk(choice(
    qname = "a:CT_LineJoinRound/a:round",
    qname = "a:CT_LineJoinBevel/a:bevel",
    qname = "a:CT_LineJoinMiterProperties/a:miter"
  ))]
  pub line_properties_type_choice3: Option<LinePropertiesTypeChoice3>,
  /// default head line end style is none.
  #[sdk(child(qname = "a:CT_LineEndProperties/a:headEnd"))]
  pub a_head_end: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HeadEnd>,
  /// default tail line end style is none.
  #[sdk(child(qname = "a:CT_LineEndProperties/a:tailEnd"))]
  pub a_tail_end: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TailEnd>,
  /// Future extensions..
  #[sdk(child(qname = "a:CT_LinePropertiesExtensionList/a:extLst"))]
  pub a_ext_lst: Option<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LinePropertiesExtensionList,
  >,
}
/// Defines the ModifyNonVisualInkProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_ModifyNonVisualInkProps/oac:cNvInkPr")]
pub struct ModifyNonVisualInkProps {
  /// noGrp
  #[sdk(attr(office2016, qname = ":noGrp"))]
  pub no_grp: Option<crate::simple_type::BooleanValue>,
  /// noSelect
  #[sdk(attr(office2016, qname = ":noSelect"))]
  pub no_select: Option<crate::simple_type::BooleanValue>,
  /// noRot
  #[sdk(attr(office2016, qname = ":noRot"))]
  pub no_rot: Option<crate::simple_type::BooleanValue>,
  /// noChangeAspect
  #[sdk(attr(office2016, qname = ":noChangeAspect"))]
  pub no_change_aspect: Option<crate::simple_type::BooleanValue>,
  /// noMove
  #[sdk(attr(office2016, qname = ":noMove"))]
  pub no_move: Option<crate::simple_type::BooleanValue>,
  /// noResize
  #[sdk(attr(office2016, qname = ":noResize"))]
  pub no_resize: Option<crate::simple_type::BooleanValue>,
  /// noEditPoints
  #[sdk(attr(office2016, qname = ":noEditPoints"))]
  pub no_edit_points: Option<crate::simple_type::BooleanValue>,
  /// noAdjustHandles
  #[sdk(attr(office2016, qname = ":noAdjustHandles"))]
  pub no_adjust_handles: Option<crate::simple_type::BooleanValue>,
  /// noChangeArrowheads
  #[sdk(attr(office2016, qname = ":noChangeArrowheads"))]
  pub no_change_arrowheads: Option<crate::simple_type::BooleanValue>,
  /// noChangeShapeType
  #[sdk(attr(office2016, qname = ":noChangeShapeType"))]
  pub no_change_shape_type: Option<crate::simple_type::BooleanValue>,
  /// isComment
  #[sdk(attr(office2016, qname = ":isComment"))]
  pub is_comment: Option<crate::simple_type::BooleanValue>,
}
/// Defines the HlinkClickHyperlinkProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_HyperlinkProps/oac:hlinkClick")]
pub struct HlinkClickHyperlinkProps {
  /// source
  #[sdk(attr(office2016, qname = ":source"))]
  pub source: Option<crate::simple_type::StringValue>,
  /// action
  #[sdk(attr(office2016, qname = ":action"))]
  pub action: Option<crate::simple_type::StringValue>,
  /// tgtFrame
  #[sdk(attr(office2016, qname = ":tgtFrame"))]
  pub tgt_frame: Option<crate::simple_type::StringValue>,
  /// tooltip
  #[sdk(attr(office2016, qname = ":tooltip"))]
  pub tooltip: Option<crate::simple_type::StringValue>,
  /// highlightClick
  #[sdk(attr(office2016, qname = ":highlightClick"))]
  pub highlight_click: Option<crate::simple_type::BooleanValue>,
  /// endSnd
  #[sdk(attr(office2016, qname = ":endSnd"))]
  pub end_snd: Option<crate::simple_type::BooleanValue>,
  /// sndName
  #[sdk(attr(office2016, qname = ":sndName"))]
  pub snd_name: Option<crate::simple_type::StringValue>,
  /// Defines the SndDataImgData Class.
  #[sdk(text_child(office2016, qname = "oac:CT_ImgData/oac:sndData"))]
  pub snd_data_img_data: Option<crate::simple_type::Base64BinaryValue>,
}
/// Defines the HlinkHoverHyperlinkProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_HyperlinkProps/oac:hlinkHover")]
pub struct HlinkHoverHyperlinkProps {
  /// source
  #[sdk(attr(office2016, qname = ":source"))]
  pub source: Option<crate::simple_type::StringValue>,
  /// action
  #[sdk(attr(office2016, qname = ":action"))]
  pub action: Option<crate::simple_type::StringValue>,
  /// tgtFrame
  #[sdk(attr(office2016, qname = ":tgtFrame"))]
  pub tgt_frame: Option<crate::simple_type::StringValue>,
  /// tooltip
  #[sdk(attr(office2016, qname = ":tooltip"))]
  pub tooltip: Option<crate::simple_type::StringValue>,
  /// highlightClick
  #[sdk(attr(office2016, qname = ":highlightClick"))]
  pub highlight_click: Option<crate::simple_type::BooleanValue>,
  /// endSnd
  #[sdk(attr(office2016, qname = ":endSnd"))]
  pub end_snd: Option<crate::simple_type::BooleanValue>,
  /// sndName
  #[sdk(attr(office2016, qname = ":sndName"))]
  pub snd_name: Option<crate::simple_type::StringValue>,
  /// Defines the SndDataImgData Class.
  #[sdk(text_child(office2016, qname = "oac:CT_ImgData/oac:sndData"))]
  pub snd_data_img_data: Option<crate::simple_type::Base64BinaryValue>,
}
/// Defines the ModifyHyperlinkProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_ModifyHyperlinkProps/oac:hlink")]
pub struct ModifyHyperlinkProps {
  /// Defines the HlinkClickHyperlinkProps Class.
  #[sdk(child(office2016, qname = "oac:CT_HyperlinkProps/oac:hlinkClick"))]
  pub hlink_click_hyperlink_props: Option<HlinkClickHyperlinkProps>,
  /// Defines the HlinkHoverHyperlinkProps Class.
  #[sdk(child(office2016, qname = "oac:CT_HyperlinkProps/oac:hlinkHover"))]
  pub hlink_hover_hyperlink_props: Option<HlinkHoverHyperlinkProps>,
}
/// Defines the ResetHyperlinkProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_ResetHyperlinkProps/oac:hlink")]
pub struct ResetHyperlinkProps {
  /// Defines the HlinkClickEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:hlinkClick"))]
  pub hlink_click_empty: Option<()>,
  /// Defines the HlinkHoverEmpty Class.
  #[sdk(empty_child(office2016, qname = "oac:CT_Empty/oac:hlinkHover"))]
  pub hlink_hover_empty: Option<()>,
}
/// Defines the TextCharRangeContext Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "oac:CT_TextCharRangeContext/oac:context")]
pub struct TextCharRangeContext {
  /// len
  #[sdk(attr(office2016, qname = ":len"))]
  pub len: Option<crate::simple_type::UInt32Value>,
  /// hash
  #[sdk(attr(office2016, qname = ":hash"))]
  pub hash: crate::simple_type::UInt32Value,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GroupCommandChoice {
  #[sdk(child(office2016, qname = "oac:CT_ShapeMoniker/oac:spMk"))]
  OacSpMk(std::boxed::Box<ShapeMoniker>),
  #[sdk(child(office2016, qname = "oac:CT_GroupShapeMoniker/oac:grpSpMk"))]
  OacGrpSpMk(std::boxed::Box<GroupShapeMoniker>),
  #[sdk(child(office2016, qname = "oac:CT_GraphicFrameMoniker/oac:graphicFrameMk"))]
  OacGraphicFrameMk(std::boxed::Box<GraphicFrameMoniker>),
  #[sdk(child(office2016, qname = "oac:CT_ConnectorMoniker/oac:cxnSpMk"))]
  OacCxnSpMk(std::boxed::Box<ConnectorMoniker>),
  #[sdk(child(office2016, qname = "oac:CT_PictureMoniker/oac:picMk"))]
  OacPicMk(std::boxed::Box<PictureMoniker>),
  #[sdk(child(office2016, qname = "oac:CT_InkMoniker/oac:inkMk"))]
  OacInkMk(std::boxed::Box<InkMoniker>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextParagraphPropertiesTypeChoice {
  /// Follow Text.
  #[sdk(empty_child(qname = "a:CT_TextBulletColorFollowText/a:buClrTx"))]
  ABuClrTx,
  /// Color Specified.
  #[sdk(child(qname = "a:CT_Color/a:buClr"))]
  ABuClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BulletColor>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextParagraphPropertiesTypeChoice2 {
  /// Bullet Size Follows Text.
  #[sdk(empty_child(qname = "a:CT_TextBulletSizeFollowText/a:buSzTx"))]
  ABuSzTx,
  /// Bullet Size Percentage.
  #[sdk(child(qname = "a:CT_TextBulletSizePercent/a:buSzPct"))]
  ABuSzPct(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BulletSizePercentage,
    >,
  ),
  /// Bullet Size Points.
  #[sdk(child(qname = "a:CT_TextBulletSizePoint/a:buSzPts"))]
  ABuSzPts(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BulletSizePoints,
    >,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextParagraphPropertiesTypeChoice3 {
  /// Follow text.
  #[sdk(empty_child(qname = "a:CT_TextBulletTypefaceFollowText/a:buFontTx"))]
  ABuFontTx,
  /// Specified.
  #[sdk(child(qname = "a:CT_TextFont/a:buFont"))]
  ABuFont(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BulletFont>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextParagraphPropertiesTypeChoice4 {
  /// No Bullet.
  #[sdk(empty_child(qname = "a:CT_TextNoBullet/a:buNone"))]
  ABuNone,
  /// Auto-Numbered Bullet.
  #[sdk(child(qname = "a:CT_TextAutonumberBullet/a:buAutoNum"))]
  ABuAutoNum(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AutoNumberedBullet,
    >,
  ),
  /// Character Bullet.
  #[sdk(child(qname = "a:CT_TextCharBullet/a:buChar"))]
  ABuChar(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::CharacterBullet,
    >,
  ),
  /// Picture Bullet.
  #[sdk(child(qname = "a:CT_TextBlipBullet/a:buBlip"))]
  ABuBlip(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PictureBullet>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextBodyPropertiesChoice {
  /// No AutoFit.
  #[sdk(empty_child(qname = "a:CT_TextNoAutofit/a:noAutofit"))]
  ANoAutofit,
  /// Normal AutoFit.
  #[sdk(child(qname = "a:CT_TextNormalAutofit/a:normAutofit"))]
  ANormAutofit(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NormalAutoFit>,
  ),
  /// Shape AutoFit.
  #[sdk(child(qname = "a:CT_TextShapeAutofit/a:spAutoFit"))]
  ASpAutoFit(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ShapeAutoFit>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextBodyPropertiesChoice2 {
  /// Apply 3D shape properties.
  #[sdk(child(qname = "a:CT_Shape3D/a:sp3d"))]
  ASp3d(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Shape3DType>,
  ),
  /// No text in 3D scene.
  #[sdk(child(qname = "a:CT_FlatText/a:flatTx"))]
  AFlatTx(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FlatText>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice {
  /// Custom geometry.
  #[sdk(child(qname = "a:CT_CustomGeometry2D/a:custGeom"))]
  ACustGeom(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::CustomGeometry>,
  ),
  /// Preset geometry.
  #[sdk(child(qname = "a:CT_PresetGeometry2D/a:prstGeom"))]
  APrstGeom(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetGeometry>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice2 {
  /// Defines the NoFill Class.
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoFill>),
  /// Defines the SolidFill Class.
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SolidFill>,
  ),
  /// Defines the GradientFill Class.
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GradientFill>,
  ),
  /// Defines the BlipFill Class.
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipFill>,
  ),
  /// Pattern Fill.
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PatternFill>,
  ),
  /// Group Fill.
  #[sdk(empty_child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice3 {
  /// Effect Container.
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectList>,
  ),
  /// Effect Container.
  #[sdk(child(qname = "a:CT_EffectContainer/a:effectDag"))]
  AEffectDag(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectDag>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LnRefStyleMatrixReferenceChoice {
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
pub enum FillRefStyleMatrixReferenceChoice {
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
pub enum EffectRefStyleMatrixReferenceChoice {
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
pub enum FontReferenceChoice {
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
pub enum BlipFillPropertiesChoice {
  #[sdk(child(qname = "a:CT_TileInfoProperties/a:tile"))]
  ATile(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Tile>),
  #[sdk(child(qname = "a:CT_StretchInfoProperties/a:stretch"))]
  AStretch(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Stretch>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GroupShapePropertiesChoice {
  /// Defines the NoFill Class.
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoFill>),
  /// Defines the SolidFill Class.
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SolidFill>,
  ),
  /// Defines the GradientFill Class.
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GradientFill>,
  ),
  /// Defines the BlipFill Class.
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipFill>,
  ),
  /// Pattern Fill.
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PatternFill>,
  ),
  /// Group Fill.
  #[sdk(empty_child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GroupShapePropertiesChoice2 {
  /// Effect Container.
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectList>,
  ),
  /// Effect Container.
  #[sdk(child(qname = "a:CT_EffectContainer/a:effectDag"))]
  AEffectDag(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectDag>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LinePropertiesTypeChoice {
  /// Defines the NoFill Class.
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoFill>),
  /// Defines the SolidFill Class.
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SolidFill>,
  ),
  /// Defines the GradientFill Class.
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GradientFill>,
  ),
  /// Pattern Fill.
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PatternFill>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LinePropertiesTypeChoice2 {
  /// Preset Dash.
  #[sdk(child(qname = "a:CT_PresetLineDashProperties/a:prstDash"))]
  APrstDash(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetDash>,
  ),
  /// Custom Dash.
  #[sdk(child(qname = "a:CT_DashStopList/a:custDash"))]
  ACustDash(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::CustomDash>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LinePropertiesTypeChoice3 {
  /// Round Line Join.
  #[sdk(empty_child(qname = "a:CT_LineJoinRound/a:round"))]
  ARound,
  /// Line Join Bevel.
  #[sdk(empty_child(qname = "a:CT_LineJoinBevel/a:bevel"))]
  ABevel,
  /// Miter Line Join.
  #[sdk(child(qname = "a:CT_LineJoinMiterProperties/a:miter"))]
  AMiter(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Miter>),
}
