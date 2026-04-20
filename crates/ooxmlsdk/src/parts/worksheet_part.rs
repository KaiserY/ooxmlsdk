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
    #[sdk(part_rid)]
    pub r_id: String,
    #[sdk(part_relationships)]
    pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
    #[sdk(part_rels_path)]
    pub rels_path: String,
    #[sdk(part_inner_path)]
    pub inner_path: String,
    #[sdk(part_root)]
    pub root_element: crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Worksheet,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings",
            kind = "repeated"
        )
    )]
    pub spreadsheet_printer_settings_parts: Vec<
        crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing",
            kind = "optional"
        )
    )]
    pub drawings_part: Option<
        std::boxed::Box<crate::parts::drawings_part::DrawingsPart>,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing",
            kind = "repeated"
        )
    )]
    pub vml_drawing_parts: Vec<crate::parts::vml_drawing_part::VmlDrawingPart>,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments",
            kind = "optional"
        )
    )]
    pub worksheet_comments_part: Option<
        std::boxed::Box<crate::parts::worksheet_comments_part::WorksheetCommentsPart>,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotTable",
            kind = "repeated"
        )
    )]
    pub pivot_table_parts: Vec<crate::parts::pivot_table_part::PivotTablePart>,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableSingleCells",
            kind = "optional"
        )
    )]
    pub single_cell_table_part: Option<
        std::boxed::Box<crate::parts::single_cell_table_part::SingleCellTablePart>,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/table",
            kind = "repeated"
        )
    )]
    pub table_definition_parts: Vec<
        crate::parts::table_definition_part::TableDefinitionPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control",
            kind = "repeated"
        )
    )]
    pub embedded_control_persistence_parts: Vec<
        crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
    >,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/ctrlProp",
            kind = "repeated"
        )
    )]
    pub control_properties_parts: Vec<
        crate::parts::control_properties_part::ControlPropertiesPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject",
            kind = "repeated"
        )
    )]
    pub embedded_object_parts: Vec<
        crate::parts::embedded_object_part::EmbeddedObjectPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package",
            kind = "repeated"
        )
    )]
    pub embedded_package_parts: Vec<
        crate::parts::embedded_package_part::EmbeddedPackagePart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
            kind = "repeated"
        )
    )]
    pub image_parts: Vec<crate::parts::image_part::ImagePart>,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customProperty",
            kind = "repeated"
        )
    )]
    pub custom_property_parts: Vec<
        crate::parts::custom_property_part::CustomPropertyPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2006/relationships/wsSortMap",
            kind = "optional"
        )
    )]
    pub worksheet_sort_map_part: Option<
        std::boxed::Box<crate::parts::worksheet_sort_map_part::WorksheetSortMapPart>,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/queryTable",
            kind = "repeated"
        )
    )]
    pub query_table_parts: Vec<crate::parts::query_table_part::QueryTablePart>,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2006/relationships/activeXControlBinary",
            kind = "repeated"
        )
    )]
    pub embedded_control_persistence_binary_data_parts: Vec<
        crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
    >,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2007/relationships/slicer",
            kind = "repeated"
        )
    )]
    pub slicers_parts: Vec<crate::parts::slicers_part::SlicersPart>,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2011/relationships/timeline",
            kind = "repeated"
        )
    )]
    pub time_line_parts: Vec<crate::parts::time_line_part::TimeLinePart>,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2017/10/relationships/threadedComment",
            kind = "repeated"
        )
    )]
    pub worksheet_threaded_comments_parts: Vec<
        crate::parts::worksheet_threaded_comments_part::WorksheetThreadedCommentsPart,
    >,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/model3d",
            kind = "repeated"
        )
    )]
    pub model3_d_reference_relationship_parts: Vec<
        crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
    >,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2019/04/relationships/namedSheetView",
            kind = "repeated"
        )
    )]
    pub named_sheet_views_parts: Vec<
        crate::parts::named_sheet_views_part::NamedSheetViewsPart,
    >,
}
