//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2006/relationships/xlIntlMacrosheet";
pub const PATH_PREFIX: &str = "macrosheets";
pub const CONTENT_TYPE: &str = "application/vnd.ms-excel.intlmacrosheet+xml";
pub const TARGET_NAME: &str = "intlsheet";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct InternationalMacroSheetPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl InternationalMacroSheetPart {
  crate::sdk_part_child_methods! {
      repeated spreadsheet_printer_settings_parts => crate
      ::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings";
      optional drawings_part => crate ::parts::drawings_part::DrawingsPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing";
      repeated vml_drawing_parts => crate ::parts::vml_drawing_part::VmlDrawingPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing";
      optional worksheet_comments_part => crate
      ::parts::worksheet_comments_part::WorksheetCommentsPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments";
      repeated custom_property_parts => crate
      ::parts::custom_property_part::CustomPropertyPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customProperty";
      repeated embedded_object_parts => crate
      ::parts::embedded_object_part::EmbeddedObjectPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject";
      repeated embedded_package_parts => crate
      ::parts::embedded_package_part::EmbeddedPackagePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package";
      repeated image_parts => crate ::parts::image_part::ImagePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";
  }
}
