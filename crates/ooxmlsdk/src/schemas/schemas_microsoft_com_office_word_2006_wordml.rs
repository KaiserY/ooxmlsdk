//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the TemplateCommandGroup Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "wne:tcg")]
pub struct TemplateCommandGroup {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Defines the KeyMapCustomizations Class.
  #[sdk(child(qname = "wne:keymaps"))]
  pub key_map_customizations: Vec<KeyMapCustomizations>,
  /// Defines the MismatchedKeyMapCustomization Class.
  #[sdk(child(qname = "wne:keymapsBad"))]
  pub mismatched_key_map_customization: Vec<MismatchedKeyMapCustomization>,
  /// Defines the Toolbars Class.
  #[sdk(child(qname = "wne:toolbars"))]
  pub toolbars: Option<Toolbars>,
  /// Defines the AllocatedCommands Class.
  #[sdk(child(qname = "wne:acds"))]
  pub allocated_commands: Vec<AllocatedCommands>,
}
/// Defines the Mcds Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:mcds")]
pub struct Mcds {
  /// Defines the Mcd Class.
  #[sdk(child(qname = "wne:mcd"))]
  pub mcd: Vec<Mcd>,
}
/// Defines the VbaSuppData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "wne:vbaSuppData")]
pub struct VbaSuppData {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// Defines the DocEvents Class.
  #[sdk(child(qname = "wne:docEvents"))]
  pub doc_events: Option<DocEvents>,
  /// Defines the Mcds Class.
  #[sdk(child(qname = "wne:mcds"))]
  pub mcds: Option<Mcds>,
}
/// Defines the MailMergeRecipients Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "wne:recipients")]
pub struct MailMergeRecipients {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Defines the SingleDataSourceRecord Class.
  #[sdk(child(qname = "wne:recipientData"))]
  pub single_data_source_record: Vec<SingleDataSourceRecord>,
}
/// Defines the FixedCommandKeyboardCustomization Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:fci")]
pub struct FixedCommandKeyboardCustomization {
  /// fciName
  #[sdk(attr(qname = "wne:fciName"))]
  pub command_name: Option<crate::simple_type::StringValue>,
  /// fciIndex
  #[sdk(attr(qname = "wne:fciIndex"))]
  #[sdk(string_length(min = 2u32, max = 2u32))]
  pub command_index: Option<crate::simple_type::HexBinaryValue>,
  /// swArg
  #[sdk(attr(qname = "wne:swArg"))]
  #[sdk(string_length(min = 2u32, max = 2u32))]
  pub argument: Option<crate::simple_type::HexBinaryValue>,
}
/// Defines the MacroKeyboardCustomization Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:macro")]
pub struct MacroKeyboardCustomization {
  /// macroName
  #[sdk(attr(qname = "wne:macroName"))]
  pub macro_name: Option<crate::simple_type::StringValue>,
}
/// Defines the WllMacroKeyboardCustomization Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:wll")]
pub struct WllMacroKeyboardCustomization {
  /// macroName
  #[sdk(attr(qname = "wne:macroName"))]
  pub macro_name: Option<crate::simple_type::StringValue>,
}
/// Defines the AllocatedCommandKeyboardCustomization Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:acd")]
pub struct AllocatedCommandKeyboardCustomization {
  /// acdName
  #[sdk(attr(qname = "wne:acdName"))]
  pub accelerator_name: Option<crate::simple_type::StringValue>,
}
/// Defines the AllocatedCommandManifestEntry Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:acdEntry")]
pub struct AllocatedCommandManifestEntry {
  /// acdName
  #[sdk(attr(qname = "wne:acdName"))]
  pub accelerator_name: Option<crate::simple_type::StringValue>,
}
/// Defines the CharacterInsertion Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:wch")]
pub struct CharacterInsertion {
  /// val
  #[sdk(attr(qname = "wne:val"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub val: crate::simple_type::HexBinaryValue,
}
/// Defines the KeyMapEntry Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:keymap")]
pub struct KeyMapEntry {
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// chmPrimary
  #[sdk(attr(qname = "wne:chmPrimary"))]
  #[sdk(string_length(min = 2u32, max = 2u32))]
  pub character_map_primary: Option<crate::simple_type::HexBinaryValue>,
  /// chmSecondary
  #[sdk(attr(qname = "wne:chmSecondary"))]
  #[sdk(string_length(min = 2u32, max = 2u32))]
  pub character_map_secondary: Option<crate::simple_type::HexBinaryValue>,
  /// kcmPrimary
  #[sdk(attr(qname = "wne:kcmPrimary"))]
  #[sdk(string_length(min = 2u32, max = 2u32))]
  pub key_code_primary: Option<crate::simple_type::HexBinaryValue>,
  /// kcmSecondary
  #[sdk(attr(qname = "wne:kcmSecondary"))]
  #[sdk(string_length(min = 2u32, max = 2u32))]
  pub key_code_secondary: Option<crate::simple_type::HexBinaryValue>,
  /// mask
  #[sdk(attr(qname = "wne:mask"))]
  pub mask: Option<crate::simple_type::OnOffValue>,
  #[sdk(
        choice(
            child(variant = FixedCommandKeyboardCustomization, boxed, qname = "wne:fci"),
            child(variant = MacroKeyboardCustomization, qname = "wne:macro"),
            child(variant = AllocatedCommandKeyboardCustomization, qname = "wne:acd"),
            child(variant = WllMacroKeyboardCustomization, qname = "wne:wll"),
            child(variant = CharacterInsertion, qname = "wne:wch")
        )
    )]
  pub key_map_entry_choice: Option<KeyMapEntryChoice>,
}
/// Defines the AllocatedCommand Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:acd")]
pub struct AllocatedCommand {
  /// argValue
  #[sdk(attr(qname = "wne:argValue"))]
  pub argument_value: Option<crate::simple_type::StringValue>,
  /// fciBasedOn
  #[sdk(attr(qname = "wne:fciBasedOn"))]
  pub command_based_on: Option<crate::simple_type::StringValue>,
  /// fciIndexBasedOn
  #[sdk(attr(qname = "wne:fciIndexBasedOn"))]
  #[sdk(string_length(min = 2u32, max = 2u32))]
  pub command_index_based_on: Option<crate::simple_type::HexBinaryValue>,
  /// acdName
  #[sdk(attr(qname = "wne:acdName"))]
  pub accelerator_name: Option<crate::simple_type::StringValue>,
}
/// Defines the Mcd Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:mcd")]
pub struct Mcd {
  /// macroName
  #[sdk(attr(qname = "wne:macroName"))]
  pub wne_macro_name: Option<crate::simple_type::StringValue>,
  /// name
  #[sdk(attr(qname = "wne:name"))]
  pub wne_name: Option<crate::simple_type::StringValue>,
  /// menuHelp
  #[sdk(attr(qname = "wne:menuHelp"))]
  pub wne_menu_help: Option<crate::simple_type::StringValue>,
  /// bEncrypt
  #[sdk(attr(qname = "wne:bEncrypt"))]
  #[sdk(string_length(min = 1u32, max = 1u32))]
  pub wne_b_encrypt: Option<crate::simple_type::HexBinaryValue>,
  /// cmg
  #[sdk(attr(qname = "wne:cmg"))]
  #[sdk(string_length(min = 1u32, max = 1u32))]
  pub wne_cmg: Option<crate::simple_type::HexBinaryValue>,
}
/// Defines the EventDocNewXsdString Class.
pub type EventDocNewXsdString = crate::simple_type::StringValue;
/// Defines the EventDocOpenXsdString Class.
pub type EventDocOpenXsdString = crate::simple_type::StringValue;
/// Defines the EventDocCloseXsdString Class.
pub type EventDocCloseXsdString = crate::simple_type::StringValue;
/// Defines the EventDocSyncXsdString Class.
pub type EventDocSyncXsdString = crate::simple_type::StringValue;
/// Defines the EventDocXmlAfterInsertXsdString Class.
pub type EventDocXmlAfterInsertXsdString = crate::simple_type::StringValue;
/// Defines the EventDocXmlBeforeDeleteXsdString Class.
pub type EventDocXmlBeforeDeleteXsdString = crate::simple_type::StringValue;
/// Defines the EventDocContentControlAfterInsertXsdString Class.
pub type EventDocContentControlAfterInsertXsdString = crate::simple_type::StringValue;
/// Defines the EventDocContentControlBeforeDeleteXsdString Class.
pub type EventDocContentControlBeforeDeleteXsdString = crate::simple_type::StringValue;
/// Defines the EventDocContentControlOnExistXsdString Class.
pub type EventDocContentControlOnExistXsdString = crate::simple_type::StringValue;
/// Defines the EventDocContentControlOnEnterXsdString Class.
pub type EventDocContentControlOnEnterXsdString = crate::simple_type::StringValue;
/// Defines the EventDocStoreUpdateXsdString Class.
pub type EventDocStoreUpdateXsdString = crate::simple_type::StringValue;
/// Defines the EventDocContentControlUpdateXsdString Class.
pub type EventDocContentControlUpdateXsdString = crate::simple_type::StringValue;
/// Defines the EventDocBuildingBlockAfterInsertXsdString Class.
pub type EventDocBuildingBlockAfterInsertXsdString = crate::simple_type::StringValue;
/// Defines the DocEvents Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:docEvents")]
pub struct DocEvents {
  /// Defines the EventDocNewXsdString Class.
  #[sdk(text_child(qname = "wne:eventDocNew"))]
  pub event_doc_new_xsd_string: Option<EventDocNewXsdString>,
  /// Defines the EventDocOpenXsdString Class.
  #[sdk(text_child(qname = "wne:eventDocOpen"))]
  pub event_doc_open_xsd_string: Option<EventDocOpenXsdString>,
  /// Defines the EventDocCloseXsdString Class.
  #[sdk(text_child(qname = "wne:eventDocClose"))]
  pub event_doc_close_xsd_string: Option<EventDocCloseXsdString>,
  /// Defines the EventDocSyncXsdString Class.
  #[sdk(text_child(qname = "wne:eventDocSync"))]
  pub event_doc_sync_xsd_string: Option<EventDocSyncXsdString>,
  /// Defines the EventDocXmlAfterInsertXsdString Class.
  #[sdk(text_child(qname = "wne:eventDocXmlAfterInsert"))]
  pub event_doc_xml_after_insert_xsd_string: Option<EventDocXmlAfterInsertXsdString>,
  /// Defines the EventDocXmlBeforeDeleteXsdString Class.
  #[sdk(text_child(qname = "wne:eventDocXmlBeforeDelete"))]
  pub event_doc_xml_before_delete_xsd_string: Option<EventDocXmlBeforeDeleteXsdString>,
  /// Defines the EventDocContentControlAfterInsertXsdString Class.
  #[sdk(text_child(qname = "wne:eventDocContentControlAfterInsert"))]
  pub event_doc_content_control_after_insert_xsd_string:
    Option<EventDocContentControlAfterInsertXsdString>,
  /// Defines the EventDocContentControlBeforeDeleteXsdString Class.
  #[sdk(text_child(qname = "wne:eventDocContentControlBeforeDelete"))]
  pub event_doc_content_control_before_delete_xsd_string:
    Option<EventDocContentControlBeforeDeleteXsdString>,
  /// Defines the EventDocContentControlOnExistXsdString Class.
  #[sdk(text_child(qname = "wne:eventDocContentControlOnExit"))]
  pub event_doc_content_control_on_exist_xsd_string: Option<EventDocContentControlOnExistXsdString>,
  /// Defines the EventDocContentControlOnEnterXsdString Class.
  #[sdk(text_child(qname = "wne:eventDocContentControlOnEnter"))]
  pub event_doc_content_control_on_enter_xsd_string: Option<EventDocContentControlOnEnterXsdString>,
  /// Defines the EventDocStoreUpdateXsdString Class.
  #[sdk(text_child(qname = "wne:eventDocStoreUpdate"))]
  pub event_doc_store_update_xsd_string: Option<EventDocStoreUpdateXsdString>,
  /// Defines the EventDocContentControlUpdateXsdString Class.
  #[sdk(text_child(qname = "wne:eventDocContentControlContentUpdate"))]
  pub event_doc_content_control_update_xsd_string: Option<EventDocContentControlUpdateXsdString>,
  /// Defines the EventDocBuildingBlockAfterInsertXsdString Class.
  #[sdk(text_child(qname = "wne:eventDocBuildingBlockAfterInsert"))]
  pub event_doc_building_block_after_insert_xsd_string:
    Option<EventDocBuildingBlockAfterInsertXsdString>,
}
/// Defines the AllocatedCommandManifest Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:acdManifest")]
pub struct AllocatedCommandManifest {
  /// Defines the AllocatedCommandManifestEntry Class.
  #[sdk(child(qname = "wne:acdEntry"))]
  pub allocated_command_manifest_entry: Vec<AllocatedCommandManifestEntry>,
}
/// Defines the ToolbarData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:toolbarData")]
pub struct ToolbarData {
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the KeyMapCustomizations Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:keymaps")]
pub struct KeyMapCustomizations {
  /// Defines the KeyMapEntry Class.
  #[sdk(child(qname = "wne:keymap"))]
  pub key_map_entry: Vec<KeyMapEntry>,
}
/// Defines the MismatchedKeyMapCustomization Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:keymapsBad")]
pub struct MismatchedKeyMapCustomization {
  /// Defines the KeyMapEntry Class.
  #[sdk(child(qname = "wne:keymap"))]
  pub key_map_entry: Vec<KeyMapEntry>,
}
/// Defines the Toolbars Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:toolbars")]
pub struct Toolbars {
  /// Defines the AllocatedCommandManifest Class.
  #[sdk(child(qname = "wne:acdManifest"))]
  pub allocated_command_manifest: Vec<AllocatedCommandManifest>,
  /// Defines the ToolbarData Class.
  #[sdk(child(qname = "wne:toolbarData"))]
  pub toolbar_data: Vec<ToolbarData>,
}
/// Defines the AllocatedCommands Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:acds")]
pub struct AllocatedCommands {
  /// Defines the AllocatedCommand Class.
  #[sdk(child(qname = "wne:acd"))]
  pub allocated_command: Vec<AllocatedCommand>,
}
/// Defines the RecordIncluded Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:active")]
pub struct RecordIncluded {
  /// val
  #[sdk(attr(qname = "wne:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the RecordHashCode Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:hash")]
pub struct RecordHashCode {
  /// val
  #[sdk(attr(qname = "wne:val"))]
  pub val: crate::simple_type::IntegerValue,
}
/// Defines the SingleDataSourceRecord Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wne:recipientData")]
pub struct SingleDataSourceRecord {
  /// Defines the RecordIncluded Class.
  #[sdk(child(qname = "wne:active"))]
  pub record_included: Option<RecordIncluded>,
  /// Defines the RecordHashCode Class.
  #[sdk(child(qname = "wne:hash"))]
  pub record_hash_code: RecordHashCode,
}
#[derive(Clone, Debug, PartialEq)]
pub enum KeyMapEntryChoice {
  /// Defines the FixedCommandKeyboardCustomization Class.
  FixedCommandKeyboardCustomization(std::boxed::Box<FixedCommandKeyboardCustomization>),
  /// Defines the MacroKeyboardCustomization Class.
  MacroKeyboardCustomization(MacroKeyboardCustomization),
  /// Defines the AllocatedCommandKeyboardCustomization Class.
  AllocatedCommandKeyboardCustomization(AllocatedCommandKeyboardCustomization),
  /// Defines the WllMacroKeyboardCustomization Class.
  WllMacroKeyboardCustomization(WllMacroKeyboardCustomization),
  /// Defines the CharacterInsertion Class.
  CharacterInsertion(CharacterInsertion),
}
