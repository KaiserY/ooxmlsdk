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
#[sdk(
  microsoft365,
  qname = "xlpcalc:CT_AggregationInfo/xlpcalc:aggregationInfo"
)]
pub struct AggregationInfo {
  /// aggregationType
  #[sdk(attr(microsoft365, qname = ":aggregationType"))]
  pub aggregation_type: AggregationType,
  /// sourceField
  #[sdk(attr(microsoft365, qname = ":sourceField"))]
  pub source_field: crate::simple_type::UInt32Value,
}
/// Defines the FeatureSupport Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  microsoft365,
  qname = "xlpcalc:CT_FeatureSupport/xlpcalc:featureSupportInfo"
)]
pub struct FeatureSupport {
  /// featureName
  #[sdk(attr(microsoft365, qname = ":featureName"))]
  pub feature_name: crate::simple_type::StringValue,
}
/// Defines the PivotFieldSubtotals Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  microsoft365,
  qname = "xlpcalc:CT_PivotFieldSubtotals/xlpcalc:pivotFieldSubtotals"
)]
pub struct PivotFieldSubtotals {
  /// Defines the SubtotalPivotItemSubtotal Class.
  #[sdk(child(microsoft365, qname = "xlpcalc:CT_PivotItemSubtotal/xlpcalc:subtotal"))]
  pub xlpcalc_subtotal: Vec<SubtotalPivotItemSubtotal>,
}
/// Defines the PivotAreaReferenceSubtotals Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  microsoft365,
  qname = "xlpcalc:CT_PivotAreaReferenceSubtotals/xlpcalc:pivotAreaReferenceSubtotals"
)]
pub struct PivotAreaReferenceSubtotals {
  /// Defines the PivotSubtotalType Class.
  #[sdk(child(microsoft365, qname = "xlpcalc:CT_PivotSubtotalType/xlpcalc:subtotal"))]
  pub xlpcalc_subtotal: Vec<PivotSubtotalType>,
}
/// Defines the PivotTableSubtotalLineItems Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  microsoft365,
  qname = "xlpcalc:CT_PivotTableSubtotalLineItems/xlpcalc:pivotFieldSubtotalLineItems"
)]
pub struct PivotTableSubtotalLineItems {
  /// Defines the SubtotalLineItemPivotItemSubtotal Class.
  #[sdk(child(
    microsoft365,
    qname = "xlpcalc:CT_PivotItemSubtotal/xlpcalc:subtotalLineItem"
  ))]
  pub xlpcalc_subtotal_line_item: Vec<SubtotalLineItemPivotItemSubtotal>,
}
/// Defines the SubtotalPivotItemSubtotal Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xlpcalc:CT_PivotItemSubtotal/xlpcalc:subtotal")]
pub struct SubtotalPivotItemSubtotal {
  /// subtotalType
  #[sdk(attr(microsoft365, qname = ":subtotalType"))]
  pub subtotal_type: AggregationType,
  /// itemLocation
  #[sdk(attr(microsoft365, qname = ":itemLocation"))]
  pub item_location: crate::simple_type::UInt32Value,
}
/// Defines the SubtotalLineItemPivotItemSubtotal Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  microsoft365,
  qname = "xlpcalc:CT_PivotItemSubtotal/xlpcalc:subtotalLineItem"
)]
pub struct SubtotalLineItemPivotItemSubtotal {
  /// subtotalType
  #[sdk(attr(microsoft365, qname = ":subtotalType"))]
  pub subtotal_type: AggregationType,
  /// itemLocation
  #[sdk(attr(microsoft365, qname = ":itemLocation"))]
  pub item_location: crate::simple_type::UInt32Value,
}
/// Defines the PivotSubtotalType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xlpcalc:CT_PivotSubtotalType/xlpcalc:subtotal")]
pub struct PivotSubtotalType {
  /// subtotalType
  #[sdk(attr(microsoft365, qname = ":subtotalType"))]
  pub subtotal_type: AggregationType,
}
