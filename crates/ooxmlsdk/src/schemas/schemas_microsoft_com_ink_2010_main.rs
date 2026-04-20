//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum LinkDirectionValues {
  #[sdk(rename = "to")]
  #[default]
  To,
  #[sdk(rename = "from")]
  From,
  #[sdk(rename = "with")]
  With,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is msink:context.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "msink:CT_CtxNode/msink:context")]
pub struct ContextNode {
  /// id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
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
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rotatedBoundingBox
  #[sdk(attr(qname = ":rotatedBoundingBox"))]
  #[sdk(pattern(source = 0u32, regex = "-?[0-9]+,-?[0-9]+"))]
  pub rotated_bounding_box: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// alignmentLevel
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :alignmentLevel
  #[sdk(attr(qname = ":alignmentLevel"))]
  pub alignment_level: Option<crate::simple_type::Int32Value>,
  /// contentType
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :contentType
  #[sdk(attr(qname = ":contentType"))]
  pub content_type: Option<crate::simple_type::Int32Value>,
  /// ascender
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ascender
  #[sdk(attr(qname = ":ascender"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "msink:ST_Point"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "xsd:int"))]
  pub ascender: Option<crate::simple_type::StringValue>,
  /// descender
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :descender
  #[sdk(attr(qname = ":descender"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "msink:ST_Point"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "xsd:int"))]
  pub descender: Option<crate::simple_type::StringValue>,
  /// baseline
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :baseline
  #[sdk(attr(qname = ":baseline"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "msink:ST_Point"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "xsd:int"))]
  pub baseline: Option<crate::simple_type::StringValue>,
  /// midline
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :midline
  #[sdk(attr(qname = ":midline"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "msink:ST_Point"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "xsd:int"))]
  pub midline: Option<crate::simple_type::StringValue>,
  /// customRecognizerId
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :customRecognizerId
  #[sdk(attr(qname = ":customRecognizerId"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub custom_recognizer_id: Option<crate::simple_type::StringValue>,
  /// mathML
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :mathML
  #[sdk(attr(qname = ":mathML"))]
  pub math_ml: Option<crate::simple_type::StringValue>,
  /// mathStruct
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :mathStruct
  #[sdk(attr(qname = ":mathStruct"))]
  pub math_struct: Option<crate::simple_type::StringValue>,
  /// mathSymbol
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :mathSymbol
  #[sdk(attr(qname = ":mathSymbol"))]
  pub math_symbol: Option<crate::simple_type::StringValue>,
  /// beginModifierType
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :beginModifierType
  #[sdk(attr(qname = ":beginModifierType"))]
  pub begin_modifier_type: Option<crate::simple_type::StringValue>,
  /// endModifierType
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endModifierType
  #[sdk(attr(qname = ":endModifierType"))]
  pub end_modifier_type: Option<crate::simple_type::StringValue>,
  /// rotationAngle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rotationAngle
  #[sdk(attr(qname = ":rotationAngle"))]
  pub rotation_angle: Option<crate::simple_type::Int32Value>,
  /// hotPoints
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hotPoints
  #[sdk(attr(qname = ":hotPoints"))]
  #[sdk(pattern(source = 0u32, regex = "-?[0-9]+,-?[0-9]+"))]
  pub hot_points: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// centroid
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :centroid
  #[sdk(attr(qname = ":centroid"))]
  #[sdk(pattern(source = 0u32, regex = "-?[0-9]+,-?[0-9]+"))]
  pub centroid: Option<crate::simple_type::StringValue>,
  /// semanticType
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :semanticType
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
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :shapeName
  #[sdk(attr(qname = ":shapeName"))]
  pub shape_name: Option<crate::simple_type::StringValue>,
  /// shapeGeometry
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :shapeGeometry
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
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is msink:property.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "msink:CT_Property/msink:property")]
pub struct ContextNodeProperty {
  /// type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
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
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is msink:sourceLink.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "msink:CT_CtxLink/msink:sourceLink")]
pub struct SourceLink {
  /// direction
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :direction
  #[sdk(attr(qname = ":direction"))]
  pub direction: Option<LinkDirectionValues>,
  /// ref
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ref
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
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is msink:destinationLink.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "msink:CT_CtxLink/msink:destinationLink")]
pub struct DestinationLink {
  /// direction
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :direction
  #[sdk(attr(qname = ":direction"))]
  pub direction: Option<LinkDirectionValues>,
  /// ref
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  #[sdk(pattern(
    source = 0u32,
    union = 0u64,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "xsd:unsignedInt"))]
  pub reference: Option<crate::simple_type::StringValue>,
}
/// Defines the ContextLinkType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "msink:CT_CtxLink/")]
pub struct ContextLinkType {
  /// direction
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :direction
  #[sdk(attr(qname = ":direction"))]
  pub direction: Option<LinkDirectionValues>,
  /// ref
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  #[sdk(pattern(
    source = 0u32,
    union = 0u64,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "xsd:unsignedInt"))]
  pub reference: Option<crate::simple_type::StringValue>,
}
