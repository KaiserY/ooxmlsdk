//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Model3D Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:model3d.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:CT_Model3D/am3d:model3d")]
pub struct Model3D {
  /// Embedded Picture Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:embed
  #[sdk(attr(qname = "r:embed"))]
  pub embed: Option<crate::simple_type::StringValue>,
  /// Linked Picture Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:link
  #[sdk(attr(qname = "r:link"))]
  pub link: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/am3d:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// _
  #[sdk(child(qname = "am3d:CT_Model3DCamera/am3d:camera"))]
  pub model3_d_camera: std::boxed::Box<Model3DCamera>,
  /// _
  #[sdk(child(qname = "am3d:CT_Model3DTransform/am3d:trans"))]
  pub model3_d_transform: std::boxed::Box<Model3DTransform>,
  /// Optional source attribution URL describes from whence the 3D model came.
  #[sdk(child(qname = "a1611:CT_PictureAttributionSourceURL/am3d:attrSrcUrl"))]
  pub picture_attribution_source_url: Option<PictureAttributionSourceUrl>,
  /// _
  #[sdk(child(qname = "am3d:CT_Model3DRaster/am3d:raster"))]
  pub model3_d_raster: Option<std::boxed::Box<Model3DRaster>>,
  /// Future Model3D extensions
  #[sdk(child(qname = "am3d:CT_Model3DExtensionList/am3d:extLst"))]
  pub model3_d_extension_list: Option<Model3DExtensionList>,
  #[sdk(choice(
    qname = "am3d:CT_ObjectViewport/am3d:objViewport",
    qname = "am3d:CT_WindowViewport/am3d:winViewport"
  ))]
  pub model3_d_choice1: Option<Model3DChoice>,
  /// _
  #[sdk(child(qname = "am3d:CT_AmbientLight/am3d:ambientLight"))]
  pub am3d_ambient_light: Option<std::boxed::Box<AmbientLight>>,
  #[sdk(choice(
    qname = "am3d:CT_PointLight/am3d:ptLight",
    qname = "am3d:CT_SpotLight/am3d:spotLight",
    qname = "am3d:CT_DirectionalLight/am3d:dirLight",
    qname = "am3d:CT_UnknownLight/am3d:unkLight"
  ))]
  pub model3_d_choice2: Vec<Model3DChoice2>,
}
/// Defines the SxRatio Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:sx.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Ratio/am3d:sx")]
pub struct SxRatio {
  /// Numerator
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :n
  #[sdk(attr(qname = ":n"))]
  pub numerator: crate::simple_type::Int32Value,
  /// Denominator
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :d
  #[sdk(attr(qname = ":d"))]
  pub denominator: crate::simple_type::Int32Value,
}
/// Defines the SyRatio Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:sy.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Ratio/am3d:sy")]
pub struct SyRatio {
  /// Numerator
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :n
  #[sdk(attr(qname = ":n"))]
  pub numerator: crate::simple_type::Int32Value,
  /// Denominator
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :d
  #[sdk(attr(qname = ":d"))]
  pub denominator: crate::simple_type::Int32Value,
}
/// Defines the SzRatio Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:sz.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Ratio/am3d:sz")]
pub struct SzRatio {
  /// Numerator
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :n
  #[sdk(attr(qname = ":n"))]
  pub numerator: crate::simple_type::Int32Value,
  /// Denominator
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :d
  #[sdk(attr(qname = ":d"))]
  pub denominator: crate::simple_type::Int32Value,
}
/// Defines the RatioType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Ratio/")]
pub struct RatioType {
  /// Numerator
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :n
  #[sdk(attr(qname = ":n"))]
  pub numerator: crate::simple_type::Int32Value,
  /// Denominator
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :d
  #[sdk(attr(qname = ":d"))]
  pub denominator: crate::simple_type::Int32Value,
}
/// Defines the MeterPerModelUnitPositiveRatio Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:meterPerModelUnit.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:CT_PositiveRatio/am3d:meterPerModelUnit")]
pub struct MeterPerModelUnitPositiveRatio {
  /// n
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :n
  #[sdk(attr(qname = ":n"))]
  pub n: crate::simple_type::UInt64Value,
  /// d
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :d
  #[sdk(attr(qname = ":d"))]
  pub d: crate::simple_type::UInt64Value,
}
/// Defines the SzPositiveRatio Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:sz.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:CT_PositiveRatio/am3d:sz")]
pub struct SzPositiveRatio {
  /// n
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :n
  #[sdk(attr(qname = ":n"))]
  pub n: crate::simple_type::UInt64Value,
  /// d
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :d
  #[sdk(attr(qname = ":d"))]
  pub d: crate::simple_type::UInt64Value,
}
/// Defines the IlluminancePositiveRatio Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:illuminance.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:CT_PositiveRatio/am3d:illuminance")]
pub struct IlluminancePositiveRatio {
  /// n
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :n
  #[sdk(attr(qname = ":n"))]
  pub n: crate::simple_type::UInt64Value,
  /// d
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :d
  #[sdk(attr(qname = ":d"))]
  pub d: crate::simple_type::UInt64Value,
}
/// Defines the IntensityPositiveRatio Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:intensity.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:CT_PositiveRatio/am3d:intensity")]
pub struct IntensityPositiveRatio {
  /// n
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :n
  #[sdk(attr(qname = ":n"))]
  pub n: crate::simple_type::UInt64Value,
  /// d
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :d
  #[sdk(attr(qname = ":d"))]
  pub d: crate::simple_type::UInt64Value,
}
/// Defines the OpenXmlPositiveRatioElement Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:CT_PositiveRatio/")]
pub struct OpenXmlPositiveRatioElement {
  /// n
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :n
  #[sdk(attr(qname = ":n"))]
  pub n: crate::simple_type::UInt64Value,
  /// d
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :d
  #[sdk(attr(qname = ":d"))]
  pub d: crate::simple_type::UInt64Value,
}
/// Defines the PreTransVector3D Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:preTrans.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Vector3D/am3d:preTrans")]
pub struct PreTransVector3D {
  /// Distance along X-axis in 3D
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dx
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
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dy
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
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dz
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
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:postTrans.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Vector3D/am3d:postTrans")]
pub struct PostTransVector3D {
  /// Distance along X-axis in 3D
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dx
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
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dy
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
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dz
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
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:up.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Vector3D/am3d:up")]
pub struct UpVector3D {
  /// Distance along X-axis in 3D
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dx
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
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dy
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
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dz
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
/// Defines the Vector3DType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Vector3D/")]
pub struct Vector3DType {
  /// Distance along X-axis in 3D
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dx
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
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dy
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
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dz
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
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:scale.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:CT_Scale3D/am3d:scale")]
pub struct Scale3D {
  /// _
  #[sdk(child(qname = "a:CT_Ratio/am3d:sx"))]
  pub sx_ratio: std::boxed::Box<SxRatio>,
  /// _
  #[sdk(child(qname = "a:CT_Ratio/am3d:sy"))]
  pub sy_ratio: std::boxed::Box<SyRatio>,
  /// _
  #[sdk(child(qname = "a:CT_Ratio/am3d:sz"))]
  pub sz_ratio: std::boxed::Box<SzRatio>,
}
/// Defines the Rotate3D Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:rot.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:CT_Rotate3D/am3d:rot")]
pub struct Rotate3D {
  /// ax
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :ax
  #[sdk(attr(qname = ":ax"))]
  pub ax: Option<crate::simple_type::Int32Value>,
  /// ay
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :ay
  #[sdk(attr(qname = ":ay"))]
  pub ay: Option<crate::simple_type::Int32Value>,
  /// az
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :az
  #[sdk(attr(qname = ":az"))]
  pub az: Option<crate::simple_type::Int32Value>,
}
/// Defines the OfficeArtExtensionList Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_OfficeArtExtensionList/am3d:extLst")]
pub struct OfficeArtExtensionList {
  /// Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension>,
}
/// Defines the PosPoint3D Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:pos.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Point3D/am3d:pos")]
pub struct PosPoint3D {
  /// X-Coordinate in 3D
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
  /// Y-Coordinate in 3D
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
  /// Z-Coordinate in 3D
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :z
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
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:lookAt.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Point3D/am3d:lookAt")]
pub struct LookAtPoint3D {
  /// X-Coordinate in 3D
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
  /// Y-Coordinate in 3D
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
  /// Z-Coordinate in 3D
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :z
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
/// Defines the OpenXmlPoint3DElement Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Point3D/")]
pub struct OpenXmlPoint3DElement {
  /// X-Coordinate in 3D
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
  /// Y-Coordinate in 3D
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
  /// Z-Coordinate in 3D
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :z
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
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:orthographic.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:CT_OrthographicProjection/am3d:orthographic")]
pub struct OrthographicProjection {
  /// _
  #[sdk(child(qname = "am3d:CT_PositiveRatio/am3d:sz"))]
  pub sz_positive_ratio: std::boxed::Box<SzPositiveRatio>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/am3d:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the PerspectiveProjection Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:perspective.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:CT_PerspectiveProjection/am3d:perspective")]
pub struct PerspectiveProjection {
  /// fov
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :fov
  #[sdk(attr(qname = ":fov"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "10800000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub fov: crate::simple_type::Int32Value,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/am3d:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the Blip Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:blip.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Blip/am3d:blip")]
pub struct Blip {
  /// Embedded Picture Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:embed
  #[sdk(attr(qname = "r:embed"))]
  pub embed: Option<crate::simple_type::StringValue>,
  /// Linked Picture Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:link
  #[sdk(attr(qname = "r:link"))]
  pub link: Option<crate::simple_type::StringValue>,
  /// Compression state for blips.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cstate
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
  /// _
  #[sdk(child(qname = "a:CT_BlipExtensionList/a:extLst"))]
  pub a_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipExtensionList>,
}
/// Defines the ColorType Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:clr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color/am3d:clr")]
pub struct ColorType {
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub xml_children: Option<ColorTypeChoice>,
}
/// Defines the Model3DExtension Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:CT_Model3DExtension/am3d:ext")]
pub struct Model3DExtension {
  /// URI
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "a3danim:CT_EmbeddedAnimation/a3danim:embedAnim",
    qname = "a3danim:CT_PosterFrame/a3danim:posterFrame",
    any
  ))]
  pub xml_children: Option<Model3DExtensionChoice>,
}
/// Defines the ShapeProperties Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:spPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ShapeProperties/am3d:spPr")]
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
/// Defines the Model3DCamera Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:camera.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:CT_Model3DCamera/am3d:camera")]
pub struct Model3DCamera {
  /// _
  #[sdk(child(qname = "a:CT_Point3D/am3d:pos"))]
  pub pos_point3_d: std::boxed::Box<PosPoint3D>,
  /// _
  #[sdk(child(qname = "a:CT_Vector3D/am3d:up"))]
  pub up_vector3_d: std::boxed::Box<UpVector3D>,
  /// _
  #[sdk(child(qname = "a:CT_Point3D/am3d:lookAt"))]
  pub look_at_point3_d: std::boxed::Box<LookAtPoint3D>,
  #[sdk(choice(
    qname = "am3d:CT_OrthographicProjection/am3d:orthographic",
    qname = "am3d:CT_PerspectiveProjection/am3d:perspective"
  ))]
  pub model3_d_camera_choice: Option<Model3DCameraChoice>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/am3d:extLst"))]
  pub am3d_ext_lst: Option<OfficeArtExtensionList>,
}
/// Defines the Model3DTransform Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:trans.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:CT_Model3DTransform/am3d:trans")]
pub struct Model3DTransform {
  /// _
  #[sdk(child(qname = "am3d:CT_PositiveRatio/am3d:meterPerModelUnit"))]
  pub meter_per_model_unit_positive_ratio: Option<MeterPerModelUnitPositiveRatio>,
  /// _
  #[sdk(child(qname = "a:CT_Vector3D/am3d:preTrans"))]
  pub pre_trans_vector3_d: Option<PreTransVector3D>,
  /// _
  #[sdk(child(qname = "am3d:CT_Scale3D/am3d:scale"))]
  pub scale3_d: Option<std::boxed::Box<Scale3D>>,
  /// _
  #[sdk(child(qname = "am3d:CT_Rotate3D/am3d:rot"))]
  pub rotate3_d: Option<Rotate3D>,
  /// _
  #[sdk(child(qname = "a:CT_Vector3D/am3d:postTrans"))]
  pub post_trans_vector3_d: Option<PostTransVector3D>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/am3d:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Optional source attribution URL describes from whence the 3D model came..
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:attrSrcUrl.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a1611:CT_PictureAttributionSourceURL/am3d:attrSrcUrl")]
pub struct PictureAttributionSourceUrl {
  /// id
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the Model3DRaster Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:raster.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:CT_Model3DRaster/am3d:raster")]
pub struct Model3DRaster {
  /// rName
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :rName
  #[sdk(attr(qname = ":rName"))]
  pub r_name: crate::simple_type::StringValue,
  /// rVer
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :rVer
  #[sdk(attr(qname = ":rVer"))]
  pub r_ver: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "a:CT_Blip/am3d:blip"))]
  pub blip: Option<std::boxed::Box<Blip>>,
}
/// Future Model3D extensions.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:CT_Model3DExtensionList/am3d:extLst")]
pub struct Model3DExtensionList {
  /// _
  #[sdk(child(qname = "am3d:CT_Model3DExtension/am3d:ext"))]
  pub am3d_ext: Vec<Model3DExtension>,
}
/// Defines the ObjectViewport Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:objViewport.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:CT_ObjectViewport/am3d:objViewport")]
pub struct ObjectViewport {
  /// viewportSz
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :viewportSz
  #[sdk(attr(qname = ":viewportSz"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub viewport_sz: crate::simple_type::Int64Value,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/am3d:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the WindowViewport Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:winViewport.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:CT_WindowViewport/am3d:winViewport")]
pub struct WindowViewport {
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/am3d:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Ambient light in a scene.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:ambientLight.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:CT_AmbientLight/am3d:ambientLight")]
pub struct AmbientLight {
  /// enabled
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "a:CT_Color/am3d:clr"))]
  pub color_type: std::boxed::Box<ColorType>,
  /// _
  #[sdk(child(qname = "am3d:CT_PositiveRatio/am3d:illuminance"))]
  pub illuminance_positive_ratio: std::boxed::Box<IlluminancePositiveRatio>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/am3d:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the PointLight Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:ptLight.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:CT_PointLight/am3d:ptLight")]
pub struct PointLight {
  /// enabled
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// rad
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :rad
  #[sdk(attr(qname = ":rad"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub rad: crate::simple_type::Int64Value,
  /// _
  #[sdk(child(qname = "a:CT_Color/am3d:clr"))]
  pub color_type: std::boxed::Box<ColorType>,
  /// _
  #[sdk(child(qname = "am3d:CT_PositiveRatio/am3d:intensity"))]
  pub intensity_positive_ratio: std::boxed::Box<IntensityPositiveRatio>,
  /// _
  #[sdk(child(qname = "a:CT_Point3D/am3d:pos"))]
  pub pos_point3_d: std::boxed::Box<PosPoint3D>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/am3d:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the SpotLight Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:spotLight.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:CT_SpotLight/am3d:spotLight")]
pub struct SpotLight {
  /// enabled
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// rad
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :rad
  #[sdk(attr(qname = ":rad"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub rad: crate::simple_type::Int64Value,
  /// spotAng
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :spotAng
  #[sdk(attr(qname = ":spotAng"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "10800000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub spot_ang: crate::simple_type::Int32Value,
  /// _
  #[sdk(child(qname = "a:CT_Color/am3d:clr"))]
  pub color_type: std::boxed::Box<ColorType>,
  /// _
  #[sdk(child(qname = "am3d:CT_PositiveRatio/am3d:intensity"))]
  pub intensity_positive_ratio: std::boxed::Box<IntensityPositiveRatio>,
  /// _
  #[sdk(child(qname = "a:CT_Point3D/am3d:pos"))]
  pub pos_point3_d: std::boxed::Box<PosPoint3D>,
  /// _
  #[sdk(child(qname = "a:CT_Point3D/am3d:lookAt"))]
  pub look_at_point3_d: std::boxed::Box<LookAtPoint3D>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/am3d:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DirectionalLight Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is am3d:dirLight.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "am3d:CT_DirectionalLight/am3d:dirLight")]
pub struct DirectionalLight {
  /// enabled
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// angularRad
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :angularRad
  #[sdk(attr(qname = ":angularRad"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "5400000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub angular_rad: crate::simple_type::Int32Value,
  /// _
  #[sdk(child(qname = "a:CT_Color/am3d:clr"))]
  pub color_type: std::boxed::Box<ColorType>,
  /// _
  #[sdk(child(qname = "am3d:CT_PositiveRatio/am3d:illuminance"))]
  pub illuminance_positive_ratio: std::boxed::Box<IlluminancePositiveRatio>,
  /// _
  #[sdk(child(qname = "a:CT_Point3D/am3d:pos"))]
  pub pos_point3_d: std::boxed::Box<PosPoint3D>,
  /// _
  #[sdk(child(qname = "a:CT_Point3D/am3d:lookAt"))]
  pub look_at_point3_d: std::boxed::Box<LookAtPoint3D>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/am3d:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Model3DChoice {
  #[sdk(child(qname = "am3d:CT_ObjectViewport/am3d:objViewport"))]
  Am3dObjViewport(std::boxed::Box<ObjectViewport>),
  #[sdk(child(qname = "am3d:CT_WindowViewport/am3d:winViewport"))]
  Am3dWinViewport(std::boxed::Box<WindowViewport>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Model3DChoice2 {
  #[sdk(child(qname = "am3d:CT_PointLight/am3d:ptLight"))]
  Am3dPtLight(std::boxed::Box<PointLight>),
  #[sdk(child(qname = "am3d:CT_SpotLight/am3d:spotLight"))]
  Am3dSpotLight(std::boxed::Box<SpotLight>),
  #[sdk(child(qname = "am3d:CT_DirectionalLight/am3d:dirLight"))]
  Am3dDirLight(std::boxed::Box<DirectionalLight>),
  /// Defines the UnknownLight Class.
  #[sdk(empty_child(qname = "am3d:CT_UnknownLight/am3d:unkLight"))]
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
  #[sdk(child(qname = "a3danim:CT_EmbeddedAnimation/a3danim:embedAnim"))]
    A3danimEmbedAnim(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_drawing_2018_animation_model3d::EmbeddedAnimation,
        >,
    ),
    #[sdk(child(qname = "a3danim:CT_PosterFrame/a3danim:posterFrame"))]
    A3danimPosterFrame(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_drawing_2018_animation_model3d::PosterFrame,
        >,
    ),
    #[sdk(any)]
    UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
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
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
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
  /// Group Fill.
  #[sdk(empty_child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
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
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Model3DCameraChoice {
  #[sdk(child(qname = "am3d:CT_OrthographicProjection/am3d:orthographic"))]
  Am3dOrthographic(std::boxed::Box<OrthographicProjection>),
  #[sdk(child(qname = "am3d:CT_PerspectiveProjection/am3d:perspective"))]
  Am3dPerspective(std::boxed::Box<PerspectiveProjection>),
}
