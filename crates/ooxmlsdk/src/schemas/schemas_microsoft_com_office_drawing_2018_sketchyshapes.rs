//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the LineSketchStyleProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2021,
  qname = "ask:CT_LineSketchStyleProperties/ask:lineSketchStyleProps"
)]
pub struct LineSketchStyleProperties {
  /// sd
  #[sdk(attr(office2021, qname = ":sd"))]
  pub sd: Option<crate::simple_type::UInt32Value>,
  #[sdk(choice(
    qname = "a:CT_CustomGeometry2D/a:custGeom",
    qname = "a:CT_PresetGeometry2D/a:prstGeom"
  ))]
  pub line_sketch_style_properties_choice: Option<LineSketchStylePropertiesChoice>,
  /// Defines the LineSketchTypeProperties Class.
  #[sdk(child(office2021, qname = "ask:CT_LineSketchTypeProperties/ask:type"))]
  pub ask_type: Option<std::boxed::Box<LineSketchTypeProperties>>,
  /// Defines the LineSketchSeed Class.
  #[sdk(text_child(office2021, qname = "ask:ST_LineSketchSeed/ask:seed"))]
  pub ask_seed: Option<crate::simple_type::UInt32Value>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2021, qname = "a:CT_OfficeArtExtensionList/ask:extLst"))]
  pub ask_ext_lst: Option<OfficeArtExtensionList>,
}
/// Defines the LineSketchTypeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "ask:CT_LineSketchTypeProperties/ask:type")]
pub struct LineSketchTypeProperties {
  #[sdk(choice(
    qname = "ask:CT_Empty/ask:lineSketchNone",
    qname = "ask:CT_Empty/ask:lineSketchCurved",
    qname = "ask:CT_Empty/ask:lineSketchFreehand",
    qname = "ask:CT_Empty/ask:lineSketchScribble"
  ))]
  pub line_sketch_type_properties_choice: Option<LineSketchTypePropertiesChoice>,
}
/// Defines the LineSketchSeed Class.
pub type LineSketchSeed = crate::simple_type::UInt32Value;
/// Defines the OfficeArtExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "a:CT_OfficeArtExtensionList/ask:extLst")]
pub struct OfficeArtExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub a_ext: Vec<crate::schemas::a::Extension>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LineSketchStylePropertiesChoice {
  #[sdk(child(qname = "a:CT_CustomGeometry2D/a:custGeom"))]
  ACustGeom(std::boxed::Box<crate::schemas::a::CustomGeometry>),
  #[sdk(child(qname = "a:CT_PresetGeometry2D/a:prstGeom"))]
  APrstGeom(std::boxed::Box<crate::schemas::a::PresetGeometry>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LineSketchTypePropertiesChoice {
  /// Defines the LineSketchNoneEmpty Class.
  #[sdk(empty_child(office2021, qname = "ask:CT_Empty/ask:lineSketchNone"))]
  AskLineSketchNone,
  /// Defines the LineSketchCurvedEmpty Class.
  #[sdk(empty_child(office2021, qname = "ask:CT_Empty/ask:lineSketchCurved"))]
  AskLineSketchCurved,
  /// Defines the LineSketchFreehandEmpty Class.
  #[sdk(empty_child(office2021, qname = "ask:CT_Empty/ask:lineSketchFreehand"))]
  AskLineSketchFreehand,
  /// Defines the LineSketchScribbleEmpty Class.
  #[sdk(empty_child(office2021, qname = "ask:CT_Empty/ask:lineSketchScribble"))]
  AskLineSketchScribble,
}
