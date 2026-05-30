//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the PivotTableDefinition16 Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xpdl:pivotTableDefinition16")]
pub struct PivotTableDefinition16 {
  /// EnabledSubtotalsDefault
  #[sdk(attr(qname = ":EnabledSubtotalsDefault"))]
  pub enabled_subtotals_default: Option<crate::simple_type::BooleanValue>,
  /// SubtotalsOnTopDefault
  #[sdk(attr(qname = ":SubtotalsOnTopDefault"))]
  pub subtotals_on_top_default: Option<crate::simple_type::BooleanValue>,
  /// InsertBlankRowDefault
  #[sdk(attr(qname = ":InsertBlankRowDefault"))]
  pub insert_blank_row_default: Option<crate::simple_type::BooleanValue>,
}
