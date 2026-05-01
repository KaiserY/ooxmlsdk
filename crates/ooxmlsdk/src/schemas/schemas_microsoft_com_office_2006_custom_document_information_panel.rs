//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CustomPropertyEditors Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cdip:CT_CustomPropertyEditors/cdip:customPropertyEditors")]
pub struct CustomPropertyEditors {
  /// Defines the ShowOnOpen Class.
  #[sdk(text_child(qname = "xsd:boolean/cdip:showOnOpen"))]
  pub show_on_open: crate::simple_type::BooleanValue,
  /// Defines the DefaultPropertyEditorNamespace Class.
  #[sdk(text_child(qname = "xsd:anyURI/cdip:defaultPropertyEditorNamespace"))]
  pub default_property_editor_namespace: crate::simple_type::StringValue,
  /// Defines the CustomPropertyEditor Class.
  #[sdk(child(qname = "cdip:CT_CustomPropertyEditor/cdip:customPropertyEditor"))]
  pub cdip_custom_property_editor: Vec<CustomPropertyEditor>,
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
#[sdk(qname = "cdip:CT_CustomPropertyEditor/cdip:customPropertyEditor")]
pub struct CustomPropertyEditor {
  /// Defines the PropertyEditorNamespace Class.
  #[sdk(text_child(qname = "xsd:anyURI/cdip:XMLNamespace"))]
  pub property_editor_namespace: crate::simple_type::StringValue,
  /// Defines the XsnFileLocation Class.
  #[sdk(text_child(qname = "xsd:string/cdip:XSNLocation"))]
  pub xsn_file_location: crate::simple_type::StringValue,
}
