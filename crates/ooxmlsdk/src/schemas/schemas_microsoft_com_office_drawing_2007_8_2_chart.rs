//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the PivotOptions Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c14:pivotOptions")]
pub struct PivotOptions {
  /// Defines the DropZoneFilter Class.
  #[sdk(child(qname = "c14:dropZoneFilter"))]
  pub drop_zone_filter: Option<DropZoneFilter>,
  /// Defines the DropZoneCategories Class.
  #[sdk(child(qname = "c14:dropZoneCategories"))]
  pub drop_zone_categories: Option<DropZoneCategories>,
  /// Defines the DropZoneData Class.
  #[sdk(child(qname = "c14:dropZoneData"))]
  pub drop_zone_data: Option<DropZoneData>,
  /// Defines the DropZoneSeries Class.
  #[sdk(child(qname = "c14:dropZoneSeries"))]
  pub drop_zone_series: Option<DropZoneSeries>,
  /// Defines the DropZonesVisible Class.
  #[sdk(child(qname = "c14:dropZonesVisible"))]
  pub drop_zones_visible: Option<DropZonesVisible>,
}
/// Defines the SketchOptions Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c14:sketchOptions")]
pub struct SketchOptions {
  /// Defines the InSketchMode Class.
  #[sdk(child(qname = "c14:inSketchMode"))]
  pub in_sketch_mode: Option<InSketchMode>,
  /// Defines the ShowSketchButton Class.
  #[sdk(child(qname = "c14:showSketchBtn"))]
  pub show_sketch_button: Option<ShowSketchButton>,
}
/// Defines the InvertSolidFillFormat Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c14:invertSolidFillFmt")]
pub struct InvertSolidFillFormat {
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "c14:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
}
/// Defines the Style Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c14:style")]
pub struct Style {
  /// val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 101..= 148))]
  pub val: crate::simple_type::ByteValue,
}
/// Defines the ShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c14:spPr")]
pub struct ShapeProperties {
  /// Black and White Mode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(kind = "token"))]
  pub black_white_mode: Option<crate::schemas::a::BlackWhiteModeValues>,
  /// 2D Transform for Individual Objects
  #[sdk(child(qname = "a:xfrm"))]
  pub transform2_d: Option<std::boxed::Box<crate::schemas::a::Transform2D>>,
  #[sdk(
        choice(
            child(variant = CustomGeometry, qname = "a:custGeom"),
            child(variant = PresetGeometry, qname = "a:prstGeom")
        )
    )]
  pub shape_properties_choice1: Option<ShapePropertiesChoice>,
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, qname = "a:solidFill"),
            child(variant = GradientFill, qname = "a:gradFill"),
            child(variant = BlipFill, qname = "a:blipFill"),
            child(variant = PatternFill, qname = "a:pattFill"),
            empty_child(variant = GroupFill, qname = "a:grpFill")
        )
    )]
  pub shape_properties_choice2: Option<ShapePropertiesChoice2>,
  /// Defines the Outline Class.
  #[sdk(child(qname = "a:ln"))]
  pub outline: Option<std::boxed::Box<crate::schemas::a::Outline>>,
  #[sdk(
        choice(
            child(variant = EffectList, qname = "a:effectLst"),
            child(variant = EffectDag, qname = "a:effectDag")
        )
    )]
  pub shape_properties_choice3: Option<ShapePropertiesChoice3>,
  /// 3D Scene Properties.
  #[sdk(child(qname = "a:scene3d"))]
  pub scene3_d_type: Option<std::boxed::Box<crate::schemas::a::Scene3DType>>,
  /// Apply 3D shape properties.
  #[sdk(child(qname = "a:sp3d"))]
  pub shape3_d_type: Option<std::boxed::Box<crate::schemas::a::Shape3DType>>,
  /// Defines the ShapePropertiesExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub shape_properties_extension_list: Option<crate::schemas::a::ShapePropertiesExtensionList>,
}
/// Defines the DropZoneFilter Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c14:dropZoneFilter")]
pub struct DropZoneFilter {
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the DropZoneCategories Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c14:dropZoneCategories")]
pub struct DropZoneCategories {
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the DropZoneData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c14:dropZoneData")]
pub struct DropZoneData {
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the DropZoneSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c14:dropZoneSeries")]
pub struct DropZoneSeries {
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the DropZonesVisible Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c14:dropZonesVisible")]
pub struct DropZonesVisible {
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the InSketchMode Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c14:inSketchMode")]
pub struct InSketchMode {
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the ShowSketchButton Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c14:showSketchBtn")]
pub struct ShowSketchButton {
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum ShapePropertiesChoice {
  /// Custom geometry.
  CustomGeometry(std::boxed::Box<crate::schemas::a::CustomGeometry>),
  /// Preset geometry.
  PresetGeometry(std::boxed::Box<crate::schemas::a::PresetGeometry>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ShapePropertiesChoice2 {
  /// Defines the NoFill Class.
  NoFill(crate::schemas::a::NoFill),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<crate::schemas::a::SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<crate::schemas::a::GradientFill>),
  /// Defines the BlipFill Class.
  BlipFill(std::boxed::Box<crate::schemas::a::BlipFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<crate::schemas::a::PatternFill>),
  /// Group Fill.
  GroupFill,
}
#[derive(Clone, Debug, PartialEq)]
pub enum ShapePropertiesChoice3 {
  /// Effect Container.
  EffectList(std::boxed::Box<crate::schemas::a::EffectList>),
  /// Effect Container.
  EffectDag(std::boxed::Box<crate::schemas::a::EffectDag>),
}
