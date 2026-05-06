//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartsheet";
pub const PATH_PREFIX: &str = "chartsheets";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml";
pub const TARGET_NAME: &str = "sheet";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct ChartsheetPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl ChartsheetPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Chartsheet,
    ChartsheetPart,
    as_chartsheet_part,
    as_chartsheet_part_mut
  );
  crate::sdk_part_child_methods! {
      repeated spreadsheet_printer_settings_parts => crate
      ::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings";
      optional drawings_part => crate ::parts::drawings_part::DrawingsPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing";
      repeated vml_drawing_parts => crate ::parts::vml_drawing_part::VmlDrawingPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing";
      repeated image_parts => crate ::parts::image_part::ImagePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";
  }
}
