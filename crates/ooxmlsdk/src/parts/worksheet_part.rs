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
#[derive(Clone, Copy, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct WorksheetPart {
    pub(crate) id: crate::common::PartId,
    #[sdk(part_root(accessor = "as_worksheet_part"))]
    pub(crate) root_element: crate::sdk::PartRoot<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Worksheet,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings"
        )
    )]
    pub(crate) spreadsheet_printer_settings_parts: crate::sdk::RepeatedPart<
        crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing"
        )
    )]
    pub(crate) drawings_part: crate::sdk::OptionalPart<
        crate::parts::drawings_part::DrawingsPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing"
        )
    )]
    pub(crate) vml_drawing_parts: crate::sdk::RepeatedPart<
        crate::parts::vml_drawing_part::VmlDrawingPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments"
        )
    )]
    pub(crate) worksheet_comments_part: crate::sdk::OptionalPart<
        crate::parts::worksheet_comments_part::WorksheetCommentsPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotTable"
        )
    )]
    pub(crate) pivot_table_parts: crate::sdk::RepeatedPart<
        crate::parts::pivot_table_part::PivotTablePart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableSingleCells"
        )
    )]
    pub(crate) single_cell_table_part: crate::sdk::OptionalPart<
        crate::parts::single_cell_table_part::SingleCellTablePart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/table"
        )
    )]
    pub(crate) table_definition_parts: crate::sdk::RepeatedPart<
        crate::parts::table_definition_part::TableDefinitionPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control"
        )
    )]
    pub(crate) embedded_control_persistence_parts: crate::sdk::RepeatedPart<
        crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
    >,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/ctrlProp"
        )
    )]
    pub(crate) control_properties_parts: crate::sdk::RepeatedPart<
        crate::parts::control_properties_part::ControlPropertiesPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject"
        )
    )]
    pub(crate) embedded_object_parts: crate::sdk::RepeatedPart<
        crate::parts::embedded_object_part::EmbeddedObjectPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package"
        )
    )]
    pub(crate) embedded_package_parts: crate::sdk::RepeatedPart<
        crate::parts::embedded_package_part::EmbeddedPackagePart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image"
        )
    )]
    pub(crate) image_parts: crate::sdk::RepeatedPart<
        crate::parts::image_part::ImagePart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customProperty"
        )
    )]
    pub(crate) custom_property_parts: crate::sdk::RepeatedPart<
        crate::parts::custom_property_part::CustomPropertyPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2006/relationships/wsSortMap"
        )
    )]
    pub(crate) worksheet_sort_map_part: crate::sdk::OptionalPart<
        crate::parts::worksheet_sort_map_part::WorksheetSortMapPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/queryTable"
        )
    )]
    pub(crate) query_table_parts: crate::sdk::RepeatedPart<
        crate::parts::query_table_part::QueryTablePart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2006/relationships/activeXControlBinary"
        )
    )]
    pub(crate) embedded_control_persistence_binary_data_parts: crate::sdk::RepeatedPart<
        crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
    >,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2007/relationships/slicer"
        )
    )]
    pub(crate) slicers_parts: crate::sdk::RepeatedPart<
        crate::parts::slicers_part::SlicersPart,
    >,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2011/relationships/timeline"
        )
    )]
    pub(crate) time_line_parts: crate::sdk::RepeatedPart<
        crate::parts::time_line_part::TimeLinePart,
    >,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2017/10/relationships/threadedComment"
        )
    )]
    pub(crate) worksheet_threaded_comments_parts: crate::sdk::RepeatedPart<
        crate::parts::worksheet_threaded_comments_part::WorksheetThreadedCommentsPart,
    >,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/model3d"
        )
    )]
    pub(crate) model3_d_reference_relationship_parts: crate::sdk::RepeatedPart<
        crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
    >,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2019/04/relationships/namedSheetView"
        )
    )]
    pub(crate) named_sheet_views_parts: crate::sdk::RepeatedPart<
        crate::parts::named_sheet_views_part::NamedSheetViewsPart,
    >,
}
