//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CustomPropertyEditors Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is cdip:customPropertyEditors.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cdip:CT_CustomPropertyEditors/cdip:customPropertyEditors")]
pub struct CustomPropertyEditors {
  /// _
  #[sdk(text_child(qname = "xsd:boolean/cdip:showOnOpen"))]
  pub show_on_open: crate::simple_type::BooleanValue,
  /// _
  #[sdk(text_child(qname = "xsd:anyURI/cdip:defaultPropertyEditorNamespace"))]
  pub default_property_editor_namespace: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "cdip:CT_CustomPropertyEditor/cdip:customPropertyEditor"))]
  pub cdip_custom_property_editor: Vec<CustomPropertyEditor>,
}
/// Defines the PropertyEditorNamespace Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is cdip:XMLNamespace.
pub type PropertyEditorNamespace = crate::simple_type::StringValue;
/// Defines the DefaultPropertyEditorNamespace Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is cdip:defaultPropertyEditorNamespace.
pub type DefaultPropertyEditorNamespace = crate::simple_type::StringValue;
/// Defines the XsnFileLocation Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is cdip:XSNLocation.
pub type XsnFileLocation = crate::simple_type::StringValue;
/// Defines the ShowOnOpen Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is cdip:showOnOpen.
pub type ShowOnOpen = crate::simple_type::BooleanValue;
/// Defines the CustomPropertyEditor Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is cdip:customPropertyEditor.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cdip:CT_CustomPropertyEditor/cdip:customPropertyEditor")]
pub struct CustomPropertyEditor {
  /// _
  #[sdk(text_child(qname = "xsd:anyURI/cdip:XMLNamespace"))]
  pub property_editor_namespace: crate::simple_type::StringValue,
  /// _
  #[sdk(text_child(qname = "xsd:string/cdip:XSNLocation"))]
  pub xsn_file_location: crate::simple_type::StringValue,
}
