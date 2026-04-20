//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ax:ocx.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ax:CT_Ocx/ax:ocx")]
pub struct ActiveXControlData {
  /// classid
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: ax:classid
  #[sdk(attr(qname = "ax:classid"))]
  pub active_x_control_class_id: crate::simple_type::StringValue,
  /// license
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: ax:license
  #[sdk(attr(qname = "ax:license"))]
  pub license: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// persistence
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: ax:persistence
  #[sdk(attr(qname = "ax:persistence"))]
  pub persistence: PersistenceValues,
  /// _
  #[sdk(child(qname = "ax:CT_OcxPr/ax:ocxPr"))]
  pub ax_ocx_pr: Vec<ActiveXObjectProperty>,
}
/// Defines the ActiveXObjectProperty Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ax:ocxPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ax:CT_OcxPr/ax:ocxPr")]
pub struct ActiveXObjectProperty {
  /// name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: ax:name
  #[sdk(attr(qname = "ax:name"))]
  pub name: crate::simple_type::StringValue,
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: ax:value
  #[sdk(attr(qname = "ax:value"))]
  pub value: Option<crate::simple_type::StringValue>,
  #[sdk(choice)]
  pub xml_children: Option<ActiveXObjectPropertyChoice>,
}
/// Defines the SharedComFont Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ax:font.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ax:CT_Font/ax:font")]
pub struct SharedComFont {
  /// persistence
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: ax:persistence
  #[sdk(attr(qname = "ax:persistence"))]
  pub persistence: Option<PersistenceValues>,
  /// id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "ax:CT_OcxPr/ax:ocxPr"))]
  pub ax_ocx_pr: Vec<ActiveXObjectProperty>,
}
/// Defines the SharedComPicture Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ax:picture.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ax:CT_Picture/ax:picture")]
pub struct SharedComPicture {
  /// id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum ActiveXObjectPropertyChoice {
  #[sdk(child(qname = "ax:CT_Font/ax:font"))]
  AxFont(std::boxed::Box<SharedComFont>),
  #[sdk(child(qname = "ax:CT_Picture/ax:picture"))]
  AxPicture(std::boxed::Box<SharedComPicture>),
}
