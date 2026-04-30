//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the MultiLvlStrData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "c:CT_MultiLvlStrData/c16ac:multiLvlStrLit")]
pub struct MultiLvlStrData {
  /// _
  #[sdk(child(qname = "c:CT_UnsignedInt/c:ptCount"))]
  pub point_count:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::PointCount>,
  /// _
  #[sdk(child(qname = "c:CT_Lvl/c:lvl"))]
  pub c_lvl: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Level>,
  /// _
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub c_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ExtensionList>,
}
