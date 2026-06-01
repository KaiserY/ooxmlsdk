//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct DialogsheetPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DialogSheet,
  >,
  pub(crate) spreadsheet_printer_settings_parts: crate::sdk::RepeatedPart<
    crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart,
  >,
  pub(crate) drawings_part: crate::sdk::OptionalPart<crate::parts::drawings_part::DrawingsPart>,
  pub(crate) vml_drawing_parts:
    crate::sdk::RepeatedPart<crate::parts::vml_drawing_part::VmlDrawingPart>,
  pub(crate) embedded_object_parts:
    crate::sdk::RepeatedPart<crate::parts::embedded_object_part::EmbeddedObjectPart>,
}
