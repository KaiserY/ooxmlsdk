//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2006/relationships/xlMacrosheet";
pub const PATH_PREFIX: &str = "macrosheets";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct MacroSheetPart {
  pub r_id: String,
  pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
  pub rels_path: String,
  pub extended_parts: Vec<crate::common::extended_part::ExtendedPart>,
  pub inner_path: String,
  pub root_element: crate::schemas::schemas_microsoft_com_office_excel_2006_main::Macrosheet,
  pub spreadsheet_printer_settings_parts:
    Vec<crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart>,
  pub drawings_part: Option<std::boxed::Box<crate::parts::drawings_part::DrawingsPart>>,
  pub vml_drawing_parts: Vec<crate::parts::vml_drawing_part::VmlDrawingPart>,
  pub worksheet_comments_part:
    Option<std::boxed::Box<crate::parts::worksheet_comments_part::WorksheetCommentsPart>>,
  pub custom_property_parts: Vec<crate::parts::custom_property_part::CustomPropertyPart>,
  pub embedded_object_parts: Vec<crate::parts::embedded_object_part::EmbeddedObjectPart>,
  pub embedded_package_parts: Vec<crate::parts::embedded_package_part::EmbeddedPackagePart>,
  pub image_parts: Vec<crate::parts::image_part::ImagePart>,
}
