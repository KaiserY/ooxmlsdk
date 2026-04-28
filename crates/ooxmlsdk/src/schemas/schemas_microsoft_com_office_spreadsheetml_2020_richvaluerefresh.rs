//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the RichValueRefreshIntervals Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrvr:refreshIntervals.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrvr:CT_RichValueRefreshIntervals/xlrvr:refreshIntervals")]
pub struct RichValueRefreshIntervals {
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
  #[sdk(child(qname = "xlrvr:CT_RichValueRefreshInterval/xlrvr:refreshInterval"))]
  pub xlrvr_refresh_interval: Vec<RichValueRefreshInterval>,
}
/// Defines the RichValueRefreshInterval Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrvr:refreshInterval.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrvr:CT_RichValueRefreshInterval/xlrvr:refreshInterval")]
pub struct RichValueRefreshInterval {
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
  /// resourceIdInt
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :resourceIdInt
  #[sdk(attr(qname = ":resourceIdInt"))]
  pub resource_id_int: Option<crate::simple_type::Int32Value>,
  /// resourceIdStr
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :resourceIdStr
  #[sdk(attr(qname = ":resourceIdStr"))]
  pub resource_id_str: Option<crate::simple_type::StringValue>,
  /// interval
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :interval
  #[sdk(attr(qname = ":interval"))]
  pub interval: crate::simple_type::Int32Value,
}
