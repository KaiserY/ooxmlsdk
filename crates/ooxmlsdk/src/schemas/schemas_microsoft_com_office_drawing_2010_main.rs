//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CameraTool Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:cameraTool")]
pub struct CameraTool {
  /// cellRange
  #[sdk(attr(qname = ":cellRange"))]
  pub cell_range: Option<crate::simple_type::StringValue>,
  /// spid
  #[sdk(attr(qname = ":spid"))]
  pub shape_id: Option<crate::simple_type::StringValue>,
}
/// Defines the CompatExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:compatExt")]
pub struct CompatExtension {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// spid
  #[sdk(attr(qname = ":spid"))]
  pub shape_id: Option<crate::simple_type::StringValue>,
}
/// Defines the IsCanvas Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:isCanvas")]
pub struct IsCanvas {
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::BooleanValue,
}
/// Defines the GvmlContentPart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:contentPart")]
pub struct GvmlContentPart {
  /// bwMode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(kind = "token"))]
  pub black_white_mode: Option<crate::schemas::a::BlackWhiteModeValues>,
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub relationship_id: crate::simple_type::StringValue,
  /// Defines the NonVisualContentPartProperties Class.
  #[sdk(child(qname = "a14:nvContentPartPr"))]
  pub non_visual_content_part_properties: Option<std::boxed::Box<NonVisualContentPartProperties>>,
  /// Defines the Transform2D Class.
  #[sdk(child(qname = "a14:xfrm"))]
  pub transform2_d: Option<std::boxed::Box<Transform2D>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "a14:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the ShadowObscured Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:shadowObscured")]
pub struct ShadowObscured {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the HiddenFillProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:hiddenFill")]
pub struct HiddenFillProperties {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, boxed, qname = "a:solidFill"),
            child(variant = GradientFill, boxed, qname = "a:gradFill"),
            child(variant = BlipFill, boxed, qname = "a:blipFill"),
            child(variant = PatternFill, boxed, qname = "a:pattFill"),
            empty_child(variant = GroupFill, qname = "a:grpFill")
        )
    )]
  pub hidden_fill_properties_choice: Option<HiddenFillPropertiesChoice>,
}
/// Defines the HiddenLineProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:hiddenLine")]
pub struct HiddenLineProperties {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// line width
  #[sdk(attr(qname = ":w"))]
  #[sdk(number_range(range = 0..= 20116800))]
  pub width: Option<crate::simple_type::Int32Value>,
  /// line cap
  #[sdk(attr(qname = ":cap"))]
  #[sdk(string_format(kind = "token"))]
  pub cap_type: Option<crate::schemas::a::LineCapValues>,
  /// compound line type
  #[sdk(attr(qname = ":cmpd"))]
  #[sdk(string_format(kind = "token"))]
  pub compound_line_type: Option<crate::schemas::a::CompoundLineValues>,
  /// pen alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(kind = "token"))]
  pub alignment: Option<crate::schemas::a::PenAlignmentValues>,
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, boxed, qname = "a:solidFill"),
            child(variant = GradientFill, boxed, qname = "a:gradFill"),
            child(variant = PatternFill, boxed, qname = "a:pattFill")
        )
    )]
  pub hidden_line_properties_choice1: Option<HiddenLinePropertiesChoice>,
  #[sdk(
        choice(
            child(variant = PresetDash, qname = "a:prstDash"),
            child(variant = CustomDash, qname = "a:custDash")
        )
    )]
  pub hidden_line_properties_choice2: Option<HiddenLinePropertiesChoice2>,
  #[sdk(
        choice(
            empty_child(variant = Round, qname = "a:round"),
            empty_child(variant = LineJoinBevel, qname = "a:bevel"),
            child(variant = Miter, qname = "a:miter")
        )
    )]
  pub hidden_line_properties_choice3: Option<HiddenLinePropertiesChoice3>,
  /// default head line end style is none.
  #[sdk(child(qname = "a:headEnd"))]
  pub head_end: Option<crate::schemas::a::HeadEnd>,
  /// default tail line end style is none.
  #[sdk(child(qname = "a:tailEnd"))]
  pub tail_end: Option<crate::schemas::a::TailEnd>,
  /// Future extensions..
  #[sdk(child(qname = "a:extLst"))]
  pub line_properties_extension_list: Option<crate::schemas::a::LinePropertiesExtensionList>,
}
/// Defines the HiddenEffectsProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:hiddenEffects")]
pub struct HiddenEffectsProperties {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  #[sdk(
        choice(
            child(variant = EffectList, boxed, qname = "a:effectLst"),
            child(variant = EffectDag, boxed, qname = "a:effectDag")
        )
    )]
  pub hidden_effects_properties_choice: Option<HiddenEffectsPropertiesChoice>,
}
/// Defines the HiddenScene3D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:hiddenScene3d")]
pub struct HiddenScene3D {
  /// Camera
  #[sdk(child(qname = "a:camera"))]
  pub camera: std::boxed::Box<crate::schemas::a::Camera>,
  /// Light Rig
  #[sdk(child(qname = "a:lightRig"))]
  pub light_rig: std::boxed::Box<crate::schemas::a::LightRig>,
  /// Backdrop Plane
  #[sdk(child(qname = "a:backdrop"))]
  pub backdrop: Option<std::boxed::Box<crate::schemas::a::Backdrop>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Defines the HiddenShape3D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:hiddenSp3d")]
pub struct HiddenShape3D {
  /// Shape Depth
  #[sdk(attr(qname = ":z"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub z: Option<crate::simple_type::Int64Value>,
  /// Extrusion Height
  #[sdk(attr(qname = ":extrusionH"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub extrusion_height: Option<crate::simple_type::Int64Value>,
  /// Contour Width
  #[sdk(attr(qname = ":contourW"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub contour_width: Option<crate::simple_type::Int64Value>,
  /// Preset Material Type
  #[sdk(attr(qname = ":prstMaterial"))]
  #[sdk(string_format(kind = "token"))]
  pub preset_material: Option<crate::schemas::a::PresetMaterialTypeValues>,
  /// Top Bevel
  #[sdk(child(qname = "a:bevelT"))]
  pub bevel_top: Option<crate::schemas::a::BevelTop>,
  /// Bottom Bevel
  #[sdk(child(qname = "a:bevelB"))]
  pub bevel_bottom: Option<crate::schemas::a::BevelBottom>,
  /// Extrusion Color
  #[sdk(child(qname = "a:extrusionClr"))]
  pub extrusion_color: Option<std::boxed::Box<crate::schemas::a::ExtrusionColor>>,
  /// Contour Color
  #[sdk(child(qname = "a:contourClr"))]
  pub contour_color: Option<std::boxed::Box<crate::schemas::a::ContourColor>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Defines the ImageProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:imgProps")]
pub struct ImageProperties {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Defines the ImageLayer Class.
  #[sdk(child(qname = "a14:imgLayer"))]
  pub image_layer: ImageLayer,
}
/// Defines the UseLocalDpi Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:useLocalDpi")]
pub struct UseLocalDpi {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the TextMath Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:m")]
pub struct TextMath {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_other_children: Vec<std::boxed::Box<[u8]>>,
}
/// Defines the OfficeArtExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:extLst")]
pub struct OfficeArtExtensionList {
  /// Extension.
  #[sdk(child(qname = "a:ext"))]
  pub extension: Vec<crate::schemas::a::Extension>,
}
/// Defines the ContentPartLocks Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:cpLocks")]
pub struct ContentPartLocks {
  /// Disallow Shape Grouping
  #[sdk(attr(qname = ":noGrp"))]
  pub no_grouping: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Selection
  #[sdk(attr(qname = ":noSelect"))]
  pub no_selection: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Rotation
  #[sdk(attr(qname = ":noRot"))]
  pub no_rotation: Option<crate::simple_type::BooleanValue>,
  /// Disallow Aspect Ratio Change
  #[sdk(attr(qname = ":noChangeAspect"))]
  pub no_change_aspect: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Movement
  #[sdk(attr(qname = ":noMove"))]
  pub no_move: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Resize
  #[sdk(attr(qname = ":noResize"))]
  pub no_resize: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Point Editing
  #[sdk(attr(qname = ":noEditPoints"))]
  pub no_edit_points: Option<crate::simple_type::BooleanValue>,
  /// Disallow Showing Adjust Handles
  #[sdk(attr(qname = ":noAdjustHandles"))]
  pub no_adjust_handles: Option<crate::simple_type::BooleanValue>,
  /// Disallow Arrowhead Changes
  #[sdk(attr(qname = ":noChangeArrowheads"))]
  pub no_change_arrowheads: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Type Change
  #[sdk(attr(qname = ":noChangeShapeType"))]
  pub no_change_shape_type: Option<crate::simple_type::BooleanValue>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "a14:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the ForegroundMark Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:foregroundMark")]
pub struct ForegroundMark {
  /// x1
  #[sdk(attr(qname = ":x1"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub first_x_coordinate: crate::simple_type::Int32Value,
  /// y1
  #[sdk(attr(qname = ":y1"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub first_y_coordinate: crate::simple_type::Int32Value,
  /// x2
  #[sdk(attr(qname = ":x2"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub second_x_coordinate: crate::simple_type::Int32Value,
  /// y2
  #[sdk(attr(qname = ":y2"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub second_y_coordinate: crate::simple_type::Int32Value,
}
/// Defines the BackgroundMark Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:backgroundMark")]
pub struct BackgroundMark {
  /// x1
  #[sdk(attr(qname = ":x1"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub first_x_coordinate: crate::simple_type::Int32Value,
  /// y1
  #[sdk(attr(qname = ":y1"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub first_y_coordinate: crate::simple_type::Int32Value,
  /// x2
  #[sdk(attr(qname = ":x2"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub second_x_coordinate: crate::simple_type::Int32Value,
  /// y2
  #[sdk(attr(qname = ":y2"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub second_y_coordinate: crate::simple_type::Int32Value,
}
/// Defines the ArtisticBlur Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:artisticBlur")]
pub struct ArtisticBlur {
  /// radius
  #[sdk(attr(qname = ":radius"))]
  #[sdk(number_range(range = 0..= 100))]
  pub radius: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticCement Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:artisticCement")]
pub struct ArtisticCement {
  /// trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// crackSpacing
  #[sdk(attr(qname = ":crackSpacing"))]
  #[sdk(number_range(range = 0..= 100))]
  pub crack_spacing: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticChalkSketch Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:artisticChalkSketch")]
pub struct ArtisticChalkSketch {
  /// trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// pressure
  #[sdk(attr(qname = ":pressure"))]
  #[sdk(number_range(range = 0..= 4))]
  pub pressure: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticCrisscrossEtching Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:artisticCrisscrossEtching")]
pub struct ArtisticCrisscrossEtching {
  /// trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// pressure
  #[sdk(attr(qname = ":pressure"))]
  #[sdk(number_range(range = 0..= 100))]
  pub pressure: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticCutout Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:artisticCutout")]
pub struct ArtisticCutout {
  /// trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// numberOfShades
  #[sdk(attr(qname = ":numberOfShades"))]
  #[sdk(number_range(range = 0..= 6))]
  pub number_of_shades: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticFilmGrain Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:artisticFilmGrain")]
pub struct ArtisticFilmGrain {
  /// trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// grainSize
  #[sdk(attr(qname = ":grainSize"))]
  #[sdk(number_range(range = 0..= 100))]
  pub grain_size: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticGlass Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:artisticGlass")]
pub struct ArtisticGlass {
  /// trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// scaling
  #[sdk(attr(qname = ":scaling"))]
  #[sdk(number_range(range = 0..= 100))]
  pub scaling: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticGlowDiffused Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:artisticGlowDiffused")]
pub struct ArtisticGlowDiffused {
  /// trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// intensity
  #[sdk(attr(qname = ":intensity"))]
  #[sdk(number_range(range = 0..= 10))]
  pub intensity: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticGlowEdges Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:artisticGlowEdges")]
pub struct ArtisticGlowEdges {
  /// trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// smoothness
  #[sdk(attr(qname = ":smoothness"))]
  #[sdk(number_range(range = 0..= 10))]
  pub smoothness: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticLightScreen Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:artisticLightScreen")]
pub struct ArtisticLightScreen {
  /// trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// gridSize
  #[sdk(attr(qname = ":gridSize"))]
  #[sdk(number_range(range = 0..= 10))]
  pub grid_size: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticLineDrawing Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:artisticLineDrawing")]
pub struct ArtisticLineDrawing {
  /// trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// pencilSize
  #[sdk(attr(qname = ":pencilSize"))]
  #[sdk(number_range(range = 0..= 100))]
  pub pencil_size: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticMarker Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:artisticMarker")]
pub struct ArtisticMarker {
  /// trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// size
  #[sdk(attr(qname = ":size"))]
  #[sdk(number_range(range = 0..= 100))]
  pub size: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticMosaicBubbles Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:artisticMosiaicBubbles")]
pub struct ArtisticMosaicBubbles {
  /// trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// pressure
  #[sdk(attr(qname = ":pressure"))]
  #[sdk(number_range(range = 0..= 100))]
  pub pressure: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticPaintStrokes Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:artisticPaintStrokes")]
pub struct ArtisticPaintStrokes {
  /// trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// intensity
  #[sdk(attr(qname = ":intensity"))]
  #[sdk(number_range(range = 0..= 10))]
  pub intensity: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticPaintBrush Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:artisticPaintBrush")]
pub struct ArtisticPaintBrush {
  /// trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// brushSize
  #[sdk(attr(qname = ":brushSize"))]
  #[sdk(number_range(range = 0..= 10))]
  pub brush_size: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticPastelsSmooth Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:artisticPastelsSmooth")]
pub struct ArtisticPastelsSmooth {
  /// trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// scaling
  #[sdk(attr(qname = ":scaling"))]
  #[sdk(number_range(range = 0..= 100))]
  pub brush_size: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticPencilGrayscale Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:artisticPencilGrayscale")]
pub struct ArtisticPencilGrayscale {
  /// trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// pencilSize
  #[sdk(attr(qname = ":pencilSize"))]
  #[sdk(number_range(range = 0..= 100))]
  pub brush_size: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticPencilSketch Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:artisticPencilSketch")]
pub struct ArtisticPencilSketch {
  /// trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// pressure
  #[sdk(attr(qname = ":pressure"))]
  #[sdk(number_range(range = 0..= 100))]
  pub pressure: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticPhotocopy Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:artisticPhotocopy")]
pub struct ArtisticPhotocopy {
  /// trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// detail
  #[sdk(attr(qname = ":detail"))]
  #[sdk(number_range(range = 0..= 10))]
  pub detail: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticPlasticWrap Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:artisticPlasticWrap")]
pub struct ArtisticPlasticWrap {
  /// trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// smoothness
  #[sdk(attr(qname = ":smoothness"))]
  #[sdk(number_range(range = 0..= 10))]
  pub smoothness: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticTexturizer Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:artisticTexturizer")]
pub struct ArtisticTexturizer {
  /// trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// scaling
  #[sdk(attr(qname = ":scaling"))]
  #[sdk(number_range(range = 0..= 100))]
  pub scaling: Option<crate::simple_type::Int32Value>,
}
/// Defines the ArtisticWatercolorSponge Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:artisticWatercolorSponge")]
pub struct ArtisticWatercolorSponge {
  /// trans
  #[sdk(attr(qname = ":trans"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub transparancy: Option<crate::simple_type::Int32Value>,
  /// brushSize
  #[sdk(attr(qname = ":brushSize"))]
  #[sdk(number_range(range = 0..= 10))]
  pub brush_size: Option<crate::simple_type::Int32Value>,
}
/// Defines the BackgroundRemoval Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:backgroundRemoval")]
pub struct BackgroundRemoval {
  /// t
  #[sdk(attr(qname = ":t"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub marquee_top: crate::simple_type::Int32Value,
  /// b
  #[sdk(attr(qname = ":b"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub marquee_bottom: crate::simple_type::Int32Value,
  /// l
  #[sdk(attr(qname = ":l"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub marquee_left: crate::simple_type::Int32Value,
  /// r
  #[sdk(attr(qname = ":r"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub marquee_right: crate::simple_type::Int32Value,
  /// Defines the ForegroundMark Class.
  #[sdk(child(qname = "a14:foregroundMark"))]
  pub foreground_mark: Vec<ForegroundMark>,
  /// Defines the BackgroundMark Class.
  #[sdk(child(qname = "a14:backgroundMark"))]
  pub background_mark: Vec<BackgroundMark>,
}
/// Defines the BrightnessContrast Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:brightnessContrast")]
pub struct BrightnessContrast {
  /// bright
  #[sdk(attr(qname = ":bright"))]
  #[sdk(number_range(range = -100000..= 100000))]
  pub bright: Option<crate::simple_type::Int32Value>,
  /// contrast
  #[sdk(attr(qname = ":contrast"))]
  #[sdk(number_range(range = -100000..= 100000))]
  pub contrast: Option<crate::simple_type::Int32Value>,
}
/// Defines the ColorTemperature Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:colorTemperature")]
pub struct ColorTemperature {
  /// colorTemp
  #[sdk(attr(qname = ":colorTemp"))]
  #[sdk(number_range(range = 1500..= 11500))]
  pub color_temperature_value: Option<crate::simple_type::Int32Value>,
}
/// Defines the Saturation Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:saturation")]
pub struct Saturation {
  /// sat
  #[sdk(attr(qname = ":sat"))]
  #[sdk(number_range(range = 0..= 400000))]
  pub saturation_amount: Option<crate::simple_type::Int32Value>,
}
/// Defines the SharpenSoften Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:sharpenSoften")]
pub struct SharpenSoften {
  /// amount
  #[sdk(attr(qname = ":amount"))]
  #[sdk(number_range(range = -100000..= 100000))]
  pub amount: Option<crate::simple_type::Int32Value>,
}
/// Defines the ImageEffect Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:imgEffect")]
pub struct ImageEffect {
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  #[sdk(
        choice(
            child(variant = ArtisticBlur, qname = "a14:artisticBlur"),
            child(variant = ArtisticCement, qname = "a14:artisticCement"),
            child(variant = ArtisticChalkSketch, qname = "a14:artisticChalkSketch"),
            child(
                variant = ArtisticCrisscrossEtching,
                qname = "a14:artisticCrisscrossEtching"
            ),
            child(variant = ArtisticCutout, qname = "a14:artisticCutout"),
            child(variant = ArtisticFilmGrain, qname = "a14:artisticFilmGrain"),
            child(variant = ArtisticGlass, qname = "a14:artisticGlass"),
            child(variant = ArtisticGlowDiffused, qname = "a14:artisticGlowDiffused"),
            child(variant = ArtisticGlowEdges, qname = "a14:artisticGlowEdges"),
            child(variant = ArtisticLightScreen, qname = "a14:artisticLightScreen"),
            child(variant = ArtisticLineDrawing, qname = "a14:artisticLineDrawing"),
            child(variant = ArtisticMarker, qname = "a14:artisticMarker"),
            child(variant = ArtisticMosaicBubbles, qname = "a14:artisticMosiaicBubbles"),
            child(variant = ArtisticPaintStrokes, qname = "a14:artisticPaintStrokes"),
            child(variant = ArtisticPaintBrush, qname = "a14:artisticPaintBrush"),
            child(variant = ArtisticPastelsSmooth, qname = "a14:artisticPastelsSmooth"),
            child(
                variant = ArtisticPencilGrayscale,
                qname = "a14:artisticPencilGrayscale"
            ),
            child(variant = ArtisticPencilSketch, qname = "a14:artisticPencilSketch"),
            child(variant = ArtisticPhotocopy, qname = "a14:artisticPhotocopy"),
            child(variant = ArtisticPlasticWrap, qname = "a14:artisticPlasticWrap"),
            child(variant = ArtisticTexturizer, qname = "a14:artisticTexturizer"),
            child(
                variant = ArtisticWatercolorSponge,
                qname = "a14:artisticWatercolorSponge"
            ),
            child(variant = BackgroundRemoval, qname = "a14:backgroundRemoval"),
            child(variant = BrightnessContrast, qname = "a14:brightnessContrast"),
            child(variant = ColorTemperature, qname = "a14:colorTemperature"),
            child(variant = Saturation, qname = "a14:saturation"),
            child(variant = SharpenSoften, qname = "a14:sharpenSoften")
        )
    )]
  pub image_effect_choice: Option<ImageEffectChoice>,
}
/// Defines the ImageLayer Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:imgLayer")]
pub struct ImageLayer {
  /// embed
  #[sdk(attr(qname = "r:embed"))]
  pub embed: Option<crate::simple_type::StringValue>,
  /// Defines the ImageEffect Class.
  #[sdk(child(qname = "a14:imgEffect"))]
  pub image_effect: Vec<ImageEffect>,
}
/// Defines the NonVisualDrawingProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:cNvPr")]
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
/// Defines the NonVisualInkContentPartProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:cNvContentPartPr")]
pub struct NonVisualInkContentPartProperties {
  /// isComment
  #[sdk(attr(qname = ":isComment"))]
  pub is_comment: Option<crate::simple_type::BooleanValue>,
  /// Defines the ContentPartLocks Class.
  #[sdk(child(qname = "a14:cpLocks"))]
  pub content_part_locks: Option<std::boxed::Box<ContentPartLocks>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "a14:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the NonVisualContentPartProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:nvContentPartPr")]
pub struct NonVisualContentPartProperties {
  /// Defines the NonVisualDrawingProperties Class.
  #[sdk(child(qname = "a14:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Defines the NonVisualInkContentPartProperties Class.
  #[sdk(child(qname = "a14:cNvContentPartPr"))]
  pub non_visual_ink_content_part_properties:
    Option<std::boxed::Box<NonVisualInkContentPartProperties>>,
}
/// Defines the Transform2D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a14:xfrm")]
pub struct Transform2D {
  /// Rotation
  #[sdk(attr(qname = ":rot"))]
  pub rotation: Option<crate::simple_type::Int32Value>,
  /// Horizontal Flip
  #[sdk(attr(qname = ":flipH"))]
  pub horizontal_flip: Option<crate::simple_type::BooleanValue>,
  /// Vertical Flip
  #[sdk(attr(qname = ":flipV"))]
  pub vertical_flip: Option<crate::simple_type::BooleanValue>,
  /// Offset
  #[sdk(child(qname = "a:off"))]
  pub offset: Option<crate::schemas::a::Offset>,
  /// Extents
  #[sdk(child(qname = "a:ext"))]
  pub extents: Option<crate::schemas::a::Extents>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum HiddenFillPropertiesChoice {
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
pub enum HiddenLinePropertiesChoice {
  /// Defines the NoFill Class.
  NoFill(crate::schemas::a::NoFill),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<crate::schemas::a::SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<crate::schemas::a::GradientFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<crate::schemas::a::PatternFill>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum HiddenLinePropertiesChoice2 {
  /// Preset Dash.
  PresetDash(crate::schemas::a::PresetDash),
  /// Custom Dash.
  CustomDash(crate::schemas::a::CustomDash),
}
#[derive(Clone, Debug, PartialEq)]
pub enum HiddenLinePropertiesChoice3 {
  /// Round Line Join.
  Round,
  /// Line Join Bevel.
  LineJoinBevel,
  /// Miter Line Join.
  Miter(crate::schemas::a::Miter),
}
#[derive(Clone, Debug, PartialEq)]
pub enum HiddenEffectsPropertiesChoice {
  /// Effect Container.
  EffectList(std::boxed::Box<crate::schemas::a::EffectList>),
  /// Effect Container.
  EffectDag(std::boxed::Box<crate::schemas::a::EffectDag>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ImageEffectChoice {
  /// Defines the ArtisticBlur Class.
  ArtisticBlur(ArtisticBlur),
  /// Defines the ArtisticCement Class.
  ArtisticCement(ArtisticCement),
  /// Defines the ArtisticChalkSketch Class.
  ArtisticChalkSketch(ArtisticChalkSketch),
  /// Defines the ArtisticCrisscrossEtching Class.
  ArtisticCrisscrossEtching(ArtisticCrisscrossEtching),
  /// Defines the ArtisticCutout Class.
  ArtisticCutout(ArtisticCutout),
  /// Defines the ArtisticFilmGrain Class.
  ArtisticFilmGrain(ArtisticFilmGrain),
  /// Defines the ArtisticGlass Class.
  ArtisticGlass(ArtisticGlass),
  /// Defines the ArtisticGlowDiffused Class.
  ArtisticGlowDiffused(ArtisticGlowDiffused),
  /// Defines the ArtisticGlowEdges Class.
  ArtisticGlowEdges(ArtisticGlowEdges),
  /// Defines the ArtisticLightScreen Class.
  ArtisticLightScreen(ArtisticLightScreen),
  /// Defines the ArtisticLineDrawing Class.
  ArtisticLineDrawing(ArtisticLineDrawing),
  /// Defines the ArtisticMarker Class.
  ArtisticMarker(ArtisticMarker),
  /// Defines the ArtisticMosaicBubbles Class.
  ArtisticMosaicBubbles(ArtisticMosaicBubbles),
  /// Defines the ArtisticPaintStrokes Class.
  ArtisticPaintStrokes(ArtisticPaintStrokes),
  /// Defines the ArtisticPaintBrush Class.
  ArtisticPaintBrush(ArtisticPaintBrush),
  /// Defines the ArtisticPastelsSmooth Class.
  ArtisticPastelsSmooth(ArtisticPastelsSmooth),
  /// Defines the ArtisticPencilGrayscale Class.
  ArtisticPencilGrayscale(ArtisticPencilGrayscale),
  /// Defines the ArtisticPencilSketch Class.
  ArtisticPencilSketch(ArtisticPencilSketch),
  /// Defines the ArtisticPhotocopy Class.
  ArtisticPhotocopy(ArtisticPhotocopy),
  /// Defines the ArtisticPlasticWrap Class.
  ArtisticPlasticWrap(ArtisticPlasticWrap),
  /// Defines the ArtisticTexturizer Class.
  ArtisticTexturizer(ArtisticTexturizer),
  /// Defines the ArtisticWatercolorSponge Class.
  ArtisticWatercolorSponge(ArtisticWatercolorSponge),
  /// Defines the BackgroundRemoval Class.
  BackgroundRemoval(BackgroundRemoval),
  /// Defines the BrightnessContrast Class.
  BrightnessContrast(BrightnessContrast),
  /// Defines the ColorTemperature Class.
  ColorTemperature(ColorTemperature),
  /// Defines the Saturation Class.
  Saturation(Saturation),
  /// Defines the SharpenSoften Class.
  SharpenSoften(SharpenSoften),
}
