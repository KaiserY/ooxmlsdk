//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the PlaceholderTypeExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p232:phTypeExt")]
pub struct PlaceholderTypeExtension {
  /// Defines the PlaceholderTypeACB Class.
  #[sdk(child(qname = "p232:type"))]
  pub placeholder_type_acb: std::boxed::Box<PlaceholderTypeAcb>,
}
/// Defines the PlaceholderTypeACB Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p232:type")]
pub struct PlaceholderTypeAcb {
  #[sdk(
        choice(
            empty_child(variant = CameoEmpty, qname = "p232:cameo"),
            empty_child(variant = UnknownEmpty, qname = "p232:unknown")
        )
    )]
  pub placeholder_type_acb_choice: Option<PlaceholderTypeAcbChoice>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum PlaceholderTypeAcbChoice {
  /// Defines the CameoEmpty Class.
  CameoEmpty,
  /// Defines the UnknownEmpty Class.
  UnknownEmpty,
}
