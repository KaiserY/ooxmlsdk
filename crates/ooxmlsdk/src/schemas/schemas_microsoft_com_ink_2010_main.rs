//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum KnownContextNodeTypeValues {
  #[sdk(rename = "root")]
  #[default]
  Root,
  #[sdk(rename = "unclassifiedInk")]
  UnclassifiedInk,
  #[sdk(rename = "writingRegion")]
  WritingRegion,
  #[sdk(rename = "analysisHint")]
  AnalysisHint,
  #[sdk(rename = "object")]
  Object,
  #[sdk(rename = "inkDrawing")]
  InkDrawing,
  #[sdk(rename = "image")]
  Image,
  #[sdk(rename = "paragraph")]
  Paragraph,
  #[sdk(rename = "line")]
  Line,
  #[sdk(rename = "inkBullet")]
  InkBullet,
  #[sdk(rename = "inkWord")]
  InkWord,
  #[sdk(rename = "textWord")]
  TextWord,
  #[sdk(rename = "customRecognizer")]
  CustomRecognizer,
  #[sdk(rename = "mathRegion")]
  MathRegion,
  #[sdk(rename = "mathEquation")]
  MathEquation,
  #[sdk(rename = "mathStruct")]
  MathStruct,
  #[sdk(rename = "mathSymbol")]
  MathSymbol,
  #[sdk(rename = "mathIdentifier")]
  MathIdentifier,
  #[sdk(rename = "mathOperator")]
  MathOperator,
  #[sdk(rename = "mathNumber")]
  MathNumber,
  #[sdk(rename = "nonInkDrawing")]
  NonInkDrawing,
  #[sdk(rename = "groupNode")]
  GroupNode,
  #[sdk(rename = "mixedDrawing")]
  MixedDrawing,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum LinkDirectionValues {
  #[sdk(rename = "to")]
  #[default]
  To,
  #[sdk(rename = "from")]
  From,
  #[sdk(rename = "with")]
  With,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum KnownSemanticTypeValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "underline")]
  Underline,
  #[sdk(rename = "strikethrough")]
  Strikethrough,
  #[sdk(rename = "highlight")]
  Highlight,
  #[sdk(rename = "scratchOut")]
  ScratchOut,
  #[sdk(rename = "verticalRange")]
  VerticalRange,
  #[sdk(rename = "callout")]
  Callout,
  #[sdk(rename = "enclosure")]
  Enclosure,
  #[sdk(rename = "comment")]
  Comment,
  #[sdk(rename = "container")]
  Container,
  #[sdk(rename = "connector")]
  Connector,
}
/// Defines the ContextNode Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "msink:CT_CtxNode/msink:context")]
pub struct ContextNode {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// type
  #[sdk(attr(qname = ":type"))]
  #[sdk(
        string_set(
            source = 1u32,
            union = 0u64,
            values = &["root",
            "unclassifiedInk",
            "writingRegion",
            "analysisHint",
            "object",
            "inkDrawing",
            "image",
            "paragraph",
            "line",
            "inkBullet",
            "inkWord",
            "textWord",
            "customRecognizer",
            "mathRegion",
            "mathEquation",
            "mathStruct",
            "mathSymbol",
            "mathIdentifier",
            "mathOperator",
            "mathNumber",
            "nonInkDrawing",
            "groupNode",
            "mixedDrawing"]
        )
    )]
  #[sdk(pattern(
    source = 2u32,
    union = 0u64,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  pub r#type: crate::simple_type::StringValue,
  /// rotatedBoundingBox
  #[sdk(attr(qname = ":rotatedBoundingBox"))]
  #[sdk(pattern(source = 0u32, regex = "-?[0-9]+,-?[0-9]+"))]
  pub rotated_bounding_box: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// alignmentLevel
  #[sdk(attr(qname = ":alignmentLevel"))]
  pub alignment_level: Option<crate::simple_type::Int32Value>,
  /// contentType
  #[sdk(attr(qname = ":contentType"))]
  pub content_type: Option<crate::simple_type::Int32Value>,
  /// ascender
  #[sdk(attr(qname = ":ascender"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "msink:ST_Point"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "xsd:int"))]
  pub ascender: Option<crate::simple_type::StringValue>,
  /// descender
  #[sdk(attr(qname = ":descender"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "msink:ST_Point"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "xsd:int"))]
  pub descender: Option<crate::simple_type::StringValue>,
  /// baseline
  #[sdk(attr(qname = ":baseline"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "msink:ST_Point"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "xsd:int"))]
  pub baseline: Option<crate::simple_type::StringValue>,
  /// midline
  #[sdk(attr(qname = ":midline"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "msink:ST_Point"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "xsd:int"))]
  pub midline: Option<crate::simple_type::StringValue>,
  /// customRecognizerId
  #[sdk(attr(qname = ":customRecognizerId"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub custom_recognizer_id: Option<crate::simple_type::StringValue>,
  /// mathML
  #[sdk(attr(qname = ":mathML"))]
  pub math_ml: Option<crate::simple_type::StringValue>,
  /// mathStruct
  #[sdk(attr(qname = ":mathStruct"))]
  pub math_struct: Option<crate::simple_type::StringValue>,
  /// mathSymbol
  #[sdk(attr(qname = ":mathSymbol"))]
  pub math_symbol: Option<crate::simple_type::StringValue>,
  /// beginModifierType
  #[sdk(attr(qname = ":beginModifierType"))]
  pub begin_modifier_type: Option<crate::simple_type::StringValue>,
  /// endModifierType
  #[sdk(attr(qname = ":endModifierType"))]
  pub end_modifier_type: Option<crate::simple_type::StringValue>,
  /// rotationAngle
  #[sdk(attr(qname = ":rotationAngle"))]
  pub rotation_angle: Option<crate::simple_type::Int32Value>,
  /// hotPoints
  #[sdk(attr(qname = ":hotPoints"))]
  #[sdk(pattern(source = 0u32, regex = "-?[0-9]+,-?[0-9]+"))]
  pub hot_points: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// centroid
  #[sdk(attr(qname = ":centroid"))]
  #[sdk(pattern(source = 0u32, regex = "-?[0-9]+,-?[0-9]+"))]
  pub centroid: Option<crate::simple_type::StringValue>,
  /// semanticType
  #[sdk(attr(qname = ":semanticType"))]
  #[sdk(
        string_set(
            source = 0u32,
            union = 0u64,
            values = &["none",
            "underline",
            "strikethrough",
            "highlight",
            "scratchOut",
            "verticalRange",
            "callout",
            "enclosure",
            "comment",
            "container",
            "connector"]
        )
    )]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "xsd:unsignedInt"))]
  pub semantic_type: Option<crate::simple_type::StringValue>,
  /// shapeName
  #[sdk(attr(qname = ":shapeName"))]
  pub shape_name: Option<crate::simple_type::StringValue>,
  /// shapeGeometry
  #[sdk(attr(qname = ":shapeGeometry"))]
  #[sdk(pattern(source = 0u32, regex = "-?[0-9]+,-?[0-9]+"))]
  pub shape_geometry: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "msink:CT_Property/msink:property"))]
  pub msink_property: Vec<ContextNodeProperty>,
  /// _
  #[sdk(child(qname = "msink:CT_CtxLink/msink:sourceLink"))]
  pub msink_source_link: Vec<SourceLink>,
  /// _
  #[sdk(child(qname = "msink:CT_CtxLink/msink:destinationLink"))]
  pub msink_destination_link: Vec<DestinationLink>,
}
/// Defines the ContextNodeProperty Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "msink:CT_Property/msink:property")]
pub struct ContextNodeProperty {
  /// type
  #[sdk(attr(qname = ":type"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub r#type: Option<crate::simple_type::StringValue>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::HexBinaryValue>,
}
/// Defines the SourceLink Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "msink:CT_CtxLink/msink:sourceLink")]
pub struct SourceLink {
  /// direction
  #[sdk(attr(qname = ":direction"))]
  pub direction: Option<LinkDirectionValues>,
  /// ref
  #[sdk(attr(qname = ":ref"))]
  #[sdk(pattern(
    source = 0u32,
    union = 0u64,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "xsd:unsignedInt"))]
  pub reference: Option<crate::simple_type::StringValue>,
}
/// Defines the DestinationLink Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "msink:CT_CtxLink/msink:destinationLink")]
pub struct DestinationLink {
  /// direction
  #[sdk(attr(qname = ":direction"))]
  pub direction: Option<LinkDirectionValues>,
  /// ref
  #[sdk(attr(qname = ":ref"))]
  #[sdk(pattern(
    source = 0u32,
    union = 0u64,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "xsd:unsignedInt"))]
  pub reference: Option<crate::simple_type::StringValue>,
}
