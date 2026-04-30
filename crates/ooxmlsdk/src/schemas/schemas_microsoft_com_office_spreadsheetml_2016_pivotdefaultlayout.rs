//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the PivotTableDefinition16 Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2019,
  qname = "xpdl:CT_PivotTableDefinition16/xpdl:pivotTableDefinition16"
)]
pub struct PivotTableDefinition16 {
  /// EnabledSubtotalsDefault
  #[sdk(attr(office2019, qname = ":EnabledSubtotalsDefault"))]
  pub enabled_subtotals_default: Option<crate::simple_type::BooleanValue>,
  /// SubtotalsOnTopDefault
  #[sdk(attr(office2019, qname = ":SubtotalsOnTopDefault"))]
  pub subtotals_on_top_default: Option<crate::simple_type::BooleanValue>,
  /// InsertBlankRowDefault
  #[sdk(attr(office2019, qname = ":InsertBlankRowDefault"))]
  pub insert_blank_row_default: Option<crate::simple_type::BooleanValue>,
}
