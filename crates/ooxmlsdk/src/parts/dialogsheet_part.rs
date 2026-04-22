//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/dialogsheet";
pub const PATH_PREFIX: &str = "dialogsheets";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct DialogsheetPart {
  pub r_id: String,
  pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
  pub rels_path: String,
  pub extended_parts: Vec<crate::common::extended_part::ExtendedPart>,
  pub inner_path: String,
  pub root_element: crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DialogSheet,
  pub spreadsheet_printer_settings_parts:
    Vec<crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart>,
  pub drawings_part: Option<std::boxed::Box<crate::parts::drawings_part::DrawingsPart>>,
  pub vml_drawing_parts: Vec<crate::parts::vml_drawing_part::VmlDrawingPart>,
  pub embedded_object_parts: Vec<crate::parts::embedded_object_part::EmbeddedObjectPart>,
}
