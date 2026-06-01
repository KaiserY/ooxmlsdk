//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2006/relationships/xlMacrosheet";
pub const PATH_PREFIX: &str = "macrosheets";
pub const CONTENT_TYPE: &str = "application/vnd.ms-excel.macrosheet+xml";
pub const TARGET_NAME: &str = "sheet";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct MacroSheetPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_macro_sheet_part"))]
  pub(crate) root_element:
    crate::sdk::PartRoot<crate::schemas::schemas_microsoft_com_office_excel_2006_main::Macrosheet>,
  #[sdk(part_child(relationship_type = RelationshipPrinterSettings))]
  pub(crate) spreadsheet_printer_settings_parts: crate::sdk::RepeatedPart<
    crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart,
  >,
  #[sdk(part_child(relationship_type = RelationshipDrawing))]
  pub(crate) drawings_part: crate::sdk::OptionalPart<crate::parts::drawings_part::DrawingsPart>,
  #[sdk(part_child(relationship_type = RelationshipVmlDrawing))]
  pub(crate) vml_drawing_parts:
    crate::sdk::RepeatedPart<crate::parts::vml_drawing_part::VmlDrawingPart>,
  #[sdk(part_child(relationship_type = RelationshipComments2))]
  pub(crate) worksheet_comments_part:
    crate::sdk::OptionalPart<crate::parts::worksheet_comments_part::WorksheetCommentsPart>,
  #[sdk(part_child(relationship_type = RelationshipCustomProperty))]
  pub(crate) custom_property_parts:
    crate::sdk::RepeatedPart<crate::parts::custom_property_part::CustomPropertyPart>,
  #[sdk(part_child(relationship_type = RelationshipOleObject))]
  pub(crate) embedded_object_parts:
    crate::sdk::RepeatedPart<crate::parts::embedded_object_part::EmbeddedObjectPart>,
  #[sdk(part_child(relationship_type = RelationshipPackage))]
  pub(crate) embedded_package_parts:
    crate::sdk::RepeatedPart<crate::parts::embedded_package_part::EmbeddedPackagePart>,
  #[sdk(part_child(relationship_type = RelationshipImage))]
  pub(crate) image_parts: crate::sdk::RepeatedPart<crate::parts::image_part::ImagePart>,
}
