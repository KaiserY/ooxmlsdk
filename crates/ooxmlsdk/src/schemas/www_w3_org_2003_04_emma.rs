//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum EndPointRoleValues {
  #[sdk(rename = "source")]
  #[default]
  Source,
  #[sdk(rename = "sink")]
  Sink,
  #[sdk(rename = "reply-to")]
  Replyto,
  #[sdk(rename = "router")]
  Router,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum MediumValues {
  #[sdk(rename = "acoustic")]
  #[default]
  Acoustic,
  #[sdk(rename = "tactile")]
  Tactile,
  #[sdk(rename = "visual")]
  Visual,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum AnchorPointValues {
  #[sdk(rename = "start")]
  #[default]
  Start,
  #[sdk(rename = "end")]
  End,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum DisjunctionTypeValues {
  #[sdk(rename = "recognition")]
  #[default]
  Recognition,
  #[sdk(rename = "understanding")]
  Understanding,
  #[sdk(rename = "multi-device")]
  Multidevice,
  #[sdk(rename = "multi-process")]
  Multiprocess,
}
/// Defines the DerivedFrom Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is emma:derived-from.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_DerivedFrom/emma:derived-from")]
pub struct DerivedFrom {
  /// resource
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :resource
  #[sdk(attr(qname = ":resource"))]
  #[sdk(string_format(source = 1u32, kind = "uri"))]
  pub resource: crate::simple_type::StringValue,
  /// composite
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :composite
  #[sdk(attr(qname = ":composite"))]
  pub composite: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Info Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is emma:info.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_Info/emma:info")]
pub struct Info {
  /// id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the Lattice Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is emma:lattice.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_Lattice/emma:lattice")]
pub struct Lattice {
  /// initial
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :initial
  #[sdk(attr(qname = ":initial"))]
  #[sdk(number_sign(source = 1u32, kind = "non_negative"))]
  pub initial: crate::simple_type::IntegerValue,
  /// final
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :final
  #[sdk(attr(qname = ":final"))]
  pub r#final: crate::simple_type::ListValue<crate::simple_type::DecimalValue>,
  /// time-ref-uri
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:time-ref-uri
  #[sdk(attr(qname = "emma:time-ref-uri"))]
  pub time_reference: Option<crate::simple_type::StringValue>,
  /// time-ref-anchor-point
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:time-ref-anchor-point
  #[sdk(attr(qname = "emma:time-ref-anchor-point"))]
  pub time_reference_anchor_point: Option<AnchorPointValues>,
  #[sdk(choice)]
  pub xml_children: Vec<LatticeChoice>,
}
/// Defines the Literal Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is emma:literal.
pub type Literal = crate::simple_type::StringValue;
/// Defines the Interpretation Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is emma:interpretation.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_Interpretation/emma:interpretation")]
pub struct Interpretation {
  /// id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  #[sdk(string_format(source = 1u32, kind = "ncname"))]
  #[sdk(string_format(source = 1u32, kind = "id"))]
  pub id: crate::simple_type::StringValue,
  /// tokens
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:tokens
  #[sdk(attr(qname = "emma:tokens"))]
  pub tokens: Option<crate::simple_type::StringValue>,
  /// process
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:process
  #[sdk(attr(qname = "emma:process"))]
  pub process: Option<crate::simple_type::StringValue>,
  /// lang
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:lang
  #[sdk(attr(qname = "emma:lang"))]
  pub language: Option<crate::simple_type::StringValue>,
  /// signal
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:signal
  #[sdk(attr(qname = "emma:signal"))]
  pub signal: Option<crate::simple_type::StringValue>,
  /// signal-size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:signal-size
  #[sdk(attr(qname = "emma:signal-size"))]
  pub signal_size: Option<crate::simple_type::IntegerValue>,
  /// media-type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:media-type
  #[sdk(attr(qname = "emma:media-type"))]
  pub media_type: Option<crate::simple_type::StringValue>,
  /// confidence
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:confidence
  #[sdk(attr(qname = "emma:confidence"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "1",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub confidence: Option<crate::simple_type::DecimalValue>,
  /// source
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:source
  #[sdk(attr(qname = "emma:source"))]
  pub source: Option<crate::simple_type::StringValue>,
  /// start
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:start
  #[sdk(attr(qname = "emma:start"))]
  pub start: Option<crate::simple_type::UInt64Value>,
  /// end
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:end
  #[sdk(attr(qname = "emma:end"))]
  pub end: Option<crate::simple_type::UInt64Value>,
  /// time-ref-uri
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:time-ref-uri
  #[sdk(attr(qname = "emma:time-ref-uri"))]
  pub time_reference: Option<crate::simple_type::StringValue>,
  /// time-ref-anchor-point
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:time-ref-anchor-point
  #[sdk(attr(qname = "emma:time-ref-anchor-point"))]
  pub time_reference_anchor_point: Option<AnchorPointValues>,
  /// offset-to-start
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:offset-to-start
  #[sdk(attr(qname = "emma:offset-to-start"))]
  pub offset_to_start: Option<crate::simple_type::IntegerValue>,
  /// duration
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:duration
  #[sdk(attr(qname = "emma:duration"))]
  pub duration: Option<crate::simple_type::IntegerValue>,
  /// medium
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:medium
  #[sdk(attr(qname = "emma:medium"))]
  pub medium: Option<crate::simple_type::ListValue<MediumValues>>,
  /// mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:mode
  #[sdk(attr(qname = "emma:mode"))]
  pub mode: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// function
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:function
  #[sdk(attr(qname = "emma:function"))]
  pub function: Option<crate::simple_type::StringValue>,
  /// verbal
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:verbal
  #[sdk(attr(qname = "emma:verbal"))]
  pub verbal: Option<crate::simple_type::BooleanValue>,
  /// cost
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:cost
  #[sdk(attr(qname = "emma:cost"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "10000000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub cost: Option<crate::simple_type::DecimalValue>,
  /// grammar-ref
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:grammar-ref
  #[sdk(attr(qname = "emma:grammar-ref"))]
  pub grammar_ref: Option<crate::simple_type::StringValue>,
  /// endpoint-info-ref
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:endpoint-info-ref
  #[sdk(attr(qname = "emma:endpoint-info-ref"))]
  pub endpoint_info_ref: Option<crate::simple_type::StringValue>,
  /// model-ref
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:model-ref
  #[sdk(attr(qname = "emma:model-ref"))]
  pub model_ref: Option<crate::simple_type::StringValue>,
  /// dialog-turn
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:dialog-turn
  #[sdk(attr(qname = "emma:dialog-turn"))]
  pub dialog_turn: Option<crate::simple_type::StringValue>,
  /// no-input
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:no-input
  #[sdk(attr(qname = "emma:no-input"))]
  pub no_input: Option<crate::simple_type::BooleanValue>,
  /// uninterpreted
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:uninterpreted
  #[sdk(attr(qname = "emma:uninterpreted"))]
  pub uninterpreted: Option<crate::simple_type::BooleanValue>,
  #[sdk(choice)]
  pub xml_children: Vec<InterpretationChoice>,
}
/// Defines the OneOf Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is emma:one-of.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_OneOf/emma:one-of")]
pub struct OneOf {
  /// disjunction-type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :disjunction-type
  #[sdk(attr(qname = ":disjunction-type"))]
  pub disjunction_type: Option<DisjunctionTypeValues>,
  /// id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  #[sdk(string_format(source = 1u32, kind = "ncname"))]
  #[sdk(string_format(source = 1u32, kind = "id"))]
  pub id: crate::simple_type::StringValue,
  /// tokens
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:tokens
  #[sdk(attr(qname = "emma:tokens"))]
  pub tokens: Option<crate::simple_type::StringValue>,
  /// process
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:process
  #[sdk(attr(qname = "emma:process"))]
  pub process: Option<crate::simple_type::StringValue>,
  /// lang
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:lang
  #[sdk(attr(qname = "emma:lang"))]
  pub language: Option<crate::simple_type::StringValue>,
  /// signal
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:signal
  #[sdk(attr(qname = "emma:signal"))]
  pub signal: Option<crate::simple_type::StringValue>,
  /// signal-size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:signal-size
  #[sdk(attr(qname = "emma:signal-size"))]
  pub signal_size: Option<crate::simple_type::IntegerValue>,
  /// media-type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:media-type
  #[sdk(attr(qname = "emma:media-type"))]
  pub media_type: Option<crate::simple_type::StringValue>,
  /// confidence
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:confidence
  #[sdk(attr(qname = "emma:confidence"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "1",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub confidence: Option<crate::simple_type::DecimalValue>,
  /// source
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:source
  #[sdk(attr(qname = "emma:source"))]
  pub source: Option<crate::simple_type::StringValue>,
  /// start
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:start
  #[sdk(attr(qname = "emma:start"))]
  pub start: Option<crate::simple_type::UInt64Value>,
  /// end
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:end
  #[sdk(attr(qname = "emma:end"))]
  pub end: Option<crate::simple_type::UInt64Value>,
  /// time-ref-uri
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:time-ref-uri
  #[sdk(attr(qname = "emma:time-ref-uri"))]
  pub time_reference: Option<crate::simple_type::StringValue>,
  /// time-ref-anchor-point
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:time-ref-anchor-point
  #[sdk(attr(qname = "emma:time-ref-anchor-point"))]
  pub time_reference_anchor_point: Option<AnchorPointValues>,
  /// offset-to-start
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:offset-to-start
  #[sdk(attr(qname = "emma:offset-to-start"))]
  pub offset_to_start: Option<crate::simple_type::IntegerValue>,
  /// duration
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:duration
  #[sdk(attr(qname = "emma:duration"))]
  pub duration: Option<crate::simple_type::IntegerValue>,
  /// medium
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:medium
  #[sdk(attr(qname = "emma:medium"))]
  pub medium: Option<crate::simple_type::ListValue<MediumValues>>,
  /// mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:mode
  #[sdk(attr(qname = "emma:mode"))]
  pub mode: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// function
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:function
  #[sdk(attr(qname = "emma:function"))]
  pub function: Option<crate::simple_type::StringValue>,
  /// verbal
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:verbal
  #[sdk(attr(qname = "emma:verbal"))]
  pub verbal: Option<crate::simple_type::BooleanValue>,
  /// cost
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:cost
  #[sdk(attr(qname = "emma:cost"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "10000000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub cost: Option<crate::simple_type::DecimalValue>,
  /// grammar-ref
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:grammar-ref
  #[sdk(attr(qname = "emma:grammar-ref"))]
  pub grammar_ref: Option<crate::simple_type::StringValue>,
  /// endpoint-info-ref
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:endpoint-info-ref
  #[sdk(attr(qname = "emma:endpoint-info-ref"))]
  pub endpoint_info_ref: Option<crate::simple_type::StringValue>,
  /// model-ref
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:model-ref
  #[sdk(attr(qname = "emma:model-ref"))]
  pub model_ref: Option<crate::simple_type::StringValue>,
  /// dialog-turn
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:dialog-turn
  #[sdk(attr(qname = "emma:dialog-turn"))]
  pub dialog_turn: Option<crate::simple_type::StringValue>,
  #[sdk(choice)]
  pub xml_children: Vec<OneOfChoice>,
}
/// Defines the Group Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is emma:group.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_Group/emma:group")]
pub struct Group {
  /// id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  #[sdk(string_format(source = 1u32, kind = "ncname"))]
  #[sdk(string_format(source = 1u32, kind = "id"))]
  pub id: crate::simple_type::StringValue,
  /// tokens
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:tokens
  #[sdk(attr(qname = "emma:tokens"))]
  pub tokens: Option<crate::simple_type::StringValue>,
  /// process
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:process
  #[sdk(attr(qname = "emma:process"))]
  pub process: Option<crate::simple_type::StringValue>,
  /// lang
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:lang
  #[sdk(attr(qname = "emma:lang"))]
  pub language: Option<crate::simple_type::StringValue>,
  /// signal
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:signal
  #[sdk(attr(qname = "emma:signal"))]
  pub signal: Option<crate::simple_type::StringValue>,
  /// signal-size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:signal-size
  #[sdk(attr(qname = "emma:signal-size"))]
  pub signal_size: Option<crate::simple_type::IntegerValue>,
  /// media-type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:media-type
  #[sdk(attr(qname = "emma:media-type"))]
  pub media_type: Option<crate::simple_type::StringValue>,
  /// confidence
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:confidence
  #[sdk(attr(qname = "emma:confidence"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "1",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub confidence: Option<crate::simple_type::DecimalValue>,
  /// source
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:source
  #[sdk(attr(qname = "emma:source"))]
  pub source: Option<crate::simple_type::StringValue>,
  /// start
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:start
  #[sdk(attr(qname = "emma:start"))]
  pub start: Option<crate::simple_type::UInt64Value>,
  /// end
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:end
  #[sdk(attr(qname = "emma:end"))]
  pub end: Option<crate::simple_type::UInt64Value>,
  /// time-ref-uri
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:time-ref-uri
  #[sdk(attr(qname = "emma:time-ref-uri"))]
  pub time_reference: Option<crate::simple_type::StringValue>,
  /// time-ref-anchor-point
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:time-ref-anchor-point
  #[sdk(attr(qname = "emma:time-ref-anchor-point"))]
  pub time_reference_anchor_point: Option<AnchorPointValues>,
  /// offset-to-start
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:offset-to-start
  #[sdk(attr(qname = "emma:offset-to-start"))]
  pub offset_to_start: Option<crate::simple_type::IntegerValue>,
  /// duration
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:duration
  #[sdk(attr(qname = "emma:duration"))]
  pub duration: Option<crate::simple_type::IntegerValue>,
  /// medium
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:medium
  #[sdk(attr(qname = "emma:medium"))]
  pub medium: Option<crate::simple_type::ListValue<MediumValues>>,
  /// mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:mode
  #[sdk(attr(qname = "emma:mode"))]
  pub mode: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// function
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:function
  #[sdk(attr(qname = "emma:function"))]
  pub function: Option<crate::simple_type::StringValue>,
  /// verbal
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:verbal
  #[sdk(attr(qname = "emma:verbal"))]
  pub verbal: Option<crate::simple_type::BooleanValue>,
  /// cost
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:cost
  #[sdk(attr(qname = "emma:cost"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "10000000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub cost: Option<crate::simple_type::DecimalValue>,
  /// grammar-ref
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:grammar-ref
  #[sdk(attr(qname = "emma:grammar-ref"))]
  pub grammar_ref: Option<crate::simple_type::StringValue>,
  /// endpoint-info-ref
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:endpoint-info-ref
  #[sdk(attr(qname = "emma:endpoint-info-ref"))]
  pub endpoint_info_ref: Option<crate::simple_type::StringValue>,
  /// model-ref
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:model-ref
  #[sdk(attr(qname = "emma:model-ref"))]
  pub model_ref: Option<crate::simple_type::StringValue>,
  /// dialog-turn
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:dialog-turn
  #[sdk(attr(qname = "emma:dialog-turn"))]
  pub dialog_turn: Option<crate::simple_type::StringValue>,
  #[sdk(choice)]
  pub xml_children: Vec<GroupChoice>,
}
/// Defines the Sequence Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is emma:sequence.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_Sequence/emma:sequence")]
pub struct Sequence {
  /// id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  #[sdk(string_format(source = 1u32, kind = "ncname"))]
  #[sdk(string_format(source = 1u32, kind = "id"))]
  pub id: crate::simple_type::StringValue,
  /// tokens
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:tokens
  #[sdk(attr(qname = "emma:tokens"))]
  pub tokens: Option<crate::simple_type::StringValue>,
  /// process
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:process
  #[sdk(attr(qname = "emma:process"))]
  pub process: Option<crate::simple_type::StringValue>,
  /// lang
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:lang
  #[sdk(attr(qname = "emma:lang"))]
  pub language: Option<crate::simple_type::StringValue>,
  /// signal
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:signal
  #[sdk(attr(qname = "emma:signal"))]
  pub signal: Option<crate::simple_type::StringValue>,
  /// signal-size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:signal-size
  #[sdk(attr(qname = "emma:signal-size"))]
  pub signal_size: Option<crate::simple_type::IntegerValue>,
  /// media-type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:media-type
  #[sdk(attr(qname = "emma:media-type"))]
  pub media_type: Option<crate::simple_type::StringValue>,
  /// confidence
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:confidence
  #[sdk(attr(qname = "emma:confidence"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "1",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub confidence: Option<crate::simple_type::DecimalValue>,
  /// source
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:source
  #[sdk(attr(qname = "emma:source"))]
  pub source: Option<crate::simple_type::StringValue>,
  /// start
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:start
  #[sdk(attr(qname = "emma:start"))]
  pub start: Option<crate::simple_type::UInt64Value>,
  /// end
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:end
  #[sdk(attr(qname = "emma:end"))]
  pub end: Option<crate::simple_type::UInt64Value>,
  /// time-ref-uri
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:time-ref-uri
  #[sdk(attr(qname = "emma:time-ref-uri"))]
  pub time_reference: Option<crate::simple_type::StringValue>,
  /// time-ref-anchor-point
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:time-ref-anchor-point
  #[sdk(attr(qname = "emma:time-ref-anchor-point"))]
  pub time_reference_anchor_point: Option<AnchorPointValues>,
  /// offset-to-start
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:offset-to-start
  #[sdk(attr(qname = "emma:offset-to-start"))]
  pub offset_to_start: Option<crate::simple_type::IntegerValue>,
  /// duration
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:duration
  #[sdk(attr(qname = "emma:duration"))]
  pub duration: Option<crate::simple_type::IntegerValue>,
  /// medium
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:medium
  #[sdk(attr(qname = "emma:medium"))]
  pub medium: Option<crate::simple_type::ListValue<MediumValues>>,
  /// mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:mode
  #[sdk(attr(qname = "emma:mode"))]
  pub mode: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// function
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:function
  #[sdk(attr(qname = "emma:function"))]
  pub function: Option<crate::simple_type::StringValue>,
  /// verbal
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:verbal
  #[sdk(attr(qname = "emma:verbal"))]
  pub verbal: Option<crate::simple_type::BooleanValue>,
  /// cost
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:cost
  #[sdk(attr(qname = "emma:cost"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "10000000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub cost: Option<crate::simple_type::DecimalValue>,
  /// grammar-ref
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:grammar-ref
  #[sdk(attr(qname = "emma:grammar-ref"))]
  pub grammar_ref: Option<crate::simple_type::StringValue>,
  /// endpoint-info-ref
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:endpoint-info-ref
  #[sdk(attr(qname = "emma:endpoint-info-ref"))]
  pub endpoint_info_ref: Option<crate::simple_type::StringValue>,
  /// model-ref
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:model-ref
  #[sdk(attr(qname = "emma:model-ref"))]
  pub model_ref: Option<crate::simple_type::StringValue>,
  /// dialog-turn
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:dialog-turn
  #[sdk(attr(qname = "emma:dialog-turn"))]
  pub dialog_turn: Option<crate::simple_type::StringValue>,
  #[sdk(choice)]
  pub xml_children: Vec<SequenceChoice>,
}
/// Defines the GroupInfo Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is emma:group-info.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_GroupInfo/emma:group-info")]
pub struct GroupInfo {
  /// ref
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  #[sdk(string_format(source = 0u32, kind = "uri"))]
  pub reference: Option<crate::simple_type::StringValue>,
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the Derivation Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is emma:derivation.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_Derivation/emma:derivation")]
pub struct Derivation {
  #[sdk(choice)]
  pub xml_children: Vec<DerivationChoice>,
}
/// Defines the Grammar Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is emma:grammar.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_Grammar/emma:grammar")]
pub struct Grammar {
  /// id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  #[sdk(string_format(source = 1u32, kind = "ncname"))]
  #[sdk(string_format(source = 1u32, kind = "id"))]
  pub id: crate::simple_type::StringValue,
  /// ref
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  #[sdk(string_format(source = 1u32, kind = "uri"))]
  pub reference: crate::simple_type::StringValue,
}
/// Defines the Model Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is emma:model.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_Model/emma:model")]
pub struct Model {
  /// id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  #[sdk(string_format(source = 1u32, kind = "ncname"))]
  #[sdk(string_format(source = 1u32, kind = "id"))]
  pub id: crate::simple_type::StringValue,
  /// ref
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  #[sdk(string_format(source = 0u32, kind = "uri"))]
  pub reference: Option<crate::simple_type::StringValue>,
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the EndPointInfo Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is emma:endpoint-info.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_EndPointInfo/emma:endpoint-info")]
pub struct EndPointInfo {
  /// id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  #[sdk(string_format(source = 1u32, kind = "ncname"))]
  #[sdk(string_format(source = 1u32, kind = "id"))]
  pub id: crate::simple_type::StringValue,
  ///Defines the EndPoint Class.
  #[sdk(child(qname = "emma:CT_EndPoint/emma:endpoint"))]
  pub end_point: Vec<EndPoint>,
}
/// Defines the EndPoint Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is emma:endpoint.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_EndPoint/emma:endpoint")]
pub struct EndPoint {
  /// id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  #[sdk(string_format(source = 1u32, kind = "ncname"))]
  #[sdk(string_format(source = 1u32, kind = "id"))]
  pub id: crate::simple_type::StringValue,
  /// endpoint-role
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:endpoint-role
  #[sdk(attr(qname = "emma:endpoint-role"))]
  pub endpoint_role: Option<EndPointRoleValues>,
  /// endpoint-address
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:endpoint-address
  #[sdk(attr(qname = "emma:endpoint-address"))]
  pub end_point_address: Option<crate::simple_type::StringValue>,
  /// message-id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:message-id
  #[sdk(attr(qname = "emma:message-id"))]
  pub message_id: Option<crate::simple_type::StringValue>,
  /// port-num
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:port-num
  #[sdk(attr(qname = "emma:port-num"))]
  pub port_number: Option<crate::simple_type::IntegerValue>,
  /// port-type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:port-type
  #[sdk(attr(qname = "emma:port-type"))]
  pub port_type: Option<crate::simple_type::StringValue>,
  /// endpoint-pair-ref
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:endpoint-pair-ref
  #[sdk(attr(qname = "emma:endpoint-pair-ref"))]
  pub endpoint_pair_ref: Option<crate::simple_type::StringValue>,
  /// service-name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:service-name
  #[sdk(attr(qname = "emma:service-name"))]
  pub service_name: Option<crate::simple_type::StringValue>,
  /// media-type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:media-type
  #[sdk(attr(qname = "emma:media-type"))]
  pub media_type: Option<crate::simple_type::StringValue>,
  /// medium
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:medium
  #[sdk(attr(qname = "emma:medium"))]
  pub medium: Option<crate::simple_type::ListValue<MediumValues>>,
  /// mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:mode
  #[sdk(attr(qname = "emma:mode"))]
  pub mode: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the Node Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is emma:node.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_Node/emma:node")]
pub struct Node {
  /// node-number
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :node-number
  #[sdk(attr(qname = ":node-number"))]
  #[sdk(number_sign(source = 1u32, kind = "non_negative"))]
  pub node_number: crate::simple_type::IntegerValue,
  /// confidence
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:confidence
  #[sdk(attr(qname = "emma:confidence"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "1",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub confidence: Option<crate::simple_type::DecimalValue>,
  /// cost
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:cost
  #[sdk(attr(qname = "emma:cost"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "10000000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub cost: Option<crate::simple_type::DecimalValue>,
  ///Defines the Info Class.
  #[sdk(child(qname = "emma:CT_Info/emma:info"))]
  pub info: Vec<Info>,
}
/// Defines the Arc Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is emma:arc.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_Arc/emma:arc")]
pub struct Arc {
  /// from
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :from
  #[sdk(attr(qname = ":from"))]
  #[sdk(number_sign(source = 1u32, kind = "non_negative"))]
  pub from: crate::simple_type::IntegerValue,
  /// to
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :to
  #[sdk(attr(qname = ":to"))]
  #[sdk(number_sign(source = 1u32, kind = "non_negative"))]
  pub to: crate::simple_type::IntegerValue,
  /// start
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:start
  #[sdk(attr(qname = "emma:start"))]
  pub start: Option<crate::simple_type::UInt64Value>,
  /// end
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:end
  #[sdk(attr(qname = "emma:end"))]
  pub end: Option<crate::simple_type::UInt64Value>,
  /// offset-to-start
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:offset-to-start
  #[sdk(attr(qname = "emma:offset-to-start"))]
  pub offset_to_start: Option<crate::simple_type::IntegerValue>,
  /// duration
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:duration
  #[sdk(attr(qname = "emma:duration"))]
  pub duration: Option<crate::simple_type::IntegerValue>,
  /// confidence
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:confidence
  #[sdk(attr(qname = "emma:confidence"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "1",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub confidence: Option<crate::simple_type::DecimalValue>,
  /// cost
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:cost
  #[sdk(attr(qname = "emma:cost"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "10000000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub cost: Option<crate::simple_type::DecimalValue>,
  /// lang
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:lang
  #[sdk(attr(qname = "emma:lang"))]
  pub language: Option<crate::simple_type::StringValue>,
  /// medium
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:medium
  #[sdk(attr(qname = "emma:medium"))]
  pub medium: Option<crate::simple_type::ListValue<MediumValues>>,
  /// mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:mode
  #[sdk(attr(qname = "emma:mode"))]
  pub mode: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// source
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: emma:source
  #[sdk(attr(qname = "emma:source"))]
  pub source: Option<crate::simple_type::StringValue>,
  ///Defines the Info Class.
  #[sdk(child(qname = "emma:CT_Info/emma:info"))]
  pub info: Vec<Info>,
}
/// Defines the Emma Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is emma:emma.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_Emma/emma:emma")]
pub struct Emma {
  /// version
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :version
  #[sdk(attr(qname = ":version"))]
  pub version: crate::simple_type::StringValue,
  #[sdk(choice)]
  pub xml_children: Vec<EmmaChoice>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum LatticeChoice {
  #[sdk(child(qname = "emma:CT_Arc/emma:arc"))]
  EmmaArc(std::boxed::Box<Arc>),
  #[sdk(child(qname = "emma:CT_Node/emma:node"))]
  EmmaNode(std::boxed::Box<Node>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum InterpretationChoice {
  #[sdk(child(qname = "emma:CT_DerivedFrom/emma:derived-from"))]
  EmmaDerivedFrom(std::boxed::Box<DerivedFrom>),
  #[sdk(child(qname = "emma:CT_Info/emma:info"))]
  EmmaInfo(std::boxed::Box<Info>),
  #[sdk(child(qname = "emma:CT_Lattice/emma:lattice"))]
  EmmaLattice(std::boxed::Box<Lattice>),
  #[sdk(text_child(qname = "emma:CT_Literal/emma:literal"))]
  EmmaLiteral(crate::simple_type::StringValue),
  #[sdk(child(qname = "msink:CT_CtxNode/msink:context"))]
  MsinkContext(std::boxed::Box<crate::schemas::schemas_microsoft_com_ink_2010_main::ContextNode>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum OneOfChoice {
  #[sdk(child(qname = "emma:CT_DerivedFrom/emma:derived-from"))]
  EmmaDerivedFrom(std::boxed::Box<DerivedFrom>),
  #[sdk(child(qname = "emma:CT_Info/emma:info"))]
  EmmaInfo(std::boxed::Box<Info>),
  #[sdk(child(qname = "emma:CT_Interpretation/emma:interpretation"))]
  EmmaInterpretation(std::boxed::Box<Interpretation>),
  #[sdk(child(qname = "emma:CT_OneOf/emma:one-of"))]
  EmmaOneOf(std::boxed::Box<OneOf>),
  #[sdk(child(qname = "emma:CT_Group/emma:group"))]
  EmmaGroup(std::boxed::Box<Group>),
  #[sdk(child(qname = "emma:CT_Sequence/emma:sequence"))]
  EmmaSequence(std::boxed::Box<Sequence>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum GroupChoice {
  #[sdk(child(qname = "emma:CT_DerivedFrom/emma:derived-from"))]
  EmmaDerivedFrom(std::boxed::Box<DerivedFrom>),
  #[sdk(child(qname = "emma:CT_GroupInfo/emma:group-info"))]
  EmmaGroupInfo(std::boxed::Box<GroupInfo>),
  #[sdk(child(qname = "emma:CT_Info/emma:info"))]
  EmmaInfo(std::boxed::Box<Info>),
  #[sdk(child(qname = "emma:CT_Interpretation/emma:interpretation"))]
  EmmaInterpretation(std::boxed::Box<Interpretation>),
  #[sdk(child(qname = "emma:CT_OneOf/emma:one-of"))]
  EmmaOneOf(std::boxed::Box<OneOf>),
  #[sdk(child(qname = "emma:CT_Group/emma:group"))]
  EmmaGroup(std::boxed::Box<Group>),
  #[sdk(child(qname = "emma:CT_Sequence/emma:sequence"))]
  EmmaSequence(std::boxed::Box<Sequence>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum SequenceChoice {
  #[sdk(child(qname = "emma:CT_DerivedFrom/emma:derived-from"))]
  EmmaDerivedFrom(std::boxed::Box<DerivedFrom>),
  #[sdk(child(qname = "emma:CT_Info/emma:info"))]
  EmmaInfo(std::boxed::Box<Info>),
  #[sdk(child(qname = "emma:CT_Interpretation/emma:interpretation"))]
  EmmaInterpretation(std::boxed::Box<Interpretation>),
  #[sdk(child(qname = "emma:CT_OneOf/emma:one-of"))]
  EmmaOneOf(std::boxed::Box<OneOf>),
  #[sdk(child(qname = "emma:CT_Group/emma:group"))]
  EmmaGroup(std::boxed::Box<Group>),
  #[sdk(child(qname = "emma:CT_Sequence/emma:sequence"))]
  EmmaSequence(std::boxed::Box<Sequence>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum DerivationChoice {
  #[sdk(child(qname = "emma:CT_Interpretation/emma:interpretation"))]
  EmmaInterpretation(std::boxed::Box<Interpretation>),
  #[sdk(child(qname = "emma:CT_OneOf/emma:one-of"))]
  EmmaOneOf(std::boxed::Box<OneOf>),
  #[sdk(child(qname = "emma:CT_Sequence/emma:sequence"))]
  EmmaSequence(std::boxed::Box<Sequence>),
  #[sdk(child(qname = "emma:CT_Group/emma:group"))]
  EmmaGroup(std::boxed::Box<Group>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum EmmaChoice {
  #[sdk(child(qname = "emma:CT_Derivation/emma:derivation"))]
  EmmaDerivation(std::boxed::Box<Derivation>),
  #[sdk(child(qname = "emma:CT_Grammar/emma:grammar"))]
  EmmaGrammar(std::boxed::Box<Grammar>),
  #[sdk(child(qname = "emma:CT_Model/emma:model"))]
  EmmaModel(std::boxed::Box<Model>),
  #[sdk(child(qname = "emma:CT_EndPointInfo/emma:endpoint-info"))]
  EmmaEndpointInfo(std::boxed::Box<EndPointInfo>),
  #[sdk(child(qname = "emma:CT_Info/emma:info"))]
  EmmaInfo(std::boxed::Box<Info>),
  #[sdk(child(qname = "emma:CT_Interpretation/emma:interpretation"))]
  EmmaInterpretation(std::boxed::Box<Interpretation>),
  #[sdk(child(qname = "emma:CT_OneOf/emma:one-of"))]
  EmmaOneOf(std::boxed::Box<OneOf>),
  #[sdk(child(qname = "emma:CT_Group/emma:group"))]
  EmmaGroup(std::boxed::Box<Group>),
  #[sdk(child(qname = "emma:CT_Sequence/emma:sequence"))]
  EmmaSequence(std::boxed::Box<Sequence>),
}
