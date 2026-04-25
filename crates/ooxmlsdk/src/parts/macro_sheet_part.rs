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
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments"
  ))]
  pub(crate) worksheet_comments_part:
    Option<Box<crate::parts::worksheet_comments_part::WorksheetCommentsPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customProperty"
  ))]
  pub(crate) custom_property_parts: Vec<crate::parts::custom_property_part::CustomPropertyPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject"
  ))]
  pub(crate) embedded_object_parts: Vec<crate::parts::embedded_object_part::EmbeddedObjectPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package"
  ))]
  pub(crate) embedded_package_parts: Vec<crate::parts::embedded_package_part::EmbeddedPackagePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image"
  ))]
  pub(crate) image_parts: Vec<crate::parts::image_part::ImagePart>,
  pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
  pub(crate) relationship_order: Vec<Box<str>>,
  pub(crate) data_part_reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) raw_relationships: Vec<crate::common::RelationshipInfo>,
}
impl MacroSheetPart {
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
      "worksheet_comments_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments",
      "crate::parts::worksheet_comments_part::WorksheetCommentsPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "custom_property_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customProperty",
      "crate::parts::custom_property_part::CustomPropertyPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "embedded_object_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject",
      "crate::parts::embedded_object_part::EmbeddedObjectPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "embedded_package_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package",
      "crate::parts::embedded_package_part::EmbeddedPackagePart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "image_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
      "crate::parts::image_part::ImagePart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
  ];
}
