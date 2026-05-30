//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the MultiLvlStrData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "c16ac:multiLvlStrLit")]
pub struct MultiLvlStrData {
  /// Point Count.
  #[sdk(child(qname = "c:ptCount"))]
  pub point_count: Option<crate::schemas::c::PointCount>,
  /// Level.
  #[sdk(child(qname = "c:lvl"))]
  pub level: Vec<crate::schemas::c::Level>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "c:extLst"))]
  pub extension_list: Option<crate::schemas::c::ExtensionList>,
}
