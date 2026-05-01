//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Model3D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "am3d:CT_Model3D/am3d:model3d")]
pub struct Model3D {
  /// Embedded Picture Reference
  #[sdk(attr(qname = "r:embed"))]
  pub embed: Option<crate::simple_type::StringValue>,
  /// Linked Picture Reference
  #[sdk(attr(qname = "r:link"))]
  pub link: Option<crate::simple_type::StringValue>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(office2019, qname = "a:CT_ShapeProperties/am3d:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the Model3DCamera Class.
  #[sdk(child(office2019, qname = "am3d:CT_Model3DCamera/am3d:camera"))]
  pub model3_d_camera: Option<std::boxed::Box<Model3DCamera>>,
  /// Defines the Model3DTransform Class.
  #[sdk(child(office2019, qname = "am3d:CT_Model3DTransform/am3d:trans"))]
  pub model3_d_transform: Option<std::boxed::Box<Model3DTransform>>,
  /// Optional source attribution URL describes from whence the 3D model came.
  #[sdk(child(
    office2019,
    qname = "a1611:CT_PictureAttributionSourceURL/am3d:attrSrcUrl"
  ))]
  pub picture_attribution_source_url: Option<PictureAttributionSourceUrl>,
  /// Defines the Model3DRaster Class.
  #[sdk(child(office2019, qname = "am3d:CT_Model3DRaster/am3d:raster"))]
  pub model3_d_raster: Option<std::boxed::Box<Model3DRaster>>,
  /// Future Model3D extensions
  #[sdk(child(office2019, qname = "am3d:CT_Model3DExtensionList/am3d:extLst"))]
  pub model3_d_extension_list: Option<Model3DExtensionList>,
  #[sdk(choice(
    microsoft365,
    qname = "am3d:CT_ObjectViewport/am3d:objViewport",
    qname = "am3d:CT_WindowViewport/am3d:winViewport"
  ))]
  pub model3_d_choice1: Option<Model3DChoice>,
  /// Ambient light in a scene.
  #[sdk(child(office2019, qname = "am3d:CT_AmbientLight/am3d:ambientLight"))]
  pub am3d_ambient_light: Option<std::boxed::Box<AmbientLight>>,
  #[sdk(choice(
    microsoft365,
    qname = "am3d:CT_PointLight/am3d:ptLight",
    qname = "am3d:CT_SpotLight/am3d:spotLight",
    qname = "am3d:CT_DirectionalLight/am3d:dirLight",
    qname = "am3d:CT_UnknownLight/am3d:unkLight"
  ))]
  pub model3_d_choice2: Vec<Model3DChoice2>,
}
/// Defines the SxRatio Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "a:CT_Ratio/am3d:sx")]
pub struct SxRatio {
  /// Numerator
  #[sdk(attr(qname = ":n"))]
  pub numerator: crate::simple_type::Int32Value,
  /// Denominator
  #[sdk(attr(qname = ":d"))]
  pub denominator: crate::simple_type::Int32Value,
}
/// Defines the SyRatio Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "a:CT_Ratio/am3d:sy")]
pub struct SyRatio {
  /// Numerator
  #[sdk(attr(qname = ":n"))]
  pub numerator: crate::simple_type::Int32Value,
  /// Denominator
  #[sdk(attr(qname = ":d"))]
  pub denominator: crate::simple_type::Int32Value,
}
/// Defines the SzRatio Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "a:CT_Ratio/am3d:sz")]
pub struct SzRatio {
  /// Numerator
  #[sdk(attr(qname = ":n"))]
  pub numerator: crate::simple_type::Int32Value,
  /// Denominator
  #[sdk(attr(qname = ":d"))]
  pub denominator: crate::simple_type::Int32Value,
}
/// Defines the MeterPerModelUnitPositiveRatio Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "am3d:CT_PositiveRatio/am3d:meterPerModelUnit")]
pub struct MeterPerModelUnitPositiveRatio {
  /// n
  #[sdk(attr(office2019, qname = ":n"))]
  pub n: crate::simple_type::UInt64Value,
  /// d
  #[sdk(attr(office2019, qname = ":d"))]
  pub d: crate::simple_type::UInt64Value,
}
/// Defines the SzPositiveRatio Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "am3d:CT_PositiveRatio/am3d:sz")]
pub struct SzPositiveRatio {
  /// n
  #[sdk(attr(office2019, qname = ":n"))]
  pub n: crate::simple_type::UInt64Value,
  /// d
  #[sdk(attr(office2019, qname = ":d"))]
  pub d: crate::simple_type::UInt64Value,
}
/// Defines the IlluminancePositiveRatio Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "am3d:CT_PositiveRatio/am3d:illuminance")]
pub struct IlluminancePositiveRatio {
  /// n
  #[sdk(attr(office2019, qname = ":n"))]
  pub n: crate::simple_type::UInt64Value,
  /// d
  #[sdk(attr(office2019, qname = ":d"))]
  pub d: crate::simple_type::UInt64Value,
}
/// Defines the IntensityPositiveRatio Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "am3d:CT_PositiveRatio/am3d:intensity")]
pub struct IntensityPositiveRatio {
  /// n
  #[sdk(attr(office2019, qname = ":n"))]
  pub n: crate::simple_type::UInt64Value,
  /// d
  #[sdk(attr(office2019, qname = ":d"))]
  pub d: crate::simple_type::UInt64Value,
}
/// Defines the PreTransVector3D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "a:CT_Vector3D/am3d:preTrans")]
pub struct PreTransVector3D {
  /// Distance along X-axis in 3D
  #[sdk(attr(qname = ":dx"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub dx: crate::simple_type::Int64Value,
  /// Distance along Y-axis in 3D
  #[sdk(attr(qname = ":dy"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub dy: crate::simple_type::Int64Value,
  /// Distance along Z-axis in 3D
  #[sdk(attr(qname = ":dz"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub dz: crate::simple_type::Int64Value,
}
/// Defines the PostTransVector3D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "a:CT_Vector3D/am3d:postTrans")]
pub struct PostTransVector3D {
  /// Distance along X-axis in 3D
  #[sdk(attr(qname = ":dx"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub dx: crate::simple_type::Int64Value,
  /// Distance along Y-axis in 3D
  #[sdk(attr(qname = ":dy"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub dy: crate::simple_type::Int64Value,
  /// Distance along Z-axis in 3D
  #[sdk(attr(qname = ":dz"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub dz: crate::simple_type::Int64Value,
}
/// Defines the UpVector3D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "a:CT_Vector3D/am3d:up")]
pub struct UpVector3D {
  /// Distance along X-axis in 3D
  #[sdk(attr(qname = ":dx"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub dx: crate::simple_type::Int64Value,
  /// Distance along Y-axis in 3D
  #[sdk(attr(qname = ":dy"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub dy: crate::simple_type::Int64Value,
  /// Distance along Z-axis in 3D
  #[sdk(attr(qname = ":dz"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub dz: crate::simple_type::Int64Value,
}
/// Defines the Scale3D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "am3d:CT_Scale3D/am3d:scale")]
pub struct Scale3D {
  /// Defines the SxRatio Class.
  #[sdk(child(office2019, qname = "a:CT_Ratio/am3d:sx"))]
  pub sx_ratio: std::boxed::Box<SxRatio>,
  /// Defines the SyRatio Class.
  #[sdk(child(office2019, qname = "a:CT_Ratio/am3d:sy"))]
  pub sy_ratio: std::boxed::Box<SyRatio>,
  /// Defines the SzRatio Class.
  #[sdk(child(office2019, qname = "a:CT_Ratio/am3d:sz"))]
  pub sz_ratio: std::boxed::Box<SzRatio>,
}
/// Defines the Rotate3D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "am3d:CT_Rotate3D/am3d:rot")]
pub struct Rotate3D {
  /// ax
  #[sdk(attr(office2019, qname = ":ax"))]
  pub ax: Option<crate::simple_type::Int32Value>,
  /// ay
  #[sdk(attr(office2019, qname = ":ay"))]
  pub ay: Option<crate::simple_type::Int32Value>,
  /// az
  #[sdk(attr(office2019, qname = ":az"))]
  pub az: Option<crate::simple_type::Int32Value>,
}
/// Defines the OfficeArtExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "a:CT_OfficeArtExtensionList/am3d:extLst")]
pub struct OfficeArtExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub a_ext: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension>,
}
/// Defines the PosPoint3D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "a:CT_Point3D/am3d:pos")]
pub struct PosPoint3D {
  /// X-Coordinate in 3D
  #[sdk(attr(qname = ":x"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub x: crate::simple_type::Int64Value,
  /// Y-Coordinate in 3D
  #[sdk(attr(qname = ":y"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub y: crate::simple_type::Int64Value,
  /// Z-Coordinate in 3D
  #[sdk(attr(qname = ":z"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub z: crate::simple_type::Int64Value,
}
/// Defines the LookAtPoint3D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "a:CT_Point3D/am3d:lookAt")]
pub struct LookAtPoint3D {
  /// X-Coordinate in 3D
  #[sdk(attr(qname = ":x"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub x: crate::simple_type::Int64Value,
  /// Y-Coordinate in 3D
  #[sdk(attr(qname = ":y"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub y: crate::simple_type::Int64Value,
  /// Z-Coordinate in 3D
  #[sdk(attr(qname = ":z"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub z: crate::simple_type::Int64Value,
}
/// Defines the OrthographicProjection Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "am3d:CT_OrthographicProjection/am3d:orthographic")]
pub struct OrthographicProjection {
  /// Defines the SzPositiveRatio Class.
  #[sdk(child(office2019, qname = "am3d:CT_PositiveRatio/am3d:sz"))]
  pub sz_positive_ratio: std::boxed::Box<SzPositiveRatio>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2019, qname = "a:CT_OfficeArtExtensionList/am3d:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the PerspectiveProjection Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "am3d:CT_PerspectiveProjection/am3d:perspective")]
pub struct PerspectiveProjection {
  /// fov
  #[sdk(attr(office2019, qname = ":fov"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "10800000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub fov: crate::simple_type::Int32Value,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2019, qname = "a:CT_OfficeArtExtensionList/am3d:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the Blip Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "a:CT_Blip/am3d:blip")]
pub struct Blip {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Embedded Picture Reference
  #[sdk(attr(qname = "r:embed"))]
  pub embed: Option<crate::simple_type::StringValue>,
  /// Linked Picture Reference
  #[sdk(attr(qname = "r:link"))]
  pub link: Option<crate::simple_type::StringValue>,
  /// Compression state for blips.
  #[sdk(attr(qname = ":cstate"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub compression_state:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipCompressionValues>,
  #[sdk(choice(
    qname = "a:CT_AlphaBiLevelEffect/a:alphaBiLevel",
    qname = "a:CT_AlphaCeilingEffect/a:alphaCeiling",
    qname = "a:CT_AlphaFloorEffect/a:alphaFloor",
    qname = "a:CT_AlphaInverseEffect/a:alphaInv",
    qname = "a:CT_AlphaModulateEffect/a:alphaMod",
    qname = "a:CT_AlphaModulateFixedEffect/a:alphaModFix",
    qname = "a:CT_AlphaReplaceEffect/a:alphaRepl",
    qname = "a:CT_BiLevelEffect/a:biLevel",
    qname = "a:CT_BlurEffect/a:blur",
    qname = "a:CT_ColorChangeEffect/a:clrChange",
    qname = "a:CT_ColorReplaceEffect/a:clrRepl",
    qname = "a:CT_DuotoneEffect/a:duotone",
    qname = "a:CT_FillOverlayEffect/a:fillOverlay",
    qname = "a:CT_GrayscaleEffect/a:grayscl",
    qname = "a:CT_HSLEffect/a:hsl",
    qname = "a:CT_LuminanceEffect/a:lum",
    qname = "a:CT_TintEffect/a:tint"
  ))]
  pub blip_choice: Vec<BlipChoice>,
  /// Future extensions..
  #[sdk(child(qname = "a:CT_BlipExtensionList/a:extLst"))]
  pub a_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipExtensionList>,
}
/// Defines the ColorType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "a:CT_Color/am3d:clr")]
pub struct ColorType {
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub color_type_choice: Option<ColorTypeChoice>,
}
/// Defines the Model3DExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "am3d:CT_Model3DExtension/am3d:ext")]
pub struct Model3DExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// URI
  #[sdk(attr(office2019, qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "a3danim:CT_EmbeddedAnimation/a3danim:embedAnim",
    qname = "a3danim:CT_PosterFrame/a3danim:posterFrame",
    any
  ))]
  pub model3_d_extension_choice: Option<Model3DExtensionChoice>,
}
/// Defines the ShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "a:CT_ShapeProperties/am3d:spPr")]
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
/// Defines the Model3DCamera Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "am3d:CT_Model3DCamera/am3d:camera")]
pub struct Model3DCamera {
  /// Defines the PosPoint3D Class.
  #[sdk(child(office2019, qname = "a:CT_Point3D/am3d:pos"))]
  pub pos_point3_d: Option<PosPoint3D>,
  /// Defines the UpVector3D Class.
  #[sdk(child(office2019, qname = "a:CT_Vector3D/am3d:up"))]
  pub up_vector3_d: Option<UpVector3D>,
  /// Defines the LookAtPoint3D Class.
  #[sdk(child(office2019, qname = "a:CT_Point3D/am3d:lookAt"))]
  pub look_at_point3_d: Option<LookAtPoint3D>,
  #[sdk(choice(
    microsoft365,
    qname = "am3d:CT_OrthographicProjection/am3d:orthographic",
    qname = "am3d:CT_PerspectiveProjection/am3d:perspective"
  ))]
  pub model3_d_camera_choice: Option<Model3DCameraChoice>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2019, qname = "a:CT_OfficeArtExtensionList/am3d:extLst"))]
  pub am3d_ext_lst: Option<OfficeArtExtensionList>,
}
/// Defines the Model3DTransform Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "am3d:CT_Model3DTransform/am3d:trans")]
pub struct Model3DTransform {
  /// Defines the MeterPerModelUnitPositiveRatio Class.
  #[sdk(child(office2019, qname = "am3d:CT_PositiveRatio/am3d:meterPerModelUnit"))]
  pub meter_per_model_unit_positive_ratio: Option<MeterPerModelUnitPositiveRatio>,
  /// Defines the PreTransVector3D Class.
  #[sdk(child(office2019, qname = "a:CT_Vector3D/am3d:preTrans"))]
  pub pre_trans_vector3_d: Option<PreTransVector3D>,
  /// Defines the Scale3D Class.
  #[sdk(child(office2019, qname = "am3d:CT_Scale3D/am3d:scale"))]
  pub scale3_d: Option<std::boxed::Box<Scale3D>>,
  /// Defines the Rotate3D Class.
  #[sdk(child(office2019, qname = "am3d:CT_Rotate3D/am3d:rot"))]
  pub rotate3_d: Option<Rotate3D>,
  /// Defines the PostTransVector3D Class.
  #[sdk(child(office2019, qname = "a:CT_Vector3D/am3d:postTrans"))]
  pub post_trans_vector3_d: Option<PostTransVector3D>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2019, qname = "a:CT_OfficeArtExtensionList/am3d:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Optional source attribution URL describes from whence the 3D model came..
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2019,
  qname = "a1611:CT_PictureAttributionSourceURL/am3d:attrSrcUrl"
)]
pub struct PictureAttributionSourceUrl {
  /// id
  #[sdk(attr(office2019, qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the Model3DRaster Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "am3d:CT_Model3DRaster/am3d:raster")]
pub struct Model3DRaster {
  /// rName
  #[sdk(attr(office2019, qname = ":rName"))]
  pub r_name: crate::simple_type::StringValue,
  /// rVer
  #[sdk(attr(office2019, qname = ":rVer"))]
  pub r_ver: crate::simple_type::StringValue,
  /// Defines the Blip Class.
  #[sdk(child(office2019, qname = "a:CT_Blip/am3d:blip"))]
  pub blip: Option<std::boxed::Box<Blip>>,
}
/// Future Model3D extensions.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "am3d:CT_Model3DExtensionList/am3d:extLst")]
pub struct Model3DExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the Model3DExtension Class.
  #[sdk(child(office2019, qname = "am3d:CT_Model3DExtension/am3d:ext"))]
  pub am3d_ext: Vec<Model3DExtension>,
}
/// Defines the ObjectViewport Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "am3d:CT_ObjectViewport/am3d:objViewport")]
pub struct ObjectViewport {
  /// viewportSz
  #[sdk(attr(office2019, qname = ":viewportSz"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub viewport_sz: crate::simple_type::Int64Value,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2019, qname = "a:CT_OfficeArtExtensionList/am3d:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the WindowViewport Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "am3d:CT_WindowViewport/am3d:winViewport")]
pub struct WindowViewport {
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2019, qname = "a:CT_OfficeArtExtensionList/am3d:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Ambient light in a scene.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "am3d:CT_AmbientLight/am3d:ambientLight")]
pub struct AmbientLight {
  /// enabled
  #[sdk(attr(office2019, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// Defines the ColorType Class.
  #[sdk(child(office2019, qname = "a:CT_Color/am3d:clr"))]
  pub color_type: std::boxed::Box<ColorType>,
  /// Defines the IlluminancePositiveRatio Class.
  #[sdk(child(office2019, qname = "am3d:CT_PositiveRatio/am3d:illuminance"))]
  pub illuminance_positive_ratio: std::boxed::Box<IlluminancePositiveRatio>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2019, qname = "a:CT_OfficeArtExtensionList/am3d:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the PointLight Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "am3d:CT_PointLight/am3d:ptLight")]
pub struct PointLight {
  /// enabled
  #[sdk(attr(office2019, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// rad
  #[sdk(attr(office2019, qname = ":rad"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub rad: crate::simple_type::Int64Value,
  /// Defines the ColorType Class.
  #[sdk(child(office2019, qname = "a:CT_Color/am3d:clr"))]
  pub color_type: std::boxed::Box<ColorType>,
  /// Defines the IntensityPositiveRatio Class.
  #[sdk(child(office2019, qname = "am3d:CT_PositiveRatio/am3d:intensity"))]
  pub intensity_positive_ratio: std::boxed::Box<IntensityPositiveRatio>,
  /// Defines the PosPoint3D Class.
  #[sdk(child(office2019, qname = "a:CT_Point3D/am3d:pos"))]
  pub pos_point3_d: std::boxed::Box<PosPoint3D>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2019, qname = "a:CT_OfficeArtExtensionList/am3d:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the SpotLight Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "am3d:CT_SpotLight/am3d:spotLight")]
pub struct SpotLight {
  /// enabled
  #[sdk(attr(office2019, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// rad
  #[sdk(attr(office2019, qname = ":rad"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub rad: crate::simple_type::Int64Value,
  /// spotAng
  #[sdk(attr(office2019, qname = ":spotAng"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "10800000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub spot_ang: crate::simple_type::Int32Value,
  /// Defines the ColorType Class.
  #[sdk(child(office2019, qname = "a:CT_Color/am3d:clr"))]
  pub color_type: std::boxed::Box<ColorType>,
  /// Defines the IntensityPositiveRatio Class.
  #[sdk(child(office2019, qname = "am3d:CT_PositiveRatio/am3d:intensity"))]
  pub intensity_positive_ratio: std::boxed::Box<IntensityPositiveRatio>,
  /// Defines the PosPoint3D Class.
  #[sdk(child(office2019, qname = "a:CT_Point3D/am3d:pos"))]
  pub pos_point3_d: std::boxed::Box<PosPoint3D>,
  /// Defines the LookAtPoint3D Class.
  #[sdk(child(office2019, qname = "a:CT_Point3D/am3d:lookAt"))]
  pub look_at_point3_d: std::boxed::Box<LookAtPoint3D>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2019, qname = "a:CT_OfficeArtExtensionList/am3d:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DirectionalLight Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "am3d:CT_DirectionalLight/am3d:dirLight")]
pub struct DirectionalLight {
  /// enabled
  #[sdk(attr(office2019, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// angularRad
  #[sdk(attr(office2019, qname = ":angularRad"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "5400000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub angular_rad: crate::simple_type::Int32Value,
  /// Defines the ColorType Class.
  #[sdk(child(office2019, qname = "a:CT_Color/am3d:clr"))]
  pub color_type: std::boxed::Box<ColorType>,
  /// Defines the IlluminancePositiveRatio Class.
  #[sdk(child(office2019, qname = "am3d:CT_PositiveRatio/am3d:illuminance"))]
  pub illuminance_positive_ratio: std::boxed::Box<IlluminancePositiveRatio>,
  /// Defines the PosPoint3D Class.
  #[sdk(child(office2019, qname = "a:CT_Point3D/am3d:pos"))]
  pub pos_point3_d: std::boxed::Box<PosPoint3D>,
  /// Defines the LookAtPoint3D Class.
  #[sdk(child(office2019, qname = "a:CT_Point3D/am3d:lookAt"))]
  pub look_at_point3_d: std::boxed::Box<LookAtPoint3D>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2019, qname = "a:CT_OfficeArtExtensionList/am3d:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Model3DChoice {
  /// Defines the ObjectViewport Class.
  #[sdk(child(office2019, qname = "am3d:CT_ObjectViewport/am3d:objViewport"))]
  Am3dObjViewport(std::boxed::Box<ObjectViewport>),
  /// Defines the WindowViewport Class.
  #[sdk(child(office2019, qname = "am3d:CT_WindowViewport/am3d:winViewport"))]
  Am3dWinViewport(std::boxed::Box<WindowViewport>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Model3DChoice2 {
  /// Defines the PointLight Class.
  #[sdk(child(office2019, qname = "am3d:CT_PointLight/am3d:ptLight"))]
  Am3dPtLight(std::boxed::Box<PointLight>),
  /// Defines the SpotLight Class.
  #[sdk(child(office2019, qname = "am3d:CT_SpotLight/am3d:spotLight"))]
  Am3dSpotLight(std::boxed::Box<SpotLight>),
  /// Defines the DirectionalLight Class.
  #[sdk(child(office2019, qname = "am3d:CT_DirectionalLight/am3d:dirLight"))]
  Am3dDirLight(std::boxed::Box<DirectionalLight>),
  /// Defines the UnknownLight Class.
  #[sdk(empty_child(office2019, qname = "am3d:CT_UnknownLight/am3d:unkLight"))]
  Am3dUnkLight,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BlipChoice {
  #[sdk(child(qname = "a:CT_AlphaBiLevelEffect/a:alphaBiLevel"))]
  AAlphaBiLevel(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaBiLevel>,
  ),
  /// Alpha Ceiling Effect.
  #[sdk(empty_child(qname = "a:CT_AlphaCeilingEffect/a:alphaCeiling"))]
  AAlphaCeiling,
  /// Alpha Floor Effect.
  #[sdk(empty_child(qname = "a:CT_AlphaFloorEffect/a:alphaFloor"))]
  AAlphaFloor,
  #[sdk(child(qname = "a:CT_AlphaInverseEffect/a:alphaInv"))]
  AAlphaInv(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaInverse>,
  ),
  #[sdk(child(qname = "a:CT_AlphaModulateEffect/a:alphaMod"))]
  AAlphaMod(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaModulationEffect,
    >,
  ),
  #[sdk(child(qname = "a:CT_AlphaModulateFixedEffect/a:alphaModFix"))]
  AAlphaModFix(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaModulationFixed,
    >,
  ),
  #[sdk(child(qname = "a:CT_AlphaReplaceEffect/a:alphaRepl"))]
  AAlphaRepl(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaReplace>,
  ),
  #[sdk(child(qname = "a:CT_BiLevelEffect/a:biLevel"))]
  ABiLevel(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BiLevel>,
  ),
  #[sdk(child(qname = "a:CT_BlurEffect/a:blur"))]
  ABlur(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Blur>),
  #[sdk(child(qname = "a:CT_ColorChangeEffect/a:clrChange"))]
  AClrChange(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorChange>,
  ),
  #[sdk(child(qname = "a:CT_ColorReplaceEffect/a:clrRepl"))]
  AClrRepl(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorReplacement,
    >,
  ),
  #[sdk(child(qname = "a:CT_DuotoneEffect/a:duotone"))]
  ADuotone(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Duotone>,
  ),
  #[sdk(child(qname = "a:CT_FillOverlayEffect/a:fillOverlay"))]
  AFillOverlay(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FillOverlay>,
  ),
  /// Gray Scale Effect.
  #[sdk(empty_child(qname = "a:CT_GrayscaleEffect/a:grayscl"))]
  AGrayscl,
  #[sdk(child(qname = "a:CT_HSLEffect/a:hsl"))]
  AHsl(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Hsl>),
  #[sdk(child(qname = "a:CT_LuminanceEffect/a:lum"))]
  ALum(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LuminanceEffect,
    >,
  ),
  #[sdk(child(qname = "a:CT_TintEffect/a:tint"))]
  ATint(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TintEffect>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ColorTypeChoice {
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
pub enum Model3DExtensionChoice {
  /// Defines the EmbeddedAnimation Class.
    #[sdk(child(office2019, qname = "a3danim:CT_EmbeddedAnimation/a3danim:embedAnim"))]
    A3danimEmbedAnim(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_drawing_2018_animation_model3d::EmbeddedAnimation,
        >,
    ),
    /// Defines the PosterFrame Class.
    #[sdk(child(office2019, qname = "a3danim:CT_PosterFrame/a3danim:posterFrame"))]
    A3danimPosterFrame(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_drawing_2018_animation_model3d::PosterFrame,
        >,
    ),
    #[sdk(any)]
    XmlOther(String),
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
pub enum Model3DCameraChoice {
  #[sdk(child(office2019, qname = "am3d:CT_OrthographicProjection/am3d:orthographic"))]
  Am3dOrthographic(std::boxed::Box<OrthographicProjection>),
  #[sdk(child(office2019, qname = "am3d:CT_PerspectiveProjection/am3d:perspective"))]
  Am3dPerspective(std::boxed::Box<PerspectiveProjection>),
}
