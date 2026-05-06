//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet";
pub const PATH_PREFIX: &str = "worksheets";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml";
pub const TARGET_NAME: &str = "sheet";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct WorksheetPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl WorksheetPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Worksheet,
    WorksheetPart,
    as_worksheet_part,
    as_worksheet_part_mut
  );
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
      repeated pivot_table_parts => crate ::parts::pivot_table_part::PivotTablePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotTable";
      optional single_cell_table_part => crate
      ::parts::single_cell_table_part::SingleCellTablePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableSingleCells";
      repeated table_definition_parts => crate
      ::parts::table_definition_part::TableDefinitionPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/table";
      repeated embedded_control_persistence_parts => crate
      ::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control";
      repeated control_properties_parts => crate
      ::parts::control_properties_part::ControlPropertiesPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/ctrlProp";
      repeated embedded_object_parts => crate
      ::parts::embedded_object_part::EmbeddedObjectPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject";
      repeated embedded_package_parts => crate
      ::parts::embedded_package_part::EmbeddedPackagePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package";
      repeated image_parts => crate ::parts::image_part::ImagePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";
      repeated custom_property_parts => crate
      ::parts::custom_property_part::CustomPropertyPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customProperty";
      optional worksheet_sort_map_part => crate
      ::parts::worksheet_sort_map_part::WorksheetSortMapPart,
      "http://schemas.microsoft.com/office/2006/relationships/wsSortMap"; repeated
      query_table_parts => crate ::parts::query_table_part::QueryTablePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/queryTable";
      repeated embedded_control_persistence_binary_data_parts => crate
      ::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
      "http://schemas.microsoft.com/office/2006/relationships/activeXControlBinary";
      repeated slicers_parts => crate ::parts::slicers_part::SlicersPart,
      "http://schemas.microsoft.com/office/2007/relationships/slicer"; repeated
      time_line_parts => crate ::parts::time_line_part::TimeLinePart,
      "http://schemas.microsoft.com/office/2011/relationships/timeline"; repeated
      worksheet_threaded_comments_parts => crate
      ::parts::worksheet_threaded_comments_part::WorksheetThreadedCommentsPart,
      "http://schemas.microsoft.com/office/2017/10/relationships/threadedComment";
      repeated model3_d_reference_relationship_parts => crate
      ::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
      "http://schemas.microsoft.com/office/2017/06/relationships/model3d"; repeated
      named_sheet_views_parts => crate
      ::parts::named_sheet_views_part::NamedSheetViewsPart,
      "http://schemas.microsoft.com/office/2019/04/relationships/namedSheetView";
  }
}
