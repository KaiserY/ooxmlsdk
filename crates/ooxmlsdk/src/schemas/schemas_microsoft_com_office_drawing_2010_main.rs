//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CameraTool Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:cameraTool.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_CameraTool/a14:cameraTool")]
pub struct CameraTool {
  /// cellRange
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :cellRange
  #[sdk(attr(qname = ":cellRange"))]
  pub cell_range: Option<crate::simple_type::StringValue>,
  /// spid
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :spid
  #[sdk(attr(qname = ":spid"))]
  pub shape_id: Option<crate::simple_type::StringValue>,
}
/// Defines the CompatExtension Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:compatExt.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_CompatExt/a14:compatExt")]
pub struct CompatExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// spid
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :spid
  #[sdk(attr(qname = ":spid"))]
  pub shape_id: Option<crate::simple_type::StringValue>,
}
/// Defines the IsCanvas Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:isCanvas.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_IsGvmlCanvas/a14:isCanvas")]
pub struct IsCanvas {
  /// val
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::BooleanValue,
}
/// Defines the GvmlContentPart Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:contentPart.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_GvmlContentPart/a14:contentPart")]
pub struct GvmlContentPart {
  /// bwMode
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :bwMode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub relationship_id: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "a14:CT_GvmlContentPartNonVisual/a14:nvContentPartPr"))]
  pub non_visual_content_part_properties: Option<std::boxed::Box<NonVisualContentPartProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_Transform2D/a14:xfrm"))]
  pub transform2_d: Option<std::boxed::Box<Transform2D>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a14:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the ShadowObscured Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:shadowObscured.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_ShadowObscured/a14:shadowObscured")]
pub struct ShadowObscured {
  /// val
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the HiddenFillProperties Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:hiddenFill.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_FillProperties/a14:hiddenFill")]
pub struct HiddenFillProperties {
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_BlipFillProperties/a:blipFill",
    qname = "a:CT_PatternFillProperties/a:pattFill",
    qname = "a:CT_GroupFillProperties/a:grpFill"
  ))]
  pub xml_children: Option<HiddenFillPropertiesChoice>,
}
/// Defines the HiddenLineProperties Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:hiddenLine.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_LineProperties/a14:hiddenLine")]
pub struct HiddenLineProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
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
  pub hidden_line_properties_choice1: Option<HiddenLinePropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_PresetLineDashProperties/a:prstDash",
    qname = "a:CT_DashStopList/a:custDash"
  ))]
  pub hidden_line_properties_choice2: Option<HiddenLinePropertiesChoice2>,
  #[sdk(choice(
    qname = "a:CT_LineJoinRound/a:round",
    qname = "a:CT_LineJoinBevel/a:bevel",
    qname = "a:CT_LineJoinMiterProperties/a:miter"
  ))]
  pub hidden_line_properties_choice3: Option<HiddenLinePropertiesChoice3>,
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
/// Defines the HiddenEffectsProperties Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:hiddenEffects.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_EffectProperties/a14:hiddenEffects")]
pub struct HiddenEffectsProperties {
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub xml_children: Option<HiddenEffectsPropertiesChoice>,
}
/// Defines the HiddenScene3D Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:hiddenScene3d.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Scene3D/a14:hiddenScene3d")]
pub struct HiddenScene3D {
  /// Camera
  #[sdk(child(qname = "a:CT_Camera/a:camera"))]
  pub camera:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Camera>,
  /// Light Rig
  #[sdk(child(qname = "a:CT_LightRig/a:lightRig"))]
  pub light_rig:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LightRig>,
  /// Backdrop Plane
  #[sdk(child(qname = "a:CT_Backdrop/a:backdrop"))]
  pub backdrop: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Backdrop>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Defines the HiddenShape3D Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:hiddenSp3d.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Shape3D/a14:hiddenSp3d")]
pub struct HiddenShape3D {
  /// Shape Depth
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :z
  #[sdk(attr(qname = ":z"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub z: Option<crate::simple_type::Int64Value>,
  /// Extrusion Height
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :extrusionH
  #[sdk(attr(qname = ":extrusionH"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub extrusion_height: Option<crate::simple_type::Int64Value>,
  /// Contour Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :contourW
  #[sdk(attr(qname = ":contourW"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub contour_width: Option<crate::simple_type::Int64Value>,
  /// Preset Material Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :prstMaterial
  #[sdk(attr(qname = ":prstMaterial"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub preset_material: Option<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetMaterialTypeValues,
  >,
  /// Top Bevel
  #[sdk(child(qname = "a:CT_Bevel/a:bevelT"))]
  pub bevel_top: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BevelTop>,
  /// Bottom Bevel
  #[sdk(child(qname = "a:CT_Bevel/a:bevelB"))]
  pub bevel_bottom:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BevelBottom>,
  /// Extrusion Color
  #[sdk(child(qname = "a:CT_Color/a:extrusionClr"))]
  pub extrusion_color: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtrusionColor>,
  >,
  /// Contour Color
  #[sdk(child(qname = "a:CT_Color/a:contourClr"))]
  pub contour_color: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ContourColor>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Defines the ImageProperties Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:imgProps.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_Photo/a14:imgProps")]
pub struct ImageProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// _
  #[sdk(child(qname = "a14:CT_PictureLayer/a14:imgLayer"))]
  pub image_layer: std::boxed::Box<ImageLayer>,
}
/// Defines the UseLocalDpi Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:useLocalDpi.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_UseLocalDpi/a14:useLocalDpi")]
pub struct UseLocalDpi {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// val
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the OfficeArtExtensionList Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_OfficeArtExtensionList/a14:extLst")]
pub struct OfficeArtExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension>,
}
/// Defines the ContentPartLocks Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:cpLocks.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_ContentPartLocking/a14:cpLocks")]
pub struct ContentPartLocks {
  /// Disallow Shape Grouping
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :noGrp
  #[sdk(attr(qname = ":noGrp"))]
  pub no_grouping: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Selection
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :noSelect
  #[sdk(attr(qname = ":noSelect"))]
  pub no_selection: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Rotation
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :noRot
  #[sdk(attr(qname = ":noRot"))]
  pub no_rotation: Option<crate::simple_type::BooleanValue>,
  /// Disallow Aspect Ratio Change
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :noChangeAspect
  #[sdk(attr(qname = ":noChangeAspect"))]
  pub no_change_aspect: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Movement
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :noMove
  #[sdk(attr(qname = ":noMove"))]
  pub no_move: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Resize
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :noResize
  #[sdk(attr(qname = ":noResize"))]
  pub no_resize: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Point Editing
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :noEditPoints
  #[sdk(attr(qname = ":noEditPoints"))]
  pub no_edit_points: Option<crate::simple_type::BooleanValue>,
  /// Disallow Showing Adjust Handles
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :noAdjustHandles
  #[sdk(attr(qname = ":noAdjustHandles"))]
  pub no_adjust_handles: Option<crate::simple_type::BooleanValue>,
  /// Disallow Arrowhead Changes
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :noChangeArrowheads
  #[sdk(attr(qname = ":noChangeArrowheads"))]
  pub no_change_arrowheads: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Type Change
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :noChangeShapeType
  #[sdk(attr(qname = ":noChangeShapeType"))]
  pub no_change_shape_type: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a14:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the ForegroundMark Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:foregroundMark.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureEffectBackgroundRemovalForegroundMark/a14:foregroundMark")]
pub struct ForegroundMark {
  /// x1
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :x1
  #[sdk(attr(qname = ":x1"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub first_x_coordinate: crate::simple_type::Int32Value,
  /// y1
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :y1
  #[sdk(attr(qname = ":y1"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub first_y_coordinate: crate::simple_type::Int32Value,
  /// x2
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :x2
  #[sdk(attr(qname = ":x2"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub second_x_coordinate: crate::simple_type::Int32Value,
  /// y2
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :y2
  #[sdk(attr(qname = ":y2"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub second_y_coordinate: crate::simple_type::Int32Value,
}
/// Defines the BackgroundMark Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:backgroundMark.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureEffectBackgroundRemovalBackgroundMark/a14:backgroundMark")]
pub struct BackgroundMark {
  /// x1
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :x1
  #[sdk(attr(qname = ":x1"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub first_x_coordinate: crate::simple_type::Int32Value,
  /// y1
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :y1
  #[sdk(attr(qname = ":y1"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub first_y_coordinate: crate::simple_type::Int32Value,
  /// x2
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :x2
  #[sdk(attr(qname = ":x2"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub second_x_coordinate: crate::simple_type::Int32Value,
  /// y2
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :y2
  #[sdk(attr(qname = ":y2"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub second_y_coordinate: crate::simple_type::Int32Value,
}
/// Defines the ArtisticBlur Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:artisticBlur.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureEffectBlur/a14:artisticBlur")]
pub struct ArtisticBlur {
  /// radius
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :radius
  #[sdk(attr(qname = ":radius"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub radius: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticCement Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:artisticCement.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureEffectCement/a14:artisticCement")]
pub struct ArtisticCement {
  /// trans
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// crackSpacing
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :crackSpacing
  #[sdk(attr(qname = ":crackSpacing"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub crack_spacing: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticChalkSketch Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:artisticChalkSketch.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureEffectChalkSketch/a14:artisticChalkSketch")]
pub struct ArtisticChalkSketch {
  /// trans
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// pressure
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :pressure
  #[sdk(attr(qname = ":pressure"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "4",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub pressure: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticCrisscrossEtching Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:artisticCrisscrossEtching.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureEffectCrisscrossEtching/a14:artisticCrisscrossEtching")]
pub struct ArtisticCrisscrossEtching {
  /// trans
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// pressure
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :pressure
  #[sdk(attr(qname = ":pressure"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub pressure: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticCutout Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:artisticCutout.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureEffectCutout/a14:artisticCutout")]
pub struct ArtisticCutout {
  /// trans
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// numberOfShades
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :numberOfShades
  #[sdk(attr(qname = ":numberOfShades"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "6",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub number_of_shades: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticFilmGrain Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:artisticFilmGrain.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureEffectFilmGrain/a14:artisticFilmGrain")]
pub struct ArtisticFilmGrain {
  /// trans
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// grainSize
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :grainSize
  #[sdk(attr(qname = ":grainSize"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub grain_size: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticGlass Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:artisticGlass.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureEffectGlass/a14:artisticGlass")]
pub struct ArtisticGlass {
  /// trans
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// scaling
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :scaling
  #[sdk(attr(qname = ":scaling"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub scaling: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticGlowDiffused Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:artisticGlowDiffused.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureEffectGlowDiffused/a14:artisticGlowDiffused")]
pub struct ArtisticGlowDiffused {
  /// trans
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// intensity
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :intensity
  #[sdk(attr(qname = ":intensity"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "10",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub intensity: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticGlowEdges Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:artisticGlowEdges.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureEffectGlowEdges/a14:artisticGlowEdges")]
pub struct ArtisticGlowEdges {
  /// trans
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// smoothness
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :smoothness
  #[sdk(attr(qname = ":smoothness"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "10",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub smoothness: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticLightScreen Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:artisticLightScreen.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureEffectLightScreen/a14:artisticLightScreen")]
pub struct ArtisticLightScreen {
  /// trans
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// gridSize
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :gridSize
  #[sdk(attr(qname = ":gridSize"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "10",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub grid_size: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticLineDrawing Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:artisticLineDrawing.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureEffectLineDrawing/a14:artisticLineDrawing")]
pub struct ArtisticLineDrawing {
  /// trans
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// pencilSize
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :pencilSize
  #[sdk(attr(qname = ":pencilSize"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub pencil_size: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticMarker Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:artisticMarker.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureEffectMarker/a14:artisticMarker")]
pub struct ArtisticMarker {
  /// trans
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// size
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :size
  #[sdk(attr(qname = ":size"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub size: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticMosaicBubbles Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:artisticMosiaicBubbles.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureEffectMosiaicBubbles/a14:artisticMosiaicBubbles")]
pub struct ArtisticMosaicBubbles {
  /// trans
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// pressure
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :pressure
  #[sdk(attr(qname = ":pressure"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub pressure: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticPaintStrokes Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:artisticPaintStrokes.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureEffectPaintStrokes/a14:artisticPaintStrokes")]
pub struct ArtisticPaintStrokes {
  /// trans
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// intensity
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :intensity
  #[sdk(attr(qname = ":intensity"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "10",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub intensity: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticPaintBrush Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:artisticPaintBrush.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureEffectPaintBrush/a14:artisticPaintBrush")]
pub struct ArtisticPaintBrush {
  /// trans
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// brushSize
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :brushSize
  #[sdk(attr(qname = ":brushSize"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "10",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub brush_size: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticPastelsSmooth Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:artisticPastelsSmooth.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureEffectPastelsSmooth/a14:artisticPastelsSmooth")]
pub struct ArtisticPastelsSmooth {
  /// trans
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// scaling
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :scaling
  #[sdk(attr(qname = ":scaling"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub brush_size: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticPencilGrayscale Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:artisticPencilGrayscale.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureEffectPencilGrayscale/a14:artisticPencilGrayscale")]
pub struct ArtisticPencilGrayscale {
  /// trans
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// pencilSize
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :pencilSize
  #[sdk(attr(qname = ":pencilSize"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub brush_size: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticPencilSketch Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:artisticPencilSketch.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureEffectPencilSketch/a14:artisticPencilSketch")]
pub struct ArtisticPencilSketch {
  /// trans
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// pressure
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :pressure
  #[sdk(attr(qname = ":pressure"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub pressure: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticPhotocopy Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:artisticPhotocopy.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureEffectPhotocopy/a14:artisticPhotocopy")]
pub struct ArtisticPhotocopy {
  /// trans
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// detail
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :detail
  #[sdk(attr(qname = ":detail"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "10",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub detail: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticPlasticWrap Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:artisticPlasticWrap.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureEffectPlasticWrap/a14:artisticPlasticWrap")]
pub struct ArtisticPlasticWrap {
  /// trans
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// smoothness
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :smoothness
  #[sdk(attr(qname = ":smoothness"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "10",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub smoothness: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticTexturizer Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:artisticTexturizer.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureEffectTexturizer/a14:artisticTexturizer")]
pub struct ArtisticTexturizer {
  /// trans
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// scaling
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :scaling
  #[sdk(attr(qname = ":scaling"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub scaling: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticWatercolorSponge Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:artisticWatercolorSponge.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureEffectWatercolorSponge/a14:artisticWatercolorSponge")]
pub struct ArtisticWatercolorSponge {
  /// trans
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// brushSize
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :brushSize
  #[sdk(attr(qname = ":brushSize"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "10",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub brush_size: Option<crate::simple_type::Int32Value>,
}
/// Defines the BackgroundRemoval Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:backgroundRemoval.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureEffectBackgroundRemoval/a14:backgroundRemoval")]
pub struct BackgroundRemoval {
  /// t
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :t
  #[sdk(attr(qname = ":t"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub marquee_top: crate::simple_type::Int32Value,
  /// b
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :b
  #[sdk(attr(qname = ":b"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub marquee_bottom: crate::simple_type::Int32Value,
  /// l
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :l
  #[sdk(attr(qname = ":l"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub marquee_left: crate::simple_type::Int32Value,
  /// r
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :r
  #[sdk(attr(qname = ":r"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub marquee_right: crate::simple_type::Int32Value,
  /// _
  #[sdk(child(qname = "a14:CT_PictureEffectBackgroundRemovalForegroundMark/a14:foregroundMark"))]
  pub a14_foreground_mark: Vec<ForegroundMark>,
  /// _
  #[sdk(child(qname = "a14:CT_PictureEffectBackgroundRemovalBackgroundMark/a14:backgroundMark"))]
  pub a14_background_mark: Vec<BackgroundMark>,
}
/// Defines the BrightnessContrast Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:brightnessContrast.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureEffectBrightnessContrast/a14:brightnessContrast")]
pub struct BrightnessContrast {
  /// bright
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :bright
  #[sdk(attr(qname = ":bright"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-100000",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub bright: Option<crate::simple_type::Int32Value>,
  /// contrast
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :contrast
  #[sdk(attr(qname = ":contrast"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-100000",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub contrast: Option<crate::simple_type::Int32Value>,
}
/// Defines the ColorTemperature Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:colorTemperature.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureEffectColorTemperature/a14:colorTemperature")]
pub struct ColorTemperature {
  /// colorTemp
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :colorTemp
  #[sdk(attr(qname = ":colorTemp"))]
  #[sdk(number_range(
    source = 0u32,
    min = "1500",
    max = "11500",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub color_temperature_value: Option<crate::simple_type::Int32Value>,
}
/// Defines the Saturation Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:saturation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureEffectSaturation/a14:saturation")]
pub struct Saturation {
  /// sat
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :sat
  #[sdk(attr(qname = ":sat"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "400000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub saturation_amount: Option<crate::simple_type::Int32Value>,
}
/// Defines the SharpenSoften Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:sharpenSoften.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureEffectSharpenSoften/a14:sharpenSoften")]
pub struct SharpenSoften {
  /// amount
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :amount
  #[sdk(attr(qname = ":amount"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-100000",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub amount: Option<crate::simple_type::Int32Value>,
}
/// Defines the ImageEffect Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:imgEffect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureEffect/a14:imgEffect")]
pub struct ImageEffect {
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  #[sdk(choice(
    qname = "a14:CT_PictureEffectBlur/a14:artisticBlur",
    qname = "a14:CT_PictureEffectCement/a14:artisticCement",
    qname = "a14:CT_PictureEffectChalkSketch/a14:artisticChalkSketch",
    qname = "a14:CT_PictureEffectCrisscrossEtching/a14:artisticCrisscrossEtching",
    qname = "a14:CT_PictureEffectCutout/a14:artisticCutout",
    qname = "a14:CT_PictureEffectFilmGrain/a14:artisticFilmGrain",
    qname = "a14:CT_PictureEffectGlass/a14:artisticGlass",
    qname = "a14:CT_PictureEffectGlowDiffused/a14:artisticGlowDiffused",
    qname = "a14:CT_PictureEffectGlowEdges/a14:artisticGlowEdges",
    qname = "a14:CT_PictureEffectLightScreen/a14:artisticLightScreen",
    qname = "a14:CT_PictureEffectLineDrawing/a14:artisticLineDrawing",
    qname = "a14:CT_PictureEffectMarker/a14:artisticMarker",
    qname = "a14:CT_PictureEffectMosiaicBubbles/a14:artisticMosiaicBubbles",
    qname = "a14:CT_PictureEffectPaintStrokes/a14:artisticPaintStrokes",
    qname = "a14:CT_PictureEffectPaintBrush/a14:artisticPaintBrush",
    qname = "a14:CT_PictureEffectPastelsSmooth/a14:artisticPastelsSmooth",
    qname = "a14:CT_PictureEffectPencilGrayscale/a14:artisticPencilGrayscale",
    qname = "a14:CT_PictureEffectPencilSketch/a14:artisticPencilSketch",
    qname = "a14:CT_PictureEffectPhotocopy/a14:artisticPhotocopy",
    qname = "a14:CT_PictureEffectPlasticWrap/a14:artisticPlasticWrap",
    qname = "a14:CT_PictureEffectTexturizer/a14:artisticTexturizer",
    qname = "a14:CT_PictureEffectWatercolorSponge/a14:artisticWatercolorSponge",
    qname = "a14:CT_PictureEffectBackgroundRemoval/a14:backgroundRemoval",
    qname = "a14:CT_PictureEffectBrightnessContrast/a14:brightnessContrast",
    qname = "a14:CT_PictureEffectColorTemperature/a14:colorTemperature",
    qname = "a14:CT_PictureEffectSaturation/a14:saturation",
    qname = "a14:CT_PictureEffectSharpenSoften/a14:sharpenSoften"
  ))]
  pub xml_children: Option<ImageEffectChoice>,
}
/// Defines the ImageLayer Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:imgLayer.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_PictureLayer/a14:imgLayer")]
pub struct ImageLayer {
  /// embed
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: r:embed
  #[sdk(attr(qname = "r:embed"))]
  pub embed: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "a14:CT_PictureEffect/a14:imgEffect"))]
  pub a14_img_effect: Vec<ImageEffect>,
}
/// Defines the NonVisualDrawingProperties Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:cNvPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualDrawingProps/a14:cNvPr")]
pub struct NonVisualDrawingProperties {
    pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
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
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:cNvContentPartPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_NonVisualInkContentPartProperties/a14:cNvContentPartPr")]
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
  pub content_part_locks: Option<std::boxed::Box<ContentPartLocks>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a14:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the NonVisualContentPartProperties Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:nvContentPartPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:CT_GvmlContentPartNonVisual/a14:nvContentPartPr")]
pub struct NonVisualContentPartProperties {
  /// _
  #[sdk(child(qname = "a:CT_NonVisualDrawingProps/a14:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// _
  #[sdk(child(qname = "a14:CT_NonVisualInkContentPartProperties/a14:cNvContentPartPr"))]
  pub non_visual_ink_content_part_properties:
    Option<std::boxed::Box<NonVisualInkContentPartProperties>>,
}
/// Defines the Transform2D Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is a14:xfrm.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Transform2D/a14:xfrm")]
pub struct Transform2D {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
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
  /// Offset
  #[sdk(child(qname = "a:CT_Point2D/a:off"))]
  pub offset: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Offset>,
  /// Extents
  #[sdk(child(qname = "a:CT_PositiveSize2D/a:ext"))]
  pub extents: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extents>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum HiddenFillPropertiesChoice {
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
pub enum HiddenLinePropertiesChoice {
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
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum HiddenLinePropertiesChoice2 {
  #[sdk(child(qname = "a:CT_PresetLineDashProperties/a:prstDash"))]
  APrstDash(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetDash>,
  ),
  #[sdk(child(qname = "a:CT_DashStopList/a:custDash"))]
  ACustDash(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::CustomDash>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum HiddenLinePropertiesChoice3 {
  /// Round Line Join.
  #[sdk(empty_child(qname = "a:CT_LineJoinRound/a:round"))]
  ARound,
  /// Line Join Bevel.
  #[sdk(empty_child(qname = "a:CT_LineJoinBevel/a:bevel"))]
  ABevel,
  #[sdk(child(qname = "a:CT_LineJoinMiterProperties/a:miter"))]
  AMiter(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Miter>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum HiddenEffectsPropertiesChoice {
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
pub enum ImageEffectChoice {
  /// Defines the ArtisticBlur Class.
  #[sdk(child(qname = "a14:CT_PictureEffectBlur/a14:artisticBlur"))]
  A14ArtisticBlur(std::boxed::Box<ArtisticBlur>),
  /// Defines the ArtisticCement Class.
  #[sdk(child(qname = "a14:CT_PictureEffectCement/a14:artisticCement"))]
  A14ArtisticCement(std::boxed::Box<ArtisticCement>),
  /// Defines the ArtisticChalkSketch Class.
  #[sdk(child(qname = "a14:CT_PictureEffectChalkSketch/a14:artisticChalkSketch"))]
  A14ArtisticChalkSketch(std::boxed::Box<ArtisticChalkSketch>),
  /// Defines the ArtisticCrisscrossEtching Class.
  #[sdk(child(qname = "a14:CT_PictureEffectCrisscrossEtching/a14:artisticCrisscrossEtching"))]
  A14ArtisticCrisscrossEtching(std::boxed::Box<ArtisticCrisscrossEtching>),
  /// Defines the ArtisticCutout Class.
  #[sdk(child(qname = "a14:CT_PictureEffectCutout/a14:artisticCutout"))]
  A14ArtisticCutout(std::boxed::Box<ArtisticCutout>),
  /// Defines the ArtisticFilmGrain Class.
  #[sdk(child(qname = "a14:CT_PictureEffectFilmGrain/a14:artisticFilmGrain"))]
  A14ArtisticFilmGrain(std::boxed::Box<ArtisticFilmGrain>),
  /// Defines the ArtisticGlass Class.
  #[sdk(child(qname = "a14:CT_PictureEffectGlass/a14:artisticGlass"))]
  A14ArtisticGlass(std::boxed::Box<ArtisticGlass>),
  /// Defines the ArtisticGlowDiffused Class.
  #[sdk(child(qname = "a14:CT_PictureEffectGlowDiffused/a14:artisticGlowDiffused"))]
  A14ArtisticGlowDiffused(std::boxed::Box<ArtisticGlowDiffused>),
  /// Defines the ArtisticGlowEdges Class.
  #[sdk(child(qname = "a14:CT_PictureEffectGlowEdges/a14:artisticGlowEdges"))]
  A14ArtisticGlowEdges(std::boxed::Box<ArtisticGlowEdges>),
  /// Defines the ArtisticLightScreen Class.
  #[sdk(child(qname = "a14:CT_PictureEffectLightScreen/a14:artisticLightScreen"))]
  A14ArtisticLightScreen(std::boxed::Box<ArtisticLightScreen>),
  /// Defines the ArtisticLineDrawing Class.
  #[sdk(child(qname = "a14:CT_PictureEffectLineDrawing/a14:artisticLineDrawing"))]
  A14ArtisticLineDrawing(std::boxed::Box<ArtisticLineDrawing>),
  /// Defines the ArtisticMarker Class.
  #[sdk(child(qname = "a14:CT_PictureEffectMarker/a14:artisticMarker"))]
  A14ArtisticMarker(std::boxed::Box<ArtisticMarker>),
  /// Defines the ArtisticMosaicBubbles Class.
  #[sdk(child(qname = "a14:CT_PictureEffectMosiaicBubbles/a14:artisticMosiaicBubbles"))]
  A14ArtisticMosiaicBubbles(std::boxed::Box<ArtisticMosaicBubbles>),
  /// Defines the ArtisticPaintStrokes Class.
  #[sdk(child(qname = "a14:CT_PictureEffectPaintStrokes/a14:artisticPaintStrokes"))]
  A14ArtisticPaintStrokes(std::boxed::Box<ArtisticPaintStrokes>),
  /// Defines the ArtisticPaintBrush Class.
  #[sdk(child(qname = "a14:CT_PictureEffectPaintBrush/a14:artisticPaintBrush"))]
  A14ArtisticPaintBrush(std::boxed::Box<ArtisticPaintBrush>),
  /// Defines the ArtisticPastelsSmooth Class.
  #[sdk(child(qname = "a14:CT_PictureEffectPastelsSmooth/a14:artisticPastelsSmooth"))]
  A14ArtisticPastelsSmooth(std::boxed::Box<ArtisticPastelsSmooth>),
  /// Defines the ArtisticPencilGrayscale Class.
  #[sdk(child(qname = "a14:CT_PictureEffectPencilGrayscale/a14:artisticPencilGrayscale"))]
  A14ArtisticPencilGrayscale(std::boxed::Box<ArtisticPencilGrayscale>),
  /// Defines the ArtisticPencilSketch Class.
  #[sdk(child(qname = "a14:CT_PictureEffectPencilSketch/a14:artisticPencilSketch"))]
  A14ArtisticPencilSketch(std::boxed::Box<ArtisticPencilSketch>),
  /// Defines the ArtisticPhotocopy Class.
  #[sdk(child(qname = "a14:CT_PictureEffectPhotocopy/a14:artisticPhotocopy"))]
  A14ArtisticPhotocopy(std::boxed::Box<ArtisticPhotocopy>),
  /// Defines the ArtisticPlasticWrap Class.
  #[sdk(child(qname = "a14:CT_PictureEffectPlasticWrap/a14:artisticPlasticWrap"))]
  A14ArtisticPlasticWrap(std::boxed::Box<ArtisticPlasticWrap>),
  /// Defines the ArtisticTexturizer Class.
  #[sdk(child(qname = "a14:CT_PictureEffectTexturizer/a14:artisticTexturizer"))]
  A14ArtisticTexturizer(std::boxed::Box<ArtisticTexturizer>),
  /// Defines the ArtisticWatercolorSponge Class.
  #[sdk(child(qname = "a14:CT_PictureEffectWatercolorSponge/a14:artisticWatercolorSponge"))]
  A14ArtisticWatercolorSponge(std::boxed::Box<ArtisticWatercolorSponge>),
  /// Defines the BackgroundRemoval Class.
  #[sdk(child(qname = "a14:CT_PictureEffectBackgroundRemoval/a14:backgroundRemoval"))]
  A14BackgroundRemoval(std::boxed::Box<BackgroundRemoval>),
  /// Defines the BrightnessContrast Class.
  #[sdk(child(qname = "a14:CT_PictureEffectBrightnessContrast/a14:brightnessContrast"))]
  A14BrightnessContrast(std::boxed::Box<BrightnessContrast>),
  /// Defines the ColorTemperature Class.
  #[sdk(child(qname = "a14:CT_PictureEffectColorTemperature/a14:colorTemperature"))]
  A14ColorTemperature(std::boxed::Box<ColorTemperature>),
  /// Defines the Saturation Class.
  #[sdk(child(qname = "a14:CT_PictureEffectSaturation/a14:saturation"))]
  A14Saturation(std::boxed::Box<Saturation>),
  /// Defines the SharpenSoften Class.
  #[sdk(child(qname = "a14:CT_PictureEffectSharpenSoften/a14:sharpenSoften"))]
  A14SharpenSoften(std::boxed::Box<SharpenSoften>),
}
