//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the PivotTableDefinition16 Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xpdl:pivotTableDefinition16.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xpdl:CT_PivotTableDefinition16/xpdl:pivotTableDefinition16")]
pub struct PivotTableDefinition16 {
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
  /// EnabledSubtotalsDefault
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :EnabledSubtotalsDefault
  #[sdk(attr(qname = ":EnabledSubtotalsDefault"))]
  pub enabled_subtotals_default: Option<crate::simple_type::BooleanValue>,
  /// SubtotalsOnTopDefault
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :SubtotalsOnTopDefault
  #[sdk(attr(qname = ":SubtotalsOnTopDefault"))]
  pub subtotals_on_top_default: Option<crate::simple_type::BooleanValue>,
  /// InsertBlankRowDefault
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :InsertBlankRowDefault
  #[sdk(attr(qname = ":InsertBlankRowDefault"))]
  pub insert_blank_row_default: Option<crate::simple_type::BooleanValue>,
}
