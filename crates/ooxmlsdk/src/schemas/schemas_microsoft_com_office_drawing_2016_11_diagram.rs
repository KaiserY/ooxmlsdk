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
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is dgm1611:autoBuNodeInfoLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm1611:CT_NumberDiagramInfoList/dgm1611:autoBuNodeInfoLst")]
pub struct NumberDiagramInfoList {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "dgm1611:CT_NumberDiagramInfo/dgm1611:autoBuNodeInfo"))]
  pub dgm1611_auto_bu_node_info: Vec<NumberDiagramInfo>,
}
/// Defines the DiagramAutoBullet Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is dgm1611:buPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm1611:CT_DiagramAutoBullet/dgm1611:buPr")]
pub struct DiagramAutoBullet {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// prefix
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :prefix
  #[sdk(attr(qname = ":prefix"))]
  pub auto_bullet_prefix: Option<crate::simple_type::StringValue>,
  /// leadZeros
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :leadZeros
  #[sdk(attr(qname = ":leadZeros"))]
  pub lead_zeros: Option<crate::simple_type::BooleanValue>,
  #[sdk(choice(
    qname = "a:CT_TextNoBullet/a:buNone",
    qname = "a:CT_TextAutonumberBullet/a:buAutoNum",
    qname = "a:CT_TextCharBullet/a:buChar",
    qname = "a:CT_TextBlipBullet/a:buBlip"
  ))]
  pub xml_children: Option<DiagramAutoBulletChoice>,
}
/// Defines the NumberDiagramInfo Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is dgm1611:autoBuNodeInfo.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm1611:CT_NumberDiagramInfo/dgm1611:autoBuNodeInfo")]
pub struct NumberDiagramInfo {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// lvl
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :lvl
  #[sdk(attr(qname = ":lvl"))]
  pub lvl: crate::simple_type::UInt32Value,
  /// ptType
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :ptType
  #[sdk(attr(qname = ":ptType"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub pt_type: STorageType,
  /// _
  #[sdk(child(qname = "dgm1611:CT_DiagramAutoBullet/dgm1611:buPr"))]
  pub diagram_auto_bullet: std::boxed::Box<DiagramAutoBullet>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DiagramAutoBulletChoice {
  /// No Bullet.
  #[sdk(empty_child(qname = "a:CT_TextNoBullet/a:buNone"))]
  ABuNone,
  /// Auto-Numbered Bullet.
  #[sdk(child(qname = "a:CT_TextAutonumberBullet/a:buAutoNum"))]
  ABuAutoNum(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AutoNumberedBullet,
    >,
  ),
  /// Character Bullet.
  #[sdk(child(qname = "a:CT_TextCharBullet/a:buChar"))]
  ABuChar(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::CharacterBullet,
    >,
  ),
  /// Picture Bullet.
  #[sdk(child(qname = "a:CT_TextBlipBullet/a:buBlip"))]
  ABuBlip(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PictureBullet>,
  ),
}
