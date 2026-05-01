//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ModelTimeGroupingContentType {
  #[sdk(rename = "years")]
  #[default]
  Years,
  #[sdk(rename = "quarters")]
  Quarters,
  #[sdk(rename = "monthsindex")]
  Monthsindex,
  #[sdk(rename = "months")]
  Months,
  #[sdk(rename = "daysindex")]
  Daysindex,
  #[sdk(rename = "days")]
  Days,
  #[sdk(rename = "hours")]
  Hours,
  #[sdk(rename = "minutes")]
  Minutes,
  #[sdk(rename = "seconds")]
  Seconds,
}
/// Defines the ModelTimeGroupings Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "x16:CT_ModelTimeGroupings/x16:modelTimeGroupings")]
pub struct ModelTimeGroupings {
  /// Defines the ModelTimeGrouping Class.
  #[sdk(child(office2016, qname = "x16:CT_ModelTimeGrouping/x16:modelTimeGrouping"))]
  pub x16_model_time_grouping: Vec<ModelTimeGrouping>,
}
/// Defines the ModelTimeGrouping Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "x16:CT_ModelTimeGrouping/x16:modelTimeGrouping")]
pub struct ModelTimeGrouping {
  /// tableName
  #[sdk(attr(office2016, qname = ":tableName"))]
  pub table_name: crate::simple_type::StringValue,
  /// columnName
  #[sdk(attr(office2016, qname = ":columnName"))]
  pub column_name: crate::simple_type::StringValue,
  /// columnId
  #[sdk(attr(office2016, qname = ":columnId"))]
  pub column_id: crate::simple_type::StringValue,
  /// Defines the CalculatedTimeColumn Class.
  #[sdk(child(
    office2016,
    qname = "x16:CT_CalculatedTimeColumn/x16:calculatedTimeColumn"
  ))]
  pub x16_calculated_time_column: Vec<CalculatedTimeColumn>,
}
/// Defines the CalculatedTimeColumn Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "x16:CT_CalculatedTimeColumn/x16:calculatedTimeColumn"
)]
pub struct CalculatedTimeColumn {
  /// columnName
  #[sdk(attr(office2016, qname = ":columnName"))]
  pub column_name: crate::simple_type::StringValue,
  /// columnId
  #[sdk(attr(office2016, qname = ":columnId"))]
  pub column_id: crate::simple_type::StringValue,
  /// contentType
  #[sdk(attr(office2016, qname = ":contentType"))]
  pub content_type: ModelTimeGroupingContentType,
  /// isSelected
  #[sdk(attr(office2016, qname = ":isSelected"))]
  pub is_selected: crate::simple_type::BooleanValue,
}
