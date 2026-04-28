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
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is x16:modelTimeGroupings.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x16:CT_ModelTimeGroupings/x16:modelTimeGroupings")]
pub struct ModelTimeGroupings {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x16:CT_ModelTimeGrouping/x16:modelTimeGrouping"))]
  pub x16_model_time_grouping: Vec<ModelTimeGrouping>,
}
/// Defines the ModelTimeGrouping Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is x16:modelTimeGrouping.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x16:CT_ModelTimeGrouping/x16:modelTimeGrouping")]
pub struct ModelTimeGrouping {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// tableName
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :tableName
  #[sdk(attr(qname = ":tableName"))]
  pub table_name: crate::simple_type::StringValue,
  /// columnName
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :columnName
  #[sdk(attr(qname = ":columnName"))]
  pub column_name: crate::simple_type::StringValue,
  /// columnId
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :columnId
  #[sdk(attr(qname = ":columnId"))]
  pub column_id: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "x16:CT_CalculatedTimeColumn/x16:calculatedTimeColumn"))]
  pub x16_calculated_time_column: Vec<CalculatedTimeColumn>,
}
/// Defines the CalculatedTimeColumn Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is x16:calculatedTimeColumn.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x16:CT_CalculatedTimeColumn/x16:calculatedTimeColumn")]
pub struct CalculatedTimeColumn {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// columnName
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :columnName
  #[sdk(attr(qname = ":columnName"))]
  pub column_name: crate::simple_type::StringValue,
  /// columnId
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :columnId
  #[sdk(attr(qname = ":columnId"))]
  pub column_id: crate::simple_type::StringValue,
  /// contentType
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :contentType
  #[sdk(attr(qname = ":contentType"))]
  pub content_type: ModelTimeGroupingContentType,
  /// isSelected
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :isSelected
  #[sdk(attr(qname = ":isSelected"))]
  pub is_selected: crate::simple_type::BooleanValue,
}
