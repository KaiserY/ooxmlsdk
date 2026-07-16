//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct WorksheetPart {
    pub(crate) id: crate::common::PartId,
    pub(crate) root_element: crate::sdk::PartRoot<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Worksheet,
    >,
    pub(crate) spreadsheet_printer_settings_parts: crate::sdk::RepeatedPart<
        crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart,
    >,
    pub(crate) drawings_part: crate::sdk::OptionalPart<
        crate::parts::drawings_part::DrawingsPart,
    >,
    pub(crate) vml_drawing_parts: crate::sdk::RepeatedPart<
        crate::parts::vml_drawing_part::VmlDrawingPart,
    >,
    pub(crate) worksheet_comments_part: crate::sdk::OptionalPart<
        crate::parts::worksheet_comments_part::WorksheetCommentsPart,
    >,
    pub(crate) pivot_table_parts: crate::sdk::RepeatedPart<
        crate::parts::pivot_table_part::PivotTablePart,
    >,
    pub(crate) single_cell_table_part: crate::sdk::OptionalPart<
        crate::parts::single_cell_table_part::SingleCellTablePart,
    >,
    pub(crate) table_definition_parts: crate::sdk::RepeatedPart<
        crate::parts::table_definition_part::TableDefinitionPart,
    >,
    pub(crate) embedded_control_persistence_parts: crate::sdk::RepeatedPart<
        crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
    >,
    pub(crate) control_properties_parts: crate::sdk::RepeatedPart<
        crate::parts::control_properties_part::ControlPropertiesPart,
    >,
    pub(crate) embedded_object_parts: crate::sdk::RepeatedPart<
        crate::parts::embedded_object_part::EmbeddedObjectPart,
    >,
    pub(crate) embedded_package_parts: crate::sdk::RepeatedPart<
        crate::parts::embedded_package_part::EmbeddedPackagePart,
    >,
    pub(crate) image_parts: crate::sdk::RepeatedPart<
        crate::parts::image_part::ImagePart,
    >,
    pub(crate) custom_property_parts: crate::sdk::RepeatedPart<
        crate::parts::custom_property_part::CustomPropertyPart,
    >,
    pub(crate) worksheet_sort_map_part: crate::sdk::OptionalPart<
        crate::parts::worksheet_sort_map_part::WorksheetSortMapPart,
    >,
    pub(crate) query_table_parts: crate::sdk::RepeatedPart<
        crate::parts::query_table_part::QueryTablePart,
    >,
    pub(crate) embedded_control_persistence_binary_data_parts: crate::sdk::RepeatedPart<
        crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
    >,
    pub(crate) slicers_parts: crate::sdk::RepeatedPart<
        crate::parts::slicers_part::SlicersPart,
    >,
    pub(crate) time_line_parts: crate::sdk::RepeatedPart<
        crate::parts::time_line_part::TimeLinePart,
    >,
    pub(crate) worksheet_threaded_comments_parts: crate::sdk::RepeatedPart<
        crate::parts::worksheet_threaded_comments_part::WorksheetThreadedCommentsPart,
    >,
    pub(crate) model3_d_reference_relationship_parts: crate::sdk::RepeatedPart<
        crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
    >,
    pub(crate) named_sheet_views_parts: crate::sdk::RepeatedPart<
        crate::parts::named_sheet_views_part::NamedSheetViewsPart,
    >,
}
