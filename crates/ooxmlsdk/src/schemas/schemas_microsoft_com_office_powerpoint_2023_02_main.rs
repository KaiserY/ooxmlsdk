//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the PlaceholderTypeExtension Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p232:phTypeExt.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p232:CT_PlaceholderTypeExtension/p232:phTypeExt")]
pub struct PlaceholderTypeExtension {
  /// _
  #[sdk(child(qname = "p232:CT_PlaceholderTypeACB/p232:type"))]
  pub placeholder_type_acb: std::boxed::Box<PlaceholderTypeAcb>,
}
/// Defines the CameoEmpty Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p232:cameo.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Empty/p232:cameo")]
pub struct CameoEmpty {}
/// Defines the UnknownEmpty Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p232:unknown.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Empty/p232:unknown")]
pub struct UnknownEmpty {}
/// Defines the EmptyType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Empty/")]
pub struct EmptyType {}
/// Defines the PlaceholderTypeACB Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p232:type.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p232:CT_PlaceholderTypeACB/p232:type")]
pub struct PlaceholderTypeAcb {
  #[sdk(choice(qname = "p:CT_Empty/p232:cameo", qname = "p:CT_Empty/p232:unknown"))]
  pub xml_children: Option<PlaceholderTypeAcbChoice>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PlaceholderTypeAcbChoice {
  #[sdk(child(qname = "p:CT_Empty/p232:cameo"))]
  P232Cameo(std::boxed::Box<CameoEmpty>),
  #[sdk(child(qname = "p:CT_Empty/p232:unknown"))]
  P232Unknown(std::boxed::Box<UnknownEmpty>),
}
