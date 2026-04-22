//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum ResourceLinkage {
  #[sdk(rename = "embed")]
  #[default]
  Embed,
  #[sdk(rename = "link")]
  Link,
  #[sdk(rename = "linkAndEmbed")]
  LinkAndEmbed,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:spMk.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_ShapeMoniker/oac:spMk")]
pub struct ShapeMoniker {
  /// id
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// creationId
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :creationId
  #[sdk(attr(qname = ":creationId"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub creation_id: Option<crate::simple_type::StringValue>,
}
/// Defines the GroupShapeMoniker Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:grpSpMk.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_GroupShapeMoniker/oac:grpSpMk")]
pub struct GroupShapeMoniker {
  /// id
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// creationId
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :creationId
  #[sdk(attr(qname = ":creationId"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub creation_id: Option<crate::simple_type::StringValue>,
}
/// Defines the GraphicFrameMoniker Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:graphicFrameMk.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_GraphicFrameMoniker/oac:graphicFrameMk")]
pub struct GraphicFrameMoniker {
  /// id
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// creationId
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :creationId
  #[sdk(attr(qname = ":creationId"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub creation_id: Option<crate::simple_type::StringValue>,
}
/// Defines the ConnectorMoniker Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:cxnSpMk.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_ConnectorMoniker/oac:cxnSpMk")]
pub struct ConnectorMoniker {
  /// id
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// creationId
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :creationId
  #[sdk(attr(qname = ":creationId"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub creation_id: Option<crate::simple_type::StringValue>,
}
/// Defines the PictureMoniker Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:picMk.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_PictureMoniker/oac:picMk")]
pub struct PictureMoniker {
  /// id
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// creationId
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :creationId
  #[sdk(attr(qname = ":creationId"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub creation_id: Option<crate::simple_type::StringValue>,
}
/// Defines the InkMoniker Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:inkMk.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_InkMoniker/oac:inkMk")]
pub struct InkMoniker {
  /// id
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// creationId
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :creationId
  #[sdk(attr(qname = ":creationId"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub creation_id: Option<crate::simple_type::StringValue>,
}
/// Defines the DrawingMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:dgMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_DrawingMonikerList/oac:dgMkLst")]
pub struct DrawingMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the Transform2D Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:xfrm.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Transform2D/oac:xfrm")]
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
/// Defines the GroupShapeMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:grpSpMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_GroupShapeMonikerList/oac:grpSpMkLst")]
pub struct GroupShapeMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the DrawingElementPackage Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:dePkg.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_DrawingElementPackage/oac:dePkg")]
pub struct DrawingElementPackage {}
/// Defines the DeMkLstDrawingElementMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:deMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_DrawingElementMonikerList/oac:deMkLst")]
pub struct DeMkLstDrawingElementMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the DeMasterMkLstDrawingElementMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:deMasterMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_DrawingElementMonikerList/oac:deMasterMkLst")]
pub struct DeMasterMkLstDrawingElementMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the DeSrcMkLstDrawingElementMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:deSrcMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_DrawingElementMonikerList/oac:deSrcMkLst")]
pub struct DeSrcMkLstDrawingElementMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the DeTgtMkLstDrawingElementMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:deTgtMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_DrawingElementMonikerList/oac:deTgtMkLst")]
pub struct DeTgtMkLstDrawingElementMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the OpenXmlDrawingElementMonikerListElement Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_DrawingElementMonikerList/")]
pub struct OpenXmlDrawingElementMonikerListElement {}
/// Defines the ImgDataImgData Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:imgData.
pub type ImgDataImgData = crate::simple_type::Base64BinaryValue;
/// Defines the OrigImgDataImgData Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:origImgData.
pub type OrigImgDataImgData = crate::simple_type::Base64BinaryValue;
/// Defines the SndDataImgData Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:sndData.
pub type SndDataImgData = crate::simple_type::Base64BinaryValue;
/// Defines the OpenXmlImgDataElement Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
pub type OpenXmlImgDataElement = crate::simple_type::Base64BinaryValue;
/// Defines the ResourceUrl Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:imgUrl.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_ResourceUrl/oac:imgUrl")]
pub struct ResourceUrl {
  /// src
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :src
  #[sdk(attr(qname = ":src"))]
  #[sdk(string_format(source = 0u32, kind = "uri"))]
  pub src: Option<crate::simple_type::StringValue>,
  /// linkage
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :linkage
  #[sdk(attr(qname = ":linkage"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub linkage: Option<ResourceLinkage>,
}
/// Defines the TextBodyPackage Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:txBodyPkg.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_TextBodyPackage/oac:txBodyPkg")]
pub struct TextBodyPackage {}
/// Defines the GroupCommand Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:grpCmd.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_GroupCommand/oac:grpCmd")]
pub struct GroupCommand {
  /// verId
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :verId
  #[sdk(attr(qname = ":verId"))]
  pub ver_id: Option<crate::simple_type::UInt32Value>,
  /// preventRegroup
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :preventRegroup
  #[sdk(attr(qname = ":preventRegroup"))]
  pub prevent_regroup: Option<crate::simple_type::BooleanValue>,
  /// grpId
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :grpId
  #[sdk(attr(qname = ":grpId"))]
  pub grp_id: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "oac:CT_DrawingMonikerList/oac:dgMkLst"))]
  pub drawing_moniker_list: std::boxed::Box<DrawingMonikerList>,
  #[sdk(choice(
    qname = "oac:CT_ShapeMoniker/oac:spMk",
    qname = "oac:CT_GroupShapeMoniker/oac:grpSpMk",
    qname = "oac:CT_GraphicFrameMoniker/oac:graphicFrameMk",
    qname = "oac:CT_ConnectorMoniker/oac:cxnSpMk",
    qname = "oac:CT_PictureMoniker/oac:picMk",
    qname = "oac:CT_InkMoniker/oac:inkMk"
  ))]
  pub group_command_choice: Vec<GroupCommandChoice>,
  /// _
  #[sdk(child(qname = "a:CT_GroupShapeProperties/oac:grpSpPr"))]
  pub oac_grp_sp_pr: Option<std::boxed::Box<GroupShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_NonVisualDrawingProps/oac:cNvPr"))]
  pub oac_c_nv_pr: Option<std::boxed::Box<NonVisualDrawingProps>>,
  /// _
  #[sdk(child(qname = "a:CT_NonVisualGroupDrawingShapeProps/oac:cNvGrpSpPr"))]
  pub oac_c_nv_grp_sp_pr: Option<std::boxed::Box<NonVisualGroupDrawingShapeProps>>,
}
/// Defines the ImgLink Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:imgLink.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_ImgLink/oac:imgLink")]
pub struct ImgLink {
  /// tgt
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :tgt
  #[sdk(attr(qname = ":tgt"))]
  #[sdk(string_format(source = 1u32, kind = "uri"))]
  pub tgt: crate::simple_type::StringValue,
}
/// Defines the DocumentContextMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:dcMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_DocumentContextMonikerList/oac:dcMkLst")]
pub struct DocumentContextMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the GraphicParentMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:graphicParentMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_GraphicParentMonikerList/oac:graphicParentMkLst")]
pub struct GraphicParentMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the ShapeMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:spMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_ShapeMonikerList/oac:spMkLst")]
pub struct ShapeMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the GraphicFrameMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:graphicFrameMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_GraphicFrameMonikerList/oac:graphicFrameMkLst")]
pub struct GraphicFrameMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the ConnectorMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:cxnSpMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_ConnectorMonikerList/oac:cxnSpMkLst")]
pub struct ConnectorMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the PictureMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:picMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_PictureMonikerList/oac:picMkLst")]
pub struct PictureMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the InkMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:inkMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_InkMonikerList/oac:inkMkLst")]
pub struct InkMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the TextBodyMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:txBodyMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_TextBodyMonikerList/oac:txBodyMkLst")]
pub struct TextBodyMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the TextCharRangeMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:txMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_TextCharRangeMonikerList/oac:txMkLst")]
pub struct TextCharRangeMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the HyperlinkMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:hlinkMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_HyperlinkMonikerList/oac:hlinkMkLst")]
pub struct HyperlinkMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the Model3DMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:model3DMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_Model3DMonikerList/oac:model3DMkLst")]
pub struct Model3DMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the ViewSelectionStgList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:viewSelLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_ViewSelectionStgList/oac:viewSelLst")]
pub struct ViewSelectionStgList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the EditorSelectionStgList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:editorSelLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_EditorSelectionStgList/oac:editorSelLst")]
pub struct EditorSelectionStgList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the DrawingSelectionStgList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:drSelLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_DrawingSelectionStgList/oac:drSelLst")]
pub struct DrawingSelectionStgList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the TableMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:tblMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_TableMonikerList/oac:tblMkLst")]
pub struct TableMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the TableCellMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:tcMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_TableCellMonikerList/oac:tcMkLst")]
pub struct TableCellMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the TableRowMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:trMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_TableRowMonikerList/oac:trMkLst")]
pub struct TableRowMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the TableColumnMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:gridColMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_TableColumnMonikerList/oac:gridColMkLst")]
pub struct TableColumnMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the ModifyNonVisualDrawingProps Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:cNvPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_ModifyNonVisualDrawingProps/oac:cNvPr")]
pub struct ModifyNonVisualDrawingProps {
  /// name
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// descr
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :descr
  #[sdk(attr(qname = ":descr"))]
  pub descr: Option<crate::simple_type::StringValue>,
  /// hidden
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :hidden
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  /// title
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// decor
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :decor
  #[sdk(attr(qname = ":decor"))]
  pub decor: Option<crate::simple_type::BooleanValue>,
  /// scriptLink
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :scriptLink
  #[sdk(attr(qname = ":scriptLink"))]
  pub script_link: Option<crate::simple_type::StringValue>,
}
/// Defines the ModifyTransformProps Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:xfrm.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_ModifyTransformProps/oac:xfrm")]
pub struct ModifyTransformProps {
  /// x
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :x
  #[sdk(attr(qname = ":x"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub x: Option<crate::simple_type::Int64Value>,
  /// y
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :y
  #[sdk(attr(qname = ":y"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub y: Option<crate::simple_type::Int64Value>,
  /// cx
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :cx
  #[sdk(attr(qname = ":cx"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub cx: Option<crate::simple_type::Int64Value>,
  /// cy
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :cy
  #[sdk(attr(qname = ":cy"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub cy: Option<crate::simple_type::Int64Value>,
  /// rot
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :rot
  #[sdk(attr(qname = ":rot"))]
  pub rot: Option<crate::simple_type::Int32Value>,
  /// flipH
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :flipH
  #[sdk(attr(qname = ":flipH"))]
  pub flip_h: Option<crate::simple_type::BooleanValue>,
  /// flipV
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :flipV
  #[sdk(attr(qname = ":flipV"))]
  pub flip_v: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Point2DType Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:off.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Point2D/oac:off")]
pub struct Point2DType {
  /// X-Axis Coordinate
  ///
  /// Available in Office2007 and above.
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
  pub x: crate::simple_type::Int64Value,
  /// Y-Axis Coordinate
  ///
  /// Available in Office2007 and above.
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
  pub y: crate::simple_type::Int64Value,
}
/// Defines the TextParagraphPropertiesType Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:pPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextParagraphProperties/oac:pPr")]
pub struct TextParagraphPropertiesType {
  /// Left Margin
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :marL
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
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :marR
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
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lvl
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
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :indent
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
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :algn
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub alignment:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextAlignmentTypeValues>,
  /// Default Tab Size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :defTabSz
  #[sdk(attr(qname = ":defTabSz"))]
  pub default_tab_size: Option<crate::simple_type::Int32Value>,
  /// Right To Left
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rtl
  #[sdk(attr(qname = ":rtl"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// East Asian Line Break
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :eaLnBrk
  #[sdk(attr(qname = ":eaLnBrk"))]
  pub east_asian_line_break: Option<crate::simple_type::BooleanValue>,
  /// Font Alignment
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fontAlgn
  #[sdk(attr(qname = ":fontAlgn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub font_alignment:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextFontAlignmentValues>,
  /// Latin Line Break
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :latinLnBrk
  #[sdk(attr(qname = ":latinLnBrk"))]
  pub latin_line_break: Option<crate::simple_type::BooleanValue>,
  /// Hanging Punctuation
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hangingPunct
  #[sdk(attr(qname = ":hangingPunct"))]
  pub height: Option<crate::simple_type::BooleanValue>,
  ///Line Spacing
  #[sdk(child(qname = "a:CT_TextSpacing/a:lnSpc"))]
  pub line_spacing: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LineSpacing>,
  >,
  ///Space Before
  #[sdk(child(qname = "a:CT_TextSpacing/a:spcBef"))]
  pub space_before: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SpaceBefore>,
  >,
  ///Space After
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
  /// _
  #[sdk(child(qname = "a:CT_TextTabStopList/a:tabLst"))]
  pub a_tab_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TabStopList>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/a:defRPr"))]
  pub a_def_r_pr: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::DefaultRunProperties,
    >,
  >,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Defines the TextBodyProperties Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:bodyPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextBodyProperties/oac:bodyPr")]
pub struct TextBodyProperties {
  /// Rotation
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rot
  #[sdk(attr(qname = ":rot"))]
  pub rotation: Option<crate::simple_type::Int32Value>,
  /// Paragraph Spacing
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :spcFirstLastPara
  #[sdk(attr(qname = ":spcFirstLastPara"))]
  pub use_paragraph_spacing: Option<crate::simple_type::BooleanValue>,
  /// Text Vertical Overflow
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :vertOverflow
  #[sdk(attr(qname = ":vertOverflow"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub vertical_overflow: Option<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextVerticalOverflowValues,
  >,
  /// Text Horizontal Overflow
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :horzOverflow
  #[sdk(attr(qname = ":horzOverflow"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub horizontal_overflow: Option<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextHorizontalOverflowValues,
  >,
  /// Vertical Text
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :vert
  #[sdk(attr(qname = ":vert"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub vertical:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextVerticalValues>,
  /// Text Wrapping Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :wrap
  #[sdk(attr(qname = ":wrap"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub wrap:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextWrappingValues>,
  /// Left Inset
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lIns
  #[sdk(attr(qname = ":lIns"))]
  pub left_inset: Option<crate::simple_type::Int32Value>,
  /// Top Inset
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tIns
  #[sdk(attr(qname = ":tIns"))]
  pub top_inset: Option<crate::simple_type::Int32Value>,
  /// Right Inset
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rIns
  #[sdk(attr(qname = ":rIns"))]
  pub right_inset: Option<crate::simple_type::Int32Value>,
  /// Bottom Inset
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bIns
  #[sdk(attr(qname = ":bIns"))]
  pub bottom_inset: Option<crate::simple_type::Int32Value>,
  /// Number of Columns
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :numCol
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
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :spcCol
  #[sdk(attr(qname = ":spcCol"))]
  #[sdk(number_range(source = 0u32, min = "0", min_inclusive = true, max_inclusive = false))]
  pub column_spacing: Option<crate::simple_type::Int32Value>,
  /// Columns Right-To-Left
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rtlCol
  #[sdk(attr(qname = ":rtlCol"))]
  pub right_to_left_columns: Option<crate::simple_type::BooleanValue>,
  /// From WordArt
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fromWordArt
  #[sdk(attr(qname = ":fromWordArt"))]
  pub from_word_art: Option<crate::simple_type::BooleanValue>,
  /// Anchor
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :anchor
  #[sdk(attr(qname = ":anchor"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub anchor:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextAnchoringTypeValues>,
  /// Anchor Center
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :anchorCtr
  #[sdk(attr(qname = ":anchorCtr"))]
  pub anchor_center: Option<crate::simple_type::BooleanValue>,
  /// Force Anti-Alias
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :forceAA
  #[sdk(attr(qname = ":forceAA"))]
  pub force_anti_alias: Option<crate::simple_type::BooleanValue>,
  /// Text Upright
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :upright
  #[sdk(attr(qname = ":upright"))]
  pub up_right: Option<crate::simple_type::BooleanValue>,
  /// Compatible Line Spacing
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :compatLnSpc
  #[sdk(attr(qname = ":compatLnSpc"))]
  pub compatible_line_spacing: Option<crate::simple_type::BooleanValue>,
  ///Preset Text Shape
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
  /// _
  #[sdk(child(qname = "a:CT_Scene3D/a:scene3d"))]
  pub a_scene3d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Scene3DType>,
  >,
  #[sdk(choice(qname = "a:CT_Shape3D/a:sp3d", qname = "a:CT_FlatText/a:flatTx"))]
  pub text_body_properties_choice2: Option<TextBodyPropertiesChoice2>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Defines the ModifyNonVisualDrawingShapeProps Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:cNvSpPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_ModifyNonVisualDrawingShapeProps/oac:cNvSpPr")]
pub struct ModifyNonVisualDrawingShapeProps {
  /// noGrp
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noGrp
  #[sdk(attr(qname = ":noGrp"))]
  pub no_grp: Option<crate::simple_type::BooleanValue>,
  /// noSelect
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noSelect
  #[sdk(attr(qname = ":noSelect"))]
  pub no_select: Option<crate::simple_type::BooleanValue>,
  /// noRot
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noRot
  #[sdk(attr(qname = ":noRot"))]
  pub no_rot: Option<crate::simple_type::BooleanValue>,
  /// noChangeAspect
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noChangeAspect
  #[sdk(attr(qname = ":noChangeAspect"))]
  pub no_change_aspect: Option<crate::simple_type::BooleanValue>,
  /// noMove
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noMove
  #[sdk(attr(qname = ":noMove"))]
  pub no_move: Option<crate::simple_type::BooleanValue>,
  /// noResize
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noResize
  #[sdk(attr(qname = ":noResize"))]
  pub no_resize: Option<crate::simple_type::BooleanValue>,
  /// noEditPoints
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noEditPoints
  #[sdk(attr(qname = ":noEditPoints"))]
  pub no_edit_points: Option<crate::simple_type::BooleanValue>,
  /// noAdjustHandles
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noAdjustHandles
  #[sdk(attr(qname = ":noAdjustHandles"))]
  pub no_adjust_handles: Option<crate::simple_type::BooleanValue>,
  /// noChangeArrowheads
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noChangeArrowheads
  #[sdk(attr(qname = ":noChangeArrowheads"))]
  pub no_change_arrowheads: Option<crate::simple_type::BooleanValue>,
  /// noChangeShapeType
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noChangeShapeType
  #[sdk(attr(qname = ":noChangeShapeType"))]
  pub no_change_shape_type: Option<crate::simple_type::BooleanValue>,
  /// noTextEdit
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noTextEdit
  #[sdk(attr(qname = ":noTextEdit"))]
  pub no_text_edit: Option<crate::simple_type::BooleanValue>,
  /// txBox
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :txBox
  #[sdk(attr(qname = ":txBox"))]
  pub tx_box: Option<crate::simple_type::BooleanValue>,
}
/// Defines the ShapePropsMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:spMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_ShapePropsMonikerList/oac:spMkLst")]
pub struct ShapePropsMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the ShapeProperties Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:spPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ShapeProperties/oac:spPr")]
pub struct ShapeProperties {
  /// Black and White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bwMode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues>,
  ///2D Transform for Individual Objects
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
  /// _
  #[sdk(child(qname = "a:CT_LineProperties/a:ln"))]
  pub a_ln: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Outline>,
  >,
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub shape_properties_choice3: Option<ShapePropertiesChoice3>,
  /// _
  #[sdk(child(qname = "a:CT_Scene3D/a:scene3d"))]
  pub a_scene3d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Scene3DType>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_Shape3D/a:sp3d"))]
  pub a_sp3d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Shape3DType>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_ShapePropertiesExtensionList/a:extLst"))]
  pub a_ext_lst: Option<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ShapePropertiesExtensionList,
  >,
}
/// Defines the XfrmEmpty Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:xfrm.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_Empty/oac:xfrm")]
pub struct XfrmEmpty {}
/// Defines the GeomEmpty Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:geom.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_Empty/oac:geom")]
pub struct GeomEmpty {}
/// Defines the FillEmpty Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:fill.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_Empty/oac:fill")]
pub struct FillEmpty {}
/// Defines the LnEmpty Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:ln.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_Empty/oac:ln")]
pub struct LnEmpty {}
/// Defines the EffectEmpty Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:effect.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_Empty/oac:effect")]
pub struct EffectEmpty {}
/// Defines the Scene3dEmpty Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:scene3d.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_Empty/oac:scene3d")]
pub struct Scene3dEmpty {}
/// Defines the Sp3dEmpty Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:sp3d.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_Empty/oac:sp3d")]
pub struct Sp3dEmpty {}
/// Defines the ExtLstEmpty Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_Empty/oac:extLst")]
pub struct ExtLstEmpty {}
/// Defines the BwModeEmpty Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:bwMode.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_Empty/oac:bwMode")]
pub struct BwModeEmpty {}
/// Defines the SrcRectEmpty Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:srcRect.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_Empty/oac:srcRect")]
pub struct SrcRectEmpty {}
/// Defines the FillModeEmpty Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:fillMode.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_Empty/oac:fillMode")]
pub struct FillModeEmpty {}
/// Defines the DpiEmpty Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:dpi.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_Empty/oac:dpi")]
pub struct DpiEmpty {}
/// Defines the RotWithShapeEmpty Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:rotWithShape.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_Empty/oac:rotWithShape")]
pub struct RotWithShapeEmpty {}
/// Defines the StCxnEmpty Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:stCxn.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_Empty/oac:stCxn")]
pub struct StCxnEmpty {}
/// Defines the EndCxnEmpty Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:endCxn.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_Empty/oac:endCxn")]
pub struct EndCxnEmpty {}
/// Defines the NoGrpEmpty Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:noGrp.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_Empty/oac:noGrp")]
pub struct NoGrpEmpty {}
/// Defines the NoSelectEmpty Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:noSelect.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_Empty/oac:noSelect")]
pub struct NoSelectEmpty {}
/// Defines the NoRotEmpty Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:noRot.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_Empty/oac:noRot")]
pub struct NoRotEmpty {}
/// Defines the NoChangeAspectEmpty Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:noChangeAspect.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_Empty/oac:noChangeAspect")]
pub struct NoChangeAspectEmpty {}
/// Defines the NoMoveEmpty Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:noMove.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_Empty/oac:noMove")]
pub struct NoMoveEmpty {}
/// Defines the NoResizeEmpty Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:noResize.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_Empty/oac:noResize")]
pub struct NoResizeEmpty {}
/// Defines the NoEditPointsEmpty Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:noEditPoints.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_Empty/oac:noEditPoints")]
pub struct NoEditPointsEmpty {}
/// Defines the NoAdjustHandlesEmpty Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:noAdjustHandles.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_Empty/oac:noAdjustHandles")]
pub struct NoAdjustHandlesEmpty {}
/// Defines the NoChangeArrowheadsEmpty Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:noChangeArrowheads.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_Empty/oac:noChangeArrowheads")]
pub struct NoChangeArrowheadsEmpty {}
/// Defines the NoChangeShapeTypeEmpty Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:noChangeShapeType.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_Empty/oac:noChangeShapeType")]
pub struct NoChangeShapeTypeEmpty {}
/// Defines the LfPrEmpty Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:lfPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_Empty/oac:lfPr")]
pub struct LfPrEmpty {}
/// Defines the HlinkClickEmpty Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:hlinkClick.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_Empty/oac:hlinkClick")]
pub struct HlinkClickEmpty {}
/// Defines the HlinkHoverEmpty Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:hlinkHover.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_Empty/oac:hlinkHover")]
pub struct HlinkHoverEmpty {}
/// Defines the OpenXmlEmptyElement Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_Empty/")]
pub struct OpenXmlEmptyElement {}
/// Defines the ResetShapeProperties Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:spPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_ResetShapeProperties/oac:spPr")]
pub struct ResetShapeProperties {
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:xfrm"))]
  pub xfrm_empty: Option<XfrmEmpty>,
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:geom"))]
  pub geom_empty: Option<GeomEmpty>,
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:fill"))]
  pub fill_empty: Option<FillEmpty>,
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:ln"))]
  pub ln_empty: Option<LnEmpty>,
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:effect"))]
  pub effect_empty: Option<EffectEmpty>,
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:scene3d"))]
  pub scene3d_empty: Option<Scene3dEmpty>,
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:sp3d"))]
  pub sp3d_empty: Option<Sp3dEmpty>,
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:extLst"))]
  pub ext_lst_empty: Option<ExtLstEmpty>,
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:bwMode"))]
  pub bw_mode_empty: Option<BwModeEmpty>,
}
/// Defines the LnRefStyleMatrixReference Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:lnRef.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_StyleMatrixReference/oac:lnRef")]
pub struct LnRefStyleMatrixReference {
  /// Style Matrix Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :idx
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
  pub xml_children: Option<LnRefStyleMatrixReferenceChoice>,
}
/// Defines the FillRefStyleMatrixReference Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:fillRef.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_StyleMatrixReference/oac:fillRef")]
pub struct FillRefStyleMatrixReference {
  /// Style Matrix Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :idx
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
  pub xml_children: Option<FillRefStyleMatrixReferenceChoice>,
}
/// Defines the EffectRefStyleMatrixReference Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:effectRef.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_StyleMatrixReference/oac:effectRef")]
pub struct EffectRefStyleMatrixReference {
  /// Style Matrix Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :idx
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
  pub xml_children: Option<EffectRefStyleMatrixReferenceChoice>,
}
/// Defines the StyleMatrixReferenceType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_StyleMatrixReference/")]
pub struct StyleMatrixReferenceType {
  /// Style Matrix Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :idx
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
  pub xml_children: Option<StyleMatrixReferenceTypeChoice>,
}
/// Defines the FontReference Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:fontRef.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_FontReference/oac:fontRef")]
pub struct FontReference {
  /// Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :idx
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
  pub xml_children: Option<FontReferenceChoice>,
}
/// Defines the ModifyShapeStyleProps Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:style.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_ModifyShapeStyleProps/oac:style")]
pub struct ModifyShapeStyleProps {
  /// _
  #[sdk(child(qname = "a:CT_StyleMatrixReference/oac:lnRef"))]
  pub ln_ref_style_matrix_reference: Option<std::boxed::Box<LnRefStyleMatrixReference>>,
  /// _
  #[sdk(child(qname = "a:CT_StyleMatrixReference/oac:fillRef"))]
  pub fill_ref_style_matrix_reference: Option<std::boxed::Box<FillRefStyleMatrixReference>>,
  /// _
  #[sdk(child(qname = "a:CT_StyleMatrixReference/oac:effectRef"))]
  pub effect_ref_style_matrix_reference: Option<std::boxed::Box<EffectRefStyleMatrixReference>>,
  /// _
  #[sdk(child(qname = "a:CT_FontReference/oac:fontRef"))]
  pub font_reference: Option<std::boxed::Box<FontReference>>,
}
/// Defines the ResetXsdboolean Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:reset.
pub type ResetXsdboolean = crate::simple_type::BooleanValue;
/// Defines the UseBoundsXsdboolean Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:useBounds.
pub type UseBoundsXsdboolean = crate::simple_type::BooleanValue;
/// Defines the BlipFillProperties Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:blipFill.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_BlipFillProperties/oac:blipFill")]
pub struct BlipFillProperties {
  /// DPI Setting
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dpi
  #[sdk(attr(qname = ":dpi"))]
  pub dpi: Option<crate::simple_type::UInt32Value>,
  /// Rotate With Shape
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rotWithShape
  #[sdk(attr(qname = ":rotWithShape"))]
  pub rotate_with_shape: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "a:CT_Blip/a:blip"))]
  pub blip:
    Option<std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Blip>>,
  ///Source Rectangle
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
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:fillRect.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_RelativeRectProps/oac:fillRect")]
pub struct FillRectRelativeRectProps {
  /// l
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :l
  #[sdk(attr(qname = ":l"))]
  pub l: Option<crate::simple_type::Int32Value>,
  /// t
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :t
  #[sdk(attr(qname = ":t"))]
  pub t: Option<crate::simple_type::Int32Value>,
  /// r
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :r
  #[sdk(attr(qname = ":r"))]
  pub r: Option<crate::simple_type::Int32Value>,
  /// b
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :b
  #[sdk(attr(qname = ":b"))]
  pub b: Option<crate::simple_type::Int32Value>,
}
/// Defines the SrcRectRelativeRectProps Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:srcRect.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_RelativeRectProps/oac:srcRect")]
pub struct SrcRectRelativeRectProps {
  /// l
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :l
  #[sdk(attr(qname = ":l"))]
  pub l: Option<crate::simple_type::Int32Value>,
  /// t
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :t
  #[sdk(attr(qname = ":t"))]
  pub t: Option<crate::simple_type::Int32Value>,
  /// r
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :r
  #[sdk(attr(qname = ":r"))]
  pub r: Option<crate::simple_type::Int32Value>,
  /// b
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :b
  #[sdk(attr(qname = ":b"))]
  pub b: Option<crate::simple_type::Int32Value>,
}
/// Defines the OpenXmlRelativeRectPropsElement Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_RelativeRectProps/")]
pub struct OpenXmlRelativeRectPropsElement {
  /// l
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :l
  #[sdk(attr(qname = ":l"))]
  pub l: Option<crate::simple_type::Int32Value>,
  /// t
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :t
  #[sdk(attr(qname = ":t"))]
  pub t: Option<crate::simple_type::Int32Value>,
  /// r
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :r
  #[sdk(attr(qname = ":r"))]
  pub r: Option<crate::simple_type::Int32Value>,
  /// b
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :b
  #[sdk(attr(qname = ":b"))]
  pub b: Option<crate::simple_type::Int32Value>,
}
/// Defines the ResetBlipFillProperties Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:blipFill.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_ResetBlipFillProperties/oac:blipFill")]
pub struct ResetBlipFillProperties {
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:srcRect"))]
  pub src_rect_empty: Option<SrcRectEmpty>,
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:fillMode"))]
  pub fill_mode_empty: Option<FillModeEmpty>,
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:dpi"))]
  pub dpi_empty: Option<DpiEmpty>,
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:rotWithShape"))]
  pub rot_with_shape_empty: Option<RotWithShapeEmpty>,
}
/// Defines the ModifyNonVisualGroupDrawingShapeProps Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:cNvGrpSpPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_ModifyNonVisualGroupDrawingShapeProps/oac:cNvGrpSpPr")]
pub struct ModifyNonVisualGroupDrawingShapeProps {
  /// noGrp
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noGrp
  #[sdk(attr(qname = ":noGrp"))]
  pub no_grp: Option<crate::simple_type::BooleanValue>,
  /// noUngrp
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noUngrp
  #[sdk(attr(qname = ":noUngrp"))]
  pub no_ungrp: Option<crate::simple_type::BooleanValue>,
  /// noSelect
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noSelect
  #[sdk(attr(qname = ":noSelect"))]
  pub no_select: Option<crate::simple_type::BooleanValue>,
  /// noRot
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noRot
  #[sdk(attr(qname = ":noRot"))]
  pub no_rot: Option<crate::simple_type::BooleanValue>,
  /// noChangeAspect
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noChangeAspect
  #[sdk(attr(qname = ":noChangeAspect"))]
  pub no_change_aspect: Option<crate::simple_type::BooleanValue>,
  /// noMove
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noMove
  #[sdk(attr(qname = ":noMove"))]
  pub no_move: Option<crate::simple_type::BooleanValue>,
  /// noResize
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noResize
  #[sdk(attr(qname = ":noResize"))]
  pub no_resize: Option<crate::simple_type::BooleanValue>,
}
/// Defines the GroupShapeProperties Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:grpSpPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GroupShapeProperties/oac:grpSpPr")]
pub struct GroupShapeProperties {
  /// Black and White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bwMode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues>,
  ///2D Transform for Grouped Objects
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
  /// _
  #[sdk(child(qname = "a:CT_Scene3D/a:scene3d"))]
  pub a_scene3d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Scene3DType>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Defines the ResetGroupShapeProperties Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:grpSpPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_ResetGroupShapeProperties/oac:grpSpPr")]
pub struct ResetGroupShapeProperties {
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:xfrm"))]
  pub xfrm_empty: Option<XfrmEmpty>,
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:fill"))]
  pub fill_empty: Option<FillEmpty>,
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:effect"))]
  pub effect_empty: Option<EffectEmpty>,
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:scene3d"))]
  pub scene3d_empty: Option<Scene3dEmpty>,
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:extLst"))]
  pub ext_lst_empty: Option<ExtLstEmpty>,
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:bwMode"))]
  pub bw_mode_empty: Option<BwModeEmpty>,
}
/// Defines the NonVisualDrawingProps Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:cNvPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualDrawingProps/oac:cNvPr")]
pub struct NonVisualDrawingProps {
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
/// Defines the NonVisualGroupDrawingShapeProps Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:cNvGrpSpPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualGroupDrawingShapeProps/oac:cNvGrpSpPr")]
pub struct NonVisualGroupDrawingShapeProps {
    /// _
    #[sdk(child(qname = "a:CT_GroupLocking/a:grpSpLocks"))]
    pub group_shape_locks: Option<
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GroupShapeLocks,
        >,
    >,
    /// _
    #[sdk(child(qname = "a:CT_NonVisualGroupDrawingShapePropsExtensionList/a:extLst"))]
    pub non_visual_group_drawing_shape_props_extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NonVisualGroupDrawingShapePropsExtensionList,
    >,
}
/// Defines the ModifyNonVisualGraphicFrameProps Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:cNvGraphicFramePr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_ModifyNonVisualGraphicFrameProps/oac:cNvGraphicFramePr")]
pub struct ModifyNonVisualGraphicFrameProps {
  /// noGrp
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noGrp
  #[sdk(attr(qname = ":noGrp"))]
  pub no_grp: Option<crate::simple_type::BooleanValue>,
  /// noDrilldown
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noDrilldown
  #[sdk(attr(qname = ":noDrilldown"))]
  pub no_drilldown: Option<crate::simple_type::BooleanValue>,
  /// noSelect
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noSelect
  #[sdk(attr(qname = ":noSelect"))]
  pub no_select: Option<crate::simple_type::BooleanValue>,
  /// noChangeAspect
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noChangeAspect
  #[sdk(attr(qname = ":noChangeAspect"))]
  pub no_change_aspect: Option<crate::simple_type::BooleanValue>,
  /// noMove
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noMove
  #[sdk(attr(qname = ":noMove"))]
  pub no_move: Option<crate::simple_type::BooleanValue>,
  /// noResize
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noResize
  #[sdk(attr(qname = ":noResize"))]
  pub no_resize: Option<crate::simple_type::BooleanValue>,
}
/// Defines the StCxnConnection Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:stCxn.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Connection/oac:stCxn")]
pub struct StCxnConnection {
  /// Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :idx
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
}
/// Defines the EndCxnConnection Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:endCxn.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Connection/oac:endCxn")]
pub struct EndCxnConnection {
  /// Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :idx
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
}
/// Defines the ConnectionType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Connection/")]
pub struct ConnectionType {
  /// Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :idx
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
}
/// Defines the ModifyNonVisualConnectorProps Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:cNvCxnSpPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_ModifyNonVisualConnectorProps/oac:cNvCxnSpPr")]
pub struct ModifyNonVisualConnectorProps {
  /// noGrp
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noGrp
  #[sdk(attr(qname = ":noGrp"))]
  pub no_grp: Option<crate::simple_type::BooleanValue>,
  /// noSelect
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noSelect
  #[sdk(attr(qname = ":noSelect"))]
  pub no_select: Option<crate::simple_type::BooleanValue>,
  /// noRot
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noRot
  #[sdk(attr(qname = ":noRot"))]
  pub no_rot: Option<crate::simple_type::BooleanValue>,
  /// noChangeAspect
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noChangeAspect
  #[sdk(attr(qname = ":noChangeAspect"))]
  pub no_change_aspect: Option<crate::simple_type::BooleanValue>,
  /// noMove
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noMove
  #[sdk(attr(qname = ":noMove"))]
  pub no_move: Option<crate::simple_type::BooleanValue>,
  /// noResize
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noResize
  #[sdk(attr(qname = ":noResize"))]
  pub no_resize: Option<crate::simple_type::BooleanValue>,
  /// noEditPoints
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noEditPoints
  #[sdk(attr(qname = ":noEditPoints"))]
  pub no_edit_points: Option<crate::simple_type::BooleanValue>,
  /// noAdjustHandles
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noAdjustHandles
  #[sdk(attr(qname = ":noAdjustHandles"))]
  pub no_adjust_handles: Option<crate::simple_type::BooleanValue>,
  /// noChangeArrowheads
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noChangeArrowheads
  #[sdk(attr(qname = ":noChangeArrowheads"))]
  pub no_change_arrowheads: Option<crate::simple_type::BooleanValue>,
  /// noChangeShapeType
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noChangeShapeType
  #[sdk(attr(qname = ":noChangeShapeType"))]
  pub no_change_shape_type: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "a:CT_Connection/oac:stCxn"))]
  pub st_cxn_connection: Option<StCxnConnection>,
  /// _
  #[sdk(child(qname = "a:CT_Connection/oac:endCxn"))]
  pub end_cxn_connection: Option<EndCxnConnection>,
}
/// Defines the ResetNonVisualConnectorProps Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:cNvCxnSpPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_ResetNonVisualConnectorProps/oac:cNvCxnSpPr")]
pub struct ResetNonVisualConnectorProps {
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:stCxn"))]
  pub st_cxn_empty: Option<StCxnEmpty>,
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:endCxn"))]
  pub end_cxn_empty: Option<EndCxnEmpty>,
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:noGrp"))]
  pub no_grp_empty: Option<NoGrpEmpty>,
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:noSelect"))]
  pub no_select_empty: Option<NoSelectEmpty>,
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:noRot"))]
  pub no_rot_empty: Option<NoRotEmpty>,
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:noChangeAspect"))]
  pub no_change_aspect_empty: Option<NoChangeAspectEmpty>,
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:noMove"))]
  pub no_move_empty: Option<NoMoveEmpty>,
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:noResize"))]
  pub no_resize_empty: Option<NoResizeEmpty>,
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:noEditPoints"))]
  pub no_edit_points_empty: Option<NoEditPointsEmpty>,
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:noAdjustHandles"))]
  pub no_adjust_handles_empty: Option<NoAdjustHandlesEmpty>,
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:noChangeArrowheads"))]
  pub no_change_arrowheads_empty: Option<NoChangeArrowheadsEmpty>,
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:noChangeShapeType"))]
  pub no_change_shape_type_empty: Option<NoChangeShapeTypeEmpty>,
}
/// Defines the CompressPictureProps Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:compressPicPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_CompressPictureProps/oac:compressPicPr")]
pub struct CompressPictureProps {
  /// removeCrop
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :removeCrop
  #[sdk(attr(qname = ":removeCrop"))]
  pub remove_crop: Option<crate::simple_type::BooleanValue>,
  /// useLocalDpi
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :useLocalDpi
  #[sdk(attr(qname = ":useLocalDpi"))]
  pub use_local_dpi: Option<crate::simple_type::BooleanValue>,
  /// cstate
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :cstate
  #[sdk(attr(qname = ":cstate"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub cstate:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipCompressionValues>,
}
/// Defines the ModifyNonVisualPictureProps Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:cNvPicPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_ModifyNonVisualPictureProps/oac:cNvPicPr")]
pub struct ModifyNonVisualPictureProps {
  /// noGrp
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noGrp
  #[sdk(attr(qname = ":noGrp"))]
  pub no_grp: Option<crate::simple_type::BooleanValue>,
  /// noSelect
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noSelect
  #[sdk(attr(qname = ":noSelect"))]
  pub no_select: Option<crate::simple_type::BooleanValue>,
  /// noRot
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noRot
  #[sdk(attr(qname = ":noRot"))]
  pub no_rot: Option<crate::simple_type::BooleanValue>,
  /// noChangeAspect
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noChangeAspect
  #[sdk(attr(qname = ":noChangeAspect"))]
  pub no_change_aspect: Option<crate::simple_type::BooleanValue>,
  /// noMove
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noMove
  #[sdk(attr(qname = ":noMove"))]
  pub no_move: Option<crate::simple_type::BooleanValue>,
  /// noResize
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noResize
  #[sdk(attr(qname = ":noResize"))]
  pub no_resize: Option<crate::simple_type::BooleanValue>,
  /// noEditPoints
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noEditPoints
  #[sdk(attr(qname = ":noEditPoints"))]
  pub no_edit_points: Option<crate::simple_type::BooleanValue>,
  /// noAdjustHandles
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noAdjustHandles
  #[sdk(attr(qname = ":noAdjustHandles"))]
  pub no_adjust_handles: Option<crate::simple_type::BooleanValue>,
  /// noChangeArrowheads
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noChangeArrowheads
  #[sdk(attr(qname = ":noChangeArrowheads"))]
  pub no_change_arrowheads: Option<crate::simple_type::BooleanValue>,
  /// noChangeShapeType
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noChangeShapeType
  #[sdk(attr(qname = ":noChangeShapeType"))]
  pub no_change_shape_type: Option<crate::simple_type::BooleanValue>,
  /// noCrop
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noCrop
  #[sdk(attr(qname = ":noCrop"))]
  pub no_crop: Option<crate::simple_type::BooleanValue>,
  /// preferRelativeResize
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :preferRelativeResize
  #[sdk(attr(qname = ":preferRelativeResize"))]
  pub prefer_relative_resize: Option<crate::simple_type::BooleanValue>,
}
/// Defines the ResetNonVisualPictureProps Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:cNvPicPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_ResetNonVisualPictureProps/oac:cNvPicPr")]
pub struct ResetNonVisualPictureProps {
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:lfPr"))]
  pub lf_pr_empty: Option<LfPrEmpty>,
}
/// Defines the BoundRect Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:bounds.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_BoundRect/oac:bounds")]
pub struct BoundRect {
  /// l
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :l
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
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :t
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
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :r
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
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :b
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
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:svgBlipMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_SVGBlipMonikerList/oac:svgBlipMkLst")]
pub struct SvgBlipMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the LinePropertiesType Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:lineProps.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_LineProperties/oac:lineProps")]
pub struct LinePropertiesType {
  /// line width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :w
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
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cap
  #[sdk(attr(qname = ":cap"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub cap_type:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LineCapValues>,
  /// compound line type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cmpd
  #[sdk(attr(qname = ":cmpd"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub compound_line_type:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::CompoundLineValues>,
  /// pen alignment
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :algn
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
  /// _
  #[sdk(child(qname = "a:CT_LineEndProperties/a:headEnd"))]
  pub a_head_end: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HeadEnd>,
  /// _
  #[sdk(child(qname = "a:CT_LineEndProperties/a:tailEnd"))]
  pub a_tail_end: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TailEnd>,
  /// _
  #[sdk(child(qname = "a:CT_LinePropertiesExtensionList/a:extLst"))]
  pub a_ext_lst: Option<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LinePropertiesExtensionList,
  >,
}
/// Defines the ModifyNonVisualInkProps Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:cNvInkPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_ModifyNonVisualInkProps/oac:cNvInkPr")]
pub struct ModifyNonVisualInkProps {
  /// noGrp
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noGrp
  #[sdk(attr(qname = ":noGrp"))]
  pub no_grp: Option<crate::simple_type::BooleanValue>,
  /// noSelect
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noSelect
  #[sdk(attr(qname = ":noSelect"))]
  pub no_select: Option<crate::simple_type::BooleanValue>,
  /// noRot
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noRot
  #[sdk(attr(qname = ":noRot"))]
  pub no_rot: Option<crate::simple_type::BooleanValue>,
  /// noChangeAspect
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noChangeAspect
  #[sdk(attr(qname = ":noChangeAspect"))]
  pub no_change_aspect: Option<crate::simple_type::BooleanValue>,
  /// noMove
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noMove
  #[sdk(attr(qname = ":noMove"))]
  pub no_move: Option<crate::simple_type::BooleanValue>,
  /// noResize
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noResize
  #[sdk(attr(qname = ":noResize"))]
  pub no_resize: Option<crate::simple_type::BooleanValue>,
  /// noEditPoints
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noEditPoints
  #[sdk(attr(qname = ":noEditPoints"))]
  pub no_edit_points: Option<crate::simple_type::BooleanValue>,
  /// noAdjustHandles
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noAdjustHandles
  #[sdk(attr(qname = ":noAdjustHandles"))]
  pub no_adjust_handles: Option<crate::simple_type::BooleanValue>,
  /// noChangeArrowheads
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noChangeArrowheads
  #[sdk(attr(qname = ":noChangeArrowheads"))]
  pub no_change_arrowheads: Option<crate::simple_type::BooleanValue>,
  /// noChangeShapeType
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :noChangeShapeType
  #[sdk(attr(qname = ":noChangeShapeType"))]
  pub no_change_shape_type: Option<crate::simple_type::BooleanValue>,
  /// isComment
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :isComment
  #[sdk(attr(qname = ":isComment"))]
  pub is_comment: Option<crate::simple_type::BooleanValue>,
}
/// Defines the HlinkClickHyperlinkProps Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:hlinkClick.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_HyperlinkProps/oac:hlinkClick")]
pub struct HlinkClickHyperlinkProps {
  /// source
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :source
  #[sdk(attr(qname = ":source"))]
  pub source: Option<crate::simple_type::StringValue>,
  /// action
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :action
  #[sdk(attr(qname = ":action"))]
  pub action: Option<crate::simple_type::StringValue>,
  /// tgtFrame
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :tgtFrame
  #[sdk(attr(qname = ":tgtFrame"))]
  pub tgt_frame: Option<crate::simple_type::StringValue>,
  /// tooltip
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :tooltip
  #[sdk(attr(qname = ":tooltip"))]
  pub tooltip: Option<crate::simple_type::StringValue>,
  /// highlightClick
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :highlightClick
  #[sdk(attr(qname = ":highlightClick"))]
  pub highlight_click: Option<crate::simple_type::BooleanValue>,
  /// endSnd
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :endSnd
  #[sdk(attr(qname = ":endSnd"))]
  pub end_snd: Option<crate::simple_type::BooleanValue>,
  /// sndName
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :sndName
  #[sdk(attr(qname = ":sndName"))]
  pub snd_name: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(qname = "oac:CT_ImgData/oac:sndData"))]
  pub snd_data_img_data: Option<crate::simple_type::Base64BinaryValue>,
}
/// Defines the HlinkHoverHyperlinkProps Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:hlinkHover.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_HyperlinkProps/oac:hlinkHover")]
pub struct HlinkHoverHyperlinkProps {
  /// source
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :source
  #[sdk(attr(qname = ":source"))]
  pub source: Option<crate::simple_type::StringValue>,
  /// action
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :action
  #[sdk(attr(qname = ":action"))]
  pub action: Option<crate::simple_type::StringValue>,
  /// tgtFrame
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :tgtFrame
  #[sdk(attr(qname = ":tgtFrame"))]
  pub tgt_frame: Option<crate::simple_type::StringValue>,
  /// tooltip
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :tooltip
  #[sdk(attr(qname = ":tooltip"))]
  pub tooltip: Option<crate::simple_type::StringValue>,
  /// highlightClick
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :highlightClick
  #[sdk(attr(qname = ":highlightClick"))]
  pub highlight_click: Option<crate::simple_type::BooleanValue>,
  /// endSnd
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :endSnd
  #[sdk(attr(qname = ":endSnd"))]
  pub end_snd: Option<crate::simple_type::BooleanValue>,
  /// sndName
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :sndName
  #[sdk(attr(qname = ":sndName"))]
  pub snd_name: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(qname = "oac:CT_ImgData/oac:sndData"))]
  pub snd_data_img_data: Option<crate::simple_type::Base64BinaryValue>,
}
/// Defines the OpenXmlHyperlinkPropsElement Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_HyperlinkProps/")]
pub struct OpenXmlHyperlinkPropsElement {
  /// source
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :source
  #[sdk(attr(qname = ":source"))]
  pub source: Option<crate::simple_type::StringValue>,
  /// action
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :action
  #[sdk(attr(qname = ":action"))]
  pub action: Option<crate::simple_type::StringValue>,
  /// tgtFrame
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :tgtFrame
  #[sdk(attr(qname = ":tgtFrame"))]
  pub tgt_frame: Option<crate::simple_type::StringValue>,
  /// tooltip
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :tooltip
  #[sdk(attr(qname = ":tooltip"))]
  pub tooltip: Option<crate::simple_type::StringValue>,
  /// highlightClick
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :highlightClick
  #[sdk(attr(qname = ":highlightClick"))]
  pub highlight_click: Option<crate::simple_type::BooleanValue>,
  /// endSnd
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :endSnd
  #[sdk(attr(qname = ":endSnd"))]
  pub end_snd: Option<crate::simple_type::BooleanValue>,
  /// sndName
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :sndName
  #[sdk(attr(qname = ":sndName"))]
  pub snd_name: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(qname = "oac:CT_ImgData/oac:sndData"))]
  pub snd_data_img_data: Vec<crate::simple_type::Base64BinaryValue>,
}
/// Defines the ModifyHyperlinkProps Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:hlink.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_ModifyHyperlinkProps/oac:hlink")]
pub struct ModifyHyperlinkProps {
  /// _
  #[sdk(child(qname = "oac:CT_HyperlinkProps/oac:hlinkClick"))]
  pub hlink_click_hyperlink_props: Option<HlinkClickHyperlinkProps>,
  /// _
  #[sdk(child(qname = "oac:CT_HyperlinkProps/oac:hlinkHover"))]
  pub hlink_hover_hyperlink_props: Option<HlinkHoverHyperlinkProps>,
}
/// Defines the ResetHyperlinkProps Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:hlink.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_ResetHyperlinkProps/oac:hlink")]
pub struct ResetHyperlinkProps {
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:hlinkClick"))]
  pub hlink_click_empty: Option<HlinkClickEmpty>,
  /// _
  #[sdk(child(qname = "oac:CT_Empty/oac:hlinkHover"))]
  pub hlink_hover_empty: Option<HlinkHoverEmpty>,
}
/// Defines the TextCharRangeContext Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is oac:context.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:CT_TextCharRangeContext/oac:context")]
pub struct TextCharRangeContext {
  /// len
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :len
  #[sdk(attr(qname = ":len"))]
  pub len: Option<crate::simple_type::UInt32Value>,
  /// hash
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :hash
  #[sdk(attr(qname = ":hash"))]
  pub hash: crate::simple_type::UInt32Value,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum GroupCommandChoice {
  #[sdk(child(qname = "oac:CT_ShapeMoniker/oac:spMk"))]
  OacSpMk(std::boxed::Box<ShapeMoniker>),
  #[sdk(child(qname = "oac:CT_GroupShapeMoniker/oac:grpSpMk"))]
  OacGrpSpMk(std::boxed::Box<GroupShapeMoniker>),
  #[sdk(child(qname = "oac:CT_GraphicFrameMoniker/oac:graphicFrameMk"))]
  OacGraphicFrameMk(std::boxed::Box<GraphicFrameMoniker>),
  #[sdk(child(qname = "oac:CT_ConnectorMoniker/oac:cxnSpMk"))]
  OacCxnSpMk(std::boxed::Box<ConnectorMoniker>),
  #[sdk(child(qname = "oac:CT_PictureMoniker/oac:picMk"))]
  OacPicMk(std::boxed::Box<PictureMoniker>),
  #[sdk(child(qname = "oac:CT_InkMoniker/oac:inkMk"))]
  OacInkMk(std::boxed::Box<InkMoniker>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum TextParagraphPropertiesTypeChoice {
  #[sdk(child(qname = "a:CT_TextBulletColorFollowText/a:buClrTx"))]
  ABuClrTx(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BulletColorText,
    >,
  ),
  #[sdk(child(qname = "a:CT_Color/a:buClr"))]
  ABuClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BulletColor>,
  ),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum TextParagraphPropertiesTypeChoice2 {
  #[sdk(child(qname = "a:CT_TextBulletSizeFollowText/a:buSzTx"))]
  ABuSzTx(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BulletSizeText>,
  ),
  #[sdk(child(qname = "a:CT_TextBulletSizePercent/a:buSzPct"))]
  ABuSzPct(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BulletSizePercentage,
    >,
  ),
  #[sdk(child(qname = "a:CT_TextBulletSizePoint/a:buSzPts"))]
  ABuSzPts(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BulletSizePoints,
    >,
  ),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum TextParagraphPropertiesTypeChoice3 {
  #[sdk(child(qname = "a:CT_TextBulletTypefaceFollowText/a:buFontTx"))]
  ABuFontTx(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BulletFontText>,
  ),
  #[sdk(child(qname = "a:CT_TextFont/a:buFont"))]
  ABuFont(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BulletFont>,
  ),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum TextParagraphPropertiesTypeChoice4 {
  #[sdk(child(qname = "a:CT_TextNoBullet/a:buNone"))]
  ABuNone(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoBullet>,
  ),
  #[sdk(child(qname = "a:CT_TextAutonumberBullet/a:buAutoNum"))]
  ABuAutoNum(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AutoNumberedBullet,
    >,
  ),
  #[sdk(child(qname = "a:CT_TextCharBullet/a:buChar"))]
  ABuChar(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::CharacterBullet,
    >,
  ),
  #[sdk(child(qname = "a:CT_TextBlipBullet/a:buBlip"))]
  ABuBlip(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PictureBullet>,
  ),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum TextBodyPropertiesChoice {
  #[sdk(child(qname = "a:CT_TextNoAutofit/a:noAutofit"))]
  ANoAutofit(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoAutoFit>,
  ),
  #[sdk(child(qname = "a:CT_TextNormalAutofit/a:normAutofit"))]
  ANormAutofit(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NormalAutoFit>,
  ),
  #[sdk(child(qname = "a:CT_TextShapeAutofit/a:spAutoFit"))]
  ASpAutoFit(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ShapeAutoFit>,
  ),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum TextBodyPropertiesChoice2 {
  #[sdk(child(qname = "a:CT_Shape3D/a:sp3d"))]
  ASp3d(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Shape3DType>,
  ),
  #[sdk(child(qname = "a:CT_FlatText/a:flatTx"))]
  AFlatTx(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FlatText>,
  ),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice {
  #[sdk(child(qname = "a:CT_CustomGeometry2D/a:custGeom"))]
  ACustGeom(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::CustomGeometry>,
  ),
  #[sdk(child(qname = "a:CT_PresetGeometry2D/a:prstGeom"))]
  APrstGeom(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetGeometry>,
  ),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice2 {
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoFill>),
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SolidFill>,
  ),
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GradientFill>,
  ),
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipFill>,
  ),
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PatternFill>,
  ),
  #[sdk(child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GroupFill>,
  ),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice3 {
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectList>,
  ),
  #[sdk(child(qname = "a:CT_EffectContainer/a:effectDag"))]
  AEffectDag(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectDag>,
  ),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum LnRefStyleMatrixReferenceChoice {
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
pub enum FillRefStyleMatrixReferenceChoice {
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
pub enum EffectRefStyleMatrixReferenceChoice {
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
pub enum StyleMatrixReferenceTypeChoice {
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
pub enum FontReferenceChoice {
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
pub enum BlipFillPropertiesChoice {
  #[sdk(child(qname = "a:CT_TileInfoProperties/a:tile"))]
  ATile(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Tile>),
  #[sdk(child(qname = "a:CT_StretchInfoProperties/a:stretch"))]
  AStretch(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Stretch>,
  ),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum GroupShapePropertiesChoice {
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoFill>),
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SolidFill>,
  ),
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GradientFill>,
  ),
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipFill>,
  ),
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PatternFill>,
  ),
  #[sdk(child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GroupFill>,
  ),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum GroupShapePropertiesChoice2 {
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectList>,
  ),
  #[sdk(child(qname = "a:CT_EffectContainer/a:effectDag"))]
  AEffectDag(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectDag>,
  ),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum LinePropertiesTypeChoice {
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoFill>),
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SolidFill>,
  ),
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GradientFill>,
  ),
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PatternFill>,
  ),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum LinePropertiesTypeChoice2 {
  #[sdk(child(qname = "a:CT_PresetLineDashProperties/a:prstDash"))]
  APrstDash(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetDash>,
  ),
  #[sdk(child(qname = "a:CT_DashStopList/a:custDash"))]
  ACustDash(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::CustomDash>,
  ),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum LinePropertiesTypeChoice3 {
  #[sdk(child(qname = "a:CT_LineJoinRound/a:round"))]
  ARound(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Round>),
  #[sdk(child(qname = "a:CT_LineJoinBevel/a:bevel"))]
  ABevel(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LineJoinBevel>,
  ),
  #[sdk(child(qname = "a:CT_LineJoinMiterProperties/a:miter"))]
  AMiter(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Miter>),
}
