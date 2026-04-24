//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2006/relationships/xlMacrosheet";
pub const PATH_PREFIX: &str = "macrosheets";
#[derive(Clone, Copy, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct MacroSheetPart {
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_macro_sheet_part"))]
  pub(crate) root_element:
    crate::sdk::PartRoot<crate::schemas::schemas_microsoft_com_office_excel_2006_main::Macrosheet>,
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
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments",
    kind = "optional"
  ))]
  pub(crate) worksheet_comments_part:
    crate::sdk::PartChild<crate::parts::worksheet_comments_part::WorksheetCommentsPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customProperty",
    kind = "repeated"
  ))]
  pub(crate) custom_property_parts:
    crate::sdk::PartChild<crate::parts::custom_property_part::CustomPropertyPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject",
    kind = "repeated"
  ))]
  pub(crate) embedded_object_parts:
    crate::sdk::PartChild<crate::parts::embedded_object_part::EmbeddedObjectPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package",
    kind = "repeated"
  ))]
  pub(crate) embedded_package_parts:
    crate::sdk::PartChild<crate::parts::embedded_package_part::EmbeddedPackagePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
    kind = "repeated"
  ))]
  pub(crate) image_parts: crate::sdk::PartChild<crate::parts::image_part::ImagePart>,
}
