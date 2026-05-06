//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/dialogsheet";
pub const PATH_PREFIX: &str = "dialogsheets";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml";
pub const TARGET_NAME: &str = "sheet";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct DialogsheetPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl DialogsheetPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DialogSheet,
    DialogsheetPart,
    as_dialogsheet_part,
    as_dialogsheet_part_mut
  );
  crate::sdk_part_child_methods! {
      repeated spreadsheet_printer_settings_parts => crate
      ::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings";
      optional drawings_part => crate ::parts::drawings_part::DrawingsPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing";
      repeated vml_drawing_parts => crate ::parts::vml_drawing_part::VmlDrawingPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing";
      repeated embedded_object_parts => crate
      ::parts::embedded_object_part::EmbeddedObjectPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject";
  }
}
