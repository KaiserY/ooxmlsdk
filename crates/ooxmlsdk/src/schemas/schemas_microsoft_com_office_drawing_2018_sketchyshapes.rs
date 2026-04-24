//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the LineSketchNoneEmpty Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is ask:lineSketchNone.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ask:CT_Empty/ask:lineSketchNone")]
pub struct LineSketchNoneEmpty {}
/// Defines the LineSketchCurvedEmpty Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is ask:lineSketchCurved.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ask:CT_Empty/ask:lineSketchCurved")]
pub struct LineSketchCurvedEmpty {}
/// Defines the LineSketchFreehandEmpty Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is ask:lineSketchFreehand.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ask:CT_Empty/ask:lineSketchFreehand")]
pub struct LineSketchFreehandEmpty {}
/// Defines the LineSketchScribbleEmpty Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is ask:lineSketchScribble.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ask:CT_Empty/ask:lineSketchScribble")]
pub struct LineSketchScribbleEmpty {}
/// Defines the OpenXmlEmptyElement Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ask:CT_Empty/")]
pub struct OpenXmlEmptyElement {}
/// Defines the LineSketchStyleProperties Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is ask:lineSketchStyleProps.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ask:CT_LineSketchStyleProperties/ask:lineSketchStyleProps")]
pub struct LineSketchStyleProperties {
  /// sd
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :sd
  #[sdk(attr(qname = ":sd"))]
  pub sd: Option<crate::simple_type::UInt32Value>,
  #[sdk(choice(
    qname = "a:CT_CustomGeometry2D/a:custGeom",
    qname = "a:CT_PresetGeometry2D/a:prstGeom"
  ))]
  pub line_sketch_style_properties_choice: Option<LineSketchStylePropertiesChoice>,
  /// _
  #[sdk(child(qname = "ask:CT_LineSketchTypeProperties/ask:type"))]
  pub ask_type: Option<std::boxed::Box<LineSketchTypeProperties>>,
  /// _
  #[sdk(text_child(qname = "ask:ST_LineSketchSeed/ask:seed"))]
  pub ask_seed: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/ask:extLst"))]
  pub ask_ext_lst: Option<OfficeArtExtensionList>,
}
/// Defines the LineSketchTypeProperties Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is ask:type.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ask:CT_LineSketchTypeProperties/ask:type")]
pub struct LineSketchTypeProperties {
  #[sdk(choice(
    qname = "ask:CT_Empty/ask:lineSketchNone",
    qname = "ask:CT_Empty/ask:lineSketchCurved",
    qname = "ask:CT_Empty/ask:lineSketchFreehand",
    qname = "ask:CT_Empty/ask:lineSketchScribble"
  ))]
  pub xml_children: Option<LineSketchTypePropertiesChoice>,
}
/// Defines the LineSketchSeed Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is ask:seed.
pub type LineSketchSeed = crate::simple_type::UInt32Value;
/// Defines the OfficeArtExtensionList Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is ask:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_OfficeArtExtensionList/ask:extLst")]
pub struct OfficeArtExtensionList {
  ///Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LineSketchStylePropertiesChoice {
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
pub enum LineSketchTypePropertiesChoice {
  #[sdk(child(qname = "ask:CT_Empty/ask:lineSketchNone"))]
  AskLineSketchNone(std::boxed::Box<LineSketchNoneEmpty>),
  #[sdk(child(qname = "ask:CT_Empty/ask:lineSketchCurved"))]
  AskLineSketchCurved(std::boxed::Box<LineSketchCurvedEmpty>),
  #[sdk(child(qname = "ask:CT_Empty/ask:lineSketchFreehand"))]
  AskLineSketchFreehand(std::boxed::Box<LineSketchFreehandEmpty>),
  #[sdk(child(qname = "ask:CT_Empty/ask:lineSketchScribble"))]
  AskLineSketchScribble(std::boxed::Box<LineSketchScribbleEmpty>),
}
