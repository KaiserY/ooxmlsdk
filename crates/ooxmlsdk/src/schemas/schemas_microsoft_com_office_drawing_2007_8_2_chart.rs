//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the PivotOptions Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is c14:pivotOptions.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c14:CT_PivotOptions/c14:pivotOptions")]
pub struct PivotOptions {
  /// _
  #[sdk(child(qname = "c14:CT_BooleanFalse/c14:dropZoneFilter"))]
  pub drop_zone_filter: Option<DropZoneFilter>,
  /// _
  #[sdk(child(qname = "c14:CT_BooleanFalse/c14:dropZoneCategories"))]
  pub drop_zone_categories: Option<DropZoneCategories>,
  /// _
  #[sdk(child(qname = "c14:CT_BooleanFalse/c14:dropZoneData"))]
  pub drop_zone_data: Option<DropZoneData>,
  /// _
  #[sdk(child(qname = "c14:CT_BooleanFalse/c14:dropZoneSeries"))]
  pub drop_zone_series: Option<DropZoneSeries>,
  /// _
  #[sdk(child(qname = "c14:CT_BooleanFalse/c14:dropZonesVisible"))]
  pub drop_zones_visible: Option<DropZonesVisible>,
}
/// Defines the SketchOptions Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is c14:sketchOptions.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c14:CT_SketchOptions/c14:sketchOptions")]
pub struct SketchOptions {
  /// _
  #[sdk(child(qname = "c14:CT_BooleanFalse/c14:inSketchMode"))]
  pub in_sketch_mode: Option<InSketchMode>,
  /// _
  #[sdk(child(qname = "c14:CT_BooleanTrue/c14:showSketchBtn"))]
  pub show_sketch_button: Option<ShowSketchButton>,
}
/// Defines the InvertSolidFillFormat Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is c14:invertSolidFillFmt.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c14:CT_InvertSolidFillFmt/c14:invertSolidFillFmt")]
pub struct InvertSolidFillFormat {
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/c14:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
}
/// Defines the Style Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is c14:style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c14:CT_Style/c14:style")]
pub struct Style {
  /// val
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 1u32,
    min = "101",
    max = "148",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: crate::simple_type::ByteValue,
}
/// Defines the ShapeProperties Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is c14:spPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ShapeProperties/c14:spPr")]
pub struct ShapeProperties {
  /// Black and White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bwMode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues>,
  ///2D Transform for Individual Objects
  #[sdk(child(qname = "a:CT_Transform2D/a:xfrm"))]
  pub transform2_d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Transform2D>,
  >,
  #[sdk(choice(
    qname = "a:CT_CustomGeometry2D/a:custGeom",
    qname = "a:CT_PresetGeometry2D/a:prstGeom"
  ))]
  pub shape_properties_choice1: Option<ShapePropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_BlipFillProperties/a:blipFill",
    qname = "a:CT_PatternFillProperties/a:pattFill",
    qname = "a:CT_GroupFillProperties/a:grpFill"
  ))]
  pub shape_properties_choice2: Option<ShapePropertiesChoice2>,
  /// _
  #[sdk(child(qname = "a:CT_LineProperties/a:ln"))]
  pub a_ln: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Outline>,
  >,
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub shape_properties_choice3: Option<ShapePropertiesChoice3>,
  /// _
  #[sdk(child(qname = "a:CT_Scene3D/a:scene3d"))]
  pub a_scene3d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Scene3DType>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_Shape3D/a:sp3d"))]
  pub a_sp3d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Shape3DType>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_ShapePropertiesExtensionList/a:extLst"))]
  pub a_ext_lst: Option<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ShapePropertiesExtensionList,
  >,
}
/// Defines the DropZoneFilter Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is c14:dropZoneFilter.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c14:CT_BooleanFalse/c14:dropZoneFilter")]
pub struct DropZoneFilter {
  /// val
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the DropZoneCategories Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is c14:dropZoneCategories.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c14:CT_BooleanFalse/c14:dropZoneCategories")]
pub struct DropZoneCategories {
  /// val
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the DropZoneData Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is c14:dropZoneData.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c14:CT_BooleanFalse/c14:dropZoneData")]
pub struct DropZoneData {
  /// val
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the DropZoneSeries Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is c14:dropZoneSeries.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c14:CT_BooleanFalse/c14:dropZoneSeries")]
pub struct DropZoneSeries {
  /// val
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the DropZonesVisible Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is c14:dropZonesVisible.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c14:CT_BooleanFalse/c14:dropZonesVisible")]
pub struct DropZonesVisible {
  /// val
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the InSketchMode Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is c14:inSketchMode.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c14:CT_BooleanFalse/c14:inSketchMode")]
pub struct InSketchMode {
  /// val
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the BooleanFalseType Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c14:CT_BooleanFalse/")]
pub struct BooleanFalseType {
  /// val
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the ShowSketchButton Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is c14:showSketchBtn.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c14:CT_BooleanTrue/c14:showSketchBtn")]
pub struct ShowSketchButton {
  /// val
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice {
  #[sdk(child(qname = "a:CT_CustomGeometry2D/a:custGeom"))]
  ACustGeom(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::CustomGeometry>,
  ),
  #[sdk(child(qname = "a:CT_PresetGeometry2D/a:prstGeom"))]
  APrstGeom(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetGeometry>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice2 {
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoFill>),
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SolidFill>,
  ),
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GradientFill>,
  ),
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipFill>,
  ),
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PatternFill>,
  ),
  #[sdk(child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GroupFill>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice3 {
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectList>,
  ),
  #[sdk(child(qname = "a:CT_EffectContainer/a:effectDag"))]
  AEffectDag(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectDag>,
  ),
}
