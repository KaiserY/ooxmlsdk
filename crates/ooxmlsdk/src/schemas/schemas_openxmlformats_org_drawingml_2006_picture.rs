//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Picture.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pic:pic")]
pub struct Picture {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_other_children: Vec<(usize, std::boxed::Box<[u8]>)>,
  /// Non-Visual Picture Properties
  #[sdk(child(qname = "pic:nvPicPr"))]
  pub non_visual_picture_properties: Option<std::boxed::Box<NonVisualPictureProperties>>,
  /// Picture Fill
  #[sdk(child(qname = "pic:blipFill"))]
  pub blip_fill: Option<std::boxed::Box<BlipFill>>,
  /// Shape Properties
  #[sdk(child(qname = "pic:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the ShapeStyle Class.
  #[sdk(child(qname = "pic14:style"))]
  pub shape_style: Option<std::boxed::Box<crate::schemas::pic14::ShapeStyle>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "pic14:extLst"))]
  pub office_art_extension_list: Option<crate::schemas::pic14::OfficeArtExtensionList>,
}
/// Non-Visual Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pic:cNvPr")]
pub struct NonVisualDrawingProperties {
  /// Application defined unique identifier.
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// Name compatible with Object Model (non-unique).
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Description of the drawing element.
  #[sdk(attr(qname = ":descr"))]
  pub description: Option<crate::simple_type::StringValue>,
  /// Flag determining to show or hide this element.
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  /// Title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Hyperlink associated with clicking or selecting the element.
  #[sdk(child(qname = "a:hlinkClick"))]
  pub hyperlink_on_click: Option<std::boxed::Box<crate::schemas::a::HyperlinkOnClick>>,
  /// Hyperlink associated with hovering over the element.
  #[sdk(child(qname = "a:hlinkHover"))]
  pub hyperlink_on_hover: Option<std::boxed::Box<crate::schemas::a::HyperlinkOnHover>>,
  /// Future extension
  #[sdk(child(qname = "a:extLst"))]
  pub non_visual_drawing_properties_extension_list:
    Option<crate::schemas::a::NonVisualDrawingPropertiesExtensionList>,
}
/// Non-Visual Picture Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pic:cNvPicPr")]
pub struct NonVisualPictureDrawingProperties {
  /// preferRelativeResize
  #[sdk(attr(qname = ":preferRelativeResize"))]
  pub prefer_relative_resize: Option<crate::simple_type::BooleanValue>,
  /// Defines the PictureLocks Class.
  #[sdk(child(qname = "a:picLocks"))]
  pub picture_locks: Option<std::boxed::Box<crate::schemas::a::PictureLocks>>,
  /// Defines the NonVisualPicturePropertiesExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub non_visual_picture_properties_extension_list:
    Option<crate::schemas::a::NonVisualPicturePropertiesExtensionList>,
}
/// Non-Visual Picture Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pic:nvPicPr")]
pub struct NonVisualPictureProperties {
  /// Non-Visual Drawing Properties
  #[sdk(child(qname = "pic:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Picture Drawing Properties
  #[sdk(child(qname = "pic:cNvPicPr"))]
  pub non_visual_picture_drawing_properties: std::boxed::Box<NonVisualPictureDrawingProperties>,
}
/// Picture Fill.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pic:blipFill")]
pub struct BlipFill {
  /// DPI Setting
  #[sdk(attr(qname = ":dpi"))]
  pub dpi: Option<crate::simple_type::UInt32Value>,
  /// Rotate With Shape
  #[sdk(attr(qname = ":rotWithShape"))]
  pub rotate_with_shape: Option<crate::simple_type::BooleanValue>,
  /// Defines the Blip Class.
  #[sdk(child(qname = "a:blip"))]
  pub blip: Option<std::boxed::Box<crate::schemas::a::Blip>>,
  /// Source Rectangle
  #[sdk(child(qname = "a:srcRect"))]
  pub source_rectangle: Option<crate::schemas::a::SourceRectangle>,
  #[sdk(
        choice(
            child(variant = Tile, qname = "a:tile"),
            child(variant = Stretch, qname = "a:stretch")
        )
    )]
  pub blip_fill_choice: Option<BlipFillChoice>,
}
/// Shape Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pic:spPr")]
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
#[derive(Clone, Debug, PartialEq)]
pub enum BlipFillChoice {
  Tile(std::boxed::Box<crate::schemas::a::Tile>),
  Stretch(std::boxed::Box<crate::schemas::a::Stretch>),
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
