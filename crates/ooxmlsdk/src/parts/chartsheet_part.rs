//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartsheet";
pub const PATH_PREFIX: &str = "chartsheets";
#[derive(Clone, Copy, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct ChartsheetPart {
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_chartsheet_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Chartsheet,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings",
    kind = "repeated"
  ))]
  pub(crate) spreadsheet_printer_settings_parts: crate::sdk::PartChild<
    crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing",
    kind = "optional"
  ))]
  pub(crate) drawings_part: crate::sdk::PartChild<crate::parts::drawings_part::DrawingsPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing",
    kind = "repeated"
  ))]
  pub(crate) vml_drawing_parts:
    crate::sdk::PartChild<crate::parts::vml_drawing_part::VmlDrawingPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
    kind = "repeated"
  ))]
  pub(crate) image_parts: crate::sdk::PartChild<crate::parts::image_part::ImagePart>,
}
