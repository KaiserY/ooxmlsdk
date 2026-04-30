//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the PlaceholderTypeExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  microsoft365,
  qname = "p232:CT_PlaceholderTypeExtension/p232:phTypeExt"
)]
pub struct PlaceholderTypeExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// _
  #[sdk(child(microsoft365, qname = "p232:CT_PlaceholderTypeACB/p232:type"))]
  pub placeholder_type_acb: std::boxed::Box<PlaceholderTypeAcb>,
}
/// Defines the PlaceholderTypeACB Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p232:CT_PlaceholderTypeACB/p232:type")]
pub struct PlaceholderTypeAcb {
  #[sdk(choice(qname = "p:CT_Empty/p232:cameo", qname = "p:CT_Empty/p232:unknown"))]
  pub xml_children: Option<PlaceholderTypeAcbChoice>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PlaceholderTypeAcbChoice {
  /// Defines the CameoEmpty Class.
  #[sdk(empty_child(microsoft365, qname = "p:CT_Empty/p232:cameo"))]
  P232Cameo,
  /// Defines the UnknownEmpty Class.
  #[sdk(empty_child(microsoft365, qname = "p:CT_Empty/p232:unknown"))]
  P232Unknown,
}
