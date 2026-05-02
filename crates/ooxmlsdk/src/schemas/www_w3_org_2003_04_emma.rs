//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum MediumValues {
  #[sdk(rename = "acoustic")]
  #[default]
  Acoustic,
  #[sdk(rename = "tactile")]
  Tactile,
  #[sdk(rename = "visual")]
  Visual,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum AnchorPointValues {
  #[sdk(rename = "start")]
  #[default]
  Start,
  #[sdk(rename = "end")]
  End,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_DerivedFrom/emma:derived-from")]
pub struct DerivedFrom {
  /// resource
  #[sdk(attr(qname = ":resource"))]
  #[sdk(string_format(kind = "uri"))]
  pub resource: crate::simple_type::StringValue,
  /// composite
  #[sdk(attr(qname = ":composite"))]
  pub composite: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Info Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_Info/emma:info")]
pub struct Info {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the Lattice Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_Lattice/emma:lattice")]
pub struct Lattice {
  /// initial
  #[sdk(attr(qname = ":initial"))]
  #[sdk(number_sign(kind = "non_negative"))]
  pub initial: crate::simple_type::IntegerValue,
  /// final
  #[sdk(attr(qname = ":final"))]
  pub r#final: crate::simple_type::ListValue<crate::simple_type::DecimalValue>,
  /// time-ref-uri
  #[sdk(attr(qname = "emma:time-ref-uri"))]
  pub time_reference: Option<crate::simple_type::StringValue>,
  /// time-ref-anchor-point
  #[sdk(attr(qname = "emma:time-ref-anchor-point"))]
  pub time_reference_anchor_point: Option<AnchorPointValues>,
  #[sdk(choice(qname = "emma:CT_Arc/emma:arc", qname = "emma:CT_Node/emma:node"))]
  pub lattice_choice: Vec<LatticeChoice>,
}
/// Defines the Literal Class.
pub type Literal = crate::simple_type::StringValue;
/// Defines the Interpretation Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_Interpretation/emma:interpretation")]
pub struct Interpretation {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: crate::simple_type::StringValue,
  /// tokens
  #[sdk(attr(qname = "emma:tokens"))]
  pub tokens: Option<crate::simple_type::StringValue>,
  /// process
  #[sdk(attr(qname = "emma:process"))]
  pub process: Option<crate::simple_type::StringValue>,
  /// lang
  #[sdk(attr(qname = "emma:lang"))]
  pub language: Option<crate::simple_type::StringValue>,
  /// signal
  #[sdk(attr(qname = "emma:signal"))]
  pub signal: Option<crate::simple_type::StringValue>,
  /// signal-size
  #[sdk(attr(qname = "emma:signal-size"))]
  pub signal_size: Option<crate::simple_type::IntegerValue>,
  /// media-type
  #[sdk(attr(qname = "emma:media-type"))]
  pub media_type: Option<crate::simple_type::StringValue>,
  /// confidence
  #[sdk(attr(qname = "emma:confidence"))]
  #[sdk(number_range(range = 0..= 1))]
  pub confidence: Option<crate::simple_type::DecimalValue>,
  /// source
  #[sdk(attr(qname = "emma:source"))]
  pub source: Option<crate::simple_type::StringValue>,
  /// start
  #[sdk(attr(qname = "emma:start"))]
  pub start: Option<crate::simple_type::UInt64Value>,
  /// end
  #[sdk(attr(qname = "emma:end"))]
  pub end: Option<crate::simple_type::UInt64Value>,
  /// time-ref-uri
  #[sdk(attr(qname = "emma:time-ref-uri"))]
  pub time_reference: Option<crate::simple_type::StringValue>,
  /// time-ref-anchor-point
  #[sdk(attr(qname = "emma:time-ref-anchor-point"))]
  pub time_reference_anchor_point: Option<AnchorPointValues>,
  /// offset-to-start
  #[sdk(attr(qname = "emma:offset-to-start"))]
  pub offset_to_start: Option<crate::simple_type::IntegerValue>,
  /// duration
  #[sdk(attr(qname = "emma:duration"))]
  pub duration: Option<crate::simple_type::IntegerValue>,
  /// medium
  #[sdk(attr(qname = "emma:medium"))]
  pub medium: Option<crate::simple_type::ListValue<MediumValues>>,
  /// mode
  #[sdk(attr(qname = "emma:mode"))]
  pub mode: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// function
  #[sdk(attr(qname = "emma:function"))]
  pub function: Option<crate::simple_type::StringValue>,
  /// verbal
  #[sdk(attr(qname = "emma:verbal"))]
  pub verbal: Option<crate::simple_type::BooleanValue>,
  /// cost
  #[sdk(attr(qname = "emma:cost"))]
  #[sdk(number_range(range = 0..= 10000000))]
  pub cost: Option<crate::simple_type::DecimalValue>,
  /// grammar-ref
  #[sdk(attr(qname = "emma:grammar-ref"))]
  pub grammar_ref: Option<crate::simple_type::StringValue>,
  /// endpoint-info-ref
  #[sdk(attr(qname = "emma:endpoint-info-ref"))]
  pub endpoint_info_ref: Option<crate::simple_type::StringValue>,
  /// model-ref
  #[sdk(attr(qname = "emma:model-ref"))]
  pub model_ref: Option<crate::simple_type::StringValue>,
  /// dialog-turn
  #[sdk(attr(qname = "emma:dialog-turn"))]
  pub dialog_turn: Option<crate::simple_type::StringValue>,
  /// no-input
  #[sdk(attr(qname = "emma:no-input"))]
  pub no_input: Option<crate::simple_type::BooleanValue>,
  /// uninterpreted
  #[sdk(attr(qname = "emma:uninterpreted"))]
  pub uninterpreted: Option<crate::simple_type::BooleanValue>,
  #[sdk(choice(
    qname = "emma:CT_DerivedFrom/emma:derived-from",
    qname = "emma:CT_Info/emma:info",
    qname = "emma:CT_Lattice/emma:lattice",
    qname = "emma:CT_Literal/emma:literal",
    qname = "msink:CT_CtxNode/msink:context"
  ))]
  pub interpretation_choice: Vec<InterpretationChoice>,
}
/// Defines the OneOf Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_OneOf/emma:one-of")]
pub struct OneOf {
  /// disjunction-type
  #[sdk(attr(qname = ":disjunction-type"))]
  pub disjunction_type: Option<DisjunctionTypeValues>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: crate::simple_type::StringValue,
  /// tokens
  #[sdk(attr(qname = "emma:tokens"))]
  pub tokens: Option<crate::simple_type::StringValue>,
  /// process
  #[sdk(attr(qname = "emma:process"))]
  pub process: Option<crate::simple_type::StringValue>,
  /// lang
  #[sdk(attr(qname = "emma:lang"))]
  pub language: Option<crate::simple_type::StringValue>,
  /// signal
  #[sdk(attr(qname = "emma:signal"))]
  pub signal: Option<crate::simple_type::StringValue>,
  /// signal-size
  #[sdk(attr(qname = "emma:signal-size"))]
  pub signal_size: Option<crate::simple_type::IntegerValue>,
  /// media-type
  #[sdk(attr(qname = "emma:media-type"))]
  pub media_type: Option<crate::simple_type::StringValue>,
  /// confidence
  #[sdk(attr(qname = "emma:confidence"))]
  #[sdk(number_range(range = 0..= 1))]
  pub confidence: Option<crate::simple_type::DecimalValue>,
  /// source
  #[sdk(attr(qname = "emma:source"))]
  pub source: Option<crate::simple_type::StringValue>,
  /// start
  #[sdk(attr(qname = "emma:start"))]
  pub start: Option<crate::simple_type::UInt64Value>,
  /// end
  #[sdk(attr(qname = "emma:end"))]
  pub end: Option<crate::simple_type::UInt64Value>,
  /// time-ref-uri
  #[sdk(attr(qname = "emma:time-ref-uri"))]
  pub time_reference: Option<crate::simple_type::StringValue>,
  /// time-ref-anchor-point
  #[sdk(attr(qname = "emma:time-ref-anchor-point"))]
  pub time_reference_anchor_point: Option<AnchorPointValues>,
  /// offset-to-start
  #[sdk(attr(qname = "emma:offset-to-start"))]
  pub offset_to_start: Option<crate::simple_type::IntegerValue>,
  /// duration
  #[sdk(attr(qname = "emma:duration"))]
  pub duration: Option<crate::simple_type::IntegerValue>,
  /// medium
  #[sdk(attr(qname = "emma:medium"))]
  pub medium: Option<crate::simple_type::ListValue<MediumValues>>,
  /// mode
  #[sdk(attr(qname = "emma:mode"))]
  pub mode: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// function
  #[sdk(attr(qname = "emma:function"))]
  pub function: Option<crate::simple_type::StringValue>,
  /// verbal
  #[sdk(attr(qname = "emma:verbal"))]
  pub verbal: Option<crate::simple_type::BooleanValue>,
  /// cost
  #[sdk(attr(qname = "emma:cost"))]
  #[sdk(number_range(range = 0..= 10000000))]
  pub cost: Option<crate::simple_type::DecimalValue>,
  /// grammar-ref
  #[sdk(attr(qname = "emma:grammar-ref"))]
  pub grammar_ref: Option<crate::simple_type::StringValue>,
  /// endpoint-info-ref
  #[sdk(attr(qname = "emma:endpoint-info-ref"))]
  pub endpoint_info_ref: Option<crate::simple_type::StringValue>,
  /// model-ref
  #[sdk(attr(qname = "emma:model-ref"))]
  pub model_ref: Option<crate::simple_type::StringValue>,
  /// dialog-turn
  #[sdk(attr(qname = "emma:dialog-turn"))]
  pub dialog_turn: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "emma:CT_DerivedFrom/emma:derived-from",
    qname = "emma:CT_Info/emma:info",
    qname = "emma:CT_Interpretation/emma:interpretation",
    qname = "emma:CT_OneOf/emma:one-of",
    qname = "emma:CT_Group/emma:group",
    qname = "emma:CT_Sequence/emma:sequence"
  ))]
  pub one_of_choice: Vec<OneOfChoice>,
}
/// Defines the Group Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_Group/emma:group")]
pub struct Group {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: crate::simple_type::StringValue,
  /// tokens
  #[sdk(attr(qname = "emma:tokens"))]
  pub tokens: Option<crate::simple_type::StringValue>,
  /// process
  #[sdk(attr(qname = "emma:process"))]
  pub process: Option<crate::simple_type::StringValue>,
  /// lang
  #[sdk(attr(qname = "emma:lang"))]
  pub language: Option<crate::simple_type::StringValue>,
  /// signal
  #[sdk(attr(qname = "emma:signal"))]
  pub signal: Option<crate::simple_type::StringValue>,
  /// signal-size
  #[sdk(attr(qname = "emma:signal-size"))]
  pub signal_size: Option<crate::simple_type::IntegerValue>,
  /// media-type
  #[sdk(attr(qname = "emma:media-type"))]
  pub media_type: Option<crate::simple_type::StringValue>,
  /// confidence
  #[sdk(attr(qname = "emma:confidence"))]
  #[sdk(number_range(range = 0..= 1))]
  pub confidence: Option<crate::simple_type::DecimalValue>,
  /// source
  #[sdk(attr(qname = "emma:source"))]
  pub source: Option<crate::simple_type::StringValue>,
  /// start
  #[sdk(attr(qname = "emma:start"))]
  pub start: Option<crate::simple_type::UInt64Value>,
  /// end
  #[sdk(attr(qname = "emma:end"))]
  pub end: Option<crate::simple_type::UInt64Value>,
  /// time-ref-uri
  #[sdk(attr(qname = "emma:time-ref-uri"))]
  pub time_reference: Option<crate::simple_type::StringValue>,
  /// time-ref-anchor-point
  #[sdk(attr(qname = "emma:time-ref-anchor-point"))]
  pub time_reference_anchor_point: Option<AnchorPointValues>,
  /// offset-to-start
  #[sdk(attr(qname = "emma:offset-to-start"))]
  pub offset_to_start: Option<crate::simple_type::IntegerValue>,
  /// duration
  #[sdk(attr(qname = "emma:duration"))]
  pub duration: Option<crate::simple_type::IntegerValue>,
  /// medium
  #[sdk(attr(qname = "emma:medium"))]
  pub medium: Option<crate::simple_type::ListValue<MediumValues>>,
  /// mode
  #[sdk(attr(qname = "emma:mode"))]
  pub mode: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// function
  #[sdk(attr(qname = "emma:function"))]
  pub function: Option<crate::simple_type::StringValue>,
  /// verbal
  #[sdk(attr(qname = "emma:verbal"))]
  pub verbal: Option<crate::simple_type::BooleanValue>,
  /// cost
  #[sdk(attr(qname = "emma:cost"))]
  #[sdk(number_range(range = 0..= 10000000))]
  pub cost: Option<crate::simple_type::DecimalValue>,
  /// grammar-ref
  #[sdk(attr(qname = "emma:grammar-ref"))]
  pub grammar_ref: Option<crate::simple_type::StringValue>,
  /// endpoint-info-ref
  #[sdk(attr(qname = "emma:endpoint-info-ref"))]
  pub endpoint_info_ref: Option<crate::simple_type::StringValue>,
  /// model-ref
  #[sdk(attr(qname = "emma:model-ref"))]
  pub model_ref: Option<crate::simple_type::StringValue>,
  /// dialog-turn
  #[sdk(attr(qname = "emma:dialog-turn"))]
  pub dialog_turn: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "emma:CT_DerivedFrom/emma:derived-from",
    qname = "emma:CT_GroupInfo/emma:group-info",
    qname = "emma:CT_Info/emma:info",
    qname = "emma:CT_Interpretation/emma:interpretation",
    qname = "emma:CT_OneOf/emma:one-of",
    qname = "emma:CT_Group/emma:group",
    qname = "emma:CT_Sequence/emma:sequence"
  ))]
  pub group_choice: Vec<GroupChoice>,
}
/// Defines the Sequence Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_Sequence/emma:sequence")]
pub struct Sequence {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: crate::simple_type::StringValue,
  /// tokens
  #[sdk(attr(qname = "emma:tokens"))]
  pub tokens: Option<crate::simple_type::StringValue>,
  /// process
  #[sdk(attr(qname = "emma:process"))]
  pub process: Option<crate::simple_type::StringValue>,
  /// lang
  #[sdk(attr(qname = "emma:lang"))]
  pub language: Option<crate::simple_type::StringValue>,
  /// signal
  #[sdk(attr(qname = "emma:signal"))]
  pub signal: Option<crate::simple_type::StringValue>,
  /// signal-size
  #[sdk(attr(qname = "emma:signal-size"))]
  pub signal_size: Option<crate::simple_type::IntegerValue>,
  /// media-type
  #[sdk(attr(qname = "emma:media-type"))]
  pub media_type: Option<crate::simple_type::StringValue>,
  /// confidence
  #[sdk(attr(qname = "emma:confidence"))]
  #[sdk(number_range(range = 0..= 1))]
  pub confidence: Option<crate::simple_type::DecimalValue>,
  /// source
  #[sdk(attr(qname = "emma:source"))]
  pub source: Option<crate::simple_type::StringValue>,
  /// start
  #[sdk(attr(qname = "emma:start"))]
  pub start: Option<crate::simple_type::UInt64Value>,
  /// end
  #[sdk(attr(qname = "emma:end"))]
  pub end: Option<crate::simple_type::UInt64Value>,
  /// time-ref-uri
  #[sdk(attr(qname = "emma:time-ref-uri"))]
  pub time_reference: Option<crate::simple_type::StringValue>,
  /// time-ref-anchor-point
  #[sdk(attr(qname = "emma:time-ref-anchor-point"))]
  pub time_reference_anchor_point: Option<AnchorPointValues>,
  /// offset-to-start
  #[sdk(attr(qname = "emma:offset-to-start"))]
  pub offset_to_start: Option<crate::simple_type::IntegerValue>,
  /// duration
  #[sdk(attr(qname = "emma:duration"))]
  pub duration: Option<crate::simple_type::IntegerValue>,
  /// medium
  #[sdk(attr(qname = "emma:medium"))]
  pub medium: Option<crate::simple_type::ListValue<MediumValues>>,
  /// mode
  #[sdk(attr(qname = "emma:mode"))]
  pub mode: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// function
  #[sdk(attr(qname = "emma:function"))]
  pub function: Option<crate::simple_type::StringValue>,
  /// verbal
  #[sdk(attr(qname = "emma:verbal"))]
  pub verbal: Option<crate::simple_type::BooleanValue>,
  /// cost
  #[sdk(attr(qname = "emma:cost"))]
  #[sdk(number_range(range = 0..= 10000000))]
  pub cost: Option<crate::simple_type::DecimalValue>,
  /// grammar-ref
  #[sdk(attr(qname = "emma:grammar-ref"))]
  pub grammar_ref: Option<crate::simple_type::StringValue>,
  /// endpoint-info-ref
  #[sdk(attr(qname = "emma:endpoint-info-ref"))]
  pub endpoint_info_ref: Option<crate::simple_type::StringValue>,
  /// model-ref
  #[sdk(attr(qname = "emma:model-ref"))]
  pub model_ref: Option<crate::simple_type::StringValue>,
  /// dialog-turn
  #[sdk(attr(qname = "emma:dialog-turn"))]
  pub dialog_turn: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "emma:CT_DerivedFrom/emma:derived-from",
    qname = "emma:CT_Info/emma:info",
    qname = "emma:CT_Interpretation/emma:interpretation",
    qname = "emma:CT_OneOf/emma:one-of",
    qname = "emma:CT_Group/emma:group",
    qname = "emma:CT_Sequence/emma:sequence"
  ))]
  pub sequence_choice: Vec<SequenceChoice>,
}
/// Defines the GroupInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_GroupInfo/emma:group-info")]
pub struct GroupInfo {
  /// ref
  #[sdk(attr(qname = ":ref"))]
  #[sdk(string_format(kind = "uri"))]
  pub reference: Option<crate::simple_type::StringValue>,
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the Derivation Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_Derivation/emma:derivation")]
pub struct Derivation {
  #[sdk(choice(
    qname = "emma:CT_Interpretation/emma:interpretation",
    qname = "emma:CT_OneOf/emma:one-of",
    qname = "emma:CT_Sequence/emma:sequence",
    qname = "emma:CT_Group/emma:group"
  ))]
  pub derivation_choice: Vec<DerivationChoice>,
}
/// Defines the Grammar Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_Grammar/emma:grammar")]
pub struct Grammar {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: crate::simple_type::StringValue,
  /// ref
  #[sdk(attr(qname = ":ref"))]
  #[sdk(string_format(kind = "uri"))]
  pub reference: crate::simple_type::StringValue,
}
/// Defines the Model Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_Model/emma:model")]
pub struct Model {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: crate::simple_type::StringValue,
  /// ref
  #[sdk(attr(qname = ":ref"))]
  #[sdk(string_format(kind = "uri"))]
  pub reference: Option<crate::simple_type::StringValue>,
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the EndPointInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_EndPointInfo/emma:endpoint-info")]
pub struct EndPointInfo {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: crate::simple_type::StringValue,
  /// Defines the EndPoint Class.
  #[sdk(child(qname = "emma:CT_EndPoint/emma:endpoint"))]
  pub end_point: Vec<EndPoint>,
}
/// Defines the EndPoint Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_EndPoint/emma:endpoint")]
pub struct EndPoint {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: crate::simple_type::StringValue,
  /// endpoint-role
  #[sdk(attr(qname = "emma:endpoint-role"))]
  pub endpoint_role: Option<EndPointRoleValues>,
  /// endpoint-address
  #[sdk(attr(qname = "emma:endpoint-address"))]
  pub end_point_address: Option<crate::simple_type::StringValue>,
  /// message-id
  #[sdk(attr(qname = "emma:message-id"))]
  pub message_id: Option<crate::simple_type::StringValue>,
  /// port-num
  #[sdk(attr(qname = "emma:port-num"))]
  pub port_number: Option<crate::simple_type::IntegerValue>,
  /// port-type
  #[sdk(attr(qname = "emma:port-type"))]
  pub port_type: Option<crate::simple_type::StringValue>,
  /// endpoint-pair-ref
  #[sdk(attr(qname = "emma:endpoint-pair-ref"))]
  pub endpoint_pair_ref: Option<crate::simple_type::StringValue>,
  /// service-name
  #[sdk(attr(qname = "emma:service-name"))]
  pub service_name: Option<crate::simple_type::StringValue>,
  /// media-type
  #[sdk(attr(qname = "emma:media-type"))]
  pub media_type: Option<crate::simple_type::StringValue>,
  /// medium
  #[sdk(attr(qname = "emma:medium"))]
  pub medium: Option<crate::simple_type::ListValue<MediumValues>>,
  /// mode
  #[sdk(attr(qname = "emma:mode"))]
  pub mode: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the Node Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_Node/emma:node")]
pub struct Node {
  /// node-number
  #[sdk(attr(qname = ":node-number"))]
  #[sdk(number_sign(kind = "non_negative"))]
  pub node_number: crate::simple_type::IntegerValue,
  /// confidence
  #[sdk(attr(qname = "emma:confidence"))]
  #[sdk(number_range(range = 0..= 1))]
  pub confidence: Option<crate::simple_type::DecimalValue>,
  /// cost
  #[sdk(attr(qname = "emma:cost"))]
  #[sdk(number_range(range = 0..= 10000000))]
  pub cost: Option<crate::simple_type::DecimalValue>,
  /// Defines the Info Class.
  #[sdk(child(qname = "emma:CT_Info/emma:info"))]
  pub info: Vec<Info>,
}
/// Defines the Arc Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_Arc/emma:arc")]
pub struct Arc {
  /// from
  #[sdk(attr(qname = ":from"))]
  #[sdk(number_sign(kind = "non_negative"))]
  pub from: crate::simple_type::IntegerValue,
  /// to
  #[sdk(attr(qname = ":to"))]
  #[sdk(number_sign(kind = "non_negative"))]
  pub to: crate::simple_type::IntegerValue,
  /// start
  #[sdk(attr(qname = "emma:start"))]
  pub start: Option<crate::simple_type::UInt64Value>,
  /// end
  #[sdk(attr(qname = "emma:end"))]
  pub end: Option<crate::simple_type::UInt64Value>,
  /// offset-to-start
  #[sdk(attr(qname = "emma:offset-to-start"))]
  pub offset_to_start: Option<crate::simple_type::IntegerValue>,
  /// duration
  #[sdk(attr(qname = "emma:duration"))]
  pub duration: Option<crate::simple_type::IntegerValue>,
  /// confidence
  #[sdk(attr(qname = "emma:confidence"))]
  #[sdk(number_range(range = 0..= 1))]
  pub confidence: Option<crate::simple_type::DecimalValue>,
  /// cost
  #[sdk(attr(qname = "emma:cost"))]
  #[sdk(number_range(range = 0..= 10000000))]
  pub cost: Option<crate::simple_type::DecimalValue>,
  /// lang
  #[sdk(attr(qname = "emma:lang"))]
  pub language: Option<crate::simple_type::StringValue>,
  /// medium
  #[sdk(attr(qname = "emma:medium"))]
  pub medium: Option<crate::simple_type::ListValue<MediumValues>>,
  /// mode
  #[sdk(attr(qname = "emma:mode"))]
  pub mode: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// source
  #[sdk(attr(qname = "emma:source"))]
  pub source: Option<crate::simple_type::StringValue>,
  /// Defines the Info Class.
  #[sdk(child(qname = "emma:CT_Info/emma:info"))]
  pub info: Vec<Info>,
}
/// Defines the Emma Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "emma:CT_Emma/emma:emma")]
pub struct Emma {
  /// version
  #[sdk(attr(qname = ":version"))]
  pub version: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "emma:CT_Derivation/emma:derivation",
    qname = "emma:CT_Grammar/emma:grammar",
    qname = "emma:CT_Model/emma:model",
    qname = "emma:CT_EndPointInfo/emma:endpoint-info",
    qname = "emma:CT_Info/emma:info",
    qname = "emma:CT_Interpretation/emma:interpretation",
    qname = "emma:CT_OneOf/emma:one-of",
    qname = "emma:CT_Group/emma:group",
    qname = "emma:CT_Sequence/emma:sequence"
  ))]
  pub emma_choice: Vec<EmmaChoice>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LatticeChoice {
  /// Defines the Arc Class.
  #[sdk(child(qname = "emma:CT_Arc/emma:arc"))]
  EmmaArc(std::boxed::Box<Arc>),
  /// Defines the Node Class.
  #[sdk(child(qname = "emma:CT_Node/emma:node"))]
  EmmaNode(std::boxed::Box<Node>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum InterpretationChoice {
  /// Defines the DerivedFrom Class.
  #[sdk(child(qname = "emma:CT_DerivedFrom/emma:derived-from"))]
  EmmaDerivedFrom(std::boxed::Box<DerivedFrom>),
  /// Defines the Info Class.
  #[sdk(child(qname = "emma:CT_Info/emma:info"))]
  EmmaInfo(std::boxed::Box<Info>),
  /// Defines the Lattice Class.
  #[sdk(child(qname = "emma:CT_Lattice/emma:lattice"))]
  EmmaLattice(std::boxed::Box<Lattice>),
  /// Defines the Literal Class.
  #[sdk(text_child(qname = "emma:CT_Literal/emma:literal"))]
  EmmaLiteral(crate::simple_type::StringValue),
  /// Defines the ContextNode Class.
  #[sdk(child(qname = "msink:CT_CtxNode/msink:context"))]
  MsinkContext(std::boxed::Box<crate::schemas::msink::ContextNode>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum OneOfChoice {
  /// Defines the DerivedFrom Class.
  #[sdk(child(qname = "emma:CT_DerivedFrom/emma:derived-from"))]
  EmmaDerivedFrom(std::boxed::Box<DerivedFrom>),
  /// Defines the Info Class.
  #[sdk(child(qname = "emma:CT_Info/emma:info"))]
  EmmaInfo(std::boxed::Box<Info>),
  /// Defines the Interpretation Class.
  #[sdk(child(qname = "emma:CT_Interpretation/emma:interpretation"))]
  EmmaInterpretation(std::boxed::Box<Interpretation>),
  /// Defines the OneOf Class.
  #[sdk(child(qname = "emma:CT_OneOf/emma:one-of"))]
  EmmaOneOf(std::boxed::Box<OneOf>),
  /// Defines the Group Class.
  #[sdk(child(qname = "emma:CT_Group/emma:group"))]
  EmmaGroup(std::boxed::Box<Group>),
  /// Defines the Sequence Class.
  #[sdk(child(qname = "emma:CT_Sequence/emma:sequence"))]
  EmmaSequence(std::boxed::Box<Sequence>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GroupChoice {
  /// Defines the DerivedFrom Class.
  #[sdk(child(qname = "emma:CT_DerivedFrom/emma:derived-from"))]
  EmmaDerivedFrom(std::boxed::Box<DerivedFrom>),
  /// Defines the GroupInfo Class.
  #[sdk(child(qname = "emma:CT_GroupInfo/emma:group-info"))]
  EmmaGroupInfo(std::boxed::Box<GroupInfo>),
  /// Defines the Info Class.
  #[sdk(child(qname = "emma:CT_Info/emma:info"))]
  EmmaInfo(std::boxed::Box<Info>),
  /// Defines the Interpretation Class.
  #[sdk(child(qname = "emma:CT_Interpretation/emma:interpretation"))]
  EmmaInterpretation(std::boxed::Box<Interpretation>),
  /// Defines the OneOf Class.
  #[sdk(child(qname = "emma:CT_OneOf/emma:one-of"))]
  EmmaOneOf(std::boxed::Box<OneOf>),
  /// Defines the Group Class.
  #[sdk(child(qname = "emma:CT_Group/emma:group"))]
  EmmaGroup(std::boxed::Box<Group>),
  /// Defines the Sequence Class.
  #[sdk(child(qname = "emma:CT_Sequence/emma:sequence"))]
  EmmaSequence(std::boxed::Box<Sequence>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SequenceChoice {
  /// Defines the DerivedFrom Class.
  #[sdk(child(qname = "emma:CT_DerivedFrom/emma:derived-from"))]
  EmmaDerivedFrom(std::boxed::Box<DerivedFrom>),
  /// Defines the Info Class.
  #[sdk(child(qname = "emma:CT_Info/emma:info"))]
  EmmaInfo(std::boxed::Box<Info>),
  /// Defines the Interpretation Class.
  #[sdk(child(qname = "emma:CT_Interpretation/emma:interpretation"))]
  EmmaInterpretation(std::boxed::Box<Interpretation>),
  /// Defines the OneOf Class.
  #[sdk(child(qname = "emma:CT_OneOf/emma:one-of"))]
  EmmaOneOf(std::boxed::Box<OneOf>),
  /// Defines the Group Class.
  #[sdk(child(qname = "emma:CT_Group/emma:group"))]
  EmmaGroup(std::boxed::Box<Group>),
  /// Defines the Sequence Class.
  #[sdk(child(qname = "emma:CT_Sequence/emma:sequence"))]
  EmmaSequence(std::boxed::Box<Sequence>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DerivationChoice {
  /// Defines the Interpretation Class.
  #[sdk(child(qname = "emma:CT_Interpretation/emma:interpretation"))]
  EmmaInterpretation(std::boxed::Box<Interpretation>),
  /// Defines the OneOf Class.
  #[sdk(child(qname = "emma:CT_OneOf/emma:one-of"))]
  EmmaOneOf(std::boxed::Box<OneOf>),
  /// Defines the Sequence Class.
  #[sdk(child(qname = "emma:CT_Sequence/emma:sequence"))]
  EmmaSequence(std::boxed::Box<Sequence>),
  /// Defines the Group Class.
  #[sdk(child(qname = "emma:CT_Group/emma:group"))]
  EmmaGroup(std::boxed::Box<Group>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum EmmaChoice {
  /// Defines the Derivation Class.
  #[sdk(child(qname = "emma:CT_Derivation/emma:derivation"))]
  EmmaDerivation(std::boxed::Box<Derivation>),
  /// Defines the Grammar Class.
  #[sdk(child(qname = "emma:CT_Grammar/emma:grammar"))]
  EmmaGrammar(std::boxed::Box<Grammar>),
  /// Defines the Model Class.
  #[sdk(child(qname = "emma:CT_Model/emma:model"))]
  EmmaModel(std::boxed::Box<Model>),
  /// Defines the EndPointInfo Class.
  #[sdk(child(qname = "emma:CT_EndPointInfo/emma:endpoint-info"))]
  EmmaEndpointInfo(std::boxed::Box<EndPointInfo>),
  /// Defines the Info Class.
  #[sdk(child(qname = "emma:CT_Info/emma:info"))]
  EmmaInfo(std::boxed::Box<Info>),
  /// Defines the Interpretation Class.
  #[sdk(child(qname = "emma:CT_Interpretation/emma:interpretation"))]
  EmmaInterpretation(std::boxed::Box<Interpretation>),
  /// Defines the OneOf Class.
  #[sdk(child(qname = "emma:CT_OneOf/emma:one-of"))]
  EmmaOneOf(std::boxed::Box<OneOf>),
  /// Defines the Group Class.
  #[sdk(child(qname = "emma:CT_Group/emma:group"))]
  EmmaGroup(std::boxed::Box<Group>),
  /// Defines the Sequence Class.
  #[sdk(child(qname = "emma:CT_Sequence/emma:sequence"))]
  EmmaSequence(std::boxed::Box<Sequence>),
}
