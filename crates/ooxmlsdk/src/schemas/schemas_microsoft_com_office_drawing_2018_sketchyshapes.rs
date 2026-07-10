//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the LineSketchStyleProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ask:lineSketchStyleProps")]
pub struct LineSketchStyleProperties {
  /// sd
  #[sdk(attr(qname = ":sd"))]
  pub sd: Option<crate::simple_type::UInt32Value>,
  #[sdk(
        choice(
            child(variant = CustomGeometry, boxed, qname = "a:custGeom"),
            child(variant = PresetGeometry, boxed, qname = "a:prstGeom")
        )
    )]
  pub line_sketch_style_properties_choice: Option<LineSketchStylePropertiesChoice>,
  /// Defines the LineSketchTypeProperties Class.
  #[sdk(child(qname = "ask:type"))]
  pub line_sketch_type_properties: Option<std::boxed::Box<LineSketchTypeProperties>>,
  /// Defines the LineSketchSeed Class.
  #[sdk(text_child(simple_type = "UInt32Value", qname = "ask:seed"))]
  pub line_sketch_seed: Option<LineSketchSeed>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "ask:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the LineSketchTypeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ask:type")]
pub struct LineSketchTypeProperties {
  #[sdk(
        choice(
            empty_child(variant = LineSketchNoneEmpty, qname = "ask:lineSketchNone"),
            empty_child(variant = LineSketchCurvedEmpty, qname = "ask:lineSketchCurved"),
            empty_child(
                variant = LineSketchFreehandEmpty,
                qname = "ask:lineSketchFreehand"
            ),
            empty_child(
                variant = LineSketchScribbleEmpty,
                qname = "ask:lineSketchScribble"
            )
        )
    )]
  pub line_sketch_type_properties_choice: Option<LineSketchTypePropertiesChoice>,
}
/// Defines the LineSketchSeed Class.
pub type LineSketchSeed = crate::simple_type::UInt32Value;
/// Defines the OfficeArtExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ask:extLst")]
pub struct OfficeArtExtensionList {
  /// Extension.
  #[sdk(child(qname = "a:ext"))]
  pub extension: Vec<crate::schemas::a::Extension>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum LineSketchStylePropertiesChoice {
  CustomGeometry(std::boxed::Box<crate::schemas::a::CustomGeometry>),
  PresetGeometry(std::boxed::Box<crate::schemas::a::PresetGeometry>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum LineSketchTypePropertiesChoice {
  /// Defines the LineSketchNoneEmpty Class.
  LineSketchNoneEmpty,
  /// Defines the LineSketchCurvedEmpty Class.
  LineSketchCurvedEmpty,
  /// Defines the LineSketchFreehandEmpty Class.
  LineSketchFreehandEmpty,
  /// Defines the LineSketchScribbleEmpty Class.
  LineSketchScribbleEmpty,
}
