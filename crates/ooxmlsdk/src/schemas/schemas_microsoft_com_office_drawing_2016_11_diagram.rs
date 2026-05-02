//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum STorageType {
  #[sdk(rename = "sibTrans")]
  #[default]
  SibTrans,
  #[sdk(rename = "parTrans")]
  ParTrans,
}
/// Defines the NumberDiagramInfoList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2019,
  qname = "dgm1611:CT_NumberDiagramInfoList/dgm1611:autoBuNodeInfoLst"
)]
pub struct NumberDiagramInfoList {
  /// Defines the NumberDiagramInfo Class.
  #[sdk(child(
    office2019,
    qname = "dgm1611:CT_NumberDiagramInfo/dgm1611:autoBuNodeInfo"
  ))]
  pub dgm1611_auto_bu_node_info: Vec<NumberDiagramInfo>,
}
/// Defines the DiagramAutoBullet Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "dgm1611:CT_DiagramAutoBullet/dgm1611:buPr")]
pub struct DiagramAutoBullet {
  /// prefix
  #[sdk(attr(office2019, qname = ":prefix"))]
  pub auto_bullet_prefix: Option<crate::simple_type::StringValue>,
  /// leadZeros
  #[sdk(attr(office2019, qname = ":leadZeros"))]
  pub lead_zeros: Option<crate::simple_type::BooleanValue>,
  #[sdk(choice(
    qname = "a:CT_TextNoBullet/a:buNone",
    qname = "a:CT_TextAutonumberBullet/a:buAutoNum",
    qname = "a:CT_TextCharBullet/a:buChar",
    qname = "a:CT_TextBlipBullet/a:buBlip"
  ))]
  pub diagram_auto_bullet_choice: Option<DiagramAutoBulletChoice>,
}
/// Defines the NumberDiagramInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2019,
  qname = "dgm1611:CT_NumberDiagramInfo/dgm1611:autoBuNodeInfo"
)]
pub struct NumberDiagramInfo {
  /// lvl
  #[sdk(attr(office2019, qname = ":lvl"))]
  pub lvl: crate::simple_type::UInt32Value,
  /// ptType
  #[sdk(attr(office2019, qname = ":ptType"))]
  #[sdk(string_format(kind = "token"))]
  pub pt_type: STorageType,
  /// Defines the DiagramAutoBullet Class.
  #[sdk(child(office2019, qname = "dgm1611:CT_DiagramAutoBullet/dgm1611:buPr"))]
  pub diagram_auto_bullet: std::boxed::Box<DiagramAutoBullet>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DiagramAutoBulletChoice {
  /// No Bullet.
  #[sdk(empty_child(qname = "a:CT_TextNoBullet/a:buNone"))]
  ABuNone,
  /// Auto-Numbered Bullet.
  #[sdk(child(qname = "a:CT_TextAutonumberBullet/a:buAutoNum"))]
  ABuAutoNum(std::boxed::Box<crate::schemas::a::AutoNumberedBullet>),
  /// Character Bullet.
  #[sdk(child(qname = "a:CT_TextCharBullet/a:buChar"))]
  ABuChar(std::boxed::Box<crate::schemas::a::CharacterBullet>),
  /// Picture Bullet.
  #[sdk(child(qname = "a:CT_TextBlipBullet/a:buBlip"))]
  ABuBlip(std::boxed::Box<crate::schemas::a::PictureBullet>),
}
