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
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlpcalc:aggregationInfo.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlpcalc:CT_AggregationInfo/xlpcalc:aggregationInfo")]
pub struct AggregationInfo {
  /// aggregationType
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :aggregationType
  #[sdk(attr(qname = ":aggregationType"))]
  pub aggregation_type: AggregationType,
  /// sourceField
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :sourceField
  #[sdk(attr(qname = ":sourceField"))]
  pub source_field: crate::simple_type::UInt32Value,
}
/// Defines the FeatureSupport Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlpcalc:featureSupportInfo.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlpcalc:CT_FeatureSupport/xlpcalc:featureSupportInfo")]
pub struct FeatureSupport {
  /// featureName
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :featureName
  #[sdk(attr(qname = ":featureName"))]
  pub feature_name: crate::simple_type::StringValue,
}
/// Defines the PivotFieldSubtotals Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlpcalc:pivotFieldSubtotals.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlpcalc:CT_PivotFieldSubtotals/xlpcalc:pivotFieldSubtotals")]
pub struct PivotFieldSubtotals {
  /// _
  #[sdk(child(qname = "xlpcalc:CT_PivotItemSubtotal/xlpcalc:subtotal"))]
  pub xlpcalc_subtotal: Vec<SubtotalPivotItemSubtotal>,
}
/// Defines the PivotAreaReferenceSubtotals Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlpcalc:pivotAreaReferenceSubtotals.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlpcalc:CT_PivotAreaReferenceSubtotals/xlpcalc:pivotAreaReferenceSubtotals")]
pub struct PivotAreaReferenceSubtotals {
  /// _
  #[sdk(child(qname = "xlpcalc:CT_PivotSubtotalType/xlpcalc:subtotal"))]
  pub xlpcalc_subtotal: Vec<PivotSubtotalType>,
}
/// Defines the PivotTableSubtotalLineItems Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlpcalc:pivotFieldSubtotalLineItems.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlpcalc:CT_PivotTableSubtotalLineItems/xlpcalc:pivotFieldSubtotalLineItems")]
pub struct PivotTableSubtotalLineItems {
  /// _
  #[sdk(child(qname = "xlpcalc:CT_PivotItemSubtotal/xlpcalc:subtotalLineItem"))]
  pub xlpcalc_subtotal_line_item: Vec<SubtotalLineItemPivotItemSubtotal>,
}
/// Defines the SubtotalPivotItemSubtotal Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlpcalc:subtotal.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlpcalc:CT_PivotItemSubtotal/xlpcalc:subtotal")]
pub struct SubtotalPivotItemSubtotal {
  /// subtotalType
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :subtotalType
  #[sdk(attr(qname = ":subtotalType"))]
  pub subtotal_type: AggregationType,
  /// itemLocation
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :itemLocation
  #[sdk(attr(qname = ":itemLocation"))]
  pub item_location: crate::simple_type::UInt32Value,
}
/// Defines the SubtotalLineItemPivotItemSubtotal Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlpcalc:subtotalLineItem.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlpcalc:CT_PivotItemSubtotal/xlpcalc:subtotalLineItem")]
pub struct SubtotalLineItemPivotItemSubtotal {
  /// subtotalType
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :subtotalType
  #[sdk(attr(qname = ":subtotalType"))]
  pub subtotal_type: AggregationType,
  /// itemLocation
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :itemLocation
  #[sdk(attr(qname = ":itemLocation"))]
  pub item_location: crate::simple_type::UInt32Value,
}
/// Defines the OpenXmlPivotItemSubtotalElement Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlpcalc:CT_PivotItemSubtotal/")]
pub struct OpenXmlPivotItemSubtotalElement {
  /// subtotalType
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :subtotalType
  #[sdk(attr(qname = ":subtotalType"))]
  pub subtotal_type: AggregationType,
  /// itemLocation
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :itemLocation
  #[sdk(attr(qname = ":itemLocation"))]
  pub item_location: crate::simple_type::UInt32Value,
}
/// Defines the PivotSubtotalType Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlpcalc:subtotal.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlpcalc:CT_PivotSubtotalType/xlpcalc:subtotal")]
pub struct PivotSubtotalType {
  /// subtotalType
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :subtotalType
  #[sdk(attr(qname = ":subtotalType"))]
  pub subtotal_type: AggregationType,
}
