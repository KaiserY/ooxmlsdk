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
#[sdk(qname = "dgm1611:autoBuNodeInfoLst")]
pub struct NumberDiagramInfoList {
  /// Defines the NumberDiagramInfo Class.
  #[sdk(child(qname = "dgm1611:autoBuNodeInfo"))]
  pub number_diagram_info: Vec<NumberDiagramInfo>,
}
/// Defines the DiagramAutoBullet Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm1611:buPr")]
pub struct DiagramAutoBullet {
  /// prefix
  #[sdk(attr(qname = ":prefix"))]
  pub auto_bullet_prefix: Option<crate::simple_type::StringValue>,
  /// leadZeros
  #[sdk(attr(qname = ":leadZeros"))]
  pub lead_zeros: Option<crate::simple_type::BooleanValue>,
  #[sdk(
        choice(
            empty_child(variant = NoBullet, qname = "a:buNone"),
            child(variant = AutoNumberedBullet, qname = "a:buAutoNum"),
            child(variant = CharacterBullet, qname = "a:buChar"),
            child(variant = PictureBullet, qname = "a:buBlip")
        )
    )]
  pub diagram_auto_bullet_choice: Option<DiagramAutoBulletChoice>,
}
/// Defines the NumberDiagramInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm1611:autoBuNodeInfo")]
pub struct NumberDiagramInfo {
  /// lvl
  #[sdk(attr(qname = ":lvl"))]
  pub lvl: crate::simple_type::UInt32Value,
  /// ptType
  #[sdk(attr(qname = ":ptType"))]
  #[sdk(string_format(kind = "token"))]
  pub pt_type: STorageType,
  /// Defines the DiagramAutoBullet Class.
  #[sdk(child(qname = "dgm1611:buPr"))]
  pub diagram_auto_bullet: std::boxed::Box<DiagramAutoBullet>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DiagramAutoBulletChoice {
  /// No Bullet.
  #[sdk(empty_child(qname = "a:buNone"))]
  NoBullet,
  /// Auto-Numbered Bullet.
  AutoNumberedBullet(std::boxed::Box<crate::schemas::a::AutoNumberedBullet>),
  /// Character Bullet.
  CharacterBullet(std::boxed::Box<crate::schemas::a::CharacterBullet>),
  /// Picture Bullet.
  PictureBullet(std::boxed::Box<crate::schemas::a::PictureBullet>),
}
