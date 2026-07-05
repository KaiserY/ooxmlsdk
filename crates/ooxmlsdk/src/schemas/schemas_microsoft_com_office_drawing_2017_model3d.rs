//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Model3D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:model3d")]
pub struct Model3D {
  /// Embedded Picture Reference
  #[sdk(attr(qname = "r:embed"))]
  pub embed: Option<crate::simple_type::StringValue>,
  /// Linked Picture Reference
  #[sdk(attr(qname = "r:link"))]
  pub link: Option<crate::simple_type::StringValue>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "am3d:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// Defines the Model3DCamera Class.
  #[sdk(child(qname = "am3d:camera"))]
  pub model3_d_camera: std::boxed::Box<Model3DCamera>,
  /// Defines the Model3DTransform Class.
  #[sdk(child(qname = "am3d:trans"))]
  pub model3_d_transform: std::boxed::Box<Model3DTransform>,
  /// Optional source attribution URL describes from whence the 3D model came.
  #[sdk(child(qname = "am3d:attrSrcUrl"))]
  pub picture_attribution_source_url: Option<PictureAttributionSourceUrl>,
  /// Defines the Model3DRaster Class.
  #[sdk(child(qname = "am3d:raster"))]
  pub model3_d_raster: Option<std::boxed::Box<Model3DRaster>>,
  /// Future Model3D extensions
  #[sdk(child(qname = "am3d:extLst"))]
  pub model3_d_extension_list: Option<Model3DExtensionList>,
  #[sdk(
        choice(
            child(variant = ObjectViewport, qname = "am3d:objViewport"),
            child(variant = WindowViewport, qname = "am3d:winViewport")
        )
    )]
  pub model3_d_choice1: Option<Model3DChoice>,
  /// Ambient light in a scene.
  #[sdk(child(qname = "am3d:ambientLight"))]
  pub ambient_light: Option<std::boxed::Box<AmbientLight>>,
  #[sdk(
        choice(
            child(variant = PointLight, qname = "am3d:ptLight"),
            child(variant = SpotLight, qname = "am3d:spotLight"),
            child(variant = DirectionalLight, qname = "am3d:dirLight"),
            empty_child(variant = UnknownLight, qname = "am3d:unkLight")
        )
    )]
  pub model3_d_choice2: Vec<Model3DChoice2>,
}
/// Defines the SxRatio Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:sx")]
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
#[sdk(qname = "am3d:sy")]
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
#[sdk(qname = "am3d:sz")]
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
#[sdk(qname = "am3d:meterPerModelUnit")]
pub struct MeterPerModelUnitPositiveRatio {
  /// n
  #[sdk(attr(qname = ":n"))]
  pub n: crate::simple_type::UInt64Value,
  /// d
  #[sdk(attr(qname = ":d"))]
  pub d: crate::simple_type::UInt64Value,
}
/// Defines the SzPositiveRatio Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:sz")]
pub struct SzPositiveRatio {
  /// n
  #[sdk(attr(qname = ":n"))]
  pub n: crate::simple_type::UInt64Value,
  /// d
  #[sdk(attr(qname = ":d"))]
  pub d: crate::simple_type::UInt64Value,
}
/// Defines the IlluminancePositiveRatio Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:illuminance")]
pub struct IlluminancePositiveRatio {
  /// n
  #[sdk(attr(qname = ":n"))]
  pub n: crate::simple_type::UInt64Value,
  /// d
  #[sdk(attr(qname = ":d"))]
  pub d: crate::simple_type::UInt64Value,
}
/// Defines the IntensityPositiveRatio Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:intensity")]
pub struct IntensityPositiveRatio {
  /// n
  #[sdk(attr(qname = ":n"))]
  pub n: crate::simple_type::UInt64Value,
  /// d
  #[sdk(attr(qname = ":d"))]
  pub d: crate::simple_type::UInt64Value,
}
/// Defines the PreTransVector3D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:preTrans")]
pub struct PreTransVector3D {
  /// Distance along X-axis in 3D
  #[sdk(attr(qname = ":dx"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub dx: crate::simple_type::Int64Value,
  /// Distance along Y-axis in 3D
  #[sdk(attr(qname = ":dy"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub dy: crate::simple_type::Int64Value,
  /// Distance along Z-axis in 3D
  #[sdk(attr(qname = ":dz"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub dz: crate::simple_type::Int64Value,
}
/// Defines the PostTransVector3D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:postTrans")]
pub struct PostTransVector3D {
  /// Distance along X-axis in 3D
  #[sdk(attr(qname = ":dx"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub dx: crate::simple_type::Int64Value,
  /// Distance along Y-axis in 3D
  #[sdk(attr(qname = ":dy"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub dy: crate::simple_type::Int64Value,
  /// Distance along Z-axis in 3D
  #[sdk(attr(qname = ":dz"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub dz: crate::simple_type::Int64Value,
}
/// Defines the UpVector3D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:up")]
pub struct UpVector3D {
  /// Distance along X-axis in 3D
  #[sdk(attr(qname = ":dx"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub dx: crate::simple_type::Int64Value,
  /// Distance along Y-axis in 3D
  #[sdk(attr(qname = ":dy"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub dy: crate::simple_type::Int64Value,
  /// Distance along Z-axis in 3D
  #[sdk(attr(qname = ":dz"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub dz: crate::simple_type::Int64Value,
}
/// Defines the Scale3D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:scale")]
pub struct Scale3D {
  /// Defines the SxRatio Class.
  #[sdk(child(qname = "am3d:sx"))]
  pub sx_ratio: std::boxed::Box<SxRatio>,
  /// Defines the SyRatio Class.
  #[sdk(child(qname = "am3d:sy"))]
  pub sy_ratio: std::boxed::Box<SyRatio>,
  /// Defines the SzRatio Class.
  #[sdk(child(qname = "am3d:sz"))]
  pub sz_ratio: std::boxed::Box<SzRatio>,
}
/// Defines the Rotate3D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:rot")]
pub struct Rotate3D {
  /// ax
  #[sdk(attr(qname = ":ax"))]
  pub ax: Option<crate::simple_type::Int32Value>,
  /// ay
  #[sdk(attr(qname = ":ay"))]
  pub ay: Option<crate::simple_type::Int32Value>,
  /// az
  #[sdk(attr(qname = ":az"))]
  pub az: Option<crate::simple_type::Int32Value>,
}
/// Defines the OfficeArtExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:extLst")]
pub struct OfficeArtExtensionList {
  /// Extension.
  #[sdk(child(qname = "a:ext"))]
  pub extension: Vec<crate::schemas::a::Extension>,
}
/// Defines the PosPoint3D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:pos")]
pub struct PosPoint3D {
  /// X-Coordinate in 3D
  #[sdk(attr(qname = ":x"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub x: crate::simple_type::Int64Value,
  /// Y-Coordinate in 3D
  #[sdk(attr(qname = ":y"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub y: crate::simple_type::Int64Value,
  /// Z-Coordinate in 3D
  #[sdk(attr(qname = ":z"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub z: crate::simple_type::Int64Value,
}
/// Defines the LookAtPoint3D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:lookAt")]
pub struct LookAtPoint3D {
  /// X-Coordinate in 3D
  #[sdk(attr(qname = ":x"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub x: crate::simple_type::Int64Value,
  /// Y-Coordinate in 3D
  #[sdk(attr(qname = ":y"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub y: crate::simple_type::Int64Value,
  /// Z-Coordinate in 3D
  #[sdk(attr(qname = ":z"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub z: crate::simple_type::Int64Value,
}
/// Defines the OrthographicProjection Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:orthographic")]
pub struct OrthographicProjection {
  /// Defines the SzPositiveRatio Class.
  #[sdk(child(qname = "am3d:sz"))]
  pub sz_positive_ratio: SzPositiveRatio,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "am3d:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the PerspectiveProjection Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:perspective")]
pub struct PerspectiveProjection {
  /// fov
  #[sdk(attr(qname = ":fov"))]
  #[sdk(number_range(range = 0..= 10800000))]
  pub fov: crate::simple_type::Int32Value,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "am3d:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the Blip Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:blip")]
pub struct Blip {
  /// Embedded Picture Reference
  #[sdk(attr(qname = "r:embed"))]
  pub embed: Option<crate::simple_type::StringValue>,
  /// Linked Picture Reference
  #[sdk(attr(qname = "r:link"))]
  pub link: Option<crate::simple_type::StringValue>,
  /// Compression state for blips.
  #[sdk(attr(qname = ":cstate"))]
  #[sdk(string_format(kind = "token"))]
  pub compression_state: Option<crate::schemas::a::BlipCompressionValues>,
  #[sdk(
        choice(
            child(variant = AlphaBiLevel, qname = "a:alphaBiLevel"),
            empty_child(variant = AlphaCeiling, qname = "a:alphaCeiling"),
            empty_child(variant = AlphaFloor, qname = "a:alphaFloor"),
            child(variant = AlphaInverse, qname = "a:alphaInv"),
            child(variant = AlphaModulationEffect, qname = "a:alphaMod"),
            child(variant = AlphaModulationFixed, qname = "a:alphaModFix"),
            child(variant = AlphaReplace, qname = "a:alphaRepl"),
            child(variant = BiLevel, qname = "a:biLevel"),
            child(variant = Blur, qname = "a:blur"),
            child(variant = ColorChange, qname = "a:clrChange"),
            child(variant = ColorReplacement, qname = "a:clrRepl"),
            child(variant = Duotone, qname = "a:duotone"),
            child(variant = FillOverlay, qname = "a:fillOverlay"),
            empty_child(variant = Grayscale, qname = "a:grayscl"),
            child(variant = Hsl, qname = "a:hsl"),
            child(variant = LuminanceEffect, qname = "a:lum"),
            child(variant = TintEffect, qname = "a:tint")
        )
    )]
  pub blip_choice: Vec<BlipChoice>,
  /// Future extensions..
  #[sdk(child(qname = "a:extLst"))]
  pub blip_extension_list: Option<crate::schemas::a::BlipExtensionList>,
}
/// Defines the ColorType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:clr")]
pub struct ColorType {
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
  pub color_type_choice: Option<ColorTypeChoice>,
}
/// Defines the Model3DExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:ext")]
pub struct Model3DExtension {
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(
        choice(
            child(variant = EmbeddedAnimation, qname = "a3danim:embedAnim"),
            child(variant = PosterFrame, qname = "a3danim:posterFrame"),
            any
        )
    )]
  pub model3_d_extension_choice: Option<Model3DExtensionChoice>,
}
/// Defines the ShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:spPr")]
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
/// Defines the Model3DCamera Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:camera")]
pub struct Model3DCamera {
  /// Defines the PosPoint3D Class.
  #[sdk(child(qname = "am3d:pos"))]
  pub pos_point3_d: PosPoint3D,
  /// Defines the UpVector3D Class.
  #[sdk(child(qname = "am3d:up"))]
  pub up_vector3_d: UpVector3D,
  /// Defines the LookAtPoint3D Class.
  #[sdk(child(qname = "am3d:lookAt"))]
  pub look_at_point3_d: LookAtPoint3D,
  #[sdk(
        choice(
            child(variant = OrthographicProjection, qname = "am3d:orthographic"),
            child(variant = PerspectiveProjection, qname = "am3d:perspective")
        )
    )]
  pub model3_d_camera_choice: Option<Model3DCameraChoice>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "am3d:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the Model3DTransform Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:trans")]
pub struct Model3DTransform {
  /// Defines the MeterPerModelUnitPositiveRatio Class.
  #[sdk(child(qname = "am3d:meterPerModelUnit"))]
  pub meter_per_model_unit_positive_ratio: Option<MeterPerModelUnitPositiveRatio>,
  /// Defines the PreTransVector3D Class.
  #[sdk(child(qname = "am3d:preTrans"))]
  pub pre_trans_vector3_d: Option<PreTransVector3D>,
  /// Defines the Scale3D Class.
  #[sdk(child(qname = "am3d:scale"))]
  pub scale3_d: Option<std::boxed::Box<Scale3D>>,
  /// Defines the Rotate3D Class.
  #[sdk(child(qname = "am3d:rot"))]
  pub rotate3_d: Option<Rotate3D>,
  /// Defines the PostTransVector3D Class.
  #[sdk(child(qname = "am3d:postTrans"))]
  pub post_trans_vector3_d: Option<PostTransVector3D>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "am3d:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Optional source attribution URL describes from whence the 3D model came..
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:attrSrcUrl")]
pub struct PictureAttributionSourceUrl {
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the Model3DRaster Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:raster")]
pub struct Model3DRaster {
  /// rName
  #[sdk(attr(qname = ":rName"))]
  pub r_name: crate::simple_type::StringValue,
  /// rVer
  #[sdk(attr(qname = ":rVer"))]
  pub r_ver: crate::simple_type::StringValue,
  /// Defines the Blip Class.
  #[sdk(child(qname = "am3d:blip"))]
  pub blip: Option<std::boxed::Box<Blip>>,
}
/// Future Model3D extensions.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:extLst")]
pub struct Model3DExtensionList {
  /// Defines the Model3DExtension Class.
  #[sdk(child(qname = "am3d:ext"))]
  pub model3_d_extension: Vec<Model3DExtension>,
}
/// Defines the ObjectViewport Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:objViewport")]
pub struct ObjectViewport {
  /// viewportSz
  #[sdk(attr(qname = ":viewportSz"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub viewport_sz: crate::simple_type::Int64Value,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "am3d:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the WindowViewport Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:winViewport")]
pub struct WindowViewport {
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "am3d:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Ambient light in a scene.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:ambientLight")]
pub struct AmbientLight {
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// Defines the ColorType Class.
  #[sdk(child(qname = "am3d:clr"))]
  pub color_type: std::boxed::Box<ColorType>,
  /// Defines the IlluminancePositiveRatio Class.
  #[sdk(child(qname = "am3d:illuminance"))]
  pub illuminance_positive_ratio: IlluminancePositiveRatio,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "am3d:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the PointLight Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:ptLight")]
pub struct PointLight {
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// rad
  #[sdk(attr(qname = ":rad"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub rad: crate::simple_type::Int64Value,
  /// Defines the ColorType Class.
  #[sdk(child(qname = "am3d:clr"))]
  pub color_type: std::boxed::Box<ColorType>,
  /// Defines the IntensityPositiveRatio Class.
  #[sdk(child(qname = "am3d:intensity"))]
  pub intensity_positive_ratio: IntensityPositiveRatio,
  /// Defines the PosPoint3D Class.
  #[sdk(child(qname = "am3d:pos"))]
  pub pos_point3_d: PosPoint3D,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "am3d:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the SpotLight Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:spotLight")]
pub struct SpotLight {
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// rad
  #[sdk(attr(qname = ":rad"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub rad: crate::simple_type::Int64Value,
  /// spotAng
  #[sdk(attr(qname = ":spotAng"))]
  #[sdk(number_range(range = 0..= 10800000))]
  pub spot_ang: crate::simple_type::Int32Value,
  /// Defines the ColorType Class.
  #[sdk(child(qname = "am3d:clr"))]
  pub color_type: std::boxed::Box<ColorType>,
  /// Defines the IntensityPositiveRatio Class.
  #[sdk(child(qname = "am3d:intensity"))]
  pub intensity_positive_ratio: IntensityPositiveRatio,
  /// Defines the PosPoint3D Class.
  #[sdk(child(qname = "am3d:pos"))]
  pub pos_point3_d: PosPoint3D,
  /// Defines the LookAtPoint3D Class.
  #[sdk(child(qname = "am3d:lookAt"))]
  pub look_at_point3_d: LookAtPoint3D,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "am3d:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DirectionalLight Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:dirLight")]
pub struct DirectionalLight {
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// angularRad
  #[sdk(attr(qname = ":angularRad"))]
  #[sdk(number_range(range = 0..= 5400000))]
  pub angular_rad: crate::simple_type::Int32Value,
  /// Defines the ColorType Class.
  #[sdk(child(qname = "am3d:clr"))]
  pub color_type: std::boxed::Box<ColorType>,
  /// Defines the IlluminancePositiveRatio Class.
  #[sdk(child(qname = "am3d:illuminance"))]
  pub illuminance_positive_ratio: IlluminancePositiveRatio,
  /// Defines the PosPoint3D Class.
  #[sdk(child(qname = "am3d:pos"))]
  pub pos_point3_d: PosPoint3D,
  /// Defines the LookAtPoint3D Class.
  #[sdk(child(qname = "am3d:lookAt"))]
  pub look_at_point3_d: LookAtPoint3D,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "am3d:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Model3DChoice {
  /// Defines the ObjectViewport Class.
  ObjectViewport(std::boxed::Box<ObjectViewport>),
  /// Defines the WindowViewport Class.
  WindowViewport(std::boxed::Box<WindowViewport>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Model3DChoice2 {
  /// Defines the PointLight Class.
  PointLight(std::boxed::Box<PointLight>),
  /// Defines the SpotLight Class.
  SpotLight(std::boxed::Box<SpotLight>),
  /// Defines the DirectionalLight Class.
  DirectionalLight(std::boxed::Box<DirectionalLight>),
  /// Defines the UnknownLight Class.
  UnknownLight,
}
#[derive(Clone, Debug, PartialEq)]
pub enum BlipChoice {
  AlphaBiLevel(std::boxed::Box<crate::schemas::a::AlphaBiLevel>),
  /// Alpha Ceiling Effect.
  AlphaCeiling,
  /// Alpha Floor Effect.
  AlphaFloor,
  AlphaInverse(std::boxed::Box<crate::schemas::a::AlphaInverse>),
  AlphaModulationEffect(std::boxed::Box<crate::schemas::a::AlphaModulationEffect>),
  AlphaModulationFixed(std::boxed::Box<crate::schemas::a::AlphaModulationFixed>),
  AlphaReplace(std::boxed::Box<crate::schemas::a::AlphaReplace>),
  BiLevel(std::boxed::Box<crate::schemas::a::BiLevel>),
  Blur(std::boxed::Box<crate::schemas::a::Blur>),
  ColorChange(std::boxed::Box<crate::schemas::a::ColorChange>),
  ColorReplacement(std::boxed::Box<crate::schemas::a::ColorReplacement>),
  Duotone(std::boxed::Box<crate::schemas::a::Duotone>),
  FillOverlay(std::boxed::Box<crate::schemas::a::FillOverlay>),
  /// Gray Scale Effect.
  Grayscale,
  Hsl(std::boxed::Box<crate::schemas::a::Hsl>),
  LuminanceEffect(std::boxed::Box<crate::schemas::a::LuminanceEffect>),
  TintEffect(std::boxed::Box<crate::schemas::a::TintEffect>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ColorTypeChoice {
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
pub enum Model3DExtensionChoice {
  /// Defines the EmbeddedAnimation Class.
  EmbeddedAnimation(std::boxed::Box<crate::schemas::a3danim::EmbeddedAnimation>),
  /// Defines the PosterFrame Class.
  PosterFrame(std::boxed::Box<crate::schemas::a3danim::PosterFrame>),
  XmlAny(std::boxed::Box<[u8]>),
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
pub enum Model3DCameraChoice {
  /// Defines the OrthographicProjection Class.
  OrthographicProjection(std::boxed::Box<OrthographicProjection>),
  /// Defines the PerspectiveProjection Class.
  PerspectiveProjection(std::boxed::Box<PerspectiveProjection>),
}
