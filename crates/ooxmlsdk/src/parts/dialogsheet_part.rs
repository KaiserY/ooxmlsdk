//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/dialogsheet";
pub const PATH_PREFIX: &str = "dialogsheets";
#[derive(Clone, Copy, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct DialogsheetPart {
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_dialogsheet_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DialogSheet,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings"
  ))]
  pub(crate) spreadsheet_printer_settings_parts: crate::sdk::RepeatedPart<
    crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing"
  ))]
  pub(crate) drawings_part: crate::sdk::OptionalPart<crate::parts::drawings_part::DrawingsPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing"
  ))]
  pub(crate) vml_drawing_parts:
    crate::sdk::RepeatedPart<crate::parts::vml_drawing_part::VmlDrawingPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject"
  ))]
  pub(crate) embedded_object_parts:
    crate::sdk::RepeatedPart<crate::parts::embedded_object_part::EmbeddedObjectPart>,
}
