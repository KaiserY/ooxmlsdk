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
#[sdk(qname = "oac:spMk")]
pub struct ShapeMoniker {
  /// id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// creationId
  #[sdk(attr(qname = ":creationId"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub creation_id: Option<crate::simple_type::StringValue>,
}
/// Defines the GroupShapeMoniker Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:grpSpMk")]
pub struct GroupShapeMoniker {
  /// id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// creationId
  #[sdk(attr(qname = ":creationId"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub creation_id: Option<crate::simple_type::StringValue>,
}
/// Defines the GraphicFrameMoniker Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:graphicFrameMk")]
pub struct GraphicFrameMoniker {
  /// id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// creationId
  #[sdk(attr(qname = ":creationId"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub creation_id: Option<crate::simple_type::StringValue>,
}
/// Defines the ConnectorMoniker Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:cxnSpMk")]
pub struct ConnectorMoniker {
  /// id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// creationId
  #[sdk(attr(qname = ":creationId"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub creation_id: Option<crate::simple_type::StringValue>,
}
/// Defines the PictureMoniker Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:picMk")]
pub struct PictureMoniker {
  /// id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// creationId
  #[sdk(attr(qname = ":creationId"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub creation_id: Option<crate::simple_type::StringValue>,
}
/// Defines the InkMoniker Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:inkMk")]
pub struct InkMoniker {
  /// id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// creationId
  #[sdk(attr(qname = ":creationId"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub creation_id: Option<crate::simple_type::StringValue>,
}
/// Defines the DrawingMonikerList Class.
pub type DrawingMonikerList = Vec<String>;
/// Defines the Transform2D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:xfrm")]
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
/// Defines the GroupShapeMonikerList Class.
pub type GroupShapeMonikerList = Vec<String>;
/// Defines the DeMkLstDrawingElementMonikerList Class.
pub type DeMkLstDrawingElementMonikerList = Vec<String>;
/// Defines the DeMasterMkLstDrawingElementMonikerList Class.
pub type DeMasterMkLstDrawingElementMonikerList = Vec<String>;
/// Defines the DeSrcMkLstDrawingElementMonikerList Class.
pub type DeSrcMkLstDrawingElementMonikerList = Vec<String>;
/// Defines the DeTgtMkLstDrawingElementMonikerList Class.
pub type DeTgtMkLstDrawingElementMonikerList = Vec<String>;
/// Defines the ImgDataImgData Class.
pub type ImgDataImgData = crate::simple_type::Base64BinaryValue;
/// Defines the OrigImgDataImgData Class.
pub type OrigImgDataImgData = crate::simple_type::Base64BinaryValue;
/// Defines the SndDataImgData Class.
pub type SndDataImgData = crate::simple_type::Base64BinaryValue;
/// Defines the ResourceUrl Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:imgUrl")]
pub struct ResourceUrl {
  /// src
  #[sdk(attr(qname = ":src"))]
  #[sdk(string_format(kind = "uri"))]
  pub src: Option<crate::simple_type::StringValue>,
  /// linkage
  #[sdk(attr(qname = ":linkage"))]
  #[sdk(string_format(kind = "token"))]
  pub linkage: Option<ResourceLinkage>,
}
/// Defines the GroupCommand Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:grpCmd")]
pub struct GroupCommand {
  /// verId
  #[sdk(attr(qname = ":verId"))]
  pub ver_id: Option<crate::simple_type::UInt32Value>,
  /// preventRegroup
  #[sdk(attr(qname = ":preventRegroup"))]
  pub prevent_regroup: Option<crate::simple_type::BooleanValue>,
  /// grpId
  #[sdk(attr(qname = ":grpId"))]
  pub grp_id: Option<crate::simple_type::UInt32Value>,
  /// Defines the DrawingMonikerList Class.
  #[sdk(any_child(qname = "oac:dgMkLst"))]
  pub drawing_moniker_list: DrawingMonikerList,
  #[sdk(
        choice(
            child(variant = ShapeMoniker, qname = "oac:spMk"),
            child(variant = GroupShapeMoniker, qname = "oac:grpSpMk"),
            child(variant = GraphicFrameMoniker, qname = "oac:graphicFrameMk"),
            child(variant = ConnectorMoniker, qname = "oac:cxnSpMk"),
            child(variant = PictureMoniker, qname = "oac:picMk"),
            child(variant = InkMoniker, qname = "oac:inkMk")
        )
    )]
  pub group_command_choice: Vec<GroupCommandChoice>,
  /// Defines the GroupShapeProperties Class.
  #[sdk(child(qname = "oac:grpSpPr"))]
  pub group_shape_properties: Option<std::boxed::Box<GroupShapeProperties>>,
  /// Defines the NonVisualDrawingProps Class.
  #[sdk(child(qname = "oac:cNvPr"))]
  pub non_visual_drawing_props: Option<std::boxed::Box<NonVisualDrawingProps>>,
  /// Defines the NonVisualGroupDrawingShapeProps Class.
  #[sdk(child(qname = "oac:cNvGrpSpPr"))]
  pub non_visual_group_drawing_shape_props:
    Option<std::boxed::Box<NonVisualGroupDrawingShapeProps>>,
}
/// Defines the ImgLink Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:imgLink")]
pub struct ImgLink {
  /// tgt
  #[sdk(attr(qname = ":tgt"))]
  #[sdk(string_format(kind = "uri"))]
  pub tgt: crate::simple_type::StringValue,
}
/// Defines the DocumentContextMonikerList Class.
pub type DocumentContextMonikerList = Vec<String>;
/// Defines the GraphicParentMonikerList Class.
pub type GraphicParentMonikerList = Vec<String>;
/// Defines the ShapeMonikerList Class.
pub type ShapeMonikerList = Vec<String>;
/// Defines the GraphicFrameMonikerList Class.
pub type GraphicFrameMonikerList = Vec<String>;
/// Defines the ConnectorMonikerList Class.
pub type ConnectorMonikerList = Vec<String>;
/// Defines the PictureMonikerList Class.
pub type PictureMonikerList = Vec<String>;
/// Defines the InkMonikerList Class.
pub type InkMonikerList = Vec<String>;
/// Defines the TextBodyMonikerList Class.
pub type TextBodyMonikerList = Vec<String>;
/// Defines the TextCharRangeMonikerList Class.
pub type TextCharRangeMonikerList = Vec<String>;
/// Defines the HyperlinkMonikerList Class.
pub type HyperlinkMonikerList = Vec<String>;
/// Defines the Model3DMonikerList Class.
pub type Model3DMonikerList = Vec<String>;
/// Defines the ViewSelectionStgList Class.
pub type ViewSelectionStgList = Vec<String>;
/// Defines the EditorSelectionStgList Class.
pub type EditorSelectionStgList = Vec<String>;
/// Defines the DrawingSelectionStgList Class.
pub type DrawingSelectionStgList = Vec<String>;
/// Defines the TableMonikerList Class.
pub type TableMonikerList = Vec<String>;
/// Defines the TableCellMonikerList Class.
pub type TableCellMonikerList = Vec<String>;
/// Defines the TableRowMonikerList Class.
pub type TableRowMonikerList = Vec<String>;
/// Defines the TableColumnMonikerList Class.
pub type TableColumnMonikerList = Vec<String>;
/// Defines the ModifyNonVisualDrawingProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:cNvPr")]
pub struct ModifyNonVisualDrawingProps {
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// descr
  #[sdk(attr(qname = ":descr"))]
  pub descr: Option<crate::simple_type::StringValue>,
  /// hidden
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  /// title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// decor
  #[sdk(attr(qname = ":decor"))]
  pub decor: Option<crate::simple_type::BooleanValue>,
  /// scriptLink
  #[sdk(attr(qname = ":scriptLink"))]
  pub script_link: Option<crate::simple_type::StringValue>,
}
/// Defines the ModifyTransformProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:xfrm")]
pub struct ModifyTransformProps {
  /// x
  #[sdk(attr(qname = ":x"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub x: Option<crate::simple_type::Int64Value>,
  /// y
  #[sdk(attr(qname = ":y"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub y: Option<crate::simple_type::Int64Value>,
  /// cx
  #[sdk(attr(qname = ":cx"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub cx: Option<crate::simple_type::Int64Value>,
  /// cy
  #[sdk(attr(qname = ":cy"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub cy: Option<crate::simple_type::Int64Value>,
  /// rot
  #[sdk(attr(qname = ":rot"))]
  pub rot: Option<crate::simple_type::Int32Value>,
  /// flipH
  #[sdk(attr(qname = ":flipH"))]
  pub flip_h: Option<crate::simple_type::BooleanValue>,
  /// flipV
  #[sdk(attr(qname = ":flipV"))]
  pub flip_v: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Point2DType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:off")]
pub struct Point2DType {
  /// X-Axis Coordinate
  #[sdk(attr(qname = ":x"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub x: crate::simple_type::Int64Value,
  /// Y-Axis Coordinate
  #[sdk(attr(qname = ":y"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub y: crate::simple_type::Int64Value,
}
/// Defines the TextParagraphPropertiesType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:pPr")]
pub struct TextParagraphPropertiesType {
  /// Left Margin
  #[sdk(attr(qname = ":marL"))]
  #[sdk(number_range(range = 0..= 51206400))]
  pub left_margin: Option<crate::simple_type::Int32Value>,
  /// Right Margin
  #[sdk(attr(qname = ":marR"))]
  #[sdk(number_range(range = 0..= 51206400))]
  pub right_margin: Option<crate::simple_type::Int32Value>,
  /// Level
  #[sdk(attr(qname = ":lvl"))]
  #[sdk(number_range(range = 0..= 8))]
  pub level: Option<crate::simple_type::Int32Value>,
  /// Indent
  #[sdk(attr(qname = ":indent"))]
  #[sdk(number_range(range = -51206400..= 51206400))]
  pub indent: Option<crate::simple_type::Int32Value>,
  /// Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(kind = "token"))]
  pub alignment: Option<crate::schemas::a::TextAlignmentTypeValues>,
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
  #[sdk(string_format(kind = "token"))]
  pub font_alignment: Option<crate::schemas::a::TextFontAlignmentValues>,
  /// Latin Line Break
  #[sdk(attr(qname = ":latinLnBrk"))]
  pub latin_line_break: Option<crate::simple_type::BooleanValue>,
  /// Hanging Punctuation
  #[sdk(attr(qname = ":hangingPunct"))]
  pub height: Option<crate::simple_type::BooleanValue>,
  /// Line Spacing
  #[sdk(child(qname = "a:lnSpc"))]
  pub line_spacing: Option<std::boxed::Box<crate::schemas::a::LineSpacing>>,
  /// Space Before
  #[sdk(child(qname = "a:spcBef"))]
  pub space_before: Option<std::boxed::Box<crate::schemas::a::SpaceBefore>>,
  /// Space After
  #[sdk(child(qname = "a:spcAft"))]
  pub space_after: Option<std::boxed::Box<crate::schemas::a::SpaceAfter>>,
  #[sdk(
        choice(
            empty_child(variant = BulletColorText, qname = "a:buClrTx"),
            child(variant = BulletColor, qname = "a:buClr")
        )
    )]
  pub text_paragraph_properties_type_choice1: Option<TextParagraphPropertiesTypeChoice>,
  #[sdk(
        choice(
            empty_child(variant = BulletSizeText, qname = "a:buSzTx"),
            child(variant = BulletSizePercentage, qname = "a:buSzPct"),
            child(variant = BulletSizePoints, qname = "a:buSzPts")
        )
    )]
  pub text_paragraph_properties_type_choice2: Option<TextParagraphPropertiesTypeChoice2>,
  #[sdk(
        choice(
            empty_child(variant = BulletFontText, qname = "a:buFontTx"),
            child(variant = BulletFont, qname = "a:buFont")
        )
    )]
  pub text_paragraph_properties_type_choice3: Option<TextParagraphPropertiesTypeChoice3>,
  #[sdk(
        choice(
            empty_child(variant = NoBullet, qname = "a:buNone"),
            child(variant = AutoNumberedBullet, qname = "a:buAutoNum"),
            child(variant = CharacterBullet, qname = "a:buChar"),
            child(variant = PictureBullet, qname = "a:buBlip")
        )
    )]
  pub text_paragraph_properties_type_choice4: Option<TextParagraphPropertiesTypeChoice4>,
  /// Tab List.
  #[sdk(child(qname = "a:tabLst"))]
  pub tab_stop_list: Option<crate::schemas::a::TabStopList>,
  /// Default Text Run Properties.
  #[sdk(child(qname = "a:defRPr"))]
  pub default_run_properties: Option<std::boxed::Box<crate::schemas::a::DefaultRunProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Defines the TextBodyProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:bodyPr")]
pub struct TextBodyProperties {
  /// Rotation
  #[sdk(attr(qname = ":rot"))]
  pub rotation: Option<crate::simple_type::Int32Value>,
  /// Paragraph Spacing
  #[sdk(attr(qname = ":spcFirstLastPara"))]
  pub use_paragraph_spacing: Option<crate::simple_type::BooleanValue>,
  /// Text Vertical Overflow
  #[sdk(attr(qname = ":vertOverflow"))]
  #[sdk(string_format(kind = "token"))]
  pub vertical_overflow: Option<crate::schemas::a::TextVerticalOverflowValues>,
  /// Text Horizontal Overflow
  #[sdk(attr(qname = ":horzOverflow"))]
  #[sdk(string_format(kind = "token"))]
  pub horizontal_overflow: Option<crate::schemas::a::TextHorizontalOverflowValues>,
  /// Vertical Text
  #[sdk(attr(qname = ":vert"))]
  #[sdk(string_format(kind = "token"))]
  pub vertical: Option<crate::schemas::a::TextVerticalValues>,
  /// Text Wrapping Type
  #[sdk(attr(qname = ":wrap"))]
  #[sdk(string_format(kind = "token"))]
  pub wrap: Option<crate::schemas::a::TextWrappingValues>,
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
  #[sdk(number_range(range = 1..= 16))]
  pub column_count: Option<crate::simple_type::Int32Value>,
  /// Space Between Columns
  #[sdk(attr(qname = ":spcCol"))]
  #[sdk(number_range(range = 0..))]
  pub column_spacing: Option<crate::simple_type::Int32Value>,
  /// Columns Right-To-Left
  #[sdk(attr(qname = ":rtlCol"))]
  pub right_to_left_columns: Option<crate::simple_type::BooleanValue>,
  /// From WordArt
  #[sdk(attr(qname = ":fromWordArt"))]
  pub from_word_art: Option<crate::simple_type::BooleanValue>,
  /// Anchor
  #[sdk(attr(qname = ":anchor"))]
  #[sdk(string_format(kind = "token"))]
  pub anchor: Option<crate::schemas::a::TextAnchoringTypeValues>,
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
  #[sdk(child(qname = "a:prstTxWarp"))]
  pub preset_text_warp: Option<std::boxed::Box<crate::schemas::a::PresetTextWarp>>,
  #[sdk(
        choice(
            empty_child(variant = NoAutoFit, qname = "a:noAutofit"),
            child(variant = NormalAutoFit, qname = "a:normAutofit"),
            empty_child(variant = ShapeAutoFit, qname = "a:spAutoFit")
        )
    )]
  pub text_body_properties_choice1: Option<TextBodyPropertiesChoice>,
  /// 3D Scene Properties.
  #[sdk(child(qname = "a:scene3d"))]
  pub scene3_d_type: Option<std::boxed::Box<crate::schemas::a::Scene3DType>>,
  #[sdk(
        choice(
            child(variant = Shape3DType, qname = "a:sp3d"),
            child(variant = FlatText, qname = "a:flatTx")
        )
    )]
  pub text_body_properties_choice2: Option<TextBodyPropertiesChoice2>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Defines the ModifyNonVisualDrawingShapeProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:cNvSpPr")]
pub struct ModifyNonVisualDrawingShapeProps {
  /// noGrp
  #[sdk(attr(qname = ":noGrp"))]
  pub no_grp: Option<crate::simple_type::BooleanValue>,
  /// noSelect
  #[sdk(attr(qname = ":noSelect"))]
  pub no_select: Option<crate::simple_type::BooleanValue>,
  /// noRot
  #[sdk(attr(qname = ":noRot"))]
  pub no_rot: Option<crate::simple_type::BooleanValue>,
  /// noChangeAspect
  #[sdk(attr(qname = ":noChangeAspect"))]
  pub no_change_aspect: Option<crate::simple_type::BooleanValue>,
  /// noMove
  #[sdk(attr(qname = ":noMove"))]
  pub no_move: Option<crate::simple_type::BooleanValue>,
  /// noResize
  #[sdk(attr(qname = ":noResize"))]
  pub no_resize: Option<crate::simple_type::BooleanValue>,
  /// noEditPoints
  #[sdk(attr(qname = ":noEditPoints"))]
  pub no_edit_points: Option<crate::simple_type::BooleanValue>,
  /// noAdjustHandles
  #[sdk(attr(qname = ":noAdjustHandles"))]
  pub no_adjust_handles: Option<crate::simple_type::BooleanValue>,
  /// noChangeArrowheads
  #[sdk(attr(qname = ":noChangeArrowheads"))]
  pub no_change_arrowheads: Option<crate::simple_type::BooleanValue>,
  /// noChangeShapeType
  #[sdk(attr(qname = ":noChangeShapeType"))]
  pub no_change_shape_type: Option<crate::simple_type::BooleanValue>,
  /// noTextEdit
  #[sdk(attr(qname = ":noTextEdit"))]
  pub no_text_edit: Option<crate::simple_type::BooleanValue>,
  /// txBox
  #[sdk(attr(qname = ":txBox"))]
  pub tx_box: Option<crate::simple_type::BooleanValue>,
}
/// Defines the ShapePropsMonikerList Class.
pub type ShapePropsMonikerList = Vec<String>;
/// Defines the ShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:spPr")]
pub struct ShapeProperties {
  /// Black and White Mode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(kind = "token"))]
  pub black_white_mode: Option<crate::schemas::a::BlackWhiteModeValues>,
  /// 2D Transform for Individual Objects
  #[sdk(child(qname = "a:xfrm"))]
  pub transform2_d: Option<std::boxed::Box<crate::schemas::a::Transform2D>>,
  #[sdk(
        choice(
            child(variant = CustomGeometry, qname = "a:custGeom"),
            child(variant = PresetGeometry, qname = "a:prstGeom")
        )
    )]
  pub shape_properties_choice1: Option<ShapePropertiesChoice>,
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, qname = "a:solidFill"),
            child(variant = GradientFill, qname = "a:gradFill"),
            child(variant = BlipFill, qname = "a:blipFill"),
            child(variant = PatternFill, qname = "a:pattFill"),
            empty_child(variant = GroupFill, qname = "a:grpFill")
        )
    )]
  pub shape_properties_choice2: Option<ShapePropertiesChoice2>,
  /// Defines the Outline Class.
  #[sdk(child(qname = "a:ln"))]
  pub outline: Option<std::boxed::Box<crate::schemas::a::Outline>>,
  #[sdk(
        choice(
            child(variant = EffectList, qname = "a:effectLst"),
            child(variant = EffectDag, qname = "a:effectDag")
        )
    )]
  pub shape_properties_choice3: Option<ShapePropertiesChoice3>,
  /// 3D Scene Properties.
  #[sdk(child(qname = "a:scene3d"))]
  pub scene3_d_type: Option<std::boxed::Box<crate::schemas::a::Scene3DType>>,
  /// Apply 3D shape properties.
  #[sdk(child(qname = "a:sp3d"))]
  pub shape3_d_type: Option<std::boxed::Box<crate::schemas::a::Shape3DType>>,
  /// Defines the ShapePropertiesExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub shape_properties_extension_list: Option<crate::schemas::a::ShapePropertiesExtensionList>,
}
/// Defines the ResetShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:spPr")]
pub struct ResetShapeProperties {
  /// Defines the XfrmEmpty Class.
  #[sdk(empty_child(qname = "oac:xfrm"))]
  pub xfrm_empty: Option<()>,
  /// Defines the GeomEmpty Class.
  #[sdk(empty_child(qname = "oac:geom"))]
  pub geom_empty: Option<()>,
  /// Defines the FillEmpty Class.
  #[sdk(empty_child(qname = "oac:fill"))]
  pub fill_empty: Option<()>,
  /// Defines the LnEmpty Class.
  #[sdk(empty_child(qname = "oac:ln"))]
  pub ln_empty: Option<()>,
  /// Defines the EffectEmpty Class.
  #[sdk(empty_child(qname = "oac:effect"))]
  pub effect_empty: Option<()>,
  /// Defines the Scene3dEmpty Class.
  #[sdk(empty_child(qname = "oac:scene3d"))]
  pub scene3d_empty: Option<()>,
  /// Defines the Sp3dEmpty Class.
  #[sdk(empty_child(qname = "oac:sp3d"))]
  pub sp3d_empty: Option<()>,
  /// Defines the ExtLstEmpty Class.
  #[sdk(empty_child(qname = "oac:extLst"))]
  pub ext_lst_empty: Option<()>,
  /// Defines the BwModeEmpty Class.
  #[sdk(empty_child(qname = "oac:bwMode"))]
  pub bw_mode_empty: Option<()>,
}
/// Defines the LnRefStyleMatrixReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:lnRef")]
pub struct LnRefStyleMatrixReference {
  /// Style Matrix Index
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub ln_ref_style_matrix_reference_choice: Option<LnRefStyleMatrixReferenceChoice>,
}
/// Defines the FillRefStyleMatrixReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:fillRef")]
pub struct FillRefStyleMatrixReference {
  /// Style Matrix Index
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub fill_ref_style_matrix_reference_choice: Option<FillRefStyleMatrixReferenceChoice>,
}
/// Defines the EffectRefStyleMatrixReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:effectRef")]
pub struct EffectRefStyleMatrixReference {
  /// Style Matrix Index
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub effect_ref_style_matrix_reference_choice: Option<EffectRefStyleMatrixReferenceChoice>,
}
/// Defines the FontReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:fontRef")]
pub struct FontReference {
  /// Identifier
  #[sdk(attr(qname = ":idx"))]
  #[sdk(string_format(kind = "token"))]
  pub index: crate::schemas::a::FontCollectionIndexValues,
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub font_reference_choice: Option<FontReferenceChoice>,
}
/// Defines the ModifyShapeStyleProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:style")]
pub struct ModifyShapeStyleProps {
  /// Defines the LnRefStyleMatrixReference Class.
  #[sdk(child(qname = "oac:lnRef"))]
  pub ln_ref_style_matrix_reference: Option<std::boxed::Box<LnRefStyleMatrixReference>>,
  /// Defines the FillRefStyleMatrixReference Class.
  #[sdk(child(qname = "oac:fillRef"))]
  pub fill_ref_style_matrix_reference: Option<std::boxed::Box<FillRefStyleMatrixReference>>,
  /// Defines the EffectRefStyleMatrixReference Class.
  #[sdk(child(qname = "oac:effectRef"))]
  pub effect_ref_style_matrix_reference: Option<std::boxed::Box<EffectRefStyleMatrixReference>>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "oac:fontRef"))]
  pub font_reference: Option<std::boxed::Box<FontReference>>,
}
/// Defines the ResetXsdboolean Class.
pub type ResetXsdboolean = crate::simple_type::BooleanValue;
/// Defines the UseBoundsXsdboolean Class.
pub type UseBoundsXsdboolean = crate::simple_type::BooleanValue;
/// Defines the BlipFillProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:blipFill")]
pub struct BlipFillProperties {
  /// DPI Setting
  #[sdk(attr(qname = ":dpi"))]
  pub dpi: Option<crate::simple_type::UInt32Value>,
  /// Rotate With Shape
  #[sdk(attr(qname = ":rotWithShape"))]
  pub rotate_with_shape: Option<crate::simple_type::BooleanValue>,
  /// Defines the Blip Class.
  #[sdk(child(qname = "a:blip"))]
  pub blip: Option<std::boxed::Box<crate::schemas::a::Blip>>,
  /// Source Rectangle
  #[sdk(child(qname = "a:srcRect"))]
  pub source_rectangle: Option<crate::schemas::a::SourceRectangle>,
  #[sdk(
        choice(
            child(variant = Tile, qname = "a:tile"),
            child(variant = Stretch, qname = "a:stretch")
        )
    )]
  pub blip_fill_properties_choice: Option<BlipFillPropertiesChoice>,
}
/// Defines the FillRectRelativeRectProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:fillRect")]
pub struct FillRectRelativeRectProps {
  /// l
  #[sdk(attr(qname = ":l"))]
  pub l: Option<crate::simple_type::Int32Value>,
  /// t
  #[sdk(attr(qname = ":t"))]
  pub t: Option<crate::simple_type::Int32Value>,
  /// r
  #[sdk(attr(qname = ":r"))]
  pub r: Option<crate::simple_type::Int32Value>,
  /// b
  #[sdk(attr(qname = ":b"))]
  pub b: Option<crate::simple_type::Int32Value>,
}
/// Defines the SrcRectRelativeRectProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:srcRect")]
pub struct SrcRectRelativeRectProps {
  /// l
  #[sdk(attr(qname = ":l"))]
  pub l: Option<crate::simple_type::Int32Value>,
  /// t
  #[sdk(attr(qname = ":t"))]
  pub t: Option<crate::simple_type::Int32Value>,
  /// r
  #[sdk(attr(qname = ":r"))]
  pub r: Option<crate::simple_type::Int32Value>,
  /// b
  #[sdk(attr(qname = ":b"))]
  pub b: Option<crate::simple_type::Int32Value>,
}
/// Defines the ResetBlipFillProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:blipFill")]
pub struct ResetBlipFillProperties {
  /// Defines the SrcRectEmpty Class.
  #[sdk(empty_child(qname = "oac:srcRect"))]
  pub src_rect_empty: Option<()>,
  /// Defines the FillModeEmpty Class.
  #[sdk(empty_child(qname = "oac:fillMode"))]
  pub fill_mode_empty: Option<()>,
  /// Defines the DpiEmpty Class.
  #[sdk(empty_child(qname = "oac:dpi"))]
  pub dpi_empty: Option<()>,
  /// Defines the RotWithShapeEmpty Class.
  #[sdk(empty_child(qname = "oac:rotWithShape"))]
  pub rot_with_shape_empty: Option<()>,
}
/// Defines the ModifyNonVisualGroupDrawingShapeProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:cNvGrpSpPr")]
pub struct ModifyNonVisualGroupDrawingShapeProps {
  /// noGrp
  #[sdk(attr(qname = ":noGrp"))]
  pub no_grp: Option<crate::simple_type::BooleanValue>,
  /// noUngrp
  #[sdk(attr(qname = ":noUngrp"))]
  pub no_ungrp: Option<crate::simple_type::BooleanValue>,
  /// noSelect
  #[sdk(attr(qname = ":noSelect"))]
  pub no_select: Option<crate::simple_type::BooleanValue>,
  /// noRot
  #[sdk(attr(qname = ":noRot"))]
  pub no_rot: Option<crate::simple_type::BooleanValue>,
  /// noChangeAspect
  #[sdk(attr(qname = ":noChangeAspect"))]
  pub no_change_aspect: Option<crate::simple_type::BooleanValue>,
  /// noMove
  #[sdk(attr(qname = ":noMove"))]
  pub no_move: Option<crate::simple_type::BooleanValue>,
  /// noResize
  #[sdk(attr(qname = ":noResize"))]
  pub no_resize: Option<crate::simple_type::BooleanValue>,
}
/// Defines the GroupShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:grpSpPr")]
pub struct GroupShapeProperties {
  /// Black and White Mode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(kind = "token"))]
  pub black_white_mode: Option<crate::schemas::a::BlackWhiteModeValues>,
  /// 2D Transform for Grouped Objects
  #[sdk(child(qname = "a:xfrm"))]
  pub transform_group: Option<std::boxed::Box<crate::schemas::a::TransformGroup>>,
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, qname = "a:solidFill"),
            child(variant = GradientFill, qname = "a:gradFill"),
            child(variant = BlipFill, qname = "a:blipFill"),
            child(variant = PatternFill, qname = "a:pattFill"),
            empty_child(variant = GroupFill, qname = "a:grpFill")
        )
    )]
  pub group_shape_properties_choice1: Option<GroupShapePropertiesChoice>,
  #[sdk(
        choice(
            child(variant = EffectList, qname = "a:effectLst"),
            child(variant = EffectDag, qname = "a:effectDag")
        )
    )]
  pub group_shape_properties_choice2: Option<GroupShapePropertiesChoice2>,
  /// 3D Scene Properties.
  #[sdk(child(qname = "a:scene3d"))]
  pub scene3_d_type: Option<std::boxed::Box<crate::schemas::a::Scene3DType>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Defines the ResetGroupShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:grpSpPr")]
pub struct ResetGroupShapeProperties {
  /// Defines the XfrmEmpty Class.
  #[sdk(empty_child(qname = "oac:xfrm"))]
  pub xfrm_empty: Option<()>,
  /// Defines the FillEmpty Class.
  #[sdk(empty_child(qname = "oac:fill"))]
  pub fill_empty: Option<()>,
  /// Defines the EffectEmpty Class.
  #[sdk(empty_child(qname = "oac:effect"))]
  pub effect_empty: Option<()>,
  /// Defines the Scene3dEmpty Class.
  #[sdk(empty_child(qname = "oac:scene3d"))]
  pub scene3d_empty: Option<()>,
  /// Defines the ExtLstEmpty Class.
  #[sdk(empty_child(qname = "oac:extLst"))]
  pub ext_lst_empty: Option<()>,
  /// Defines the BwModeEmpty Class.
  #[sdk(empty_child(qname = "oac:bwMode"))]
  pub bw_mode_empty: Option<()>,
}
/// Defines the NonVisualDrawingProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:cNvPr")]
pub struct NonVisualDrawingProps {
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
/// Defines the NonVisualGroupDrawingShapeProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:cNvGrpSpPr")]
pub struct NonVisualGroupDrawingShapeProps {
  /// Defines the GroupShapeLocks Class.
  #[sdk(child(qname = "a:grpSpLocks"))]
  pub group_shape_locks: Option<std::boxed::Box<crate::schemas::a::GroupShapeLocks>>,
  /// Defines the NonVisualGroupDrawingShapePropsExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub non_visual_group_drawing_shape_props_extension_list:
    Option<crate::schemas::a::NonVisualGroupDrawingShapePropsExtensionList>,
}
/// Defines the ModifyNonVisualGraphicFrameProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:cNvGraphicFramePr")]
pub struct ModifyNonVisualGraphicFrameProps {
  /// noGrp
  #[sdk(attr(qname = ":noGrp"))]
  pub no_grp: Option<crate::simple_type::BooleanValue>,
  /// noDrilldown
  #[sdk(attr(qname = ":noDrilldown"))]
  pub no_drilldown: Option<crate::simple_type::BooleanValue>,
  /// noSelect
  #[sdk(attr(qname = ":noSelect"))]
  pub no_select: Option<crate::simple_type::BooleanValue>,
  /// noChangeAspect
  #[sdk(attr(qname = ":noChangeAspect"))]
  pub no_change_aspect: Option<crate::simple_type::BooleanValue>,
  /// noMove
  #[sdk(attr(qname = ":noMove"))]
  pub no_move: Option<crate::simple_type::BooleanValue>,
  /// noResize
  #[sdk(attr(qname = ":noResize"))]
  pub no_resize: Option<crate::simple_type::BooleanValue>,
}
/// Defines the StCxnConnection Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:stCxn")]
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
#[sdk(qname = "oac:endCxn")]
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
#[sdk(qname = "oac:cNvCxnSpPr")]
pub struct ModifyNonVisualConnectorProps {
  /// noGrp
  #[sdk(attr(qname = ":noGrp"))]
  pub no_grp: Option<crate::simple_type::BooleanValue>,
  /// noSelect
  #[sdk(attr(qname = ":noSelect"))]
  pub no_select: Option<crate::simple_type::BooleanValue>,
  /// noRot
  #[sdk(attr(qname = ":noRot"))]
  pub no_rot: Option<crate::simple_type::BooleanValue>,
  /// noChangeAspect
  #[sdk(attr(qname = ":noChangeAspect"))]
  pub no_change_aspect: Option<crate::simple_type::BooleanValue>,
  /// noMove
  #[sdk(attr(qname = ":noMove"))]
  pub no_move: Option<crate::simple_type::BooleanValue>,
  /// noResize
  #[sdk(attr(qname = ":noResize"))]
  pub no_resize: Option<crate::simple_type::BooleanValue>,
  /// noEditPoints
  #[sdk(attr(qname = ":noEditPoints"))]
  pub no_edit_points: Option<crate::simple_type::BooleanValue>,
  /// noAdjustHandles
  #[sdk(attr(qname = ":noAdjustHandles"))]
  pub no_adjust_handles: Option<crate::simple_type::BooleanValue>,
  /// noChangeArrowheads
  #[sdk(attr(qname = ":noChangeArrowheads"))]
  pub no_change_arrowheads: Option<crate::simple_type::BooleanValue>,
  /// noChangeShapeType
  #[sdk(attr(qname = ":noChangeShapeType"))]
  pub no_change_shape_type: Option<crate::simple_type::BooleanValue>,
  /// Defines the StCxnConnection Class.
  #[sdk(child(qname = "oac:stCxn"))]
  pub st_cxn_connection: Option<StCxnConnection>,
  /// Defines the EndCxnConnection Class.
  #[sdk(child(qname = "oac:endCxn"))]
  pub end_cxn_connection: Option<EndCxnConnection>,
}
/// Defines the ResetNonVisualConnectorProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:cNvCxnSpPr")]
pub struct ResetNonVisualConnectorProps {
  /// Defines the StCxnEmpty Class.
  #[sdk(empty_child(qname = "oac:stCxn"))]
  pub st_cxn_empty: Option<()>,
  /// Defines the EndCxnEmpty Class.
  #[sdk(empty_child(qname = "oac:endCxn"))]
  pub end_cxn_empty: Option<()>,
  /// Defines the NoGrpEmpty Class.
  #[sdk(empty_child(qname = "oac:noGrp"))]
  pub no_grp_empty: Option<()>,
  /// Defines the NoSelectEmpty Class.
  #[sdk(empty_child(qname = "oac:noSelect"))]
  pub no_select_empty: Option<()>,
  /// Defines the NoRotEmpty Class.
  #[sdk(empty_child(qname = "oac:noRot"))]
  pub no_rot_empty: Option<()>,
  /// Defines the NoChangeAspectEmpty Class.
  #[sdk(empty_child(qname = "oac:noChangeAspect"))]
  pub no_change_aspect_empty: Option<()>,
  /// Defines the NoMoveEmpty Class.
  #[sdk(empty_child(qname = "oac:noMove"))]
  pub no_move_empty: Option<()>,
  /// Defines the NoResizeEmpty Class.
  #[sdk(empty_child(qname = "oac:noResize"))]
  pub no_resize_empty: Option<()>,
  /// Defines the NoEditPointsEmpty Class.
  #[sdk(empty_child(qname = "oac:noEditPoints"))]
  pub no_edit_points_empty: Option<()>,
  /// Defines the NoAdjustHandlesEmpty Class.
  #[sdk(empty_child(qname = "oac:noAdjustHandles"))]
  pub no_adjust_handles_empty: Option<()>,
  /// Defines the NoChangeArrowheadsEmpty Class.
  #[sdk(empty_child(qname = "oac:noChangeArrowheads"))]
  pub no_change_arrowheads_empty: Option<()>,
  /// Defines the NoChangeShapeTypeEmpty Class.
  #[sdk(empty_child(qname = "oac:noChangeShapeType"))]
  pub no_change_shape_type_empty: Option<()>,
}
/// Defines the CompressPictureProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:compressPicPr")]
pub struct CompressPictureProps {
  /// removeCrop
  #[sdk(attr(qname = ":removeCrop"))]
  pub remove_crop: Option<crate::simple_type::BooleanValue>,
  /// useLocalDpi
  #[sdk(attr(qname = ":useLocalDpi"))]
  pub use_local_dpi: Option<crate::simple_type::BooleanValue>,
  /// cstate
  #[sdk(attr(qname = ":cstate"))]
  #[sdk(string_format(kind = "token"))]
  pub cstate: Option<crate::schemas::a::BlipCompressionValues>,
}
/// Defines the ModifyNonVisualPictureProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:cNvPicPr")]
pub struct ModifyNonVisualPictureProps {
  /// noGrp
  #[sdk(attr(qname = ":noGrp"))]
  pub no_grp: Option<crate::simple_type::BooleanValue>,
  /// noSelect
  #[sdk(attr(qname = ":noSelect"))]
  pub no_select: Option<crate::simple_type::BooleanValue>,
  /// noRot
  #[sdk(attr(qname = ":noRot"))]
  pub no_rot: Option<crate::simple_type::BooleanValue>,
  /// noChangeAspect
  #[sdk(attr(qname = ":noChangeAspect"))]
  pub no_change_aspect: Option<crate::simple_type::BooleanValue>,
  /// noMove
  #[sdk(attr(qname = ":noMove"))]
  pub no_move: Option<crate::simple_type::BooleanValue>,
  /// noResize
  #[sdk(attr(qname = ":noResize"))]
  pub no_resize: Option<crate::simple_type::BooleanValue>,
  /// noEditPoints
  #[sdk(attr(qname = ":noEditPoints"))]
  pub no_edit_points: Option<crate::simple_type::BooleanValue>,
  /// noAdjustHandles
  #[sdk(attr(qname = ":noAdjustHandles"))]
  pub no_adjust_handles: Option<crate::simple_type::BooleanValue>,
  /// noChangeArrowheads
  #[sdk(attr(qname = ":noChangeArrowheads"))]
  pub no_change_arrowheads: Option<crate::simple_type::BooleanValue>,
  /// noChangeShapeType
  #[sdk(attr(qname = ":noChangeShapeType"))]
  pub no_change_shape_type: Option<crate::simple_type::BooleanValue>,
  /// noCrop
  #[sdk(attr(qname = ":noCrop"))]
  pub no_crop: Option<crate::simple_type::BooleanValue>,
  /// preferRelativeResize
  #[sdk(attr(qname = ":preferRelativeResize"))]
  pub prefer_relative_resize: Option<crate::simple_type::BooleanValue>,
}
/// Defines the ResetNonVisualPictureProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:cNvPicPr")]
pub struct ResetNonVisualPictureProps {
  /// Defines the LfPrEmpty Class.
  #[sdk(empty_child(qname = "oac:lfPr"))]
  pub lf_pr_empty: Option<()>,
}
/// Defines the BoundRect Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:bounds")]
pub struct BoundRect {
  /// l
  #[sdk(attr(qname = ":l"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub l: crate::simple_type::Int64Value,
  /// t
  #[sdk(attr(qname = ":t"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub t: crate::simple_type::Int64Value,
  /// r
  #[sdk(attr(qname = ":r"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub r: crate::simple_type::Int64Value,
  /// b
  #[sdk(attr(qname = ":b"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub b: crate::simple_type::Int64Value,
}
/// Defines the SVGBlipMonikerList Class.
pub type SvgBlipMonikerList = Vec<String>;
/// Defines the LinePropertiesType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:lineProps")]
pub struct LinePropertiesType {
  /// line width
  #[sdk(attr(qname = ":w"))]
  #[sdk(number_range(range = 0..= 20116800))]
  pub width: Option<crate::simple_type::Int32Value>,
  /// line cap
  #[sdk(attr(qname = ":cap"))]
  #[sdk(string_format(kind = "token"))]
  pub cap_type: Option<crate::schemas::a::LineCapValues>,
  /// compound line type
  #[sdk(attr(qname = ":cmpd"))]
  #[sdk(string_format(kind = "token"))]
  pub compound_line_type: Option<crate::schemas::a::CompoundLineValues>,
  /// pen alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(kind = "token"))]
  pub alignment: Option<crate::schemas::a::PenAlignmentValues>,
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, qname = "a:solidFill"),
            child(variant = GradientFill, qname = "a:gradFill"),
            child(variant = PatternFill, qname = "a:pattFill")
        )
    )]
  pub line_properties_type_choice1: Option<LinePropertiesTypeChoice>,
  #[sdk(
        choice(
            child(variant = PresetDash, qname = "a:prstDash"),
            child(variant = CustomDash, qname = "a:custDash")
        )
    )]
  pub line_properties_type_choice2: Option<LinePropertiesTypeChoice2>,
  #[sdk(
        choice(
            empty_child(variant = Round, qname = "a:round"),
            empty_child(variant = LineJoinBevel, qname = "a:bevel"),
            child(variant = Miter, qname = "a:miter")
        )
    )]
  pub line_properties_type_choice3: Option<LinePropertiesTypeChoice3>,
  /// default head line end style is none.
  #[sdk(child(qname = "a:headEnd"))]
  pub head_end: Option<crate::schemas::a::HeadEnd>,
  /// default tail line end style is none.
  #[sdk(child(qname = "a:tailEnd"))]
  pub tail_end: Option<crate::schemas::a::TailEnd>,
  /// Future extensions..
  #[sdk(child(qname = "a:extLst"))]
  pub line_properties_extension_list: Option<crate::schemas::a::LinePropertiesExtensionList>,
}
/// Defines the ModifyNonVisualInkProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:cNvInkPr")]
pub struct ModifyNonVisualInkProps {
  /// noGrp
  #[sdk(attr(qname = ":noGrp"))]
  pub no_grp: Option<crate::simple_type::BooleanValue>,
  /// noSelect
  #[sdk(attr(qname = ":noSelect"))]
  pub no_select: Option<crate::simple_type::BooleanValue>,
  /// noRot
  #[sdk(attr(qname = ":noRot"))]
  pub no_rot: Option<crate::simple_type::BooleanValue>,
  /// noChangeAspect
  #[sdk(attr(qname = ":noChangeAspect"))]
  pub no_change_aspect: Option<crate::simple_type::BooleanValue>,
  /// noMove
  #[sdk(attr(qname = ":noMove"))]
  pub no_move: Option<crate::simple_type::BooleanValue>,
  /// noResize
  #[sdk(attr(qname = ":noResize"))]
  pub no_resize: Option<crate::simple_type::BooleanValue>,
  /// noEditPoints
  #[sdk(attr(qname = ":noEditPoints"))]
  pub no_edit_points: Option<crate::simple_type::BooleanValue>,
  /// noAdjustHandles
  #[sdk(attr(qname = ":noAdjustHandles"))]
  pub no_adjust_handles: Option<crate::simple_type::BooleanValue>,
  /// noChangeArrowheads
  #[sdk(attr(qname = ":noChangeArrowheads"))]
  pub no_change_arrowheads: Option<crate::simple_type::BooleanValue>,
  /// noChangeShapeType
  #[sdk(attr(qname = ":noChangeShapeType"))]
  pub no_change_shape_type: Option<crate::simple_type::BooleanValue>,
  /// isComment
  #[sdk(attr(qname = ":isComment"))]
  pub is_comment: Option<crate::simple_type::BooleanValue>,
}
/// Defines the HlinkClickHyperlinkProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:hlinkClick")]
pub struct HlinkClickHyperlinkProps {
  /// source
  #[sdk(attr(qname = ":source"))]
  pub source: Option<crate::simple_type::StringValue>,
  /// action
  #[sdk(attr(qname = ":action"))]
  pub action: Option<crate::simple_type::StringValue>,
  /// tgtFrame
  #[sdk(attr(qname = ":tgtFrame"))]
  pub tgt_frame: Option<crate::simple_type::StringValue>,
  /// tooltip
  #[sdk(attr(qname = ":tooltip"))]
  pub tooltip: Option<crate::simple_type::StringValue>,
  /// highlightClick
  #[sdk(attr(qname = ":highlightClick"))]
  pub highlight_click: Option<crate::simple_type::BooleanValue>,
  /// endSnd
  #[sdk(attr(qname = ":endSnd"))]
  pub end_snd: Option<crate::simple_type::BooleanValue>,
  /// sndName
  #[sdk(attr(qname = ":sndName"))]
  pub snd_name: Option<crate::simple_type::StringValue>,
  /// Defines the SndDataImgData Class.
  #[sdk(text_child(simple_type = "Base64BinaryValue", qname = "oac:sndData"))]
  pub snd_data_img_data: Option<SndDataImgData>,
}
/// Defines the HlinkHoverHyperlinkProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:hlinkHover")]
pub struct HlinkHoverHyperlinkProps {
  /// source
  #[sdk(attr(qname = ":source"))]
  pub source: Option<crate::simple_type::StringValue>,
  /// action
  #[sdk(attr(qname = ":action"))]
  pub action: Option<crate::simple_type::StringValue>,
  /// tgtFrame
  #[sdk(attr(qname = ":tgtFrame"))]
  pub tgt_frame: Option<crate::simple_type::StringValue>,
  /// tooltip
  #[sdk(attr(qname = ":tooltip"))]
  pub tooltip: Option<crate::simple_type::StringValue>,
  /// highlightClick
  #[sdk(attr(qname = ":highlightClick"))]
  pub highlight_click: Option<crate::simple_type::BooleanValue>,
  /// endSnd
  #[sdk(attr(qname = ":endSnd"))]
  pub end_snd: Option<crate::simple_type::BooleanValue>,
  /// sndName
  #[sdk(attr(qname = ":sndName"))]
  pub snd_name: Option<crate::simple_type::StringValue>,
  /// Defines the SndDataImgData Class.
  #[sdk(text_child(simple_type = "Base64BinaryValue", qname = "oac:sndData"))]
  pub snd_data_img_data: Option<SndDataImgData>,
}
/// Defines the ModifyHyperlinkProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:hlink")]
pub struct ModifyHyperlinkProps {
  /// Defines the HlinkClickHyperlinkProps Class.
  #[sdk(child(qname = "oac:hlinkClick"))]
  pub hlink_click_hyperlink_props: Option<HlinkClickHyperlinkProps>,
  /// Defines the HlinkHoverHyperlinkProps Class.
  #[sdk(child(qname = "oac:hlinkHover"))]
  pub hlink_hover_hyperlink_props: Option<HlinkHoverHyperlinkProps>,
}
/// Defines the ResetHyperlinkProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:hlink")]
pub struct ResetHyperlinkProps {
  /// Defines the HlinkClickEmpty Class.
  #[sdk(empty_child(qname = "oac:hlinkClick"))]
  pub hlink_click_empty: Option<()>,
  /// Defines the HlinkHoverEmpty Class.
  #[sdk(empty_child(qname = "oac:hlinkHover"))]
  pub hlink_hover_empty: Option<()>,
}
/// Defines the TextCharRangeContext Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oac:context")]
pub struct TextCharRangeContext {
  /// len
  #[sdk(attr(qname = ":len"))]
  pub len: Option<crate::simple_type::UInt32Value>,
  /// hash
  #[sdk(attr(qname = ":hash"))]
  pub hash: crate::simple_type::UInt32Value,
}
#[derive(Clone, Debug, PartialEq)]
pub enum GroupCommandChoice {
  /// Defines the ShapeMoniker Class.
  ShapeMoniker(std::boxed::Box<ShapeMoniker>),
  /// Defines the GroupShapeMoniker Class.
  GroupShapeMoniker(std::boxed::Box<GroupShapeMoniker>),
  /// Defines the GraphicFrameMoniker Class.
  GraphicFrameMoniker(std::boxed::Box<GraphicFrameMoniker>),
  /// Defines the ConnectorMoniker Class.
  ConnectorMoniker(std::boxed::Box<ConnectorMoniker>),
  /// Defines the PictureMoniker Class.
  PictureMoniker(std::boxed::Box<PictureMoniker>),
  /// Defines the InkMoniker Class.
  InkMoniker(std::boxed::Box<InkMoniker>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TextParagraphPropertiesTypeChoice {
  /// Follow Text.
  BulletColorText,
  /// Color Specified.
  BulletColor(std::boxed::Box<crate::schemas::a::BulletColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TextParagraphPropertiesTypeChoice2 {
  /// Bullet Size Follows Text.
  BulletSizeText,
  /// Bullet Size Percentage.
  BulletSizePercentage(std::boxed::Box<crate::schemas::a::BulletSizePercentage>),
  /// Bullet Size Points.
  BulletSizePoints(std::boxed::Box<crate::schemas::a::BulletSizePoints>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TextParagraphPropertiesTypeChoice3 {
  /// Follow text.
  BulletFontText,
  /// Specified.
  BulletFont(std::boxed::Box<crate::schemas::a::BulletFont>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TextParagraphPropertiesTypeChoice4 {
  /// No Bullet.
  NoBullet,
  /// Auto-Numbered Bullet.
  AutoNumberedBullet(std::boxed::Box<crate::schemas::a::AutoNumberedBullet>),
  /// Character Bullet.
  CharacterBullet(std::boxed::Box<crate::schemas::a::CharacterBullet>),
  /// Picture Bullet.
  PictureBullet(std::boxed::Box<crate::schemas::a::PictureBullet>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TextBodyPropertiesChoice {
  /// No AutoFit.
  NoAutoFit,
  /// Normal AutoFit.
  NormalAutoFit(std::boxed::Box<crate::schemas::a::NormalAutoFit>),
  /// Shape AutoFit.
  ShapeAutoFit,
}
#[derive(Clone, Debug, PartialEq)]
pub enum TextBodyPropertiesChoice2 {
  /// Apply 3D shape properties.
  Shape3DType(std::boxed::Box<crate::schemas::a::Shape3DType>),
  /// No text in 3D scene.
  FlatText(std::boxed::Box<crate::schemas::a::FlatText>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ShapePropertiesChoice {
  /// Custom geometry.
  CustomGeometry(std::boxed::Box<crate::schemas::a::CustomGeometry>),
  /// Preset geometry.
  PresetGeometry(std::boxed::Box<crate::schemas::a::PresetGeometry>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ShapePropertiesChoice2 {
  /// Defines the NoFill Class.
  NoFill(std::boxed::Box<crate::schemas::a::NoFill>),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<crate::schemas::a::SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<crate::schemas::a::GradientFill>),
  /// Defines the BlipFill Class.
  BlipFill(std::boxed::Box<crate::schemas::a::BlipFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<crate::schemas::a::PatternFill>),
  /// Group Fill.
  GroupFill,
}
#[derive(Clone, Debug, PartialEq)]
pub enum ShapePropertiesChoice3 {
  /// Effect Container.
  EffectList(std::boxed::Box<crate::schemas::a::EffectList>),
  /// Effect Container.
  EffectDag(std::boxed::Box<crate::schemas::a::EffectDag>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum LnRefStyleMatrixReferenceChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<crate::schemas::a::RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<crate::schemas::a::RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<crate::schemas::a::HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<crate::schemas::a::SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<crate::schemas::a::SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<crate::schemas::a::PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum FillRefStyleMatrixReferenceChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<crate::schemas::a::RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<crate::schemas::a::RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<crate::schemas::a::HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<crate::schemas::a::SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<crate::schemas::a::SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<crate::schemas::a::PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum EffectRefStyleMatrixReferenceChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<crate::schemas::a::RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<crate::schemas::a::RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<crate::schemas::a::HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<crate::schemas::a::SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<crate::schemas::a::SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<crate::schemas::a::PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum FontReferenceChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<crate::schemas::a::RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<crate::schemas::a::RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<crate::schemas::a::HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<crate::schemas::a::SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<crate::schemas::a::SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<crate::schemas::a::PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BlipFillPropertiesChoice {
  Tile(std::boxed::Box<crate::schemas::a::Tile>),
  Stretch(std::boxed::Box<crate::schemas::a::Stretch>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum GroupShapePropertiesChoice {
  /// Defines the NoFill Class.
  NoFill(std::boxed::Box<crate::schemas::a::NoFill>),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<crate::schemas::a::SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<crate::schemas::a::GradientFill>),
  /// Defines the BlipFill Class.
  BlipFill(std::boxed::Box<crate::schemas::a::BlipFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<crate::schemas::a::PatternFill>),
  /// Group Fill.
  GroupFill,
}
#[derive(Clone, Debug, PartialEq)]
pub enum GroupShapePropertiesChoice2 {
  /// Effect Container.
  EffectList(std::boxed::Box<crate::schemas::a::EffectList>),
  /// Effect Container.
  EffectDag(std::boxed::Box<crate::schemas::a::EffectDag>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum LinePropertiesTypeChoice {
  /// Defines the NoFill Class.
  NoFill(std::boxed::Box<crate::schemas::a::NoFill>),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<crate::schemas::a::SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<crate::schemas::a::GradientFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<crate::schemas::a::PatternFill>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum LinePropertiesTypeChoice2 {
  /// Preset Dash.
  PresetDash(std::boxed::Box<crate::schemas::a::PresetDash>),
  /// Custom Dash.
  CustomDash(std::boxed::Box<crate::schemas::a::CustomDash>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum LinePropertiesTypeChoice3 {
  /// Round Line Join.
  Round,
  /// Line Join Bevel.
  LineJoinBevel,
  /// Miter Line Join.
  Miter(std::boxed::Box<crate::schemas::a::Miter>),
}
