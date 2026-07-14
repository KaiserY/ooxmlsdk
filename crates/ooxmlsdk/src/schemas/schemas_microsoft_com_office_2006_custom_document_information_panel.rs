//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CustomPropertyEditors Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cdip:customPropertyEditors")]
pub struct CustomPropertyEditors {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Defines the ShowOnOpen Class.
  #[sdk(text_child(qname = "cdip:showOnOpen"))]
  pub show_on_open: ShowOnOpen,
  /// Defines the DefaultPropertyEditorNamespace Class.
  #[sdk(text_child(qname = "cdip:defaultPropertyEditorNamespace"))]
  pub default_property_editor_namespace: DefaultPropertyEditorNamespace,
  /// Defines the CustomPropertyEditor Class.
  #[sdk(child(qname = "cdip:customPropertyEditor"))]
  pub custom_property_editor: Vec<CustomPropertyEditor>,
}
/// Defines the PropertyEditorNamespace Class.
pub type PropertyEditorNamespace = crate::simple_type::StringValue;
/// Defines the DefaultPropertyEditorNamespace Class.
pub type DefaultPropertyEditorNamespace = crate::simple_type::StringValue;
/// Defines the XsnFileLocation Class.
pub type XsnFileLocation = crate::simple_type::StringValue;
/// Defines the ShowOnOpen Class.
pub type ShowOnOpen = crate::simple_type::BooleanValue;
/// Defines the CustomPropertyEditor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cdip:customPropertyEditor")]
pub struct CustomPropertyEditor {
  /// Defines the PropertyEditorNamespace Class.
  #[sdk(text_child(qname = "cdip:XMLNamespace"))]
  pub property_editor_namespace: PropertyEditorNamespace,
  /// Defines the XsnFileLocation Class.
  #[sdk(text_child(qname = "cdip:XSNLocation"))]
  pub xsn_file_location: XsnFileLocation,
}
