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
    #[sdk(part_root(accessor = "as_worksheet_part"))]
    pub(crate) root_element: crate::sdk::PartRoot<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Worksheet,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings",
            kind = "repeated"
        )
    )]
    pub(crate) spreadsheet_printer_settings_parts: Vec<
        crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing",
            kind = "optional"
        )
    )]
    pub(crate) drawings_part: Option<Box<crate::parts::drawings_part::DrawingsPart>>,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing",
            kind = "repeated"
        )
    )]
    pub(crate) vml_drawing_parts: Vec<crate::parts::vml_drawing_part::VmlDrawingPart>,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments",
            kind = "optional"
        )
    )]
    pub(crate) worksheet_comments_part: Option<
        Box<crate::parts::worksheet_comments_part::WorksheetCommentsPart>,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotTable",
            kind = "repeated"
        )
    )]
    pub(crate) pivot_table_parts: Vec<crate::parts::pivot_table_part::PivotTablePart>,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableSingleCells",
            kind = "optional"
        )
    )]
    pub(crate) single_cell_table_part: Option<
        Box<crate::parts::single_cell_table_part::SingleCellTablePart>,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/table",
            kind = "repeated"
        )
    )]
    pub(crate) table_definition_parts: Vec<
        crate::parts::table_definition_part::TableDefinitionPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control",
            kind = "repeated"
        )
    )]
    pub(crate) embedded_control_persistence_parts: Vec<
        crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
    >,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/ctrlProp",
            kind = "repeated"
        )
    )]
    pub(crate) control_properties_parts: Vec<
        crate::parts::control_properties_part::ControlPropertiesPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject",
            kind = "repeated"
        )
    )]
    pub(crate) embedded_object_parts: Vec<
        crate::parts::embedded_object_part::EmbeddedObjectPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package",
            kind = "repeated"
        )
    )]
    pub(crate) embedded_package_parts: Vec<
        crate::parts::embedded_package_part::EmbeddedPackagePart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
            kind = "repeated"
        )
    )]
    pub(crate) image_parts: Vec<crate::parts::image_part::ImagePart>,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customProperty",
            kind = "repeated"
        )
    )]
    pub(crate) custom_property_parts: Vec<
        crate::parts::custom_property_part::CustomPropertyPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2006/relationships/wsSortMap",
            kind = "optional"
        )
    )]
    pub(crate) worksheet_sort_map_part: Option<
        Box<crate::parts::worksheet_sort_map_part::WorksheetSortMapPart>,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/queryTable",
            kind = "repeated"
        )
    )]
    pub(crate) query_table_parts: Vec<crate::parts::query_table_part::QueryTablePart>,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2006/relationships/activeXControlBinary",
            kind = "repeated"
        )
    )]
    pub(crate) embedded_control_persistence_binary_data_parts: Vec<
        crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
    >,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2007/relationships/slicer",
            kind = "repeated"
        )
    )]
    pub(crate) slicers_parts: Vec<crate::parts::slicers_part::SlicersPart>,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2011/relationships/timeline",
            kind = "repeated"
        )
    )]
    pub(crate) time_line_parts: Vec<crate::parts::time_line_part::TimeLinePart>,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2017/10/relationships/threadedComment",
            kind = "repeated"
        )
    )]
    pub(crate) worksheet_threaded_comments_parts: Vec<
        crate::parts::worksheet_threaded_comments_part::WorksheetThreadedCommentsPart,
    >,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/model3d",
            kind = "repeated"
        )
    )]
    pub(crate) model3_d_reference_relationship_parts: Vec<
        crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
    >,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2019/04/relationships/namedSheetView",
            kind = "repeated"
        )
    )]
    pub(crate) named_sheet_views_parts: Vec<
        crate::parts::named_sheet_views_part::NamedSheetViewsPart,
    >,
    pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
    pub(crate) relationship_order: Vec<crate::sdk::RelationshipModelEntry>,
    pub(crate) data_part_reference_relationships: Vec<crate::common::RelationshipInfo>,
    pub(crate) reference_relationships: Vec<crate::common::RelationshipInfo>,
    pub(crate) raw_relationships: Vec<crate::common::RelationshipInfo>,
}
