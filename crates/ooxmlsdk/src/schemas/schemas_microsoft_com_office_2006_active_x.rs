//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PersistenceValues {
  #[sdk(rename = "persistPropertyBag")]
  #[default]
  PersistPropertyBag,
  #[sdk(rename = "persistStream")]
  PersistStream,
  #[sdk(rename = "persistStreamInit")]
  PersistStreamInit,
  #[sdk(rename = "persistStorage")]
  PersistStorage,
}
/// Defines the ActiveXControlData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ax:CT_Ocx/ax:ocx")]
pub struct ActiveXControlData {
  /// classid
  #[sdk(attr(qname = "ax:classid"))]
  pub active_x_control_class_id: crate::simple_type::StringValue,
  /// license
  #[sdk(attr(qname = "ax:license"))]
  pub license: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// persistence
  #[sdk(attr(qname = "ax:persistence"))]
  pub persistence: PersistenceValues,
  /// _
  #[sdk(child(qname = "ax:CT_OcxPr/ax:ocxPr"))]
  pub ax_ocx_pr: Vec<ActiveXObjectProperty>,
}
/// Defines the ActiveXObjectProperty Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ax:CT_OcxPr/ax:ocxPr")]
pub struct ActiveXObjectProperty {
  /// name
  #[sdk(attr(qname = "ax:name"))]
  pub name: crate::simple_type::StringValue,
  /// value
  #[sdk(attr(qname = "ax:value"))]
  pub value: Option<crate::simple_type::StringValue>,
  #[sdk(choice(qname = "ax:CT_Font/ax:font", qname = "ax:CT_Picture/ax:picture"))]
  pub xml_children: Option<ActiveXObjectPropertyChoice>,
}
/// Defines the SharedComFont Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ax:CT_Font/ax:font")]
pub struct SharedComFont {
  /// persistence
  #[sdk(attr(qname = "ax:persistence"))]
  pub persistence: Option<PersistenceValues>,
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "ax:CT_OcxPr/ax:ocxPr"))]
  pub ax_ocx_pr: Vec<ActiveXObjectProperty>,
}
/// Defines the SharedComPicture Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ax:CT_Picture/ax:picture")]
pub struct SharedComPicture {
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ActiveXObjectPropertyChoice {
  /// Defines the SharedComFont Class.
  #[sdk(child(qname = "ax:CT_Font/ax:font"))]
  AxFont(std::boxed::Box<SharedComFont>),
  /// Defines the SharedComPicture Class.
  #[sdk(child(qname = "ax:CT_Picture/ax:picture"))]
  AxPicture(std::boxed::Box<SharedComPicture>),
}
