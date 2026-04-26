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
  #[sdk(part_root(accessor = "as_dialogsheet_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DialogSheet,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings"
  ))]
  pub(crate) spreadsheet_printer_settings_parts:
    Vec<crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing"
  ))]
  pub(crate) drawings_part: Option<Box<crate::parts::drawings_part::DrawingsPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing"
  ))]
  pub(crate) vml_drawing_parts: Vec<crate::parts::vml_drawing_part::VmlDrawingPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject"
  ))]
  pub(crate) embedded_object_parts: Vec<crate::parts::embedded_object_part::EmbeddedObjectPart>,
  pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
  pub(crate) relationship_order: Vec<crate::sdk::RelationshipModelEntry>,
  pub(crate) data_part_reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) raw_relationships: Vec<crate::common::RelationshipInfo>,
}
impl DialogsheetPart {
  pub const GENERATED_CHILD_DESCRIPTORS: &'static [crate::sdk::PartChildDescriptor] = &[
    crate::sdk::PartChildDescriptor::new(
      "spreadsheet_printer_settings_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings",
      "crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "drawings_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing",
      "crate::parts::drawings_part::DrawingsPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "vml_drawing_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing",
      "crate::parts::vml_drawing_part::VmlDrawingPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "embedded_object_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject",
      "crate::parts::embedded_object_part::EmbeddedObjectPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
  ];
}
