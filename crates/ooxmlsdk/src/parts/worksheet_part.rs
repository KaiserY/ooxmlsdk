//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet";
pub const PATH_PREFIX: &str = "worksheets";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct WorksheetPart {
    pub r_id: String,
    pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
    pub rels_path: String,
    pub extended_parts: Vec<crate::common::extended_part::ExtendedPart>,
    pub inner_path: String,
    pub root_element: crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Worksheet,
    pub spreadsheet_printer_settings_parts: Vec<
        crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart,
    >,
    pub drawings_part: Option<
        std::boxed::Box<crate::parts::drawings_part::DrawingsPart>,
    >,
    pub vml_drawing_parts: Vec<crate::parts::vml_drawing_part::VmlDrawingPart>,
    pub worksheet_comments_part: Option<
        std::boxed::Box<crate::parts::worksheet_comments_part::WorksheetCommentsPart>,
    >,
    pub pivot_table_parts: Vec<crate::parts::pivot_table_part::PivotTablePart>,
    pub single_cell_table_part: Option<
        std::boxed::Box<crate::parts::single_cell_table_part::SingleCellTablePart>,
    >,
    pub table_definition_parts: Vec<
        crate::parts::table_definition_part::TableDefinitionPart,
    >,
    pub embedded_control_persistence_parts: Vec<
        crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
    >,
    #[cfg(feature = "microsoft365")]
    pub control_properties_parts: Vec<
        crate::parts::control_properties_part::ControlPropertiesPart,
    >,
    pub embedded_object_parts: Vec<
        crate::parts::embedded_object_part::EmbeddedObjectPart,
    >,
    pub embedded_package_parts: Vec<
        crate::parts::embedded_package_part::EmbeddedPackagePart,
    >,
    pub image_parts: Vec<crate::parts::image_part::ImagePart>,
    pub custom_property_parts: Vec<
        crate::parts::custom_property_part::CustomPropertyPart,
    >,
    pub worksheet_sort_map_part: Option<
        std::boxed::Box<crate::parts::worksheet_sort_map_part::WorksheetSortMapPart>,
    >,
    pub query_table_parts: Vec<crate::parts::query_table_part::QueryTablePart>,
    pub embedded_control_persistence_binary_data_parts: Vec<
        crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
    >,
    #[cfg(feature = "microsoft365")]
    pub slicers_parts: Vec<crate::parts::slicers_part::SlicersPart>,
    #[cfg(feature = "microsoft365")]
    pub time_line_parts: Vec<crate::parts::time_line_part::TimeLinePart>,
    #[cfg(feature = "microsoft365")]
    pub worksheet_threaded_comments_parts: Vec<
        crate::parts::worksheet_threaded_comments_part::WorksheetThreadedCommentsPart,
    >,
    #[cfg(feature = "microsoft365")]
    pub model3_d_reference_relationship_parts: Vec<
        crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
    >,
    #[cfg(feature = "microsoft365")]
    pub named_sheet_views_parts: Vec<
        crate::parts::named_sheet_views_part::NamedSheetViewsPart,
    >,
}
