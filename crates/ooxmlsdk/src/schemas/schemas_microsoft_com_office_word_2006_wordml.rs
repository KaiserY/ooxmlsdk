//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the TemplateCommandGroup Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:tcg.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:CT_Tcg/wne:tcg")]
pub struct TemplateCommandGroup {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// _
  #[sdk(child(qname = "wne:CT_Keymaps/wne:keymaps"))]
  pub wne_keymaps: Vec<KeyMapCustomizations>,
  /// _
  #[sdk(child(qname = "wne:CT_Keymaps/wne:keymapsBad"))]
  pub wne_keymaps_bad: Vec<MismatchedKeyMapCustomization>,
  /// _
  #[sdk(child(qname = "wne:CT_Toolbars/wne:toolbars"))]
  pub wne_toolbars: Option<Toolbars>,
  /// _
  #[sdk(child(qname = "wne:CT_Acds/wne:acds"))]
  pub wne_acds: Vec<AllocatedCommands>,
}
/// Defines the Mcds Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:mcds.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:CT_Mcds/wne:mcds")]
pub struct Mcds {
  /// _
  #[sdk(child(qname = "wne:CT_Mcd/wne:mcd"))]
  pub wne_mcd: Vec<Mcd>,
}
/// Defines the VbaSuppData Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:vbaSuppData.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:CT_VbaSuppData/wne:vbaSuppData")]
pub struct VbaSuppData {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// _
  #[sdk(child(qname = "wne:CT_DocEvents/wne:docEvents"))]
  pub doc_events: Option<DocEvents>,
  /// _
  #[sdk(child(qname = "wne:CT_Mcds/wne:mcds"))]
  pub mcds: Option<Mcds>,
}
/// Defines the MailMergeRecipients Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:recipients.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:CT_HashedRecipients/wne:recipients")]
pub struct MailMergeRecipients {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// _
  #[sdk(child(qname = "wne:CT_HashedRecipientData/wne:recipientData"))]
  pub wne_recipient_data: Vec<SingleDataSourceRecord>,
}
/// Defines the FixedCommandKeyboardCustomization Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:fci.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:CT_Fci/wne:fci")]
pub struct FixedCommandKeyboardCustomization {
  /// fciName
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: wne:fciName
  #[sdk(attr(qname = "wne:fciName"))]
  pub command_name: Option<crate::simple_type::StringValue>,
  /// fciIndex
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: wne:fciIndex
  #[sdk(attr(qname = "wne:fciIndex"))]
  #[sdk(string_length(source = 0u32, min = 2u32, max = 2u32))]
  pub command_index: Option<crate::simple_type::HexBinaryValue>,
  /// swArg
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: wne:swArg
  #[sdk(attr(qname = "wne:swArg"))]
  #[sdk(string_length(source = 0u32, min = 2u32, max = 2u32))]
  pub argument: Option<crate::simple_type::HexBinaryValue>,
}
/// Defines the MacroKeyboardCustomization Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:macro.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:CT_MacroWll/wne:macro")]
pub struct MacroKeyboardCustomization {
  /// macroName
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: wne:macroName
  #[sdk(attr(qname = "wne:macroName"))]
  pub macro_name: Option<crate::simple_type::StringValue>,
}
/// Defines the WllMacroKeyboardCustomization Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:wll.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:CT_MacroWll/wne:wll")]
pub struct WllMacroKeyboardCustomization {
  /// macroName
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: wne:macroName
  #[sdk(attr(qname = "wne:macroName"))]
  pub macro_name: Option<crate::simple_type::StringValue>,
}
/// Defines the MacroWllType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:CT_MacroWll/")]
pub struct MacroWllType {
  /// macroName
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: wne:macroName
  #[sdk(attr(qname = "wne:macroName"))]
  pub macro_name: Option<crate::simple_type::StringValue>,
}
/// Defines the AllocatedCommandKeyboardCustomization Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:acd.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:CT_AcdKeymap/wne:acd")]
pub struct AllocatedCommandKeyboardCustomization {
  /// acdName
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: wne:acdName
  #[sdk(attr(qname = "wne:acdName"))]
  pub accelerator_name: Option<crate::simple_type::StringValue>,
}
/// Defines the AllocatedCommandManifestEntry Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:acdEntry.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:CT_AcdKeymap/wne:acdEntry")]
pub struct AllocatedCommandManifestEntry {
  /// acdName
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: wne:acdName
  #[sdk(attr(qname = "wne:acdName"))]
  pub accelerator_name: Option<crate::simple_type::StringValue>,
}
/// Defines the AcceleratorKeymapType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:CT_AcdKeymap/")]
pub struct AcceleratorKeymapType {
  /// acdName
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: wne:acdName
  #[sdk(attr(qname = "wne:acdName"))]
  pub accelerator_name: Option<crate::simple_type::StringValue>,
}
/// Defines the CharacterInsertion Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:wch.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:CT_LongHexNumber/wne:wch")]
pub struct CharacterInsertion {
  /// val
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: wne:val
  #[sdk(attr(qname = "wne:val"))]
  #[sdk(string_length(source = 1u32, min = 4u32, max = 4u32))]
  pub val: crate::simple_type::HexBinaryValue,
}
/// Defines the KeyMapEntry Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:keymap.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:CT_Keymap/wne:keymap")]
pub struct KeyMapEntry {
  /// chmPrimary
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: wne:chmPrimary
  #[sdk(attr(qname = "wne:chmPrimary"))]
  #[sdk(string_length(source = 0u32, min = 2u32, max = 2u32))]
  pub character_map_primary: Option<crate::simple_type::HexBinaryValue>,
  /// chmSecondary
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: wne:chmSecondary
  #[sdk(attr(qname = "wne:chmSecondary"))]
  #[sdk(string_length(source = 0u32, min = 2u32, max = 2u32))]
  pub character_map_secondary: Option<crate::simple_type::HexBinaryValue>,
  /// kcmPrimary
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: wne:kcmPrimary
  #[sdk(attr(qname = "wne:kcmPrimary"))]
  #[sdk(string_length(source = 0u32, min = 2u32, max = 2u32))]
  pub key_code_primary: Option<crate::simple_type::HexBinaryValue>,
  /// kcmSecondary
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: wne:kcmSecondary
  #[sdk(attr(qname = "wne:kcmSecondary"))]
  #[sdk(string_length(source = 0u32, min = 2u32, max = 2u32))]
  pub key_code_secondary: Option<crate::simple_type::HexBinaryValue>,
  /// mask
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: wne:mask
  #[sdk(attr(qname = "wne:mask"))]
  pub mask: Option<crate::simple_type::OnOffValue>,
  #[sdk(choice(
    qname = "wne:CT_Fci/wne:fci",
    qname = "wne:CT_MacroWll/wne:macro",
    qname = "wne:CT_AcdKeymap/wne:acd",
    qname = "wne:CT_MacroWll/wne:wll",
    qname = "wne:CT_LongHexNumber/wne:wch"
  ))]
  pub xml_children: Option<KeyMapEntryChoice>,
}
/// Defines the AllocatedCommand Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:acd.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:CT_Acd/wne:acd")]
pub struct AllocatedCommand {
  /// argValue
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: wne:argValue
  #[sdk(attr(qname = "wne:argValue"))]
  pub argument_value: Option<crate::simple_type::StringValue>,
  /// fciBasedOn
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: wne:fciBasedOn
  #[sdk(attr(qname = "wne:fciBasedOn"))]
  pub command_based_on: Option<crate::simple_type::StringValue>,
  /// fciIndexBasedOn
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: wne:fciIndexBasedOn
  #[sdk(attr(qname = "wne:fciIndexBasedOn"))]
  #[sdk(string_length(source = 0u32, min = 2u32, max = 2u32))]
  pub command_index_based_on: Option<crate::simple_type::HexBinaryValue>,
  /// acdName
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: wne:acdName
  #[sdk(attr(qname = "wne:acdName"))]
  pub accelerator_name: Option<crate::simple_type::StringValue>,
}
/// Defines the Mcd Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:mcd.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:CT_Mcd/wne:mcd")]
pub struct Mcd {
  /// macroName
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: wne:macroName
  #[sdk(attr(qname = "wne:macroName"))]
  pub wne_macro_name: Option<crate::simple_type::StringValue>,
  /// name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: wne:name
  #[sdk(attr(qname = "wne:name"))]
  pub wne_name: Option<crate::simple_type::StringValue>,
  /// menuHelp
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: wne:menuHelp
  #[sdk(attr(qname = "wne:menuHelp"))]
  pub wne_menu_help: Option<crate::simple_type::StringValue>,
  /// bEncrypt
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: wne:bEncrypt
  #[sdk(attr(qname = "wne:bEncrypt"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1u32))]
  pub wne_b_encrypt: Option<crate::simple_type::HexBinaryValue>,
  /// cmg
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: wne:cmg
  #[sdk(attr(qname = "wne:cmg"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1u32))]
  pub wne_cmg: Option<crate::simple_type::HexBinaryValue>,
}
/// Defines the EventDocNewXsdString Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:eventDocNew.
pub type EventDocNewXsdString = crate::simple_type::StringValue;
/// Defines the EventDocOpenXsdString Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:eventDocOpen.
pub type EventDocOpenXsdString = crate::simple_type::StringValue;
/// Defines the EventDocCloseXsdString Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:eventDocClose.
pub type EventDocCloseXsdString = crate::simple_type::StringValue;
/// Defines the EventDocSyncXsdString Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:eventDocSync.
pub type EventDocSyncXsdString = crate::simple_type::StringValue;
/// Defines the EventDocXmlAfterInsertXsdString Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:eventDocXmlAfterInsert.
pub type EventDocXmlAfterInsertXsdString = crate::simple_type::StringValue;
/// Defines the EventDocXmlBeforeDeleteXsdString Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:eventDocXmlBeforeDelete.
pub type EventDocXmlBeforeDeleteXsdString = crate::simple_type::StringValue;
/// Defines the EventDocContentControlAfterInsertXsdString Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:eventDocContentControlAfterInsert.
pub type EventDocContentControlAfterInsertXsdString = crate::simple_type::StringValue;
/// Defines the EventDocContentControlBeforeDeleteXsdString Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:eventDocContentControlBeforeDelete.
pub type EventDocContentControlBeforeDeleteXsdString = crate::simple_type::StringValue;
/// Defines the EventDocContentControlOnExistXsdString Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:eventDocContentControlOnExit.
pub type EventDocContentControlOnExistXsdString = crate::simple_type::StringValue;
/// Defines the EventDocContentControlOnEnterXsdString Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:eventDocContentControlOnEnter.
pub type EventDocContentControlOnEnterXsdString = crate::simple_type::StringValue;
/// Defines the EventDocStoreUpdateXsdString Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:eventDocStoreUpdate.
pub type EventDocStoreUpdateXsdString = crate::simple_type::StringValue;
/// Defines the EventDocContentControlUpdateXsdString Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:eventDocContentControlContentUpdate.
pub type EventDocContentControlUpdateXsdString = crate::simple_type::StringValue;
/// Defines the EventDocBuildingBlockAfterInsertXsdString Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:eventDocBuildingBlockAfterInsert.
pub type EventDocBuildingBlockAfterInsertXsdString = crate::simple_type::StringValue;
/// Defines the DocEvents Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:docEvents.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:CT_DocEvents/wne:docEvents")]
pub struct DocEvents {
  /// _
  #[sdk(text_child(qname = "xsd:string/wne:eventDocNew"))]
  pub event_doc_new_xsd_string: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(qname = "xsd:string/wne:eventDocOpen"))]
  pub event_doc_open_xsd_string: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(qname = "xsd:string/wne:eventDocClose"))]
  pub event_doc_close_xsd_string: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(qname = "xsd:string/wne:eventDocSync"))]
  pub event_doc_sync_xsd_string: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(qname = "xsd:string/wne:eventDocXmlAfterInsert"))]
  pub event_doc_xml_after_insert_xsd_string: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(qname = "xsd:string/wne:eventDocXmlBeforeDelete"))]
  pub event_doc_xml_before_delete_xsd_string: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(qname = "xsd:string/wne:eventDocContentControlAfterInsert"))]
  pub event_doc_content_control_after_insert_xsd_string: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(qname = "xsd:string/wne:eventDocContentControlBeforeDelete"))]
  pub event_doc_content_control_before_delete_xsd_string: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(qname = "xsd:string/wne:eventDocContentControlOnExit"))]
  pub event_doc_content_control_on_exist_xsd_string: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(qname = "xsd:string/wne:eventDocContentControlOnEnter"))]
  pub event_doc_content_control_on_enter_xsd_string: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(qname = "xsd:string/wne:eventDocStoreUpdate"))]
  pub event_doc_store_update_xsd_string: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(qname = "xsd:string/wne:eventDocContentControlContentUpdate"))]
  pub event_doc_content_control_update_xsd_string: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(qname = "xsd:string/wne:eventDocBuildingBlockAfterInsert"))]
  pub event_doc_building_block_after_insert_xsd_string: Option<crate::simple_type::StringValue>,
}
/// Defines the AllocatedCommandManifest Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:acdManifest.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:CT_AcdManifest/wne:acdManifest")]
pub struct AllocatedCommandManifest {
  /// _
  #[sdk(child(qname = "wne:CT_AcdKeymap/wne:acdEntry"))]
  pub wne_acd_entry: Vec<AllocatedCommandManifestEntry>,
}
/// Defines the ToolbarData Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:toolbarData.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:CT_Rel/wne:toolbarData")]
pub struct ToolbarData {
  /// id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the KeyMapCustomizations Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:keymaps.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:CT_Keymaps/wne:keymaps")]
pub struct KeyMapCustomizations {
  /// _
  #[sdk(child(qname = "wne:CT_Keymap/wne:keymap"))]
  pub wne_keymap: Vec<KeyMapEntry>,
}
/// Defines the MismatchedKeyMapCustomization Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:keymapsBad.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:CT_Keymaps/wne:keymapsBad")]
pub struct MismatchedKeyMapCustomization {
  /// _
  #[sdk(child(qname = "wne:CT_Keymap/wne:keymap"))]
  pub wne_keymap: Vec<KeyMapEntry>,
}
/// Defines the KeymapsType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:CT_Keymaps/")]
pub struct KeymapsType {
  ///Defines the KeyMapEntry Class.
  #[sdk(child(qname = "wne:CT_Keymap/wne:keymap"))]
  pub key_map_entry: Vec<KeyMapEntry>,
}
/// Defines the Toolbars Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:toolbars.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:CT_Toolbars/wne:toolbars")]
pub struct Toolbars {
  /// _
  #[sdk(child(qname = "wne:CT_AcdManifest/wne:acdManifest"))]
  pub wne_acd_manifest: Vec<AllocatedCommandManifest>,
  /// _
  #[sdk(child(qname = "wne:CT_Rel/wne:toolbarData"))]
  pub wne_toolbar_data: Vec<ToolbarData>,
}
/// Defines the AllocatedCommands Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:acds.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:CT_Acds/wne:acds")]
pub struct AllocatedCommands {
  /// _
  #[sdk(child(qname = "wne:CT_Acd/wne:acd"))]
  pub wne_acd: Vec<AllocatedCommand>,
}
/// Defines the RecordIncluded Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:active.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:CT_OnOff/wne:active")]
pub struct RecordIncluded {
  /// val
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: wne:val
  #[sdk(attr(qname = "wne:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the RecordHashCode Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:hash.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:CT_DecimalNumber/wne:hash")]
pub struct RecordHashCode {
  /// val
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: wne:val
  #[sdk(attr(qname = "wne:val"))]
  pub val: crate::simple_type::IntegerValue,
}
/// Defines the SingleDataSourceRecord Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wne:recipientData.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:CT_HashedRecipientData/wne:recipientData")]
pub struct SingleDataSourceRecord {
  /// _
  #[sdk(child(qname = "wne:CT_OnOff/wne:active"))]
  pub record_included: Option<RecordIncluded>,
  /// _
  #[sdk(child(qname = "wne:CT_DecimalNumber/wne:hash"))]
  pub record_hash_code: std::boxed::Box<RecordHashCode>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum KeyMapEntryChoice {
  #[sdk(child(qname = "wne:CT_Fci/wne:fci"))]
  WneFci(std::boxed::Box<FixedCommandKeyboardCustomization>),
  #[sdk(child(qname = "wne:CT_MacroWll/wne:macro"))]
  WneMacro(std::boxed::Box<MacroKeyboardCustomization>),
  #[sdk(child(qname = "wne:CT_AcdKeymap/wne:acd"))]
  WneAcd(std::boxed::Box<AllocatedCommandKeyboardCustomization>),
  #[sdk(child(qname = "wne:CT_MacroWll/wne:wll"))]
  WneWll(std::boxed::Box<WllMacroKeyboardCustomization>),
  #[sdk(child(qname = "wne:CT_LongHexNumber/wne:wch"))]
  WneWch(std::boxed::Box<CharacterInsertion>),
}
