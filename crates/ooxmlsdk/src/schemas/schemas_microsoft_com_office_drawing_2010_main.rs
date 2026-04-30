//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CameraTool Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a14:CT_CameraTool/a14:cameraTool")]
pub struct CameraTool {
  /// cellRange
  #[sdk(attr(office2010, qname = ":cellRange"))]
  pub cell_range: Option<crate::simple_type::StringValue>,
  /// spid
  #[sdk(attr(office2010, qname = ":spid"))]
  pub shape_id: Option<crate::simple_type::StringValue>,
}
/// Defines the CompatExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a14:CT_CompatExt/a14:compatExt")]
pub struct CompatExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// spid
  #[sdk(attr(office2010, qname = ":spid"))]
  pub shape_id: Option<crate::simple_type::StringValue>,
}
/// Defines the IsCanvas Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a14:CT_IsGvmlCanvas/a14:isCanvas")]
pub struct IsCanvas {
  /// val
  #[sdk(attr(office2010, qname = ":val"))]
  pub val: crate::simple_type::BooleanValue,
}
/// Defines the GvmlContentPart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a14:CT_GvmlContentPart/a14:contentPart")]
pub struct GvmlContentPart {
  /// bwMode
  #[sdk(attr(office2010, qname = ":bwMode"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues>,
  /// id
  #[sdk(attr(office2010, qname = "r:id"))]
  pub relationship_id: crate::simple_type::StringValue,
  /// _
  #[sdk(child(
    office2010,
    qname = "a14:CT_GvmlContentPartNonVisual/a14:nvContentPartPr"
  ))]
  pub non_visual_content_part_properties: Option<std::boxed::Box<NonVisualContentPartProperties>>,
  /// _
  #[sdk(child(office2010, qname = "a:CT_Transform2D/a14:xfrm"))]
  pub transform2_d: Option<std::boxed::Box<Transform2D>>,
  /// _
  #[sdk(child(office2010, qname = "a:CT_OfficeArtExtensionList/a14:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the ShadowObscured Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a14:CT_ShadowObscured/a14:shadowObscured")]
pub struct ShadowObscured {
  /// val
  #[sdk(attr(office2010, qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the HiddenFillProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_FillProperties/a14:hiddenFill")]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_LineProperties/a14:hiddenLine")]
pub struct HiddenLineProperties {
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_EffectProperties/a14:hiddenEffects")]
pub struct HiddenEffectsProperties {
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub xml_children: Option<HiddenEffectsPropertiesChoice>,
}
/// Defines the HiddenScene3D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_Scene3D/a14:hiddenScene3d")]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_Shape3D/a14:hiddenSp3d")]
pub struct HiddenShape3D {
  /// Shape Depth
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a14:CT_Photo/a14:imgProps")]
pub struct ImageProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// _
  #[sdk(child(office2010, qname = "a14:CT_PictureLayer/a14:imgLayer"))]
  pub image_layer: std::boxed::Box<ImageLayer>,
}
/// Defines the UseLocalDpi Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a14:CT_UseLocalDpi/a14:useLocalDpi")]
pub struct UseLocalDpi {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// val
  #[sdk(attr(office2010, qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the OfficeArtExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_OfficeArtExtensionList/a14:extLst")]
pub struct OfficeArtExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension>,
}
/// Defines the ContentPartLocks Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a14:CT_ContentPartLocking/a14:cpLocks")]
pub struct ContentPartLocks {
  /// Disallow Shape Grouping
  #[sdk(attr(qname = ":noGrp"))]
  pub no_grouping: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Selection
  #[sdk(attr(qname = ":noSelect"))]
  pub no_selection: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Rotation
  #[sdk(attr(qname = ":noRot"))]
  pub no_rotation: Option<crate::simple_type::BooleanValue>,
  /// Disallow Aspect Ratio Change
  #[sdk(attr(qname = ":noChangeAspect"))]
  pub no_change_aspect: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Movement
  #[sdk(attr(qname = ":noMove"))]
  pub no_move: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Resize
  #[sdk(attr(qname = ":noResize"))]
  pub no_resize: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Point Editing
  #[sdk(attr(qname = ":noEditPoints"))]
  pub no_edit_points: Option<crate::simple_type::BooleanValue>,
  /// Disallow Showing Adjust Handles
  #[sdk(attr(qname = ":noAdjustHandles"))]
  pub no_adjust_handles: Option<crate::simple_type::BooleanValue>,
  /// Disallow Arrowhead Changes
  #[sdk(attr(qname = ":noChangeArrowheads"))]
  pub no_change_arrowheads: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Type Change
  #[sdk(attr(qname = ":noChangeShapeType"))]
  pub no_change_shape_type: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(office2010, qname = "a:CT_OfficeArtExtensionList/a14:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the ForegroundMark Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "a14:CT_PictureEffectBackgroundRemovalForegroundMark/a14:foregroundMark"
)]
pub struct ForegroundMark {
  /// x1
  #[sdk(attr(office2010, qname = ":x1"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub first_x_coordinate: crate::simple_type::Int32Value,
  /// y1
  #[sdk(attr(office2010, qname = ":y1"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub first_y_coordinate: crate::simple_type::Int32Value,
  /// x2
  #[sdk(attr(office2010, qname = ":x2"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub second_x_coordinate: crate::simple_type::Int32Value,
  /// y2
  #[sdk(attr(office2010, qname = ":y2"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "a14:CT_PictureEffectBackgroundRemovalBackgroundMark/a14:backgroundMark"
)]
pub struct BackgroundMark {
  /// x1
  #[sdk(attr(office2010, qname = ":x1"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub first_x_coordinate: crate::simple_type::Int32Value,
  /// y1
  #[sdk(attr(office2010, qname = ":y1"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub first_y_coordinate: crate::simple_type::Int32Value,
  /// x2
  #[sdk(attr(office2010, qname = ":x2"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub second_x_coordinate: crate::simple_type::Int32Value,
  /// y2
  #[sdk(attr(office2010, qname = ":y2"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a14:CT_PictureEffectBlur/a14:artisticBlur")]
pub struct ArtisticBlur {
  /// radius
  #[sdk(attr(office2010, qname = ":radius"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a14:CT_PictureEffectCement/a14:artisticCement")]
pub struct ArtisticCement {
  /// trans
  #[sdk(attr(office2010, qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// crackSpacing
  #[sdk(attr(office2010, qname = ":crackSpacing"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "a14:CT_PictureEffectChalkSketch/a14:artisticChalkSketch"
)]
pub struct ArtisticChalkSketch {
  /// trans
  #[sdk(attr(office2010, qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// pressure
  #[sdk(attr(office2010, qname = ":pressure"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "a14:CT_PictureEffectCrisscrossEtching/a14:artisticCrisscrossEtching"
)]
pub struct ArtisticCrisscrossEtching {
  /// trans
  #[sdk(attr(office2010, qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// pressure
  #[sdk(attr(office2010, qname = ":pressure"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a14:CT_PictureEffectCutout/a14:artisticCutout")]
pub struct ArtisticCutout {
  /// trans
  #[sdk(attr(office2010, qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// numberOfShades
  #[sdk(attr(office2010, qname = ":numberOfShades"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "a14:CT_PictureEffectFilmGrain/a14:artisticFilmGrain"
)]
pub struct ArtisticFilmGrain {
  /// trans
  #[sdk(attr(office2010, qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// grainSize
  #[sdk(attr(office2010, qname = ":grainSize"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a14:CT_PictureEffectGlass/a14:artisticGlass")]
pub struct ArtisticGlass {
  /// trans
  #[sdk(attr(office2010, qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// scaling
  #[sdk(attr(office2010, qname = ":scaling"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "a14:CT_PictureEffectGlowDiffused/a14:artisticGlowDiffused"
)]
pub struct ArtisticGlowDiffused {
  /// trans
  #[sdk(attr(office2010, qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// intensity
  #[sdk(attr(office2010, qname = ":intensity"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "a14:CT_PictureEffectGlowEdges/a14:artisticGlowEdges"
)]
pub struct ArtisticGlowEdges {
  /// trans
  #[sdk(attr(office2010, qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// smoothness
  #[sdk(attr(office2010, qname = ":smoothness"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "a14:CT_PictureEffectLightScreen/a14:artisticLightScreen"
)]
pub struct ArtisticLightScreen {
  /// trans
  #[sdk(attr(office2010, qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// gridSize
  #[sdk(attr(office2010, qname = ":gridSize"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "a14:CT_PictureEffectLineDrawing/a14:artisticLineDrawing"
)]
pub struct ArtisticLineDrawing {
  /// trans
  #[sdk(attr(office2010, qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// pencilSize
  #[sdk(attr(office2010, qname = ":pencilSize"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a14:CT_PictureEffectMarker/a14:artisticMarker")]
pub struct ArtisticMarker {
  /// trans
  #[sdk(attr(office2010, qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// size
  #[sdk(attr(office2010, qname = ":size"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "a14:CT_PictureEffectMosiaicBubbles/a14:artisticMosiaicBubbles"
)]
pub struct ArtisticMosaicBubbles {
  /// trans
  #[sdk(attr(office2010, qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// pressure
  #[sdk(attr(office2010, qname = ":pressure"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "a14:CT_PictureEffectPaintStrokes/a14:artisticPaintStrokes"
)]
pub struct ArtisticPaintStrokes {
  /// trans
  #[sdk(attr(office2010, qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// intensity
  #[sdk(attr(office2010, qname = ":intensity"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "a14:CT_PictureEffectPaintBrush/a14:artisticPaintBrush"
)]
pub struct ArtisticPaintBrush {
  /// trans
  #[sdk(attr(office2010, qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// brushSize
  #[sdk(attr(office2010, qname = ":brushSize"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "a14:CT_PictureEffectPastelsSmooth/a14:artisticPastelsSmooth"
)]
pub struct ArtisticPastelsSmooth {
  /// trans
  #[sdk(attr(office2010, qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// scaling
  #[sdk(attr(office2010, qname = ":scaling"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "a14:CT_PictureEffectPencilGrayscale/a14:artisticPencilGrayscale"
)]
pub struct ArtisticPencilGrayscale {
  /// trans
  #[sdk(attr(office2010, qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// pencilSize
  #[sdk(attr(office2010, qname = ":pencilSize"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "a14:CT_PictureEffectPencilSketch/a14:artisticPencilSketch"
)]
pub struct ArtisticPencilSketch {
  /// trans
  #[sdk(attr(office2010, qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// pressure
  #[sdk(attr(office2010, qname = ":pressure"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "a14:CT_PictureEffectPhotocopy/a14:artisticPhotocopy"
)]
pub struct ArtisticPhotocopy {
  /// trans
  #[sdk(attr(office2010, qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// detail
  #[sdk(attr(office2010, qname = ":detail"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "a14:CT_PictureEffectPlasticWrap/a14:artisticPlasticWrap"
)]
pub struct ArtisticPlasticWrap {
  /// trans
  #[sdk(attr(office2010, qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// smoothness
  #[sdk(attr(office2010, qname = ":smoothness"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "a14:CT_PictureEffectTexturizer/a14:artisticTexturizer"
)]
pub struct ArtisticTexturizer {
  /// trans
  #[sdk(attr(office2010, qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// scaling
  #[sdk(attr(office2010, qname = ":scaling"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "a14:CT_PictureEffectWatercolorSponge/a14:artisticWatercolorSponge"
)]
pub struct ArtisticWatercolorSponge {
  /// trans
  #[sdk(attr(office2010, qname = ":trans"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// brushSize
  #[sdk(attr(office2010, qname = ":brushSize"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "a14:CT_PictureEffectBackgroundRemoval/a14:backgroundRemoval"
)]
pub struct BackgroundRemoval {
  /// t
  #[sdk(attr(office2010, qname = ":t"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub marquee_top: crate::simple_type::Int32Value,
  /// b
  #[sdk(attr(office2010, qname = ":b"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub marquee_bottom: crate::simple_type::Int32Value,
  /// l
  #[sdk(attr(office2010, qname = ":l"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub marquee_left: crate::simple_type::Int32Value,
  /// r
  #[sdk(attr(office2010, qname = ":r"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub marquee_right: crate::simple_type::Int32Value,
  /// _
  #[sdk(child(
    office2010,
    qname = "a14:CT_PictureEffectBackgroundRemovalForegroundMark/a14:foregroundMark"
  ))]
  pub a14_foreground_mark: Vec<ForegroundMark>,
  /// _
  #[sdk(child(
    office2010,
    qname = "a14:CT_PictureEffectBackgroundRemovalBackgroundMark/a14:backgroundMark"
  ))]
  pub a14_background_mark: Vec<BackgroundMark>,
}
/// Defines the BrightnessContrast Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "a14:CT_PictureEffectBrightnessContrast/a14:brightnessContrast"
)]
pub struct BrightnessContrast {
  /// bright
  #[sdk(attr(office2010, qname = ":bright"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-100000",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub bright: Option<crate::simple_type::Int32Value>,
  /// contrast
  #[sdk(attr(office2010, qname = ":contrast"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "a14:CT_PictureEffectColorTemperature/a14:colorTemperature"
)]
pub struct ColorTemperature {
  /// colorTemp
  #[sdk(attr(office2010, qname = ":colorTemp"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a14:CT_PictureEffectSaturation/a14:saturation")]
pub struct Saturation {
  /// sat
  #[sdk(attr(office2010, qname = ":sat"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "a14:CT_PictureEffectSharpenSoften/a14:sharpenSoften"
)]
pub struct SharpenSoften {
  /// amount
  #[sdk(attr(office2010, qname = ":amount"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a14:CT_PictureEffect/a14:imgEffect")]
pub struct ImageEffect {
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a14:CT_PictureLayer/a14:imgLayer")]
pub struct ImageLayer {
  /// embed
  #[sdk(attr(office2010, qname = "r:embed"))]
  pub embed: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(office2010, qname = "a14:CT_PictureEffect/a14:imgEffect"))]
  pub a14_img_effect: Vec<ImageEffect>,
}
/// Defines the NonVisualDrawingProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_NonVisualDrawingProps/a14:cNvPr")]
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
  qname = "a14:CT_NonVisualInkContentPartProperties/a14:cNvContentPartPr"
)]
pub struct NonVisualInkContentPartProperties {
  /// isComment
  #[sdk(attr(office2010, qname = ":isComment"))]
  pub is_comment: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(office2010, qname = "a14:CT_ContentPartLocking/a14:cpLocks"))]
  pub content_part_locks: Option<std::boxed::Box<ContentPartLocks>>,
  /// _
  #[sdk(child(office2010, qname = "a:CT_OfficeArtExtensionList/a14:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the NonVisualContentPartProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "a14:CT_GvmlContentPartNonVisual/a14:nvContentPartPr"
)]
pub struct NonVisualContentPartProperties {
  /// _
  #[sdk(child(office2010, qname = "a:CT_NonVisualDrawingProps/a14:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// _
  #[sdk(child(
    office2010,
    qname = "a14:CT_NonVisualInkContentPartProperties/a14:cNvContentPartPr"
  ))]
  pub non_visual_ink_content_part_properties:
    Option<std::boxed::Box<NonVisualInkContentPartProperties>>,
}
/// Defines the Transform2D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_Transform2D/a14:xfrm")]
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
  #[sdk(child(office2010, qname = "a14:CT_PictureEffectBlur/a14:artisticBlur"))]
  A14ArtisticBlur(std::boxed::Box<ArtisticBlur>),
  /// Defines the ArtisticCement Class.
  #[sdk(child(office2010, qname = "a14:CT_PictureEffectCement/a14:artisticCement"))]
  A14ArtisticCement(std::boxed::Box<ArtisticCement>),
  /// Defines the ArtisticChalkSketch Class.
  #[sdk(child(
    office2010,
    qname = "a14:CT_PictureEffectChalkSketch/a14:artisticChalkSketch"
  ))]
  A14ArtisticChalkSketch(std::boxed::Box<ArtisticChalkSketch>),
  /// Defines the ArtisticCrisscrossEtching Class.
  #[sdk(child(
    office2010,
    qname = "a14:CT_PictureEffectCrisscrossEtching/a14:artisticCrisscrossEtching"
  ))]
  A14ArtisticCrisscrossEtching(std::boxed::Box<ArtisticCrisscrossEtching>),
  /// Defines the ArtisticCutout Class.
  #[sdk(child(office2010, qname = "a14:CT_PictureEffectCutout/a14:artisticCutout"))]
  A14ArtisticCutout(std::boxed::Box<ArtisticCutout>),
  /// Defines the ArtisticFilmGrain Class.
  #[sdk(child(
    office2010,
    qname = "a14:CT_PictureEffectFilmGrain/a14:artisticFilmGrain"
  ))]
  A14ArtisticFilmGrain(std::boxed::Box<ArtisticFilmGrain>),
  /// Defines the ArtisticGlass Class.
  #[sdk(child(office2010, qname = "a14:CT_PictureEffectGlass/a14:artisticGlass"))]
  A14ArtisticGlass(std::boxed::Box<ArtisticGlass>),
  /// Defines the ArtisticGlowDiffused Class.
  #[sdk(child(
    office2010,
    qname = "a14:CT_PictureEffectGlowDiffused/a14:artisticGlowDiffused"
  ))]
  A14ArtisticGlowDiffused(std::boxed::Box<ArtisticGlowDiffused>),
  /// Defines the ArtisticGlowEdges Class.
  #[sdk(child(
    office2010,
    qname = "a14:CT_PictureEffectGlowEdges/a14:artisticGlowEdges"
  ))]
  A14ArtisticGlowEdges(std::boxed::Box<ArtisticGlowEdges>),
  /// Defines the ArtisticLightScreen Class.
  #[sdk(child(
    office2010,
    qname = "a14:CT_PictureEffectLightScreen/a14:artisticLightScreen"
  ))]
  A14ArtisticLightScreen(std::boxed::Box<ArtisticLightScreen>),
  /// Defines the ArtisticLineDrawing Class.
  #[sdk(child(
    office2010,
    qname = "a14:CT_PictureEffectLineDrawing/a14:artisticLineDrawing"
  ))]
  A14ArtisticLineDrawing(std::boxed::Box<ArtisticLineDrawing>),
  /// Defines the ArtisticMarker Class.
  #[sdk(child(office2010, qname = "a14:CT_PictureEffectMarker/a14:artisticMarker"))]
  A14ArtisticMarker(std::boxed::Box<ArtisticMarker>),
  /// Defines the ArtisticMosaicBubbles Class.
  #[sdk(child(
    office2010,
    qname = "a14:CT_PictureEffectMosiaicBubbles/a14:artisticMosiaicBubbles"
  ))]
  A14ArtisticMosiaicBubbles(std::boxed::Box<ArtisticMosaicBubbles>),
  /// Defines the ArtisticPaintStrokes Class.
  #[sdk(child(
    office2010,
    qname = "a14:CT_PictureEffectPaintStrokes/a14:artisticPaintStrokes"
  ))]
  A14ArtisticPaintStrokes(std::boxed::Box<ArtisticPaintStrokes>),
  /// Defines the ArtisticPaintBrush Class.
  #[sdk(child(
    office2010,
    qname = "a14:CT_PictureEffectPaintBrush/a14:artisticPaintBrush"
  ))]
  A14ArtisticPaintBrush(std::boxed::Box<ArtisticPaintBrush>),
  /// Defines the ArtisticPastelsSmooth Class.
  #[sdk(child(
    office2010,
    qname = "a14:CT_PictureEffectPastelsSmooth/a14:artisticPastelsSmooth"
  ))]
  A14ArtisticPastelsSmooth(std::boxed::Box<ArtisticPastelsSmooth>),
  /// Defines the ArtisticPencilGrayscale Class.
  #[sdk(child(
    office2010,
    qname = "a14:CT_PictureEffectPencilGrayscale/a14:artisticPencilGrayscale"
  ))]
  A14ArtisticPencilGrayscale(std::boxed::Box<ArtisticPencilGrayscale>),
  /// Defines the ArtisticPencilSketch Class.
  #[sdk(child(
    office2010,
    qname = "a14:CT_PictureEffectPencilSketch/a14:artisticPencilSketch"
  ))]
  A14ArtisticPencilSketch(std::boxed::Box<ArtisticPencilSketch>),
  /// Defines the ArtisticPhotocopy Class.
  #[sdk(child(
    office2010,
    qname = "a14:CT_PictureEffectPhotocopy/a14:artisticPhotocopy"
  ))]
  A14ArtisticPhotocopy(std::boxed::Box<ArtisticPhotocopy>),
  /// Defines the ArtisticPlasticWrap Class.
  #[sdk(child(
    office2010,
    qname = "a14:CT_PictureEffectPlasticWrap/a14:artisticPlasticWrap"
  ))]
  A14ArtisticPlasticWrap(std::boxed::Box<ArtisticPlasticWrap>),
  /// Defines the ArtisticTexturizer Class.
  #[sdk(child(
    office2010,
    qname = "a14:CT_PictureEffectTexturizer/a14:artisticTexturizer"
  ))]
  A14ArtisticTexturizer(std::boxed::Box<ArtisticTexturizer>),
  /// Defines the ArtisticWatercolorSponge Class.
  #[sdk(child(
    office2010,
    qname = "a14:CT_PictureEffectWatercolorSponge/a14:artisticWatercolorSponge"
  ))]
  A14ArtisticWatercolorSponge(std::boxed::Box<ArtisticWatercolorSponge>),
  /// Defines the BackgroundRemoval Class.
  #[sdk(child(
    office2010,
    qname = "a14:CT_PictureEffectBackgroundRemoval/a14:backgroundRemoval"
  ))]
  A14BackgroundRemoval(std::boxed::Box<BackgroundRemoval>),
  /// Defines the BrightnessContrast Class.
  #[sdk(child(
    office2010,
    qname = "a14:CT_PictureEffectBrightnessContrast/a14:brightnessContrast"
  ))]
  A14BrightnessContrast(std::boxed::Box<BrightnessContrast>),
  /// Defines the ColorTemperature Class.
  #[sdk(child(
    office2010,
    qname = "a14:CT_PictureEffectColorTemperature/a14:colorTemperature"
  ))]
  A14ColorTemperature(std::boxed::Box<ColorTemperature>),
  /// Defines the Saturation Class.
  #[sdk(child(office2010, qname = "a14:CT_PictureEffectSaturation/a14:saturation"))]
  A14Saturation(std::boxed::Box<Saturation>),
  /// Defines the SharpenSoften Class.
  #[sdk(child(
    office2010,
    qname = "a14:CT_PictureEffectSharpenSoften/a14:sharpenSoften"
  ))]
  A14SharpenSoften(std::boxed::Box<SharpenSoften>),
}
