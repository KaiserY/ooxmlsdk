//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum AggregationType {
  #[sdk(rename = "distinctCount")]
  #[default]
  DistinctCount,
  #[sdk(rename = "median")]
  Median,
  #[sdk(rename = "distinctDuplicates")]
  DistinctDuplicates,
  #[sdk(rename = "countValuesDuplicated")]
  CountValuesDuplicated,
  #[sdk(rename = "countRepeatValues")]
  CountRepeatValues,
}
/// Defines the AggregationInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlpcalc:aggregationInfo")]
pub struct AggregationInfo {
  /// aggregationType
  #[sdk(attr(qname = ":aggregationType"))]
  pub aggregation_type: AggregationType,
  /// sourceField
  #[sdk(attr(qname = ":sourceField"))]
  pub source_field: crate::simple_type::UInt32Value,
}
/// Defines the FeatureSupport Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlpcalc:featureSupportInfo")]
pub struct FeatureSupport {
  /// featureName
  #[sdk(attr(qname = ":featureName"))]
  pub feature_name: crate::simple_type::StringValue,
}
/// Defines the PivotFieldSubtotals Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlpcalc:pivotFieldSubtotals")]
pub struct PivotFieldSubtotals {
  /// Defines the SubtotalPivotItemSubtotal Class.
  #[sdk(child(qname = "xlpcalc:subtotal"))]
  pub subtotal_pivot_item_subtotal: Vec<SubtotalPivotItemSubtotal>,
}
/// Defines the PivotAreaReferenceSubtotals Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlpcalc:pivotAreaReferenceSubtotals")]
pub struct PivotAreaReferenceSubtotals {
  /// Defines the PivotSubtotalType Class.
  #[sdk(child(qname = "xlpcalc:subtotal"))]
  pub pivot_subtotal_type: Vec<PivotSubtotalType>,
}
/// Defines the PivotTableSubtotalLineItems Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlpcalc:pivotFieldSubtotalLineItems")]
pub struct PivotTableSubtotalLineItems {
  /// Defines the SubtotalLineItemPivotItemSubtotal Class.
  #[sdk(child(qname = "xlpcalc:subtotalLineItem"))]
  pub subtotal_line_item_pivot_item_subtotal: Vec<SubtotalLineItemPivotItemSubtotal>,
}
/// Defines the SubtotalPivotItemSubtotal Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlpcalc:subtotal")]
pub struct SubtotalPivotItemSubtotal {
  /// subtotalType
  #[sdk(attr(qname = ":subtotalType"))]
  pub subtotal_type: AggregationType,
  /// itemLocation
  #[sdk(attr(qname = ":itemLocation"))]
  pub item_location: crate::simple_type::UInt32Value,
}
/// Defines the SubtotalLineItemPivotItemSubtotal Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlpcalc:subtotalLineItem")]
pub struct SubtotalLineItemPivotItemSubtotal {
  /// subtotalType
  #[sdk(attr(qname = ":subtotalType"))]
  pub subtotal_type: AggregationType,
  /// itemLocation
  #[sdk(attr(qname = ":itemLocation"))]
  pub item_location: crate::simple_type::UInt32Value,
}
/// Defines the PivotSubtotalType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlpcalc:subtotal")]
pub struct PivotSubtotalType {
  /// subtotalType
  #[sdk(attr(qname = ":subtotalType"))]
  pub subtotal_type: AggregationType,
}
