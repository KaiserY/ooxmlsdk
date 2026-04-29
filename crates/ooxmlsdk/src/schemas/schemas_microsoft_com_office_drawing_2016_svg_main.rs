//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the SVGBlip Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is asvg:svgBlip.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "asvg:CT_SVGBlip/asvg:svgBlip")]
pub struct SvgBlip {
  /// Embedded Picture Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:embed
  #[sdk(attr(qname = "r:embed"))]
  pub embed: Option<crate::simple_type::StringValue>,
  /// Linked Picture Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:link
  #[sdk(attr(qname = "r:link"))]
  pub link: Option<crate::simple_type::StringValue>,
}
