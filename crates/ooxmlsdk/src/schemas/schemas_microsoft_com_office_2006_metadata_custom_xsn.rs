//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CustomXsn Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ntns:customXsn.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ntns:CT_CustomXsn/ntns:customXsn")]
pub struct CustomXsn {
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
  #[sdk(text_child(qname = "xsd:string/ntns:xsnLocation"))]
  pub xsn_location: crate::simple_type::StringValue,
  /// _
  #[sdk(text_child(qname = "xsd:string/ntns:cached"))]
  pub cached_view: crate::simple_type::StringValue,
  /// _
  #[sdk(text_child(qname = "xsd:string/ntns:openByDefault"))]
  pub open_by_default: crate::simple_type::StringValue,
  /// _
  #[sdk(text_child(qname = "xsd:string/ntns:xsnScope"))]
  pub scope: crate::simple_type::StringValue,
}
/// Defines the XsnLocation Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ntns:xsnLocation.
pub type XsnLocation = crate::simple_type::StringValue;
/// Defines the CachedView Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ntns:cached.
pub type CachedView = crate::simple_type::StringValue;
/// Defines the OpenByDefault Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ntns:openByDefault.
pub type OpenByDefault = crate::simple_type::StringValue;
/// Defines the Scope Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ntns:xsnScope.
pub type Scope = crate::simple_type::StringValue;
