//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ChannelDataTypeValues {
  #[sdk(rename = "integer")]
  #[default]
  Integer,
  #[sdk(rename = "decimal")]
  Decimal,
  #[sdk(rename = "boolean")]
  Boolean,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ChannelValueOrientationValues {
  #[sdk(rename = "+ve")]
  #[default]
  PlusVe,
  #[sdk(rename = "-ve")]
  MinusVe,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum StandardChannelPropertyNameValues {
  #[sdk(rename = "threshold")]
  #[default]
  Threshold,
  #[sdk(rename = "resolution")]
  Resolution,
  #[sdk(rename = "quantization")]
  Quantization,
  #[sdk(rename = "noise")]
  Noise,
  #[sdk(rename = "accuracy")]
  Accuracy,
  #[sdk(rename = "crossCoupling")]
  CrossCoupling,
  #[sdk(rename = "skew")]
  Skew,
  #[sdk(rename = "minBandwidth")]
  MinBandwidth,
  #[sdk(rename = "peakRate")]
  PeakRate,
  #[sdk(rename = "distortion")]
  Distortion,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum StandardBrushPropertyNameValues {
  #[sdk(rename = "width")]
  #[default]
  Width,
  #[sdk(rename = "height")]
  Height,
  #[sdk(rename = "color")]
  Color,
  #[sdk(rename = "transparency")]
  Transparency,
  #[sdk(rename = "tip")]
  Tip,
  #[sdk(rename = "rasterOp")]
  RasterOp,
  #[sdk(rename = "antiAliased")]
  AntiAliased,
  #[sdk(rename = "fitToCurve")]
  FitToCurve,
  #[sdk(rename = "ignorePressure")]
  IgnorePressure,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum StandardChannelNameValues {
  #[sdk(rename = "X")]
  #[default]
  XCoordinate,
  #[sdk(rename = "Y")]
  YCoordinate,
  #[sdk(rename = "Z")]
  ZCoordinate,
  #[sdk(rename = "F")]
  PenTipForce,
  #[sdk(rename = "TP")]
  TangentPressure,
  #[sdk(rename = "BP")]
  ButtonPressure,
  #[sdk(rename = "S")]
  TipSwitchState,
  #[sdk(rename = "B1")]
  SideButton1State,
  #[sdk(rename = "B2")]
  SideButton2State,
  #[sdk(rename = "B3")]
  SideButton3State,
  #[sdk(rename = "B4")]
  SideButton4State,
  #[sdk(rename = "E")]
  PenInverted,
  #[sdk(rename = "OTx")]
  TiltAlongXAxis,
  #[sdk(rename = "OTy")]
  TiltAlongYAxis,
  #[sdk(rename = "OA")]
  PenAzimuthAngle,
  #[sdk(rename = "OE")]
  PenElevationAngle,
  #[sdk(rename = "OR")]
  PexAxisRotation,
  #[sdk(rename = "RP")]
  PitchRotation,
  #[sdk(rename = "RR")]
  RollRotation,
  #[sdk(rename = "RY")]
  YawRotation,
  #[sdk(rename = "C")]
  ColorValue,
  #[sdk(rename = "CR")]
  RedColorValue,
  #[sdk(rename = "CG")]
  GreenColorValue,
  #[sdk(rename = "CB")]
  BlueColorValue,
  #[sdk(rename = "CC")]
  CyanColorValue,
  #[sdk(rename = "CM")]
  MegentaColorValue,
  #[sdk(rename = "CY")]
  YellowColorValue,
  #[sdk(rename = "CK")]
  BlackColorValue,
  #[sdk(rename = "W")]
  StrokesWidth,
  #[sdk(rename = "T")]
  Time,
  #[sdk(rename = "SN")]
  SerialNumber,
  #[sdk(rename = "TW")]
  TouchWidth,
  #[sdk(rename = "TH")]
  TouchHeight,
  #[sdk(rename = "TC")]
  FingerTouch,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum StandardLengthUnitsValues {
  #[sdk(rename = "m")]
  #[default]
  Meter,
  #[sdk(rename = "cm")]
  Centimeter,
  #[sdk(rename = "mm")]
  Millimeter,
  #[sdk(rename = "in")]
  Inche,
  #[sdk(rename = "pt")]
  Point,
  #[sdk(rename = "pc")]
  Pica,
  #[sdk(rename = "em")]
  Em,
  #[sdk(rename = "ex")]
  Ex,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum StandardPerLengthUnitsValues {
  #[sdk(rename = "1/m")]
  #[default]
  PerMeter,
  #[sdk(rename = "1/cm")]
  PerCentimeter,
  #[sdk(rename = "1/mm")]
  PerMillimeter,
  #[sdk(rename = "1/in")]
  PerInche,
  #[sdk(rename = "1/pt")]
  PerPoint,
  #[sdk(rename = "1/pc")]
  PerPica,
  #[sdk(rename = "1/em")]
  PerEm,
  #[sdk(rename = "1/ex")]
  PerEx,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum StandardTimeUnitsValues {
  #[sdk(rename = "s")]
  #[default]
  Second,
  #[sdk(rename = "ms")]
  Millisecond,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum StandardPerTimeUnitsValues {
  #[sdk(rename = "1/s")]
  #[default]
  PerSecond,
  #[sdk(rename = "1/ms")]
  PerMillisecond,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum StandardMassForceUnitsValues {
  #[sdk(rename = "Kg")]
  #[default]
  Kilogram,
  #[sdk(rename = "g")]
  Gram,
  #[sdk(rename = "mg")]
  Milligram,
  #[sdk(rename = "N")]
  Newton,
  #[sdk(rename = "lb")]
  Pond,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum StandardPerMassForceUnitsValues {
  #[sdk(rename = "1/Kg")]
  #[default]
  PerKilogram,
  #[sdk(rename = "1/g")]
  PerGram,
  #[sdk(rename = "1/mg")]
  PerMilligram,
  #[sdk(rename = "1/N")]
  PerNewton,
  #[sdk(rename = "1/lb")]
  PerPond,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum StandardAngleUnitsValues {
  #[sdk(rename = "deg")]
  #[default]
  Degree,
  #[sdk(rename = "rad")]
  Radian,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum StandardPerAngleUnitsValues {
  #[sdk(rename = "1/deg")]
  #[default]
  PerDegree,
  #[sdk(rename = "1/rad")]
  PerRadian,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum StandardOtherUnitsValues {
  #[sdk(rename = "%")]
  #[default]
  Percentage,
  #[sdk(rename = "dev")]
  DeviceResolution,
  #[sdk(rename = "none")]
  None,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum StandardPerOtherUnitsValues {
  #[sdk(rename = "1/%")]
  #[default]
  PerPercentage,
  #[sdk(rename = "1/dev")]
  PerDeviceResolution,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TraceTypeValues {
  #[sdk(rename = "penDown")]
  #[default]
  PenDown,
  #[sdk(rename = "penUp")]
  PenUp,
  #[sdk(rename = "indeterminate")]
  Indeterminate,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TraceContinuationValues {
  #[sdk(rename = "begin")]
  #[default]
  Begin,
  #[sdk(rename = "end")]
  End,
  #[sdk(rename = "middle")]
  Middle,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum RasterOperationValues {
  #[sdk(rename = "black")]
  #[default]
  Black,
  #[sdk(rename = "notMergePen")]
  NotMergePen,
  #[sdk(rename = "maskNotPen")]
  MaskNotPen,
  #[sdk(rename = "notCopyPen")]
  NotCopyPen,
  #[sdk(rename = "maskPenNot")]
  MaskPenNot,
  #[sdk(rename = "not")]
  Not,
  #[sdk(rename = "xOrPen")]
  XOrPen,
  #[sdk(rename = "notMaskPen")]
  NotMaskPen,
  #[sdk(rename = "maskPen")]
  MaskPen,
  #[sdk(rename = "notXOrPen")]
  NotXOrPen,
  #[sdk(rename = "noOperation")]
  NoOperation,
  #[sdk(rename = "mergeNotPen")]
  MergeNotPen,
  #[sdk(rename = "copyPen")]
  CopyPen,
  #[sdk(rename = "mergePenNot")]
  MergePenNot,
  #[sdk(rename = "mergePen")]
  MergePen,
  #[sdk(rename = "white")]
  White,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PenTipShapeValues {
  #[sdk(rename = "ellipse")]
  #[default]
  Ellipse,
  #[sdk(rename = "rectangle")]
  Rectangle,
  #[sdk(rename = "drop")]
  Drop,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum MappingTypeValues {
  #[sdk(rename = "identity")]
  #[default]
  Identity,
  #[sdk(rename = "lookup")]
  Lookup,
  #[sdk(rename = "affine")]
  Affine,
  #[sdk(rename = "mathml")]
  MathMl,
  #[sdk(rename = "product")]
  Product,
  #[sdk(rename = "unknown")]
  Unknown,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TableApplyValues {
  #[sdk(rename = "absolute")]
  #[default]
  Absolute,
  #[sdk(rename = "relative")]
  Relative,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TableInterpolationValues {
  #[sdk(rename = "floor")]
  #[default]
  Floor,
  #[sdk(rename = "middle")]
  Middle,
  #[sdk(rename = "ceiling")]
  Ceiling,
  #[sdk(rename = "linear")]
  Linear,
  #[sdk(rename = "cubic")]
  Cubic,
}
/// Defines the Ink Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "inkml:CT_Ink/inkml:ink")]
pub struct Ink {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// documentID
  #[sdk(attr(qname = ":documentID"))]
  #[sdk(string_format(source = 0u32, kind = "uri"))]
  pub document_id: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "inkml:CT_Annotation/inkml:annotation",
    qname = "inkml:CT_AnnotationXML/inkml:annotationXML",
    qname = "inkml:CT_Definitions/inkml:definitions",
    qname = "inkml:CT_Context/inkml:context",
    qname = "inkml:CT_Trace/inkml:trace",
    qname = "inkml:CT_TraceGroup/inkml:traceGroup",
    qname = "inkml:CT_TraceView/inkml:traceView"
  ))]
  pub xml_children: Vec<InkChoice>,
}
/// Defines the Bind Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "inkml:CT_Bind/inkml:bind")]
pub struct Bind {
  /// source
  #[sdk(attr(qname = ":source"))]
  pub source: Option<crate::simple_type::StringValue>,
  /// target
  #[sdk(attr(qname = ":target"))]
  pub target: Option<crate::simple_type::StringValue>,
  /// column
  #[sdk(attr(qname = ":column"))]
  pub column: Option<crate::simple_type::StringValue>,
  /// variable
  #[sdk(attr(qname = ":variable"))]
  pub variable: Option<crate::simple_type::StringValue>,
}
/// Defines the Table Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "inkml:CT_Table/inkml:table")]
pub struct Table {
  /// id
  #[sdk(attr(qname = "xml:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// apply
  #[sdk(attr(qname = ":apply"))]
  pub apply: Option<TableApplyValues>,
  /// interpolation
  #[sdk(attr(qname = ":interpolation"))]
  pub interpolation: Option<TableInterpolationValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the Matrix Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "inkml:CT_Matrix/inkml:matrix")]
pub struct Matrix {
  /// id
  #[sdk(attr(qname = "xml:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the Mapping Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "inkml:CT_Mapping/inkml:mapping")]
pub struct Mapping {
  /// id
  #[sdk(attr(qname = "xml:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<MappingTypeValues>,
  /// mappingRef
  #[sdk(attr(qname = ":mappingRef"))]
  #[sdk(string_format(source = 0u32, kind = "uri"))]
  pub mapping_ref: Option<crate::simple_type::StringValue>,
  /// Defines the Bind Class.
  #[sdk(child(qname = "inkml:CT_Bind/inkml:bind"))]
  pub bind: Vec<Bind>,
  #[sdk(choice(
    qname = "inkml:CT_Table/inkml:table",
    qname = "inkml:CT_Matrix/inkml:matrix"
  ))]
  pub mapping_choice: Option<MappingChoice>,
  /// Defines the Mapping Class.
  #[sdk(child(qname = "inkml:CT_Mapping/inkml:mapping"))]
  pub mapping: Vec<Mapping>,
}
/// Defines the Channel Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "inkml:CT_Channel/inkml:channel")]
pub struct Channel {
  /// id
  #[sdk(attr(qname = "xml:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// name
  #[sdk(attr(qname = ":name"))]
  #[sdk(
        string_set(
            source = 1u32,
            union = 0u64,
            values = &["X",
            "Y",
            "Z",
            "F",
            "TP",
            "BP",
            "S",
            "B1",
            "B2",
            "B3",
            "B4",
            "E",
            "OTx",
            "OTy",
            "OA",
            "OE",
            "OR",
            "RP",
            "RR",
            "RY",
            "C",
            "CR",
            "CG",
            "CB",
            "CC",
            "CM",
            "CY",
            "CK",
            "W",
            "T",
            "SN",
            "TW",
            "TH",
            "TC"]
        )
    )]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub name: crate::simple_type::StringValue,
  /// type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<ChannelDataTypeValues>,
  /// default
  #[sdk(attr(qname = ":default"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "xsd:decimal"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "xsd:boolean"))]
  pub default: Option<crate::simple_type::StringValue>,
  /// min
  #[sdk(attr(qname = ":min"))]
  pub min: Option<crate::simple_type::DecimalValue>,
  /// max
  #[sdk(attr(qname = ":max"))]
  pub max: Option<crate::simple_type::DecimalValue>,
  /// orientation
  #[sdk(attr(qname = ":orientation"))]
  pub orientation: Option<ChannelValueOrientationValues>,
  /// respectTo
  #[sdk(attr(qname = ":respectTo"))]
  #[sdk(string_format(source = 0u32, kind = "uri"))]
  pub respect_to: Option<crate::simple_type::StringValue>,
  /// units
  #[sdk(attr(qname = ":units"))]
  #[sdk(
        string_set(
            source = 0u32,
            union = 0u64,
            values = &["m",
            "cm",
            "mm",
            "in",
            "pt",
            "pc",
            "em",
            "ex"]
        )
    )]
  #[sdk(
        string_set(
            source = 1u32,
            union = 0u64,
            values = &["1/m",
            "1/cm",
            "1/mm",
            "1/in",
            "1/pt",
            "1/pc",
            "1/em",
            "1/ex"]
        )
    )]
  #[sdk(string_set(source = 2u32, union = 0u64, values = &["s", "ms"]))]
  #[sdk(string_set(source = 3u32, union = 0u64, values = &["1/s", "1/ms"]))]
  #[sdk(
        string_set(source = 4u32, union = 0u64, values = &["Kg", "g", "mg", "N", "lb"])
    )]
  #[sdk(
        string_set(
            source = 5u32,
            union = 0u64,
            values = &["1/Kg",
            "1/g",
            "1/mg",
            "1/N",
            "1/lb"]
        )
    )]
  #[sdk(string_set(source = 6u32, union = 0u64, values = &["deg", "rad"]))]
  #[sdk(string_set(source = 7u32, union = 0u64, values = &["1/deg", "1/rad"]))]
  #[sdk(string_set(source = 8u32, union = 0u64, values = &["%", "dev", "none"]))]
  #[sdk(string_set(source = 9u32, union = 0u64, values = &["1/%", "1/dev"]))]
  #[sdk(string_format(source = 10u32, union = 0u64, kind = "token"))]
  pub units: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "inkml:CT_Mapping/inkml:mapping"))]
  pub inkml_mapping: Vec<Mapping>,
}
/// Defines the IntermittentChannels Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "inkml:CT_IntermittentChannels/inkml:intermittentChannels")]
pub struct IntermittentChannels {
  /// _
  #[sdk(child(qname = "inkml:CT_Channel/inkml:channel"))]
  pub inkml_channel: Vec<Channel>,
}
/// Defines the ChannelProperty Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "inkml:CT_ChannelProperty/inkml:channelProperty")]
pub struct ChannelProperty {
  /// channel
  #[sdk(attr(qname = ":channel"))]
  #[sdk(
        string_set(
            source = 1u32,
            union = 0u64,
            values = &["X",
            "Y",
            "Z",
            "F",
            "TP",
            "BP",
            "S",
            "B1",
            "B2",
            "B3",
            "B4",
            "E",
            "OTx",
            "OTy",
            "OA",
            "OE",
            "OR",
            "RP",
            "RR",
            "RY",
            "C",
            "CR",
            "CG",
            "CB",
            "CC",
            "CM",
            "CY",
            "CK",
            "W",
            "T",
            "SN",
            "TW",
            "TH",
            "TC"]
        )
    )]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub channel: crate::simple_type::StringValue,
  /// name
  #[sdk(attr(qname = ":name"))]
  #[sdk(
        string_set(
            source = 1u32,
            union = 0u64,
            values = &["threshold",
            "resolution",
            "quantization",
            "noise",
            "accuracy",
            "crossCoupling",
            "skew",
            "minBandwidth",
            "peakRate",
            "distortion"]
        )
    )]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub name: crate::simple_type::StringValue,
  /// value
  #[sdk(attr(qname = ":value"))]
  pub value: crate::simple_type::DecimalValue,
  /// units
  #[sdk(attr(qname = ":units"))]
  #[sdk(
        string_set(
            source = 0u32,
            union = 0u64,
            values = &["m",
            "cm",
            "mm",
            "in",
            "pt",
            "pc",
            "em",
            "ex"]
        )
    )]
  #[sdk(
        string_set(
            source = 1u32,
            union = 0u64,
            values = &["1/m",
            "1/cm",
            "1/mm",
            "1/in",
            "1/pt",
            "1/pc",
            "1/em",
            "1/ex"]
        )
    )]
  #[sdk(string_set(source = 2u32, union = 0u64, values = &["s", "ms"]))]
  #[sdk(string_set(source = 3u32, union = 0u64, values = &["1/s", "1/ms"]))]
  #[sdk(
        string_set(source = 4u32, union = 0u64, values = &["Kg", "g", "mg", "N", "lb"])
    )]
  #[sdk(
        string_set(
            source = 5u32,
            union = 0u64,
            values = &["1/Kg",
            "1/g",
            "1/mg",
            "1/N",
            "1/lb"]
        )
    )]
  #[sdk(string_set(source = 6u32, union = 0u64, values = &["deg", "rad"]))]
  #[sdk(string_set(source = 7u32, union = 0u64, values = &["1/deg", "1/rad"]))]
  #[sdk(string_set(source = 8u32, union = 0u64, values = &["%", "dev", "none"]))]
  #[sdk(string_set(source = 9u32, union = 0u64, values = &["1/%", "1/dev"]))]
  #[sdk(string_format(source = 10u32, union = 0u64, kind = "token"))]
  pub units: Option<crate::simple_type::StringValue>,
}
/// Defines the TraceFormat Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "inkml:CT_TraceFormat/inkml:traceFormat")]
pub struct TraceFormat {
  /// id
  #[sdk(attr(qname = "xml:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "inkml:CT_Channel/inkml:channel"))]
  pub inkml_channel: Vec<Channel>,
  /// _
  #[sdk(child(qname = "inkml:CT_IntermittentChannels/inkml:intermittentChannels"))]
  pub inkml_intermittent_channels: Option<IntermittentChannels>,
}
/// Defines the SampleRate Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "inkml:CT_SampleRate/inkml:sampleRate")]
pub struct SampleRate {
  /// uniform
  #[sdk(attr(qname = ":uniform"))]
  pub uniform: Option<crate::simple_type::BooleanValue>,
  /// value
  #[sdk(attr(qname = ":value"))]
  pub value: crate::simple_type::DecimalValue,
}
/// Defines the Latency Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "inkml:CT_Latency/inkml:latency")]
pub struct Latency {
  /// value
  #[sdk(attr(qname = ":value"))]
  pub value: crate::simple_type::DecimalValue,
}
/// Defines the ActiveArea Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "inkml:CT_ActiveArea/inkml:activeArea")]
pub struct ActiveArea {
  /// size
  #[sdk(attr(qname = ":size"))]
  pub size: Option<crate::simple_type::StringValue>,
  /// height
  #[sdk(attr(qname = ":height"))]
  pub height: Option<crate::simple_type::DecimalValue>,
  /// width
  #[sdk(attr(qname = ":width"))]
  pub width: Option<crate::simple_type::DecimalValue>,
  /// units
  #[sdk(attr(qname = ":units"))]
  #[sdk(
        string_set(
            source = 0u32,
            union = 0u64,
            values = &["m",
            "cm",
            "mm",
            "in",
            "pt",
            "pc",
            "em",
            "ex"]
        )
    )]
  #[sdk(
        string_set(
            source = 1u32,
            union = 0u64,
            values = &["1/m",
            "1/cm",
            "1/mm",
            "1/in",
            "1/pt",
            "1/pc",
            "1/em",
            "1/ex"]
        )
    )]
  #[sdk(string_set(source = 2u32, union = 0u64, values = &["s", "ms"]))]
  #[sdk(string_set(source = 3u32, union = 0u64, values = &["1/s", "1/ms"]))]
  #[sdk(
        string_set(source = 4u32, union = 0u64, values = &["Kg", "g", "mg", "N", "lb"])
    )]
  #[sdk(
        string_set(
            source = 5u32,
            union = 0u64,
            values = &["1/Kg",
            "1/g",
            "1/mg",
            "1/N",
            "1/lb"]
        )
    )]
  #[sdk(string_set(source = 6u32, union = 0u64, values = &["deg", "rad"]))]
  #[sdk(string_set(source = 7u32, union = 0u64, values = &["1/deg", "1/rad"]))]
  #[sdk(string_set(source = 8u32, union = 0u64, values = &["%", "dev", "none"]))]
  #[sdk(string_set(source = 9u32, union = 0u64, values = &["1/%", "1/dev"]))]
  #[sdk(string_format(source = 10u32, union = 0u64, kind = "token"))]
  pub units: Option<crate::simple_type::StringValue>,
}
/// Defines the SourceProperty Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "inkml:CT_SrcProperty/inkml:srcProperty")]
pub struct SourceProperty {
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// value
  #[sdk(attr(qname = ":value"))]
  pub value: crate::simple_type::DecimalValue,
  /// units
  #[sdk(attr(qname = ":units"))]
  #[sdk(
        string_set(
            source = 0u32,
            union = 0u64,
            values = &["m",
            "cm",
            "mm",
            "in",
            "pt",
            "pc",
            "em",
            "ex"]
        )
    )]
  #[sdk(
        string_set(
            source = 1u32,
            union = 0u64,
            values = &["1/m",
            "1/cm",
            "1/mm",
            "1/in",
            "1/pt",
            "1/pc",
            "1/em",
            "1/ex"]
        )
    )]
  #[sdk(string_set(source = 2u32, union = 0u64, values = &["s", "ms"]))]
  #[sdk(string_set(source = 3u32, union = 0u64, values = &["1/s", "1/ms"]))]
  #[sdk(
        string_set(source = 4u32, union = 0u64, values = &["Kg", "g", "mg", "N", "lb"])
    )]
  #[sdk(
        string_set(
            source = 5u32,
            union = 0u64,
            values = &["1/Kg",
            "1/g",
            "1/mg",
            "1/N",
            "1/lb"]
        )
    )]
  #[sdk(string_set(source = 6u32, union = 0u64, values = &["deg", "rad"]))]
  #[sdk(string_set(source = 7u32, union = 0u64, values = &["1/deg", "1/rad"]))]
  #[sdk(string_set(source = 8u32, union = 0u64, values = &["%", "dev", "none"]))]
  #[sdk(string_set(source = 9u32, union = 0u64, values = &["1/%", "1/dev"]))]
  #[sdk(string_format(source = 10u32, union = 0u64, kind = "token"))]
  pub units: Option<crate::simple_type::StringValue>,
}
/// Defines the ChannelProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "inkml:CT_ChannelProperties/inkml:channelProperties")]
pub struct ChannelProperties {
  /// _
  #[sdk(child(qname = "inkml:CT_ChannelProperty/inkml:channelProperty"))]
  pub inkml_channel_property: Vec<ChannelProperty>,
}
/// Defines the Annotation Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "inkml:CT_Annotation/inkml:annotation")]
pub struct Annotation {
  /// type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<crate::simple_type::StringValue>,
  /// encoding
  #[sdk(attr(qname = ":encoding"))]
  pub encoding: Option<crate::simple_type::StringValue>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the AnnotationXml Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "inkml:CT_AnnotationXML/inkml:annotationXML")]
pub struct AnnotationXml {
  /// type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<crate::simple_type::StringValue>,
  /// encoding
  #[sdk(attr(qname = ":encoding"))]
  pub encoding: Option<crate::simple_type::StringValue>,
  /// href
  #[sdk(attr(qname = ":href"))]
  #[sdk(string_format(source = 0u32, kind = "uri"))]
  pub href: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "emma:CT_Emma/emma:emma"))]
  pub emma: Option<crate::schemas::www_w3_org_2003_04_emma::Emma>,
}
/// Defines the BrushProperty Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "inkml:CT_BrushProperty/inkml:brushProperty")]
pub struct BrushProperty {
  /// name
  #[sdk(attr(qname = ":name"))]
  #[sdk(
        string_set(
            source = 1u32,
            union = 0u64,
            values = &["width",
            "height",
            "color",
            "transparency",
            "tip",
            "rasterOp",
            "antiAliased",
            "fitToCurve",
            "ignorePressure"]
        )
    )]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub name: crate::simple_type::StringValue,
  /// value
  #[sdk(attr(qname = ":value"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "xsd:decimal"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "xsd:boolean"))]
  #[sdk(
        string_set(
            source = 3u32,
            union = 0u64,
            values = &["ellipse",
            "rectangle",
            "drop"]
        )
    )]
  #[sdk(
        string_set(
            source = 4u32,
            union = 0u64,
            values = &["black",
            "notMergePen",
            "maskNotPen",
            "notCopyPen",
            "maskPenNot",
            "not",
            "xOrPen",
            "notMaskPen",
            "maskPen",
            "notXOrPen",
            "noOperation",
            "mergeNotPen",
            "copyPen",
            "mergePenNot",
            "mergePen",
            "white"]
        )
    )]
  #[sdk(string_format(source = 5u32, union = 0u64, kind = "token"))]
  pub value: crate::simple_type::StringValue,
  /// units
  #[sdk(attr(qname = ":units"))]
  #[sdk(
        string_set(
            source = 0u32,
            union = 0u64,
            values = &["m",
            "cm",
            "mm",
            "in",
            "pt",
            "pc",
            "em",
            "ex"]
        )
    )]
  #[sdk(
        string_set(
            source = 1u32,
            union = 0u64,
            values = &["1/m",
            "1/cm",
            "1/mm",
            "1/in",
            "1/pt",
            "1/pc",
            "1/em",
            "1/ex"]
        )
    )]
  #[sdk(string_set(source = 2u32, union = 0u64, values = &["s", "ms"]))]
  #[sdk(string_set(source = 3u32, union = 0u64, values = &["1/s", "1/ms"]))]
  #[sdk(
        string_set(source = 4u32, union = 0u64, values = &["Kg", "g", "mg", "N", "lb"])
    )]
  #[sdk(
        string_set(
            source = 5u32,
            union = 0u64,
            values = &["1/Kg",
            "1/g",
            "1/mg",
            "1/N",
            "1/lb"]
        )
    )]
  #[sdk(string_set(source = 6u32, union = 0u64, values = &["deg", "rad"]))]
  #[sdk(string_set(source = 7u32, union = 0u64, values = &["1/deg", "1/rad"]))]
  #[sdk(string_set(source = 8u32, union = 0u64, values = &["%", "dev", "none"]))]
  #[sdk(string_set(source = 9u32, union = 0u64, values = &["1/%", "1/dev"]))]
  #[sdk(string_format(source = 10u32, union = 0u64, kind = "token"))]
  pub units: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "inkml:CT_Annotation/inkml:annotation"))]
  pub inkml_annotation: Vec<Annotation>,
  /// _
  #[sdk(child(qname = "inkml:CT_AnnotationXML/inkml:annotationXML"))]
  pub inkml_annotation_xml: Vec<AnnotationXml>,
}
/// Defines the Canvas Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "inkml:CT_Canvas/inkml:canvas")]
pub struct Canvas {
  /// id
  #[sdk(attr(qname = "xml:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// traceFormatRef
  #[sdk(attr(qname = ":traceFormatRef"))]
  #[sdk(string_format(source = 0u32, kind = "uri"))]
  pub trace_format_ref: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "inkml:CT_TraceFormat/inkml:traceFormat"))]
  pub trace_format: Option<std::boxed::Box<TraceFormat>>,
}
/// Defines the CanvasTransform Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "inkml:CT_CanvasTransform/inkml:canvasTransform")]
pub struct CanvasTransform {
  /// id
  #[sdk(attr(qname = "xml:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// invertible
  #[sdk(attr(qname = ":invertible"))]
  pub invertible: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "inkml:CT_Mapping/inkml:mapping"))]
  pub inkml_mapping: Vec<Mapping>,
}
/// Defines the InkSource Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "inkml:CT_InkSource/inkml:inkSource")]
pub struct InkSource {
  /// id
  #[sdk(attr(qname = "xml:id"))]
  pub id: crate::simple_type::StringValue,
  /// manufacturer
  #[sdk(attr(qname = ":manufacturer"))]
  pub manufacturer: Option<crate::simple_type::StringValue>,
  /// model
  #[sdk(attr(qname = ":model"))]
  pub model: Option<crate::simple_type::StringValue>,
  /// serialNo
  #[sdk(attr(qname = ":serialNo"))]
  pub serial_no: Option<crate::simple_type::StringValue>,
  /// specificationRef
  #[sdk(attr(qname = ":specificationRef"))]
  #[sdk(string_format(source = 0u32, kind = "uri"))]
  pub specification_ref: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(qname = ":description"))]
  pub description: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "inkml:CT_TraceFormat/inkml:traceFormat"))]
  pub trace_format: std::boxed::Box<TraceFormat>,
  /// _
  #[sdk(child(qname = "inkml:CT_SampleRate/inkml:sampleRate"))]
  pub sample_rate: Option<SampleRate>,
  /// _
  #[sdk(child(qname = "inkml:CT_Latency/inkml:latency"))]
  pub latency: Option<Latency>,
  /// _
  #[sdk(child(qname = "inkml:CT_ActiveArea/inkml:activeArea"))]
  pub active_area: Option<ActiveArea>,
  /// _
  #[sdk(child(qname = "inkml:CT_SrcProperty/inkml:srcProperty"))]
  pub inkml_src_property: Vec<SourceProperty>,
  /// _
  #[sdk(child(qname = "inkml:CT_ChannelProperties/inkml:channelProperties"))]
  pub inkml_channel_properties: Option<ChannelProperties>,
}
/// Defines the Brush Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "inkml:CT_Brush/inkml:brush")]
pub struct Brush {
  /// id
  #[sdk(attr(qname = "xml:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// brushRef
  #[sdk(attr(qname = ":brushRef"))]
  #[sdk(string_format(source = 0u32, kind = "uri"))]
  pub brush_ref: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "inkml:CT_Annotation/inkml:annotation"))]
  pub inkml_annotation: Vec<Annotation>,
  /// _
  #[sdk(child(qname = "inkml:CT_AnnotationXML/inkml:annotationXML"))]
  pub inkml_annotation_xml: Vec<AnnotationXml>,
  /// _
  #[sdk(child(qname = "inkml:CT_BrushProperty/inkml:brushProperty"))]
  pub inkml_brush_property: Vec<BrushProperty>,
}
/// Defines the Timestamp Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "inkml:CT_Timestamp/inkml:timestamp")]
pub struct Timestamp {
  /// id
  #[sdk(attr(qname = "xml:id"))]
  pub id: crate::simple_type::StringValue,
  /// time
  #[sdk(attr(qname = ":time"))]
  pub time: Option<crate::simple_type::DecimalValue>,
  /// timestampRef
  #[sdk(attr(qname = ":timestampRef"))]
  #[sdk(string_format(source = 0u32, kind = "uri"))]
  pub timestamp_ref: Option<crate::simple_type::StringValue>,
  /// timeString
  #[sdk(attr(qname = ":timeString"))]
  pub time_string: Option<crate::simple_type::DateTimeValue>,
  /// timeOffset
  #[sdk(attr(qname = ":timeOffset"))]
  pub time_offset: Option<crate::simple_type::DecimalValue>,
}
/// Defines the Trace Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "inkml:CT_Trace/inkml:trace")]
pub struct Trace {
  /// id
  #[sdk(attr(qname = "xml:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<TraceTypeValues>,
  /// continuation
  #[sdk(attr(qname = ":continuation"))]
  pub continuation: Option<TraceContinuationValues>,
  /// priorRef
  #[sdk(attr(qname = ":priorRef"))]
  #[sdk(string_format(source = 0u32, kind = "uri"))]
  pub prior_ref: Option<crate::simple_type::StringValue>,
  /// contextRef
  #[sdk(attr(qname = ":contextRef"))]
  #[sdk(string_format(source = 0u32, kind = "uri"))]
  pub context_ref: Option<crate::simple_type::StringValue>,
  /// brushRef
  #[sdk(attr(qname = ":brushRef"))]
  #[sdk(string_format(source = 0u32, kind = "uri"))]
  pub brush_ref: Option<crate::simple_type::StringValue>,
  /// duration
  #[sdk(attr(qname = ":duration"))]
  pub duration: Option<crate::simple_type::DecimalValue>,
  /// timeOffset
  #[sdk(attr(qname = ":timeOffset"))]
  pub time_offset: Option<crate::simple_type::DecimalValue>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the TraceGroup Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "inkml:CT_TraceGroup/inkml:traceGroup")]
pub struct TraceGroup {
  /// id
  #[sdk(attr(qname = "xml:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// contextRef
  #[sdk(attr(qname = ":contextRef"))]
  #[sdk(string_format(source = 0u32, kind = "uri"))]
  pub context_ref: Option<crate::simple_type::StringValue>,
  /// brushRef
  #[sdk(attr(qname = ":brushRef"))]
  #[sdk(string_format(source = 0u32, kind = "uri"))]
  pub brush_ref: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "inkml:CT_Annotation/inkml:annotation",
    qname = "inkml:CT_AnnotationXML/inkml:annotationXML",
    qname = "inkml:CT_Trace/inkml:trace",
    qname = "inkml:CT_TraceGroup/inkml:traceGroup"
  ))]
  pub xml_children: Vec<TraceGroupChoice>,
}
/// Defines the TraceView Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "inkml:CT_TraceView/inkml:traceView")]
pub struct TraceView {
  /// id
  #[sdk(attr(qname = "xml:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// contextRef
  #[sdk(attr(qname = ":contextRef"))]
  #[sdk(string_format(source = 0u32, kind = "uri"))]
  pub context_ref: Option<crate::simple_type::StringValue>,
  /// traceDataRef
  #[sdk(attr(qname = ":traceDataRef"))]
  #[sdk(string_format(source = 0u32, kind = "uri"))]
  pub trace_data_ref: Option<crate::simple_type::StringValue>,
  /// from
  #[sdk(attr(qname = ":from"))]
  pub from: Option<crate::simple_type::StringValue>,
  /// to
  #[sdk(attr(qname = ":to"))]
  pub to: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "inkml:CT_Annotation/inkml:annotation",
    qname = "inkml:CT_AnnotationXML/inkml:annotationXML",
    qname = "inkml:CT_TraceView/inkml:traceView"
  ))]
  pub xml_children: Vec<TraceViewChoice>,
}
/// Defines the Context Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "inkml:CT_Context/inkml:context")]
pub struct Context {
  /// id
  #[sdk(attr(qname = "xml:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// contextRef
  #[sdk(attr(qname = ":contextRef"))]
  #[sdk(string_format(source = 0u32, kind = "uri"))]
  pub context_ref: Option<crate::simple_type::StringValue>,
  /// canvasRef
  #[sdk(attr(qname = ":canvasRef"))]
  #[sdk(string_format(source = 0u32, kind = "uri"))]
  pub canvas_ref: Option<crate::simple_type::StringValue>,
  /// canvasTransformRef
  #[sdk(attr(qname = ":canvasTransformRef"))]
  #[sdk(string_format(source = 0u32, kind = "uri"))]
  pub canvas_transform_ref: Option<crate::simple_type::StringValue>,
  /// traceFormatRef
  #[sdk(attr(qname = ":traceFormatRef"))]
  #[sdk(string_format(source = 0u32, kind = "uri"))]
  pub trace_fromat_ref: Option<crate::simple_type::StringValue>,
  /// inkSourceRef
  #[sdk(attr(qname = ":inkSourceRef"))]
  #[sdk(string_format(source = 0u32, kind = "uri"))]
  pub ink_source_ref: Option<crate::simple_type::StringValue>,
  /// brushRef
  #[sdk(attr(qname = ":brushRef"))]
  #[sdk(string_format(source = 0u32, kind = "uri"))]
  pub brush_ref: Option<crate::simple_type::StringValue>,
  /// timestampRef
  #[sdk(attr(qname = ":timestampRef"))]
  #[sdk(string_format(source = 0u32, kind = "uri"))]
  pub timestamp_ref: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "inkml:CT_Canvas/inkml:canvas"))]
  pub canvas: Option<std::boxed::Box<Canvas>>,
  /// _
  #[sdk(child(qname = "inkml:CT_CanvasTransform/inkml:canvasTransform"))]
  pub canvas_transform: Option<CanvasTransform>,
  /// _
  #[sdk(child(qname = "inkml:CT_TraceFormat/inkml:traceFormat"))]
  pub trace_format: Option<std::boxed::Box<TraceFormat>>,
  /// _
  #[sdk(child(qname = "inkml:CT_InkSource/inkml:inkSource"))]
  pub ink_source: Option<std::boxed::Box<InkSource>>,
  /// _
  #[sdk(child(qname = "inkml:CT_Brush/inkml:brush"))]
  pub brush: Option<Brush>,
  /// _
  #[sdk(child(qname = "inkml:CT_Timestamp/inkml:timestamp"))]
  pub timestamp: Option<Timestamp>,
}
/// Defines the Definitions Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "inkml:CT_Definitions/inkml:definitions")]
pub struct Definitions {
  #[sdk(choice(
    qname = "inkml:CT_Brush/inkml:brush",
    qname = "inkml:CT_Canvas/inkml:canvas",
    qname = "inkml:CT_CanvasTransform/inkml:canvasTransform",
    qname = "inkml:CT_Context/inkml:context",
    qname = "inkml:CT_InkSource/inkml:inkSource",
    qname = "inkml:CT_Mapping/inkml:mapping",
    qname = "inkml:CT_Timestamp/inkml:timestamp",
    qname = "inkml:CT_Trace/inkml:trace",
    qname = "inkml:CT_TraceFormat/inkml:traceFormat",
    qname = "inkml:CT_TraceGroup/inkml:traceGroup",
    qname = "inkml:CT_TraceView/inkml:traceView"
  ))]
  pub xml_children: Vec<DefinitionsChoice>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum InkChoice {
  /// Defines the Annotation Class.
  #[sdk(child(qname = "inkml:CT_Annotation/inkml:annotation"))]
  InkmlAnnotation(std::boxed::Box<Annotation>),
  /// Defines the AnnotationXml Class.
  #[sdk(child(qname = "inkml:CT_AnnotationXML/inkml:annotationXML"))]
  InkmlAnnotationXml(std::boxed::Box<AnnotationXml>),
  /// Defines the Definitions Class.
  #[sdk(child(qname = "inkml:CT_Definitions/inkml:definitions"))]
  InkmlDefinitions(std::boxed::Box<Definitions>),
  /// Defines the Context Class.
  #[sdk(child(qname = "inkml:CT_Context/inkml:context"))]
  InkmlContext(std::boxed::Box<Context>),
  /// Defines the Trace Class.
  #[sdk(child(qname = "inkml:CT_Trace/inkml:trace"))]
  InkmlTrace(std::boxed::Box<Trace>),
  /// Defines the TraceGroup Class.
  #[sdk(child(qname = "inkml:CT_TraceGroup/inkml:traceGroup"))]
  InkmlTraceGroup(std::boxed::Box<TraceGroup>),
  /// Defines the TraceView Class.
  #[sdk(child(qname = "inkml:CT_TraceView/inkml:traceView"))]
  InkmlTraceView(std::boxed::Box<TraceView>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum MappingChoice {
  #[sdk(child(qname = "inkml:CT_Table/inkml:table"))]
  InkmlTable(std::boxed::Box<Table>),
  #[sdk(child(qname = "inkml:CT_Matrix/inkml:matrix"))]
  InkmlMatrix(std::boxed::Box<Matrix>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TraceGroupChoice {
  /// Defines the Annotation Class.
  #[sdk(child(qname = "inkml:CT_Annotation/inkml:annotation"))]
  InkmlAnnotation(std::boxed::Box<Annotation>),
  /// Defines the AnnotationXml Class.
  #[sdk(child(qname = "inkml:CT_AnnotationXML/inkml:annotationXML"))]
  InkmlAnnotationXml(std::boxed::Box<AnnotationXml>),
  /// Defines the Trace Class.
  #[sdk(child(qname = "inkml:CT_Trace/inkml:trace"))]
  InkmlTrace(std::boxed::Box<Trace>),
  /// Defines the TraceGroup Class.
  #[sdk(child(qname = "inkml:CT_TraceGroup/inkml:traceGroup"))]
  InkmlTraceGroup(std::boxed::Box<TraceGroup>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TraceViewChoice {
  /// Defines the Annotation Class.
  #[sdk(child(qname = "inkml:CT_Annotation/inkml:annotation"))]
  InkmlAnnotation(std::boxed::Box<Annotation>),
  /// Defines the AnnotationXml Class.
  #[sdk(child(qname = "inkml:CT_AnnotationXML/inkml:annotationXML"))]
  InkmlAnnotationXml(std::boxed::Box<AnnotationXml>),
  /// Defines the TraceView Class.
  #[sdk(child(qname = "inkml:CT_TraceView/inkml:traceView"))]
  InkmlTraceView(std::boxed::Box<TraceView>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DefinitionsChoice {
  /// Defines the Brush Class.
  #[sdk(child(qname = "inkml:CT_Brush/inkml:brush"))]
  InkmlBrush(std::boxed::Box<Brush>),
  /// Defines the Canvas Class.
  #[sdk(child(qname = "inkml:CT_Canvas/inkml:canvas"))]
  InkmlCanvas(std::boxed::Box<Canvas>),
  /// Defines the CanvasTransform Class.
  #[sdk(child(qname = "inkml:CT_CanvasTransform/inkml:canvasTransform"))]
  InkmlCanvasTransform(std::boxed::Box<CanvasTransform>),
  /// Defines the Context Class.
  #[sdk(child(qname = "inkml:CT_Context/inkml:context"))]
  InkmlContext(std::boxed::Box<Context>),
  /// Defines the InkSource Class.
  #[sdk(child(qname = "inkml:CT_InkSource/inkml:inkSource"))]
  InkmlInkSource(std::boxed::Box<InkSource>),
  /// Defines the Mapping Class.
  #[sdk(child(qname = "inkml:CT_Mapping/inkml:mapping"))]
  InkmlMapping(std::boxed::Box<Mapping>),
  /// Defines the Timestamp Class.
  #[sdk(child(qname = "inkml:CT_Timestamp/inkml:timestamp"))]
  InkmlTimestamp(std::boxed::Box<Timestamp>),
  /// Defines the Trace Class.
  #[sdk(child(qname = "inkml:CT_Trace/inkml:trace"))]
  InkmlTrace(std::boxed::Box<Trace>),
  /// Defines the TraceFormat Class.
  #[sdk(child(qname = "inkml:CT_TraceFormat/inkml:traceFormat"))]
  InkmlTraceFormat(std::boxed::Box<TraceFormat>),
  /// Defines the TraceGroup Class.
  #[sdk(child(qname = "inkml:CT_TraceGroup/inkml:traceGroup"))]
  InkmlTraceGroup(std::boxed::Box<TraceGroup>),
  /// Defines the TraceView Class.
  #[sdk(child(qname = "inkml:CT_TraceView/inkml:traceView"))]
  InkmlTraceView(std::boxed::Box<TraceView>),
}
