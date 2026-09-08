use std::io::Cursor;

use image::codecs::png::PngEncoder;
use image::{ColorType, ImageEncoder};
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use ooxmlsdk::units::DrawingmlPercentageValue;

use crate::model::RgbColor;
use crate::render::emf_wmf;

use super::color_math::HslColor;

mod gaussian;

#[cfg(test)]
mod raster_tests;

/// Direct2D's Balanced Gaussian tier width, expressed as a device-kernel
/// radius. Exact Word WPS controls on both sides of successive boundaries pin
/// this to 3.84 pixels and keep an exact multiple in the preceding tier.
pub(crate) const DIRECT2D_BALANCED_BLUR_PRESCALE_STEP_PX: f32 = 3.84;

/// Resolves Direct2D Balanced's integer pre-scale tier from its public
/// 96-DPI kernel radius. Office controls on both sides of the first three
/// boundaries prove that an exactly representable multiple remains in the
/// preceding tier; stabilize the float round trip before applying `ceil`.
pub(crate) fn direct2d_balanced_blur_prescale_divisor(public_radius_px: f32) -> u32 {
  if !public_radius_px.is_finite() || public_radius_px <= f32::EPSILON {
    return 1;
  }
  let raw_tier = public_radius_px / DIRECT2D_BALANCED_BLUR_PRESCALE_STEP_PX;
  let nearest_integer = raw_tier.round();
  let round_trip_tolerance = f32::EPSILON * raw_tier.abs().max(1.0) * 2.0;
  let stable_tier = if (raw_tier - nearest_integer).abs() <= round_trip_tolerance {
    nearest_integer
  } else {
    raw_tier
  };
  stable_tier.ceil().max(1.0) as u32
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ImageEffect {
  AlphaBiLevel(u8),
  AlphaCeiling,
  AlphaFloor,
  AlphaInverse(Option<RgbColor>),
  /// Multiplies the input alpha by the alpha produced by the nested effect
  /// container. Unlike `alphaModFix`, this is not a constant percentage.
  AlphaModulate(ImageEffectContainer),
  AlphaModulateFixed(f32),
  AlphaOutset(f32),
  AlphaReplace(u8),
  BiLevel(u8),
  Blur {
    radius_px: f32,
    grow_bounds: bool,
  },
  Blend {
    container: ImageEffectContainer,
    blend_mode: ImageEffectBlendMode,
  },
  ColorChange(ColorChangeEffect),
  ColorReplacement(RgbColor),
  Duotone(RgbColor, RgbColor),
  Grayscale,
  Hsl {
    hue_degrees: f32,
    saturation_offset: f32,
    luminance_offset: f32,
  },
  Luminance {
    brightness: Option<i32>,
    contrast: Option<i32>,
  },
  Tint {
    hue_degrees: f32,
    amount: f32,
  },
  FillOverlay {
    fill: ImageEffectFill,
    blend_mode: ImageEffectBlendMode,
  },
  Fill(ImageEffectFill),
  Glow {
    radius_px: f32,
    /// Scales the radius used by the raster kernel without changing the
    /// caller's coordinate system.
    raster_length_scale: f32,
    /// Scales the authored radius when reserving the filter output range.
    ///
    /// Most DrawingML hosts use the authored radius directly. Word run-level
    /// effects normalize it by the owning font size.
    bounds_radius_scale: f32,
    /// Adds terminal transparent support to the filter output range without
    /// widening the sampled kernel.
    bounds_radius_offset_px: f32,
    spread_ratio: f32,
    spread_kernel: GlowSpreadKernel,
    spread_radius_rounding: GlowSpreadRadiusRounding,
    blur_kernel: GlowBlurKernel,
    color: ResolvedEffectColor,
  },
  Identity,
  InnerShadow {
    blur_radius_px: f32,
    distance_px: f32,
    direction_degrees: f32,
    color: ResolvedEffectColor,
  },
  OuterShadow {
    blur_radius_px: f32,
    distance_px: f32,
    /// Scales only the sampled blur kernel.
    raster_length_scale: f32,
    /// Scales the polar distance in both geometry and raster operations.
    distance_length_scale: f32,
    /// Scales only the authored blur radius used to reserve output bounds.
    bounds_radius_scale: f32,
    /// Adds terminal transparent support to the output range without
    /// widening the sampled blur kernel.
    bounds_radius_offset_px: f32,
    blur_kernel: ShadowBlurKernel,
    direction_degrees: f32,
    distance_mode: ShadowDistanceMode,
    transform: ImageEffectTransform,
    alignment: (f32, f32),
    rotate_with_shape: bool,
    color: ResolvedEffectColor,
  },
  Reflection(ImageReflectionEffect),
  SourceReference(ImageEffectSourceReference),
  RelativeOffset {
    offset_x: f32,
    offset_y: f32,
  },
  SoftEdge(f32),
  Transform(ImageEffectTransform),
  Container(ImageEffectContainer),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum GlowSpreadKernel {
  Square,
  Disk,
  /// Word's flat run-level realization of the ECMA alphaOutset graph. Exact
  /// Office controls pin its positive, integer device support to a centered
  /// radial footprint; WPS and projected static-3-D sources retain their
  /// separately calibrated internal alpha-blur realization.
  WordFlatAlphaOutset,
  /// Office's positive `alphaOutset` graph: alpha ceiling, the internal
  /// three-box alpha blur, then a second alpha ceiling.
  AlphaOutset,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum GlowSpreadRadiusRounding {
  Outward,
  Inward,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum GlowBlurKernel {
  Gaussian,
  /// A standalone WordprocessingShape glow uses a finite Gaussian with
  /// `sigma = R / 6`, support `floor(R / 2)`, and an A8 intermediate between
  /// its horizontal and vertical passes.
  WordShapeGaussian,
  /// A projected Word run uses the same small-radius WPS profile, but its
  /// first Balanced large-radius tier has a separately observable effective
  /// alphaOutset/Gaussian split on the fixed 200-DPI surface.
  WordStatic3dGaussian,
  /// Word's WPG glow uses the public DrawingML blur stage after alphaOutset.
  /// Its finite Gaussian is separable, materializes an A8 horizontal pass,
  /// then runs the vertical pass over those quantized samples.
  WordGroupGaussian,
  // Retained only as a LibreOffice comparison control in unit tests.
  #[cfg(test)]
  Stack,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ShadowBlurKernel {
  /// DrawingML shape shadows use Direct2D's Gaussian shadow contract, where
  /// the authored blur radius is three standard deviations and the input
  /// alpha coverage is preserved verbatim.
  Direct2dGaussian,
  /// Word run-level fixed output realizes Direct2D's Balanced shadow profile
  /// after the W14 text-plane affine. The public 96-DPI radius selects an
  /// integer pre-scale tier before the effect is mapped to its 200-DPI work
  /// surface; retain that resolved tier here so later raster scaling cannot
  /// select a different profile. Keep this distinct from ordinary DrawingML
  /// shape shadows, which expose Direct2D's unoptimized finite Gaussian
  /// followed by its documented 2-D affine stage.
  WordTextBalanced { prescale_divisor: u32 },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ShadowDistanceMode {
  /// DrawingML's ordinary outer shadow translates the already transformed
  /// shadow image by the polar distance vector.
  PostTransformOffset,
  /// W14 run shadow distance belongs to the authored text plane and is
  /// transformed together with that plane's scale/skew before any host 3-D
  /// camera projection.
  PreTransformOffset,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ImageEffectSourceReference {
  Fill,
  Line,
  FillLine,
  Children,
  /// A caller-realized coverage source, independent of the painted input.
  /// This is an internal graph binding, not an authored DrawingML source name.
  EffectMask,
  /// Unlit reflected material, independent of the root effect coverage.
  ReflectionPaint,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct ImageEffectSourceImages<'a> {
  pub(crate) fill: Option<&'a image::RgbaImage>,
  pub(crate) line: Option<&'a image::RgbaImage>,
  pub(crate) fill_line: Option<&'a image::RgbaImage>,
  pub(crate) children: Option<&'a image::RgbaImage>,
  pub(crate) effect_mask: Option<&'a image::RgbaImage>,
  pub(crate) reflection_paint: Option<&'a image::RgbaImage>,
  /// Continuous source rectangles on this raster canvas, not tight alpha boxes.
  pub(crate) bounds: ImageEffectSourcePixelBounds,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct ImageEffectSourcePixelBounds {
  pub(crate) fill: Option<ImageEffectContentBounds>,
  pub(crate) line: Option<ImageEffectContentBounds>,
  pub(crate) fill_line: Option<ImageEffectContentBounds>,
  pub(crate) children: Option<ImageEffectContentBounds>,
  pub(crate) effect_mask: Option<ImageEffectContentBounds>,
  pub(crate) reflection_paint: Option<ImageEffectContentBounds>,
}

/// Normalize units only at the geometry API boundary. In particular, the
/// pixel evaluator must not discard source rectangles or treat pixels as points.
#[derive(Clone, Copy)]
enum EffectSourceBounds {
  Points(ImageEffectSourceBounds),
  Pixels(ImageEffectSourcePixelBounds),
}

impl From<ImageEffectSourceBounds> for EffectSourceBounds {
  fn from(value: ImageEffectSourceBounds) -> Self {
    Self::Points(value)
  }
}

impl From<ImageEffectSourcePixelBounds> for EffectSourceBounds {
  fn from(value: ImageEffectSourcePixelBounds) -> Self {
    Self::Pixels(value)
  }
}

impl EffectSourceBounds {
  fn get(self, reference: ImageEffectSourceReference) -> Option<PixelBounds> {
    match self {
      Self::Points(sources) => {
        let bounds = match reference {
          ImageEffectSourceReference::Fill => sources.fill,
          ImageEffectSourceReference::Line => sources.line,
          ImageEffectSourceReference::FillLine => sources.fill_line,
          ImageEffectSourceReference::Children => sources.children,
          ImageEffectSourceReference::EffectMask => sources.effect_mask,
          ImageEffectSourceReference::ReflectionPaint => sources.reflection_paint,
        }?;
        Some(PixelBounds {
          left: bounds.left_pt * (96.0 / 72.0),
          top: bounds.top_pt * (96.0 / 72.0),
          right: bounds.right_pt * (96.0 / 72.0),
          bottom: bounds.bottom_pt * (96.0 / 72.0),
        })
      }
      Self::Pixels(sources) => {
        let bounds = match reference {
          ImageEffectSourceReference::Fill => sources.fill,
          ImageEffectSourceReference::Line => sources.line,
          ImageEffectSourceReference::FillLine => sources.fill_line,
          ImageEffectSourceReference::Children => sources.children,
          ImageEffectSourceReference::EffectMask => sources.effect_mask,
          ImageEffectSourceReference::ReflectionPaint => sources.reflection_paint,
        }?;
        Some(PixelBounds {
          left: bounds.left_px,
          top: bounds.top_px,
          right: bounds.left_px + bounds.width_px,
          bottom: bounds.top_px + bounds.height_px,
        })
      }
    }
  }
}

/// Independently realized source rectangles in the caller's point space.
/// An omitted rectangle retains the caller's conservative root allocation;
/// it does not assert that the corresponding pixel source is absent.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct ImageEffectSourceBounds {
  pub(crate) fill: Option<EffectOutputBounds>,
  pub(crate) line: Option<EffectOutputBounds>,
  pub(crate) fill_line: Option<EffectOutputBounds>,
  pub(crate) children: Option<EffectOutputBounds>,
  pub(crate) effect_mask: Option<EffectOutputBounds>,
  pub(crate) reflection_paint: Option<EffectOutputBounds>,
}

/// Corrects continuous effect lengths from the requested raster density to
/// the actual per-axis pitch of the materialized effect bitmap.
///
/// Word rounds the bitmap width and height independently, so the PDF matrix
/// can map the two axes at slightly different pixels-per-point values. Each
/// effect profile owns whether this scale applies to a continuous sigma or an
/// alpha-outset radius; discrete kernel support is independently quantized.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct AlphaOutsetSurfaceScale {
  pub(crate) x: f32,
  pub(crate) y: f32,
}

/// Maps a logical effect canvas to its independently allocated texture.
/// Geometry and authored effect lengths remain in the logical canvas until
/// a raster operation consumes them. This is not a blur-profile correction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct EffectRasterScale {
  pub(crate) x: f32,
  pub(crate) y: f32,
}

impl Default for EffectRasterScale {
  fn default() -> Self {
    Self { x: 1.0, y: 1.0 }
  }
}

impl EffectRasterScale {
  fn is_valid(self) -> bool {
    self.x.is_finite() && self.y.is_finite() && self.x > 0.0 && self.y > 0.0
  }

  fn bounds(self, bounds: PixelBounds) -> PixelBounds {
    PixelBounds {
      left: bounds.left * self.x,
      top: bounds.top * self.y,
      right: bounds.right * self.x,
      bottom: bounds.bottom * self.y,
    }
  }

  /// Conjugate a logical affine by the canvas-to-texture axis scale.
  /// Multiplying just its translation loses skew and directional ownership.
  fn transform(self, transform: ImageEffectTransform) -> ImageEffectTransform {
    if self == Self::default() {
      return transform;
    }
    ImageEffectTransform {
      skew_x: transform.skew_x * (self.x / self.y),
      skew_y: transform.skew_y * (self.y / self.x),
      shift_x_px: transform.shift_x_px * self.x,
      shift_y_px: transform.shift_y_px * self.y,
      ..transform
    }
  }
}

#[derive(Clone, Copy, Debug)]
struct EffectRasterContext {
  scale: EffectRasterScale,
  alpha_outset: AlphaOutsetSurfaceScale,
}

impl From<AlphaOutsetSurfaceScale> for EffectRasterContext {
  fn from(alpha_outset: AlphaOutsetSurfaceScale) -> Self {
    Self {
      scale: EffectRasterScale::default(),
      alpha_outset,
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ImageEffectContentBounds {
  pub(crate) left_px: f32,
  pub(crate) top_px: f32,
  pub(crate) width_px: f32,
  pub(crate) height_px: f32,
}

impl Default for AlphaOutsetSurfaceScale {
  fn default() -> Self {
    Self { x: 1.0, y: 1.0 }
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ImageEffectSourceGeometry {
  pub(crate) paint_left_px: f32,
  pub(crate) paint_top_px: f32,
  pub(crate) paint_width_px: f32,
  pub(crate) paint_height_px: f32,
  pub(crate) shadow_anchor_left_px: f32,
  pub(crate) shadow_anchor_top_px: f32,
  pub(crate) shadow_anchor_width_px: f32,
  pub(crate) shadow_anchor_height_px: f32,
  pub(crate) anchor_left_px: f32,
  pub(crate) anchor_top_px: f32,
  pub(crate) anchor_width_px: f32,
  pub(crate) anchor_height_px: f32,
  pub(crate) ramp_left_px: f32,
  pub(crate) ramp_top_px: f32,
  pub(crate) ramp_width_px: f32,
  pub(crate) ramp_height_px: f32,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct ImageEffectSourceRequirements {
  pub(crate) fill: bool,
  pub(crate) line: bool,
  pub(crate) fill_line: bool,
  pub(crate) children: bool,
  pub(crate) effect_mask: bool,
  pub(crate) reflection_paint: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ImageEffectContainer {
  pub(crate) kind: ImageEffectContainerKind,
  pub(crate) effects: Vec<ImageEffect>,
}

impl ImageEffectContainer {
  /// An independently realized drawable owns its continuous filter domain.
  /// Fixed-output bitmap guards remain on the caller's allocation graph;
  /// they must not enlarge this source's UV rectangle or finite alpha mask.
  pub(crate) fn without_bitmap_allocation_guards(&self) -> Self {
    fn strip(container: &mut ImageEffectContainer) {
      for effect in &mut container.effects {
        match effect {
          ImageEffect::Glow {
            bounds_radius_offset_px,
            ..
          }
          | ImageEffect::OuterShadow {
            bounds_radius_offset_px,
            ..
          } => {
            *bounds_radius_offset_px = 0.0;
          }
          ImageEffect::Reflection(reflection) => reflection.bounds_radius_offset_px = 0.0,
          ImageEffect::Container(nested)
          | ImageEffect::AlphaModulate(nested)
          | ImageEffect::Blend {
            container: nested, ..
          } => strip(nested),
          _ => {}
        }
      }
    }
    let mut source = self.clone();
    strip(&mut source);
    source
  }
}

/// Geometry of an outer-shadow branch whose affine component is an identity
/// and whose only spatial transform is the DrawingML distance/direction pair.
///
/// Such a branch can use an expanded fixed-output work surface whose final
/// crop is independent of the translated input range. Scaled and skewed
/// shadows require a more general inverse effect mapping and are deliberately
/// excluded from this profile.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SimpleOuterShadowTranslation {
  pub(crate) blur_radius_px: f32,
  pub(crate) offset_x_px: f32,
  pub(crate) offset_y_px: f32,
}

pub(crate) fn simple_outer_shadow_translation(
  container: &ImageEffectContainer,
) -> Option<SimpleOuterShadowTranslation> {
  let [
    ImageEffect::OuterShadow {
      blur_radius_px,
      distance_px,
      distance_length_scale,
      bounds_radius_scale,
      bounds_radius_offset_px,
      direction_degrees,
      transform,
      ..
    },
  ] = container.effects.as_slice()
  else {
    return None;
  };
  if transform.scale_x != 1.0
    || transform.scale_y != 1.0
    || transform.skew_x != 0.0
    || transform.skew_y != 0.0
    || transform.shift_x_px != 0.0
    || transform.shift_y_px != 0.0
  {
    return None;
  }
  let blur_radius_px = blur_radius_px.mul_add(*bounds_radius_scale, *bounds_radius_offset_px);
  let distance_px = *distance_px * *distance_length_scale;
  let direction = direction_degrees.to_radians();
  let offset_x_px = direction.cos() * distance_px;
  let offset_y_px = direction.sin() * distance_px;
  if !blur_radius_px.is_finite() || !offset_x_px.is_finite() || !offset_y_px.is_finite() {
    return None;
  }
  Some(SimpleOuterShadowTranslation {
    blur_radius_px,
    offset_x_px,
    offset_y_px,
  })
}

/// Builds a sharp outer-shadow branch while preserving the original vector
/// foreground as the final sibling. This is the normalized form used by
/// source formats, such as VML, which express a shadow as an x/y offset
/// instead of DrawingML's polar distance and direction.
pub(crate) fn offset_outer_shadow_with_identity(
  offset_x_px: f32,
  offset_y_px: f32,
  color: ResolvedEffectColor,
) -> ImageEffectContainer {
  let distance_px = offset_x_px.hypot(offset_y_px);
  let direction_degrees = offset_y_px.atan2(offset_x_px).to_degrees();
  ImageEffectContainer {
    kind: ImageEffectContainerKind::Sibling,
    effects: vec![
      ImageEffect::OuterShadow {
        blur_radius_px: 0.0,
        distance_px,
        raster_length_scale: 1.0,
        distance_length_scale: 1.0,
        bounds_radius_scale: 1.0,
        bounds_radius_offset_px: 0.0,
        blur_kernel: ShadowBlurKernel::Direct2dGaussian,
        direction_degrees,
        distance_mode: ShadowDistanceMode::PostTransformOffset,
        transform: ImageEffectTransform {
          scale_x: 1.0,
          scale_y: 1.0,
          skew_x: 0.0,
          skew_y: 0.0,
          shift_x_px: 0.0,
          shift_y_px: 0.0,
        },
        alignment: (0.5, 0.5),
        rotate_with_shape: false,
        color,
      },
      ImageEffect::Identity,
    ],
  }
}

/// Separates effect branches that are explicitly composited behind an
/// unchanged source branch.
///
/// ECMA-376 Part 1 §20.1.8.26 defines the fixed `effectLst` output as a
/// sibling container whose final branch is the main shape. In the unchanged
/// case `from_effect_list` represents that branch as `Tree(Identity)`, after
/// glow, outer shadow, and reflection. LibreOffice likewise constructs
/// picture output as `[shadow primitive, original content]`, and Apache POI
/// paints the shadow before painting the original shape. Returning only the
/// preceding sibling branches lets a host preserve vector/image foreground
/// content instead of needlessly resampling it into the effect raster.
pub(crate) fn unchanged_foreground_backdrop(
  container: &ImageEffectContainer,
) -> Option<ImageEffectContainer> {
  if container.kind != ImageEffectContainerKind::Sibling {
    return None;
  }
  let (foreground, backdrop) = container.effects.split_last()?;
  let unchanged_foreground = matches!(foreground, ImageEffect::Identity)
    || matches!(
      foreground,
      ImageEffect::Container(foreground)
        if foreground.kind == ImageEffectContainerKind::Tree
          && foreground.effects.as_slice() == [ImageEffect::Identity]
    );
  if !unchanged_foreground || backdrop.is_empty() {
    return None;
  }
  Some(ImageEffectContainer {
    kind: ImageEffectContainerKind::Sibling,
    effects: backdrop.to_vec(),
  })
}

pub(crate) fn contains_reflection(container: &ImageEffectContainer) -> bool {
  container.effects.iter().any(|effect| match effect {
    ImageEffect::Reflection(_) => true,
    ImageEffect::AlphaModulate(container)
    | ImageEffect::Container(container)
    | ImageEffect::Blend { container, .. } => contains_reflection(container),
    _ => false,
  })
}

pub(crate) fn contains_glow(container: &ImageEffectContainer) -> bool {
  container.effects.iter().any(|effect| match effect {
    ImageEffect::Glow { .. } => true,
    ImageEffect::AlphaModulate(container)
    | ImageEffect::Container(container)
    | ImageEffect::Blend { container, .. } => contains_glow(container),
    _ => false,
  })
}

/// Returns the largest blur-kernel radius used by an effect branch whose
/// output can be painted behind a separately retained foreground.
///
/// DrawingML's authored glow radius covers both the spread and blur stages;
/// LibreOffice's `GlowPrimitive2D` uses half of that radius for the actual
/// blur. Other spatial effects store their blur radius directly. Hosts use
/// this value to choose a working-surface density without confusing authored
/// output bounds with the smaller filter kernel.
pub(crate) fn effective_backdrop_blur_radius_px(container: &ImageEffectContainer) -> f32 {
  fn finite_radius(radius: f32) -> f32 {
    if radius.is_finite() {
      radius.max(0.0)
    } else {
      0.0
    }
  }

  fn visit_effect(effect: &ImageEffect) -> f32 {
    match effect {
      ImageEffect::Blur { radius_px, .. } => finite_radius(*radius_px),
      ImageEffect::Glow {
        radius_px,
        raster_length_scale,
        ..
      } => finite_radius(*radius_px * *raster_length_scale * 0.5),
      ImageEffect::InnerShadow { blur_radius_px, .. } => finite_radius(*blur_radius_px),
      ImageEffect::OuterShadow {
        blur_radius_px,
        raster_length_scale,
        ..
      } => finite_radius(*blur_radius_px * *raster_length_scale),
      ImageEffect::Reflection(reflection) => finite_radius(reflection.blur_radius_px),
      ImageEffect::SoftEdge(radius_px) => finite_radius(*radius_px),
      ImageEffect::AlphaModulate(nested)
      | ImageEffect::Container(nested)
      | ImageEffect::Blend {
        container: nested, ..
      } => effective_backdrop_blur_radius_px(nested),
      _ => 0.0,
    }
  }

  container
    .effects
    .iter()
    .map(visit_effect)
    .fold(0.0, f32::max)
}

/// Removes soft-edge effects from a DrawingML effect graph.
///
/// LibreOffice's DrawingML importer applies the Office precedence rule in
/// `oox/source/drawingml/shape.cxx`: any camera, light-rig, or shape 3-D
/// properties override `softEdge`. Effect DAGs may nest containers, so the
/// suppression has to cover the complete graph rather than only an
/// `effectLst` root.
pub(crate) fn suppress_soft_edge(container: &mut ImageEffectContainer) {
  container.effects.retain_mut(|effect| match effect {
    ImageEffect::SoftEdge(_) => false,
    ImageEffect::AlphaModulate(nested)
    | ImageEffect::Blend {
      container: nested, ..
    }
    | ImageEffect::Container(nested) => {
      suppress_soft_edge(nested);
      true
    }
    _ => true,
  });
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum ImageEffectContainerKind {
  #[default]
  Sibling,
  Tree,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ImageEffectFill {
  None,
  Solid(ResolvedEffectColor),
  Gradient {
    stops: Vec<(f32, ResolvedEffectColor)>,
    kind: ImageEffectGradientKind,
    tile: ImageEffectRelativeRect,
    flip: a::TileFlipValues,
  },
  Pattern {
    style: emfsdk::emfplus::EmfPlusHatchStyle,
    foreground: ResolvedEffectColor,
    background: ResolvedEffectColor,
    tile_px: f32,
  },
  Image(image::RgbaImage),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ImageEffectGradientKind {
  Linear(f32),
  Circle(ImageEffectRelativeRect),
  Rectangle(ImageEffectRelativeRect),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ImageEffectRelativeRect {
  left: f32,
  top: f32,
  right: f32,
  bottom: f32,
}

impl Default for ImageEffectRelativeRect {
  fn default() -> Self {
    Self {
      left: 0.0,
      top: 0.0,
      right: 0.0,
      bottom: 0.0,
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ImageEffectTransform {
  scale_x: f32,
  scale_y: f32,
  skew_x: f32,
  skew_y: f32,
  shift_x_px: f32,
  shift_y_px: f32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ReflectionDistanceMode {
  /// DrawingML applies `dist` as a translation after the authored affine
  /// reflection transform.
  PostTransformOffset,
  /// The legacy Word run lowering moves the alignment pivot by `dist`. The opacity ramp
  /// includes the interval between the original and shifted pivot, so a
  /// negative scale moves painted alpha by `(I - A) * dist` while retaining
  /// that interval in the fade coordinates.
  AlignmentPivot,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ReflectionReference {
  /// Use the geometry propagated through the preceding effect nodes.
  EffectInput,
  /// Reflect the completed paint, but retain the root text's coordinate domains.
  /// This does not select how the polar distance is applied. In particular,
  /// Word's metric-bound reflection can retain text coordinates while applying
  /// its distance after the affine transform.
  RootText,
  /// Device metrics are relative to the shared run baseline. None selects
  /// the path-geometry fallback after an empty inset run rectangle.
  WordRunMetrics {
    ascent_px: Option<f32>,
    ramp_extension_px: f32,
  },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct WordRunReflectionBinding {
  ascent_px: Option<f32>,
  distance_scale: f32,
}

impl WordRunReflectionBinding {
  pub(crate) fn new(font_size_pt: f64, metrics: Option<(f64, f64)>, dpi: f64) -> Self {
    let font = font_size_pt.floor();
    let distance_scale = match metrics {
      Some((a, d)) if a != 0.0 && font * d / a != 0.0 => font * d / a * 0.25,
      _ if font != 0.0 => font / 72.0,
      _ => 1.0,
    };
    Self {
      ascent_px: metrics.map(|(a, _)| (a * 96.0 / dpi) as f32),
      distance_scale: distance_scale as f32,
    }
  }
}

pub(crate) fn bind_wordprocessing_reflection_metrics(
  container: &mut ImageEffectContainer,
  binding: WordRunReflectionBinding,
) {
  for effect in &mut container.effects {
    match effect {
      ImageEffect::Reflection(reflection)
        if reflection.reference == ReflectionReference::RootText =>
      {
        reflection.reference = ReflectionReference::WordRunMetrics {
          ascent_px: binding.ascent_px,
          ramp_extension_px: reflection.distance_px * binding.distance_scale * 0.5,
        };
        reflection.distance_mode = ReflectionDistanceMode::PostTransformOffset;
        reflection.distance_length_scale = binding.distance_scale;
      }
      ImageEffect::Container(child)
      | ImageEffect::AlphaModulate(child)
      | ImageEffect::Blend {
        container: child, ..
      } => bind_wordprocessing_reflection_metrics(child, binding),
      _ => {}
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ImageReflectionEffect {
  blur_radius_px: f32,
  /// Scales only the sampled Gaussian kernel.
  raster_length_scale: f32,
  /// Scales the authored blur radius used to reserve output bounds.
  ///
  /// Generic DrawingML keeps this at one. Word run-level reflection uses its
  /// font-normalized text geometry scale independently of the sampled kernel.
  bounds_radius_scale: f32,
  /// Adds host-owned transparent terminal support without widening the
  /// sampled Gaussian kernel.
  bounds_radius_offset_px: f32,
  start_opacity: f32,
  start_position: f32,
  end_opacity: f32,
  end_position: f32,
  fade_direction_degrees: f32,
  distance_px: f32,
  /// Scales the polar distance in both output geometry and raster execution.
  /// Word run-level reflection uses a font-relative scale; generic DrawingML
  /// reflection keeps this at one.
  distance_length_scale: f32,
  distance_mode: ReflectionDistanceMode,
  reference: ReflectionReference,
  direction_degrees: f32,
  transform: ImageEffectTransform,
  alignment: (f32, f32),
  rotate_with_shape: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ImageEffectBlendMode {
  Over,
  Multiply,
  Screen,
  Darken,
  Lighten,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ColorChangeEffect {
  pub(crate) from: RgbColor,
  pub(crate) to: RgbColor,
  pub(crate) from_alpha: u8,
  pub(crate) to_alpha: u8,
  pub(crate) use_alpha: bool,
  pub(crate) tolerance: u8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ResolvedEffectColor {
  pub(crate) color: RgbColor,
  pub(crate) alpha: u8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct WordprocessingTextGlow {
  pub(crate) radius_px: f32,
  /// Scales the sampled alpha-outset and Gaussian kernels.
  pub(crate) raster_length_scale: f32,
  /// Scales output geometry, display bounds, and baseline participation
  /// independently of the sampled kernel.
  pub(crate) geometry_length_scale: f32,
  pub(crate) color: ResolvedEffectColor,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct WordprocessingTextShadow {
  pub(crate) blur_radius_px: f32,
  pub(crate) distance_px: f32,
  /// Scales only the sampled shadow blur kernel.
  pub(crate) raster_length_scale: f32,
  /// Scales shadow distance and output geometry independently of the kernel.
  pub(crate) geometry_length_scale: f32,
  pub(crate) direction_degrees: f32,
  pub(crate) scale_x: f32,
  pub(crate) scale_y: f32,
  pub(crate) skew_x_degrees: f32,
  pub(crate) skew_y_degrees: f32,
  pub(crate) alignment: (f32, f32),
  pub(crate) color: ResolvedEffectColor,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct WordprocessingTextReflection {
  pub(crate) blur_radius_px: f32,
  /// Scales only the sampled Gaussian kernel.
  pub(crate) raster_length_scale: f32,
  /// Scales reflection blur output bounds independently of the kernel.
  pub(crate) geometry_length_scale: f32,
  pub(crate) start_opacity: f32,
  pub(crate) start_position: f32,
  pub(crate) end_opacity: f32,
  pub(crate) end_position: f32,
  pub(crate) distance_px: f32,
  pub(crate) distance_length_scale: f32,
  pub(crate) direction_degrees: f32,
  pub(crate) fade_direction_degrees: f32,
  pub(crate) scale_x: f32,
  pub(crate) scale_y: f32,
  pub(crate) skew_x_degrees: f32,
  pub(crate) skew_y_degrees: f32,
  pub(crate) alignment: (f32, f32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum WordprocessingTextEffectHost {
  FlatText,
  Static3d,
}

/// Normalize a length on Word's 600-DPI effect plane, retaining the
/// device-space intercept instead of treating normalization as pure scaling.
/// The input and result use the effect graph's 96-DPI length units.
fn word_static_3d_effect_length_scale(length_px: f32, scale: f32, floor_pixels: f32) -> f32 {
  let floor_px = floor_pixels * crate::units::CSS_PIXELS_PER_INCH / 600.0;
  if !length_px.is_finite() || !scale.is_finite() || scale < 0.0 || length_px <= floor_px {
    return 0.0;
  }
  // GEL's shared normalizer: s * (length - floor) + floor. Glow supplies
  // one device pixel, then splits the result between outset and Gaussian;
  // shadow supplies 0.4 pixels independently for its blur and its distance.
  // Actual creator inputs and Gaussian bounds over 18/24/36/48pt establish
  // this 600-DPI domain. Line Services calls the same rule in reference units,
  // so this adjustment must not overwrite the run's logical metric lengths.
  ((f64::from(length_px) - f64::from(floor_px)) * f64::from(scale) + f64::from(floor_px)) as f32
    / length_px
}

pub(crate) fn from_wordprocessing_text_effects(
  glow: Option<WordprocessingTextGlow>,
  shadow: Option<WordprocessingTextShadow>,
  reflection: Option<WordprocessingTextReflection>,
  host: WordprocessingTextEffectHost,
) -> Option<ImageEffectContainer> {
  // Word's fixed 200-DPI text-effect surface retains two terminal samples
  // beyond standalone shadow support. Express that physical length on the
  // parser's 96-DPI coordinate baseline; raster scaling later maps it back to
  // two samples at 200 DPI and one sample at 100 DPI.
  const WORD_TEXT_EFFECT_TERMINAL_BOUNDS_PX: f32 = 2.0 * crate::units::CSS_PIXELS_PER_INCH / 200.0;
  // Standalone reflection bounds are resolved on Word's 600-DPI fixed-output
  // grid before its 200-DPI effect surface is allocated. Exact Office XPS/PDF
  // controls retain one output-grid pixel per edge; a reflected glow or shadow
  // already owns its spatial support and does not allocate this again.
  const WORD_TEXT_REFLECTION_TERMINAL_BOUNDS_PX: f32 = crate::units::CSS_PIXELS_PER_INCH / 600.0;
  // These offsets belong to bitmap allocation, not the continuous Gaussian
  // node. The bounds evaluator feeds the working surface and final crop;
  // removing its guard from a continuous-node observation changes both grids.
  // Keep the guard independent of the normalized sampled radius below.
  const WORD_TEXT_FLAT_GLOW_TERMINAL_BOUNDS_PX: f32 = crate::units::CSS_PIXELS_PER_INCH / 200.0;
  // A shadow which consumes the already padded glow surface does not allocate
  // those terminal samples a second time. The half-sample is the pixel-center
  // to output-edge ownership retained by the following filter. The complete
  // glow/shadow presence matrix separates this from both zero and the
  // standalone two-sample allowance on all four fixed-output edges.
  const WORD_TEXT_NESTED_SHADOW_TERMINAL_BOUNDS_PX: f32 =
    0.5 * crate::units::CSS_PIXELS_PER_INCH / 200.0;
  let reflection_source_has_spatial_effect = glow.is_some() || shadow.is_some();
  let mut branches = Vec::new();
  let glow_terminal_bounds_px = match host {
    WordprocessingTextEffectHost::FlatText => WORD_TEXT_FLAT_GLOW_TERMINAL_BOUNDS_PX,
    WordprocessingTextEffectHost::Static3d => WORD_TEXT_EFFECT_TERMINAL_BOUNDS_PX,
  };
  let glow_spread_kernel = match host {
    WordprocessingTextEffectHost::FlatText => GlowSpreadKernel::WordFlatAlphaOutset,
    WordprocessingTextEffectHost::Static3d => GlowSpreadKernel::AlphaOutset,
  };
  let glow_blur_kernel = match host {
    WordprocessingTextEffectHost::FlatText => GlowBlurKernel::WordShapeGaussian,
    WordprocessingTextEffectHost::Static3d => GlowBlurKernel::WordStatic3dGaussian,
  };
  let glow_effect = glow.map(|glow| ImageEffect::Glow {
    radius_px: glow.radius_px,
    raster_length_scale: match host {
      WordprocessingTextEffectHost::FlatText => glow.raster_length_scale,
      WordprocessingTextEffectHost::Static3d => {
        word_static_3d_effect_length_scale(glow.radius_px, glow.raster_length_scale, 1.0)
      }
    },
    bounds_radius_scale: match host {
      WordprocessingTextEffectHost::FlatText => glow.geometry_length_scale,
      WordprocessingTextEffectHost::Static3d => {
        word_static_3d_effect_length_scale(glow.radius_px, glow.geometry_length_scale, 1.0)
      }
    },
    bounds_radius_offset_px: glow_terminal_bounds_px,
    // The flat and projected hosts share the ECMA alphaOutset-plus-finite-
    // Gaussian graph, but not its discrete alpha-blur footprint. Exact Office
    // font-size/radius controls and shadow/no-shadow page controls pin flat
    // text to centered radial support; projected 3-D retains the WPS-style
    // internal alpha blur. Stack Blur is the direct counterexample for both.
    spread_ratio: 0.5,
    spread_kernel: glow_spread_kernel,
    spread_radius_rounding: GlowSpreadRadiusRounding::Inward,
    blur_kernel: glow_blur_kernel,
    color: glow.color,
  });
  if let Some(shadow) = shadow {
    let normalized_scale = |length, scale| match host {
      WordprocessingTextEffectHost::FlatText => scale,
      WordprocessingTextEffectHost::Static3d => {
        word_static_3d_effect_length_scale(length, scale, 0.4)
      }
    };
    let raster_length_scale = normalized_scale(shadow.blur_radius_px, shadow.raster_length_scale);
    let prescale_divisor =
      direct2d_balanced_blur_prescale_divisor(shadow.blur_radius_px * raster_length_scale);
    let shadow_effect = ImageEffect::OuterShadow {
      blur_radius_px: shadow.blur_radius_px,
      distance_px: shadow.distance_px,
      raster_length_scale,
      distance_length_scale: normalized_scale(shadow.distance_px, shadow.geometry_length_scale),
      bounds_radius_scale: normalized_scale(shadow.blur_radius_px, shadow.geometry_length_scale),
      bounds_radius_offset_px: if glow_effect.is_some() {
        WORD_TEXT_NESTED_SHADOW_TERMINAL_BOUNDS_PX
      } else {
        WORD_TEXT_EFFECT_TERMINAL_BOUNDS_PX
      },
      blur_kernel: ShadowBlurKernel::WordTextBalanced { prescale_divisor },
      direction_degrees: shadow.direction_degrees,
      distance_mode: ShadowDistanceMode::PreTransformOffset,
      transform: ImageEffectTransform {
        scale_x: shadow.scale_x,
        scale_y: shadow.scale_y,
        skew_x: shadow.skew_x_degrees.to_radians().tan(),
        skew_y: shadow.skew_y_degrees.to_radians().tan(),
        shift_x_px: 0.0,
        shift_y_px: 0.0,
      },
      alignment: shadow.alignment,
      rotate_with_shape: true,
      color: shadow.color,
    };
    if let Some(glow_effect) = glow_effect.as_ref() {
      // W14 orders glow before shadow. The shadow therefore consumes the
      // composite alpha of the glow and original glyph, producing a finite
      // core plus the low-opacity diffused tail visible in Office output.
      // Keep this shadow branch behind the independently visible glow branch.
      branches.push(ImageEffect::Container(ImageEffectContainer {
        kind: ImageEffectContainerKind::Tree,
        effects: vec![
          ImageEffect::Container(ImageEffectContainer {
            kind: ImageEffectContainerKind::Sibling,
            effects: vec![glow_effect.clone(), ImageEffect::Identity],
          }),
          shadow_effect,
        ],
      }));
    } else {
      branches.push(shadow_effect);
    }
  }
  if let Some(glow_effect) = glow_effect {
    branches.push(glow_effect);
  }
  let reflection_effect = reflection.map(|reflection| {
    ImageEffect::Reflection(ImageReflectionEffect {
      blur_radius_px: reflection.blur_radius_px,
      raster_length_scale: reflection.raster_length_scale,
      bounds_radius_scale: reflection.geometry_length_scale,
      bounds_radius_offset_px: if reflection_source_has_spatial_effect {
        0.0
      } else {
        WORD_TEXT_REFLECTION_TERMINAL_BOUNDS_PX
      },
      start_opacity: reflection.start_opacity,
      start_position: reflection.start_position,
      end_opacity: reflection.end_opacity,
      end_position: reflection.end_position,
      fade_direction_degrees: reflection.fade_direction_degrees,
      distance_px: reflection.distance_px,
      distance_length_scale: reflection.distance_length_scale,
      distance_mode: ReflectionDistanceMode::AlignmentPivot,
      reference: ReflectionReference::RootText,
      direction_degrees: reflection.direction_degrees,
      transform: ImageEffectTransform {
        scale_x: reflection.scale_x,
        scale_y: reflection.scale_y,
        skew_x: reflection.skew_x_degrees.to_radians().tan(),
        skew_y: reflection.skew_y_degrees.to_radians().tan(),
        shift_x_px: 0.0,
        shift_y_px: 0.0,
      },
      alignment: reflection.alignment,
      rotate_with_shape: true,
    })
  });
  if branches.is_empty() && reflection_effect.is_none() {
    return None;
  }
  branches.push(ImageEffect::Identity);
  if let Some(reflection_effect) = reflection_effect {
    // Word reflects the completed visible object, not the root glyph in
    // isolation. The no-3-D Office controls separate all four glow/shadow
    // combinations: the reflected image contains whichever effects are
    // present, while the same upright effect branches are emitted again in
    // front of it. Keep the outer Identity last so a host can still retain
    // its searchable/vector foreground.
    let reflected_source = ImageEffect::Container(ImageEffectContainer {
      kind: ImageEffectContainerKind::Sibling,
      effects: branches.clone(),
    });
    let reflection_branch = ImageEffect::Container(ImageEffectContainer {
      kind: ImageEffectContainerKind::Tree,
      effects: vec![reflected_source, reflection_effect],
    });
    branches.insert(0, reflection_branch);
  }
  Some(ImageEffectContainer {
    kind: ImageEffectContainerKind::Sibling,
    effects: branches,
  })
}

/// Splits Word's flat shadow-of-glow graph at its image-DPI boundary.
///
/// The parser represents the authored ordering as
/// `Sibling(Tree(Sibling(Glow, Identity), OuterShadow))`.  Direct2D realizes
/// the inner image at its fixed source DPI, inserts DPI compensation when the
/// following effect is realized at a different target DPI, then evaluates the
/// outer shadow.  Returning two containers lets the host preserve that
/// boundary without teaching the fixed-size effect evaluator to resize images
/// in the middle of an otherwise ordinary graph.
pub(crate) fn split_wordprocessing_shadow_of_glow_dpi_stages(
  container: &ImageEffectContainer,
) -> Option<(ImageEffectContainer, ImageEffectContainer)> {
  if container.kind != ImageEffectContainerKind::Sibling {
    return None;
  }
  let [ImageEffect::Container(shadow_branch)] = container.effects.as_slice() else {
    return None;
  };
  if shadow_branch.kind != ImageEffectContainerKind::Tree {
    return None;
  }
  let [
    ImageEffect::Container(glow_source),
    shadow @ ImageEffect::OuterShadow { .. },
  ] = shadow_branch.effects.as_slice()
  else {
    return None;
  };
  if glow_source.kind != ImageEffectContainerKind::Sibling
    || !matches!(
      glow_source.effects.as_slice(),
      [ImageEffect::Glow { .. }, ImageEffect::Identity]
    )
  {
    return None;
  }

  Some((
    glow_source.clone(),
    ImageEffectContainer {
      kind: ImageEffectContainerKind::Sibling,
      effects: vec![shadow.clone()],
    },
  ))
}

/// Extracts the generated W14 outer-shadow branch and replaces every copy of
/// that branch with the children source.
///
/// `from_wordprocessing_text_effects` duplicates the completed shadow branch
/// once in the upright sibling graph and once inside reflection's completed
/// source. A static-3-D host has to realize that branch on the authored text
/// plane and project it through the camera before the remaining effect graph
/// runs. Returning the branch while substituting `Children` preserves both
/// uses without teaching the generic effect evaluator about 3-D cameras.
pub(crate) fn extract_projected_wordprocessing_shadow_branch(
  container: &mut ImageEffectContainer,
) -> Option<ImageEffectContainer> {
  fn is_generated_shadow_branch(effect: &ImageEffect) -> bool {
    match effect {
      ImageEffect::OuterShadow { .. } => true,
      ImageEffect::Container(container) if container.kind == ImageEffectContainerKind::Tree => {
        matches!(
          container.effects.last(),
          Some(ImageEffect::OuterShadow { .. })
        )
      }
      _ => false,
    }
  }

  fn replace_shadow_branches(container: &mut ImageEffectContainer) {
    for effect in &mut container.effects {
      if is_generated_shadow_branch(effect) {
        *effect = ImageEffect::SourceReference(ImageEffectSourceReference::Children);
        continue;
      }
      match effect {
        ImageEffect::AlphaModulate(nested)
        | ImageEffect::Blend {
          container: nested, ..
        }
        | ImageEffect::Container(nested) => replace_shadow_branches(nested),
        _ => {}
      }
    }
  }

  let shadow = container
    .effects
    .iter()
    .find(|effect| is_generated_shadow_branch(effect))?
    .clone();
  replace_shadow_branches(container);
  Some(ImageEffectContainer {
    kind: ImageEffectContainerKind::Sibling,
    effects: vec![shadow],
  })
}

/// Places completed shadow planes without translating their input glyphs or
/// unrelated sibling effects. Update duplicated reflection sources as well.
/// Lengths have the same CSS-pixel units as the unresolved effect graph.
pub(crate) fn translate_outer_shadow_outputs(
  container: &mut ImageEffectContainer,
  translation_px: (f32, f32),
) {
  for effect in &mut container.effects {
    match effect {
      ImageEffect::OuterShadow { transform, .. } => {
        transform.shift_x_px += translation_px.0;
        transform.shift_y_px += translation_px.1;
      }
      ImageEffect::AlphaModulate(nested)
      | ImageEffect::Blend {
        container: nested, ..
      }
      | ImageEffect::Container(nested) => translate_outer_shadow_outputs(nested, translation_px),
      _ => {}
    }
  }
}

pub(crate) trait ImageEffectColorResolver {
  fn alpha_inverse(&self, choice: &a::AlphaInverseChoice) -> Option<ResolvedEffectColor>;
  fn color_from(&self, choice: &a::ColorFromChoice) -> Option<ResolvedEffectColor>;
  fn color_to(&self, choice: &a::ColorToChoice) -> Option<ResolvedEffectColor>;
  fn color_replacement(&self, choice: &a::ColorReplacementChoice) -> Option<ResolvedEffectColor>;
  fn duotone(&self, choice: &a::DuotoneChoice) -> Option<ResolvedEffectColor>;
  fn solid_fill(&self, choice: &a::SolidFillChoice) -> Option<ResolvedEffectColor>;
  fn gradient_stop(&self, choice: &a::GradientStopChoice) -> Option<ResolvedEffectColor>;
  fn foreground(&self, choice: &a::ForegroundColorChoice) -> Option<ResolvedEffectColor>;
  fn background(&self, choice: &a::BackgroundColorChoice) -> Option<ResolvedEffectColor>;
  fn glow(&self, choice: &a::GlowChoice) -> Option<ResolvedEffectColor>;
  fn inner_shadow(&self, choice: &a::InnerShadowChoice) -> Option<ResolvedEffectColor>;
  fn outer_shadow(&self, choice: &a::OuterShadowChoice) -> Option<ResolvedEffectColor>;
  fn preset_shadow(&self, choice: &a::PresetShadowChoice) -> Option<ResolvedEffectColor>;
  fn extrusion_color(&self, _choice: &a::ExtrusionColorChoice) -> Option<ResolvedEffectColor> {
    None
  }
  fn contour_color(&self, _choice: &a::ContourColorChoice) -> Option<ResolvedEffectColor> {
    None
  }
  fn blip_fill(&self, _fill: &a::BlipFill) -> Option<ImageEffectFill> {
    None
  }
}

pub(crate) fn from_blip_choices(
  choices: &[a::BlipChoice],
  content_type: Option<&str>,
  resolver: &impl ImageEffectColorResolver,
) -> Vec<ImageEffect> {
  choices
    .iter()
    .filter_map(|choice| match choice {
      a::BlipChoice::AlphaBiLevel(effect) => Some(alpha_bilevel(effect)),
      a::BlipChoice::AlphaCeiling => Some(ImageEffect::AlphaCeiling),
      a::BlipChoice::AlphaFloor => Some(ImageEffect::AlphaFloor),
      a::BlipChoice::AlphaInverse(effect) => Some(ImageEffect::AlphaInverse(
        effect
          .alpha_inverse_choice
          .as_ref()
          .and_then(|choice| resolver.alpha_inverse(choice))
          .map(|color| color.color),
      )),
      a::BlipChoice::AlphaModulationEffect(effect) => Some(ImageEffect::AlphaModulate(
        from_effect_container(&effect.effect_container, content_type, resolver),
      )),
      a::BlipChoice::AlphaModulationFixed(effect) => Some(alpha_modulate_fixed(effect)),
      a::BlipChoice::AlphaReplace(effect) => Some(alpha_replace(effect)),
      a::BlipChoice::BiLevel(effect) => Some(bilevel(effect)),
      a::BlipChoice::Blur(effect) => Some(blur(effect)),
      a::BlipChoice::ColorChange(effect) => color_change(effect, content_type, resolver),
      a::BlipChoice::ColorReplacement(effect) => effect
        .color_replacement_choice
        .as_ref()
        .and_then(|choice| resolver.color_replacement(choice))
        .map(|color| ImageEffect::ColorReplacement(color.color)),
      a::BlipChoice::Duotone(effect) => duotone(effect, resolver),
      a::BlipChoice::Grayscale => Some(ImageEffect::Grayscale),
      a::BlipChoice::Hsl(effect) => Some(hsl(effect)),
      a::BlipChoice::LuminanceEffect(effect) => Some(luminance(effect)),
      a::BlipChoice::TintEffect(effect) => Some(tint(effect)),
      a::BlipChoice::FillOverlay(effect) => fill_overlay(effect, resolver),
    })
    .collect()
}

pub(crate) fn from_effect_container(
  container: &a::EffectContainer,
  content_type: Option<&str>,
  resolver: &impl ImageEffectColorResolver,
) -> ImageEffectContainer {
  let mut named = Vec::new();
  collect_named_containers(container, &mut named);
  from_effect_container_with_context(container, content_type, resolver, &named, &mut Vec::new())
}

/// Lowers an `a:effectDag` without collapsing repeated effects or losing the
/// authored sibling/tree container semantics.
///
/// `CT_EffectContainer` and the generated `EffectDag` type have the same XML
/// content model but distinct generated choice enums. Convert that generated
/// boundary once, then keep the actual executor shared with nested `a:cont`
/// and `a:blip` effect graphs.
pub(crate) fn from_effect_dag(
  dag: &a::EffectDag,
  content_type: Option<&str>,
  resolver: &impl ImageEffectColorResolver,
) -> ImageEffectContainer {
  let container = a::EffectContainer {
    r#type: dag.r#type,
    name: dag.name.clone(),
    effect_container_choice: dag
      .effect_dag_choice
      .iter()
      .map(effect_dag_choice_as_container_choice)
      .collect(),
  };
  from_effect_container(&container, content_type, resolver)
}

pub(crate) fn source_requirements(
  container: &ImageEffectContainer,
) -> ImageEffectSourceRequirements {
  fn visit_effect(effect: &ImageEffect, requirements: &mut ImageEffectSourceRequirements) {
    match effect {
      ImageEffect::SourceReference(ImageEffectSourceReference::Fill) => requirements.fill = true,
      ImageEffect::SourceReference(ImageEffectSourceReference::Line) => requirements.line = true,
      ImageEffect::SourceReference(ImageEffectSourceReference::Children) => {
        requirements.children = true;
      }
      ImageEffect::SourceReference(ImageEffectSourceReference::FillLine) => {
        requirements.fill_line = true;
      }
      ImageEffect::SourceReference(ImageEffectSourceReference::EffectMask) => {
        requirements.effect_mask = true;
      }
      ImageEffect::SourceReference(ImageEffectSourceReference::ReflectionPaint) => {
        requirements.reflection_paint = true;
      }
      ImageEffect::AlphaModulate(container)
      | ImageEffect::Blend { container, .. }
      | ImageEffect::Container(container) => visit_container(container, requirements),
      _ => {}
    }
  }

  fn visit_container(
    container: &ImageEffectContainer,
    requirements: &mut ImageEffectSourceRequirements,
  ) {
    for effect in &container.effects {
      visit_effect(effect, requirements);
    }
  }

  let mut requirements = ImageEffectSourceRequirements::default();
  visit_container(container, &mut requirements);
  requirements
}

/// Lowers the fixed `a:effectLst` pipeline into explicit compositing branches.
///
/// Glow, outer/preset shadow, and reflection are derived from the source and
/// painted behind the main shape. Blur, fill overlay, inner shadow, and soft
/// edge form the main-shape pipeline. This mirrors the fixed DrawingML list
/// semantics without pretending the list is an authored tree DAG.
pub(crate) fn from_effect_list(
  list: &a::EffectList,
  _content_type: Option<&str>,
  resolver: &impl ImageEffectColorResolver,
) -> ImageEffectContainer {
  let mut branches = Vec::new();
  if let Some(effect) = list
    .glow
    .as_deref()
    .and_then(|effect| glow(effect, resolver))
  {
    branches.push(effect);
  }
  if let Some(effect) = list
    .outer_shadow
    .as_deref()
    .and_then(|effect| outer_shadow(effect, resolver))
  {
    branches.push(effect);
  }
  if let Some(effect) = list
    .preset_shadow
    .as_deref()
    .and_then(|effect| preset_shadow(effect, resolver))
  {
    branches.push(effect);
  }
  if let Some(effect) = &list.reflection {
    branches.push(reflection(effect));
  }

  let mut main = Vec::new();
  if let Some(effect) = &list.blur {
    main.push(blur(effect));
  }
  if let Some(effect) = list
    .fill_overlay
    .as_deref()
    .and_then(|effect| fill_overlay(effect, resolver))
  {
    main.push(effect);
  }
  if let Some(effect) = list
    .inner_shadow
    .as_deref()
    .and_then(|effect| inner_shadow(effect, resolver))
  {
    main.push(effect);
  }
  if let Some(effect) = &list.soft_edge {
    main.push(ImageEffect::SoftEdge(
      effect.radius.to_emu().max(0) as f32 / 9_525.0,
    ));
  }
  if branches.is_empty() && main.is_empty() {
    return ImageEffectContainer {
      kind: ImageEffectContainerKind::Sibling,
      effects: Vec::new(),
    };
  }
  if main.is_empty() {
    main.push(ImageEffect::Identity);
  }
  branches.push(ImageEffect::Container(ImageEffectContainer {
    kind: ImageEffectContainerKind::Tree,
    effects: main,
  }));
  ImageEffectContainer {
    kind: ImageEffectContainerKind::Sibling,
    effects: branches,
  }
}

fn effect_dag_choice_as_container_choice(choice: &a::EffectDagChoice) -> a::EffectContainerChoice {
  match choice {
    a::EffectDagChoice::EffectContainer(value) => {
      a::EffectContainerChoice::EffectContainer(value.clone())
    }
    a::EffectDagChoice::Effect(value) => a::EffectContainerChoice::Effect(value.clone()),
    a::EffectDagChoice::AlphaBiLevel(value) => {
      a::EffectContainerChoice::AlphaBiLevel(value.clone())
    }
    a::EffectDagChoice::AlphaCeiling => a::EffectContainerChoice::AlphaCeiling,
    a::EffectDagChoice::AlphaFloor => a::EffectContainerChoice::AlphaFloor,
    a::EffectDagChoice::AlphaInverse(value) => {
      a::EffectContainerChoice::AlphaInverse(value.clone())
    }
    a::EffectDagChoice::AlphaModulationEffect(value) => {
      a::EffectContainerChoice::AlphaModulationEffect(value.clone())
    }
    a::EffectDagChoice::AlphaModulationFixed(value) => {
      a::EffectContainerChoice::AlphaModulationFixed(value.clone())
    }
    a::EffectDagChoice::AlphaOutset(value) => a::EffectContainerChoice::AlphaOutset(value.clone()),
    a::EffectDagChoice::AlphaReplace(value) => {
      a::EffectContainerChoice::AlphaReplace(value.clone())
    }
    a::EffectDagChoice::BiLevel(value) => a::EffectContainerChoice::BiLevel(value.clone()),
    a::EffectDagChoice::Blend(value) => a::EffectContainerChoice::Blend(value.clone()),
    a::EffectDagChoice::Blur(value) => a::EffectContainerChoice::Blur(value.clone()),
    a::EffectDagChoice::ColorChange(value) => a::EffectContainerChoice::ColorChange(value.clone()),
    a::EffectDagChoice::ColorReplacement(value) => {
      a::EffectContainerChoice::ColorReplacement(value.clone())
    }
    a::EffectDagChoice::Duotone(value) => a::EffectContainerChoice::Duotone(value.clone()),
    a::EffectDagChoice::Fill(value) => a::EffectContainerChoice::Fill(value.clone()),
    a::EffectDagChoice::FillOverlay(value) => a::EffectContainerChoice::FillOverlay(value.clone()),
    a::EffectDagChoice::Glow(value) => a::EffectContainerChoice::Glow(value.clone()),
    a::EffectDagChoice::Grayscale => a::EffectContainerChoice::Grayscale,
    a::EffectDagChoice::Hsl(value) => a::EffectContainerChoice::Hsl(value.clone()),
    a::EffectDagChoice::InnerShadow(value) => a::EffectContainerChoice::InnerShadow(value.clone()),
    a::EffectDagChoice::LuminanceEffect(value) => {
      a::EffectContainerChoice::LuminanceEffect(value.clone())
    }
    a::EffectDagChoice::OuterShadow(value) => a::EffectContainerChoice::OuterShadow(value.clone()),
    a::EffectDagChoice::PresetShadow(value) => {
      a::EffectContainerChoice::PresetShadow(value.clone())
    }
    a::EffectDagChoice::Reflection(value) => a::EffectContainerChoice::Reflection(value.clone()),
    a::EffectDagChoice::RelativeOffset(value) => {
      a::EffectContainerChoice::RelativeOffset(value.clone())
    }
    a::EffectDagChoice::SoftEdge(value) => a::EffectContainerChoice::SoftEdge(value.clone()),
    a::EffectDagChoice::TintEffect(value) => a::EffectContainerChoice::TintEffect(value.clone()),
    a::EffectDagChoice::TransformEffect(value) => {
      a::EffectContainerChoice::TransformEffect(value.clone())
    }
  }
}

fn collect_named_containers<'a>(
  container: &'a a::EffectContainer,
  named: &mut Vec<(&'a str, &'a a::EffectContainer)>,
) {
  if let Some(name) = container.name.as_ref() {
    named.push((name.as_str(), container));
  }
  for choice in &container.effect_container_choice {
    match choice {
      a::EffectContainerChoice::EffectContainer(child) => collect_named_containers(child, named),
      a::EffectContainerChoice::AlphaModulationEffect(effect) => {
        collect_named_containers(&effect.effect_container, named);
      }
      a::EffectContainerChoice::Blend(effect) => {
        collect_named_containers(&effect.effect_container, named);
      }
      _ => {}
    }
  }
}

fn from_effect_container_with_context(
  container: &a::EffectContainer,
  content_type: Option<&str>,
  resolver: &impl ImageEffectColorResolver,
  named: &[(&str, &a::EffectContainer)],
  resolving: &mut Vec<String>,
) -> ImageEffectContainer {
  let effects = container
    .effect_container_choice
    .iter()
    .filter_map(|choice| match choice {
      a::EffectContainerChoice::EffectContainer(container) => Some(ImageEffect::Container(
        from_effect_container_with_context(container, content_type, resolver, named, resolving),
      )),
      a::EffectContainerChoice::Effect(effect) => {
        effect_reference(effect, content_type, resolver, named, resolving)
      }
      a::EffectContainerChoice::AlphaBiLevel(effect) => Some(alpha_bilevel(effect)),
      a::EffectContainerChoice::AlphaCeiling => Some(ImageEffect::AlphaCeiling),
      a::EffectContainerChoice::AlphaFloor => Some(ImageEffect::AlphaFloor),
      a::EffectContainerChoice::AlphaInverse(effect) => Some(ImageEffect::AlphaInverse(
        effect
          .alpha_inverse_choice
          .as_ref()
          .and_then(|choice| resolver.alpha_inverse(choice))
          .map(|color| color.color),
      )),
      a::EffectContainerChoice::AlphaModulationEffect(effect) => Some(ImageEffect::AlphaModulate(
        from_effect_container_with_context(
          &effect.effect_container,
          content_type,
          resolver,
          named,
          resolving,
        ),
      )),
      a::EffectContainerChoice::AlphaModulationFixed(effect) => Some(alpha_modulate_fixed(effect)),
      a::EffectContainerChoice::AlphaOutset(effect) => Some(ImageEffect::AlphaOutset(
        effect
          .radius
          .map(|radius| radius.to_emu() as f32 / 9_525.0)
          .unwrap_or_default(),
      )),
      a::EffectContainerChoice::AlphaReplace(effect) => Some(alpha_replace(effect)),
      a::EffectContainerChoice::BiLevel(effect) => Some(bilevel(effect)),
      a::EffectContainerChoice::Blend(effect) => Some(ImageEffect::Blend {
        container: from_effect_container_with_context(
          &effect.effect_container,
          content_type,
          resolver,
          named,
          resolving,
        ),
        blend_mode: image_effect_blend_mode(effect.blend_mode),
      }),
      a::EffectContainerChoice::Blur(effect) => Some(blur(effect)),
      a::EffectContainerChoice::ColorChange(effect) => color_change(effect, content_type, resolver),
      a::EffectContainerChoice::ColorReplacement(effect) => effect
        .color_replacement_choice
        .as_ref()
        .and_then(|choice| resolver.color_replacement(choice))
        .map(|color| ImageEffect::ColorReplacement(color.color)),
      a::EffectContainerChoice::Duotone(effect) => duotone(effect, resolver),
      a::EffectContainerChoice::Grayscale => Some(ImageEffect::Grayscale),
      a::EffectContainerChoice::Hsl(effect) => Some(hsl(effect)),
      a::EffectContainerChoice::LuminanceEffect(effect) => Some(luminance(effect)),
      a::EffectContainerChoice::TintEffect(effect) => Some(tint(effect)),
      a::EffectContainerChoice::Fill(effect) => {
        fill_effect(effect, resolver).map(ImageEffect::Fill)
      }
      a::EffectContainerChoice::FillOverlay(effect) => fill_overlay(effect, resolver),
      a::EffectContainerChoice::RelativeOffset(effect) => Some(ImageEffect::RelativeOffset {
        offset_x: effect
          .offset_x
          .as_ref()
          .map(|value| value.as_ratio() as f32)
          .unwrap_or_default(),
        offset_y: effect
          .offset_y
          .as_ref()
          .map(|value| value.as_ratio() as f32)
          .unwrap_or_default(),
      }),
      a::EffectContainerChoice::SoftEdge(effect) => Some(ImageEffect::SoftEdge(
        effect.radius.to_emu() as f32 / 9_525.0,
      )),
      a::EffectContainerChoice::TransformEffect(effect) => {
        Some(ImageEffect::Transform(ImageEffectTransform {
          scale_x: effect
            .horizontal_ratio
            .as_ref()
            .map(|value| value.as_ratio() as f32)
            .unwrap_or(1.0),
          scale_y: effect
            .vertical_ratio
            .as_ref()
            .map(|value| value.as_ratio() as f32)
            .unwrap_or(1.0),
          skew_x: (effect.horizontal_skew.unwrap_or_default() as f32 / 60_000.0)
            .to_radians()
            .tan(),
          skew_y: (effect.vertical_skew.unwrap_or_default() as f32 / 60_000.0)
            .to_radians()
            .tan(),
          shift_x_px: effect
            .horizontal_shift
            .map(|value| value.to_emu() as f32 / 9_525.0)
            .unwrap_or_default(),
          shift_y_px: effect
            .vertical_shift
            .map(|value| value.to_emu() as f32 / 9_525.0)
            .unwrap_or_default(),
        }))
      }
      a::EffectContainerChoice::Glow(effect) => glow(effect, resolver),
      a::EffectContainerChoice::InnerShadow(effect) => inner_shadow(effect, resolver),
      a::EffectContainerChoice::OuterShadow(effect) => outer_shadow(effect, resolver),
      a::EffectContainerChoice::Reflection(effect) => Some(reflection(effect)),
      a::EffectContainerChoice::PresetShadow(effect) => preset_shadow(effect, resolver),
    })
    .collect();
  ImageEffectContainer {
    kind: match container.r#type.unwrap_or_default() {
      a::EffectContainerValues::Sibling => ImageEffectContainerKind::Sibling,
      a::EffectContainerValues::Tree => ImageEffectContainerKind::Tree,
    },
    effects,
  }
}

fn effect_reference(
  effect: &a::Effect,
  content_type: Option<&str>,
  resolver: &impl ImageEffectColorResolver,
  named: &[(&str, &a::EffectContainer)],
  resolving: &mut Vec<String>,
) -> Option<ImageEffect> {
  let reference = effect.reference.as_ref()?.as_str();
  match reference {
    "fill" => Some(ImageEffect::SourceReference(
      ImageEffectSourceReference::Fill,
    )),
    "line" => Some(ImageEffect::SourceReference(
      ImageEffectSourceReference::Line,
    )),
    "fillLine" => Some(ImageEffect::SourceReference(
      ImageEffectSourceReference::FillLine,
    )),
    "children" => Some(ImageEffect::SourceReference(
      ImageEffectSourceReference::Children,
    )),
    _ => {
      if resolving.iter().any(|name| name == reference) {
        return None;
      }
      let container = named
        .iter()
        .find_map(|(name, container)| (*name == reference).then_some(*container))?;
      resolving.push(reference.to_string());
      let result =
        from_effect_container_with_context(container, content_type, resolver, named, resolving);
      resolving.pop();
      Some(ImageEffect::Container(result))
    }
  }
}

fn alpha_bilevel(effect: &a::AlphaBiLevel) -> ImageEffect {
  ImageEffect::AlphaBiLevel(
    (effect.threshold.as_ratio() * 255.0)
      .round()
      .clamp(0.0, 255.0) as u8,
  )
}

fn alpha_modulate_fixed(effect: &a::AlphaModulationFixed) -> ImageEffect {
  ImageEffect::AlphaModulateFixed(
    effect
      .amount
      .as_ref()
      .map(|amount| office_alpha_modulate_amount(*amount))
      .unwrap_or(1.0),
  )
}

fn alpha_replace(effect: &a::AlphaReplace) -> ImageEffect {
  ImageEffect::AlphaReplace((effect.alpha.as_ratio() * 255.0).round().clamp(0.0, 255.0) as u8)
}

fn bilevel(effect: &a::BiLevel) -> ImageEffect {
  ImageEffect::BiLevel(
    (effect.threshold.as_ratio() * 255.0)
      .round()
      .clamp(0.0, 255.0) as u8,
  )
}

fn blur(effect: &a::Blur) -> ImageEffect {
  ImageEffect::Blur {
    radius_px: effect
      .radius
      .map(|radius| radius.to_emu() as f32 / 9_525.0)
      .unwrap_or_default(),
    grow_bounds: effect.grow.as_ref().is_none_or(|value| value.as_bool()),
  }
}

fn glow(effect: &a::Glow, resolver: &impl ImageEffectColorResolver) -> Option<ImageEffect> {
  Some(ImageEffect::Glow {
    // ISO/IEC 29500 defines `rad` as the full glow radius. PowerPoint fixed
    // output uses one third of that range for the opaque spread; the remaining
    // range contains the Gaussian fringe and transparent edge padding.
    radius_px: effect
      .radius
      .map(|value| value.to_emu() as f32 / 9_525.0)
      .unwrap_or_default(),
    raster_length_scale: 1.0,
    bounds_radius_scale: 1.0,
    bounds_radius_offset_px: 0.0,
    spread_ratio: 1.0 / 3.0,
    spread_kernel: GlowSpreadKernel::Square,
    spread_radius_rounding: GlowSpreadRadiusRounding::Outward,
    blur_kernel: GlowBlurKernel::Gaussian,
    color: resolver.glow(effect.glow_choice.as_ref()?)?,
  })
}

fn inner_shadow(
  effect: &a::InnerShadow,
  resolver: &impl ImageEffectColorResolver,
) -> Option<ImageEffect> {
  Some(ImageEffect::InnerShadow {
    blur_radius_px: effect
      .blur_radius
      .map(|value| value.to_emu() as f32 / 9_525.0)
      .unwrap_or_default(),
    distance_px: effect
      .distance
      .map(|value| value.to_emu() as f32 / 9_525.0)
      .unwrap_or_default(),
    direction_degrees: effect.direction.unwrap_or_default() as f32 / 60_000.0,
    color: resolver.inner_shadow(effect.inner_shadow_choice.as_ref()?)?,
  })
}

fn outer_shadow(
  effect: &a::OuterShadow,
  resolver: &impl ImageEffectColorResolver,
) -> Option<ImageEffect> {
  Some(ImageEffect::OuterShadow {
    blur_radius_px: effect
      .blur_radius
      .map(|value| value.to_emu() as f32 / 9_525.0)
      .unwrap_or_default(),
    distance_px: effect
      .distance
      .map(|value| value.to_emu() as f32 / 9_525.0)
      .unwrap_or_default(),
    raster_length_scale: 1.0,
    distance_length_scale: 1.0,
    bounds_radius_scale: 1.0,
    bounds_radius_offset_px: 0.0,
    blur_kernel: ShadowBlurKernel::Direct2dGaussian,
    direction_degrees: effect.direction.unwrap_or_default() as f32 / 60_000.0,
    distance_mode: ShadowDistanceMode::PostTransformOffset,
    transform: ImageEffectTransform {
      scale_x: effect
        .horizontal_ratio
        .as_ref()
        .map(|value| value.as_ratio() as f32)
        .unwrap_or(1.0),
      scale_y: effect
        .vertical_ratio
        .as_ref()
        .map(|value| value.as_ratio() as f32)
        .unwrap_or(1.0),
      skew_x: (effect.horizontal_skew.unwrap_or_default() as f32 / 60_000.0)
        .to_radians()
        .tan(),
      skew_y: (effect.vertical_skew.unwrap_or_default() as f32 / 60_000.0)
        .to_radians()
        .tan(),
      shift_x_px: 0.0,
      shift_y_px: 0.0,
    },
    alignment: effect_alignment(effect.alignment),
    rotate_with_shape: effect
      .rotate_with_shape
      .as_ref()
      .is_none_or(|value| value.as_bool()),
    color: resolver.outer_shadow(effect.outer_shadow_choice.as_ref()?)?,
  })
}

fn preset_shadow(
  effect: &a::PresetShadow,
  resolver: &impl ImageEffectColorResolver,
) -> Option<ImageEffect> {
  let color = resolver.preset_shadow(effect.preset_shadow_choice.as_ref()?)?;
  let distance_px = effect
    .distance
    .map(|value| value.to_emu() as f32 / 9_525.0)
    .unwrap_or_default();
  let direction_degrees = effect.direction.unwrap_or_default() as f32 / 60_000.0;
  let mut transform = ImageEffectTransform {
    scale_x: 1.0,
    scale_y: 1.0,
    skew_x: 0.0,
    skew_y: 0.0,
    shift_x_px: 0.0,
    shift_y_px: 0.0,
  };
  let mut alignment = (0.5, 1.0);

  // ECMA-376 Part 1 §20.1.10.52 defines these as the non-default
  // CT_OuterShadowEffect parameters for each preset. The two-box presets are
  // explicitly two outer shadows, not a single approximated transform.
  match effect.preset {
    a::PresetShadowValues::BackLeftPerspectiveShadow => {
      transform.skew_y = 40.89_f32.to_radians().tan();
      transform.scale_y = 0.5;
    }
    a::PresetShadowValues::BackRightPerspectiveShadow => {
      transform.skew_x = (-40.89_f32).to_radians().tan();
      transform.scale_y = 0.5;
    }
    a::PresetShadowValues::FrontLeftPerspectiveShadow => {
      transform.skew_x = 40.89_f32.to_radians().tan();
      transform.scale_y = -0.5;
    }
    a::PresetShadowValues::FrontRightPerspectiveShadow => {
      transform.skew_x = (-40.89_f32).to_radians().tan();
      transform.scale_y = -0.5;
    }
    a::PresetShadowValues::TopLeftSmallDropShadow => {
      transform.scale_x = 0.75;
      transform.scale_y = 0.75;
      alignment = (0.0, 0.0);
    }
    a::PresetShadowValues::TopLeftLargeDropShadow => {
      transform.scale_x = 1.25;
      transform.scale_y = 1.25;
      alignment = (1.0, 1.0);
    }
    a::PresetShadowValues::BackLeftLongPerspectiveShadow => {
      transform.skew_x = 40.89_f32.to_radians().tan();
      transform.scale_y = 0.5;
    }
    a::PresetShadowValues::BackRightLongPerspectiveShadow => {
      transform.skew_x = (-40.89_f32).to_radians().tan();
      transform.scale_y = 0.5;
    }
    a::PresetShadowValues::FrontLeftLongPerspectiveShadow => {
      transform.skew_x = 40.89_f32.to_radians().tan();
      transform.scale_y = -0.5;
    }
    a::PresetShadowValues::FrontRightLongPerspectiveShadow => {
      transform.skew_x = (-40.89_f32).to_radians().tan();
      transform.scale_y = -0.5;
    }
    a::PresetShadowValues::BackCenterPerspectiveShadow => transform.scale_y = 0.5,
    a::PresetShadowValues::FrontBottomShadow => transform.scale_y = -1.0,
    _ => {}
  }

  let outer = |distance_px, direction_degrees, color| ImageEffect::OuterShadow {
    blur_radius_px: 0.0,
    distance_px,
    raster_length_scale: 1.0,
    distance_length_scale: 1.0,
    bounds_radius_scale: 1.0,
    bounds_radius_offset_px: 0.0,
    blur_kernel: ShadowBlurKernel::Direct2dGaussian,
    direction_degrees,
    distance_mode: ShadowDistanceMode::PostTransformOffset,
    transform,
    alignment,
    rotate_with_shape: false,
    color,
  };
  let second_color = ResolvedEffectColor {
    color: RgbColor {
      r: color.color.r.saturating_add(102),
      g: color.color.g.saturating_add(102),
      b: color.color.b.saturating_add(102),
    },
    alpha: color.alpha,
  };
  match effect.preset {
    a::PresetShadowValues::TopLeftDoubleDropShadow => {
      Some(ImageEffect::Container(ImageEffectContainer {
        kind: ImageEffectContainerKind::Sibling,
        effects: vec![
          outer(distance_px, direction_degrees, color),
          outer(distance_px * 2.0, direction_degrees, second_color),
        ],
      }))
    }
    a::PresetShadowValues::ThreeDimensionalOuterBoxShadow
    | a::PresetShadowValues::ThreeDimensionalInnerBoxShadow => {
      Some(ImageEffect::Container(ImageEffectContainer {
        kind: ImageEffectContainerKind::Sibling,
        effects: vec![
          outer(distance_px, direction_degrees, color),
          outer(
            distance_px,
            (direction_degrees + 180.0).rem_euclid(360.0),
            second_color,
          ),
        ],
      }))
    }
    _ => Some(outer(distance_px, direction_degrees, color)),
  }
}

fn reflection(effect: &a::Reflection) -> ImageEffect {
  ImageEffect::Reflection(ImageReflectionEffect {
    blur_radius_px: effect
      .blur_radius
      .map(|value| value.to_emu() as f32 / 9_525.0)
      .unwrap_or_default(),
    raster_length_scale: 1.0,
    bounds_radius_scale: 1.0,
    bounds_radius_offset_px: 0.0,
    start_opacity: effect
      .start_opacity
      .map(|value| value.as_ratio() as f32)
      .unwrap_or(1.0),
    start_position: effect
      .start_position
      .map(|value| value.as_ratio() as f32)
      .unwrap_or(0.0),
    end_opacity: effect
      .end_alpha
      .map(|value| value.as_ratio() as f32)
      .unwrap_or(0.0),
    end_position: effect
      .end_position
      .map(|value| value.as_ratio() as f32)
      .unwrap_or(1.0),
    fade_direction_degrees: effect.fade_direction.unwrap_or(5_400_000) as f32 / 60_000.0,
    distance_px: effect
      .distance
      .map(|value| value.to_emu() as f32 / 9_525.0)
      .unwrap_or_default(),
    distance_length_scale: 1.0,
    distance_mode: ReflectionDistanceMode::PostTransformOffset,
    reference: ReflectionReference::EffectInput,
    direction_degrees: effect.direction.unwrap_or_default() as f32 / 60_000.0,
    transform: ImageEffectTransform {
      scale_x: effect
        .horizontal_ratio
        .as_ref()
        .map(|value| value.as_ratio() as f32)
        .unwrap_or(1.0),
      scale_y: effect
        .vertical_ratio
        .as_ref()
        .map(|value| value.as_ratio() as f32)
        .unwrap_or(1.0),
      skew_x: (effect.horizontal_skew.unwrap_or_default() as f32 / 60_000.0)
        .to_radians()
        .tan(),
      skew_y: (effect.vertical_skew.unwrap_or_default() as f32 / 60_000.0)
        .to_radians()
        .tan(),
      shift_x_px: 0.0,
      shift_y_px: 0.0,
    },
    alignment: effect_alignment(effect.alignment),
    rotate_with_shape: effect
      .rotate_with_shape
      .as_ref()
      .is_none_or(|value| value.as_bool()),
  })
}

fn effect_alignment(alignment: Option<a::RectangleAlignmentValues>) -> (f32, f32) {
  match alignment.unwrap_or(a::RectangleAlignmentValues::Bottom) {
    a::RectangleAlignmentValues::TopLeft => (0.0, 0.0),
    a::RectangleAlignmentValues::Top => (0.5, 0.0),
    a::RectangleAlignmentValues::TopRight => (1.0, 0.0),
    a::RectangleAlignmentValues::Left => (0.0, 0.5),
    a::RectangleAlignmentValues::Center => (0.5, 0.5),
    a::RectangleAlignmentValues::Right => (1.0, 0.5),
    a::RectangleAlignmentValues::BottomLeft => (0.0, 1.0),
    a::RectangleAlignmentValues::Bottom => (0.5, 1.0),
    a::RectangleAlignmentValues::BottomRight => (1.0, 1.0),
  }
}

fn color_change(
  effect: &a::ColorChange,
  content_type: Option<&str>,
  resolver: &impl ImageEffectColorResolver,
) -> Option<ImageEffect> {
  let from = effect
    .color_from
    .color_from_choice
    .as_ref()
    .and_then(|choice| resolver.color_from(choice))?;
  let to = effect
    .color_to
    .color_to_choice
    .as_ref()
    .and_then(|choice| resolver.color_to(choice))?;
  let use_alpha = effect
    .use_alpha
    .as_ref()
    .is_none_or(|value| value.as_bool());
  (from.color != to.color || (use_alpha && from.alpha != to.alpha)).then_some(
    ImageEffect::ColorChange(ColorChangeEffect {
      from: from.color,
      to: to.color,
      from_alpha: from.alpha,
      to_alpha: to.alpha,
      use_alpha,
      tolerance: color_change_tolerance(content_type),
    }),
  )
}

fn duotone(effect: &a::Duotone, resolver: &impl ImageEffectColorResolver) -> Option<ImageEffect> {
  let colors = effect
    .duotone_choice
    .iter()
    .filter_map(|choice| resolver.duotone(choice))
    .map(|color| color.color)
    .collect::<Vec<_>>();
  let [first, second] = colors.as_slice() else {
    return None;
  };
  Some(ImageEffect::Duotone(*first, *second))
}

fn hsl(effect: &a::Hsl) -> ImageEffect {
  ImageEffect::Hsl {
    hue_degrees: effect.hue.unwrap_or_default() as f32 / 60_000.0,
    saturation_offset: effect
      .saturation
      .map(|value| value.as_ratio() as f32)
      .unwrap_or_default(),
    luminance_offset: effect
      .luminance
      .map(|value| value.as_ratio() as f32)
      .unwrap_or_default(),
  }
}

fn luminance(effect: &a::LuminanceEffect) -> ImageEffect {
  let brightness = effect
    .brightness
    .as_ref()
    .map(|value| (value.as_ratio() * 100.0).round() as i32);
  let contrast = effect
    .contrast
    .as_ref()
    .map(|value| (value.as_ratio() * 100.0).round() as i32);
  ImageEffect::Luminance {
    brightness,
    contrast,
  }
}

fn tint(effect: &a::TintEffect) -> ImageEffect {
  ImageEffect::Tint {
    hue_degrees: effect.hue.unwrap_or_default() as f32 / 60_000.0,
    amount: effect
      .amount
      .as_ref()
      .map(|value| value.as_ratio() as f32)
      .unwrap_or_default(),
  }
}

fn fill_overlay(
  effect: &a::FillOverlay,
  resolver: &impl ImageEffectColorResolver,
) -> Option<ImageEffect> {
  let fill = match effect.fill_overlay_choice.as_ref()? {
    a::FillOverlayChoice::NoFill(_) => ImageEffectFill::None,
    a::FillOverlayChoice::SolidFill(fill) => ImageEffectFill::Solid(
      fill
        .solid_fill_choice
        .as_ref()
        .and_then(|choice| resolver.solid_fill(choice))?,
    ),
    a::FillOverlayChoice::GradientFill(fill) => gradient_fill(fill, resolver)?,
    a::FillOverlayChoice::PatternFill(fill) => {
      let foreground = fill
        .foreground_color
        .as_ref()?
        .foreground_color_choice
        .as_ref()
        .and_then(|choice| resolver.foreground(choice))?;
      let background = fill
        .background_color
        .as_ref()?
        .background_color_choice
        .as_ref()
        .and_then(|choice| resolver.background(choice))?;
      ImageEffectFill::Pattern {
        style: super::drawingml_pattern::hatch_style(fill.preset),
        foreground,
        background,
        tile_px: 8.0,
      }
    }
    a::FillOverlayChoice::BlipFill(fill) => resolver.blip_fill(fill)?,
    // Group fill requires group ancestry context and is resolved by the
    // owning shape pipeline rather than being guessed from the bitmap.
    a::FillOverlayChoice::GroupFill => return None,
  };
  Some(ImageEffect::FillOverlay {
    fill,
    blend_mode: image_effect_blend_mode(effect.blend),
  })
}

fn fill_effect(
  effect: &a::Fill,
  resolver: &impl ImageEffectColorResolver,
) -> Option<ImageEffectFill> {
  match effect.fill_choice.as_ref()? {
    a::FillChoice::NoFill(_) => Some(ImageEffectFill::None),
    a::FillChoice::SolidFill(fill) => Some(ImageEffectFill::Solid(
      resolver.solid_fill(fill.solid_fill_choice.as_ref()?)?,
    )),
    a::FillChoice::GradientFill(fill) => gradient_fill(fill, resolver),
    a::FillChoice::BlipFill(fill) => resolver.blip_fill(fill),
    a::FillChoice::PatternFill(fill) => {
      let foreground = resolver.foreground(
        fill
          .foreground_color
          .as_ref()?
          .foreground_color_choice
          .as_ref()?,
      )?;
      let background = resolver.background(
        fill
          .background_color
          .as_ref()?
          .background_color_choice
          .as_ref()?,
      )?;
      Some(ImageEffectFill::Pattern {
        style: super::drawingml_pattern::hatch_style(fill.preset),
        foreground,
        background,
        tile_px: 8.0,
      })
    }
    a::FillChoice::GroupFill => None,
  }
}

fn gradient_fill(
  fill: &a::GradientFill,
  resolver: &impl ImageEffectColorResolver,
) -> Option<ImageEffectFill> {
  let mut stops = fill
    .gradient_stop_list
    .as_ref()?
    .gradient_stop
    .iter()
    .filter_map(|stop| {
      Some((
        stop.position.as_ratio() as f32,
        resolver.gradient_stop(stop.gradient_stop_choice.as_ref()?)?,
      ))
    })
    .collect::<Vec<_>>();
  if stops.is_empty() {
    return None;
  }
  stops.sort_by(|left, right| left.0.total_cmp(&right.0));
  let kind = match fill.gradient_fill_choice.as_ref() {
    Some(a::GradientFillChoice::LinearGradientFill(linear)) => {
      ImageEffectGradientKind::Linear(linear.angle.unwrap_or_default() as f32 / 60_000.0)
    }
    Some(a::GradientFillChoice::PathGradientFill(path)) => {
      let focus = path
        .fill_to_rectangle
        .as_ref()
        .map(fill_to_relative_rect)
        .unwrap_or(ImageEffectRelativeRect {
          left: 0.5,
          top: 0.5,
          right: 0.5,
          bottom: 0.5,
        });
      match path.path.unwrap_or_default() {
        a::PathShadeValues::Circle => ImageEffectGradientKind::Circle(focus),
        a::PathShadeValues::Rectangle | a::PathShadeValues::Shape => {
          // A blip has a rectangular geometry, so DrawingML `shape` and
          // `rect` path gradients have the same boundary here.
          ImageEffectGradientKind::Rectangle(focus)
        }
      }
    }
    None => ImageEffectGradientKind::Linear(0.0),
  };
  Some(ImageEffectFill::Gradient {
    stops,
    kind,
    tile: fill
      .tile_rectangle
      .as_ref()
      .map(tile_to_relative_rect)
      .unwrap_or_default(),
    // MS-OI29500 §20.1.8.33: Office ignores the authored value and uses
    // alternating horizontal-and-vertical gradient tiles.
    flip: a::TileFlipValues::HorizontalAndVertical,
  })
}

fn fill_to_relative_rect(rect: &a::FillToRectangle) -> ImageEffectRelativeRect {
  ImageEffectRelativeRect {
    left: optional_percentage(rect.left.as_ref()),
    top: optional_percentage(rect.top.as_ref()),
    right: optional_percentage(rect.right.as_ref()),
    bottom: optional_percentage(rect.bottom.as_ref()),
  }
}

fn tile_to_relative_rect(rect: &a::TileRectangle) -> ImageEffectRelativeRect {
  ImageEffectRelativeRect {
    left: optional_percentage(rect.left.as_ref()),
    top: optional_percentage(rect.top.as_ref()),
    right: optional_percentage(rect.right.as_ref()),
    bottom: optional_percentage(rect.bottom.as_ref()),
  }
}

fn optional_percentage(value: Option<&DrawingmlPercentageValue>) -> f32 {
  value
    .map(|value| value.as_ratio() as f32)
    .unwrap_or_default()
}

fn image_effect_blend_mode(mode: a::BlendModeValues) -> ImageEffectBlendMode {
  match mode {
    a::BlendModeValues::Overlay => ImageEffectBlendMode::Over,
    a::BlendModeValues::Multiply => ImageEffectBlendMode::Multiply,
    a::BlendModeValues::Screen => ImageEffectBlendMode::Screen,
    a::BlendModeValues::Darken => ImageEffectBlendMode::Darken,
    a::BlendModeValues::Lighten => ImageEffectBlendMode::Lighten,
  }
}

pub(crate) fn apply(
  data: &[u8],
  content_type: Option<&str>,
  effects: &[ImageEffect],
) -> Option<Vec<u8>> {
  let raster_data = emf_wmf::decode_metafile_as_raster(data, content_type)
    .ok()
    .flatten()
    .map(|raster| raster.data);
  let image_data = raster_data.as_deref().unwrap_or(data);
  let mut image = image::load_from_memory(image_data).ok()?.to_rgba8();
  apply_to_image(&mut image, effects);

  let mut output = Vec::new();
  PngEncoder::new(Cursor::new(&mut output))
    .write_image(
      image.as_raw(),
      image.width(),
      image.height(),
      ColorType::Rgba8.into(),
    )
    .ok()?;
  Some(output)
}

/// Realizes Direct2D-style linear DPI compensation with a hard border.
///
/// Pixel centers are mapped by the physical DPI ratio rather than by the two
/// integer bitmap extents.  This distinction matters when a logical image
/// range ends between target pixels.  Each of the four linear taps outside the
/// input is transparent black, matching `D2D1_BORDER_MODE_HARD`; RGB is
/// interpolated in associated-alpha space and converted back to this module's
/// straight-RGBA storage.
pub(crate) fn dpi_compensate_linear_hard(
  source: &image::RgbaImage,
  source_pixels_per_point: f32,
  target_pixels_per_point: f32,
  target_width_px: u32,
  target_height_px: u32,
) -> Option<image::RgbaImage> {
  dpi_compensate_linear_hard_with_source_phase(
    source,
    source_pixels_per_point,
    target_pixels_per_point,
    target_width_px,
    target_height_px,
    (0.0, 0.0),
  )
}

/// Realizes linear hard-border DPI compensation while retaining an effect
/// image's device-pixel phase relative to the target surface.
///
/// A fixed-DPI image inserted inside an effect graph has its own integer
/// device origin.  Realizing that image at a different DPI must preserve the
/// origin phase as well as the physical scale; treating it as an origin-zero
/// bitmap shifts the filtered result even when the DPI ratio is integral.
pub(crate) fn dpi_compensate_linear_hard_with_source_phase(
  source: &image::RgbaImage,
  source_pixels_per_point: f32,
  target_pixels_per_point: f32,
  target_width_px: u32,
  target_height_px: u32,
  source_phase_px: (f32, f32),
) -> Option<image::RgbaImage> {
  if source.width() == 0
    || source.height() == 0
    || target_width_px == 0
    || target_height_px == 0
    || !source_pixels_per_point.is_finite()
    || source_pixels_per_point <= 0.0
    || !target_pixels_per_point.is_finite()
    || target_pixels_per_point <= 0.0
    || !source_phase_px.0.is_finite()
    || !source_phase_px.1.is_finite()
  {
    return None;
  }

  let scale = source_pixels_per_point / target_pixels_per_point;
  Some(image::RgbaImage::from_fn(
    target_width_px,
    target_height_px,
    |x, y| {
      let source_x = (x as f32 + 0.5).mul_add(scale, source_phase_px.0 - 0.5);
      let source_y = (y as f32 + 0.5).mul_add(scale, source_phase_px.1 - 0.5);
      bilinear_sample_premultiplied_hard(source, source_x, source_y)
    },
  ))
}

fn bilinear_sample_premultiplied_hard(
  source: &image::RgbaImage,
  x: f32,
  y: f32,
) -> image::Rgba<u8> {
  let x0 = x.floor() as i64;
  let y0 = y.floor() as i64;
  let x_amount = x - x.floor();
  let y_amount = y - y.floor();
  let sample = |sample_x: i64, sample_y: i64| {
    if sample_x < 0
      || sample_y < 0
      || sample_x >= i64::from(source.width())
      || sample_y >= i64::from(source.height())
    {
      [0; 4]
    } else {
      source.get_pixel(sample_x as u32, sample_y as u32).0
    }
  };
  let samples = [
    sample(x0, y0),
    sample(x0 + 1, y0),
    sample(x0, y0 + 1),
    sample(x0 + 1, y0 + 1),
  ];
  let weights = [
    (1.0 - x_amount) * (1.0 - y_amount),
    x_amount * (1.0 - y_amount),
    (1.0 - x_amount) * y_amount,
    x_amount * y_amount,
  ];
  let alpha = samples
    .iter()
    .zip(weights)
    .map(|(sample, weight)| f32::from(sample[3]) * weight)
    .sum::<f32>();
  if alpha <= f32::EPSILON {
    return image::Rgba([0; 4]);
  }

  let mut output = [0; 4];
  output[3] = alpha.round().clamp(0.0, 255.0) as u8;
  for channel in 0..3 {
    let associated = samples
      .iter()
      .zip(weights)
      .map(|(sample, weight)| f32::from(sample[channel]) * f32::from(sample[3]) / 255.0 * weight)
      .sum::<f32>();
    output[channel] = (associated * 255.0 / alpha).round().clamp(0.0, 255.0) as u8;
  }
  image::Rgba(output)
}

#[cfg(test)]
pub(crate) fn apply_container_to_padded_image(
  image: &mut image::RgbaImage,
  container: &ImageEffectContainer,
  content_left_px: f32,
  content_top_px: f32,
  content_width_px: f32,
  content_height_px: f32,
) {
  let content_bounds = PixelBounds {
    left: content_left_px,
    top: content_top_px,
    right: content_left_px + content_width_px,
    bottom: content_top_px + content_height_px,
  };
  let geometry = EffectGeometry {
    paint: content_bounds,
    shadow_anchor: content_bounds,
    anchor: content_bounds,
    ramp: content_bounds,
  };
  *image = apply_container_with_bounds(
    image,
    container,
    geometry,
    geometry,
    ImageEffectSourceImages::default(),
    AlphaOutsetSurfaceScale::default(),
  );
}

pub(crate) fn apply_container_to_padded_image_with_sources(
  image: &mut image::RgbaImage,
  container: &ImageEffectContainer,
  content_left_px: f32,
  content_top_px: f32,
  content_width_px: f32,
  content_height_px: f32,
  sources: ImageEffectSourceImages<'_>,
) {
  apply_container_to_padded_image_with_sources_and_alpha_outset_scale(
    image,
    container,
    ImageEffectContentBounds {
      left_px: content_left_px,
      top_px: content_top_px,
      width_px: content_width_px,
      height_px: content_height_px,
    },
    sources,
    AlphaOutsetSurfaceScale::default(),
  );
}

pub(crate) fn apply_container_to_padded_image_with_sources_and_alpha_outset_scale(
  image: &mut image::RgbaImage,
  container: &ImageEffectContainer,
  content_bounds: ImageEffectContentBounds,
  sources: ImageEffectSourceImages<'_>,
  alpha_outset_surface_scale: AlphaOutsetSurfaceScale,
) {
  let ImageEffectContentBounds {
    left_px: content_left_px,
    top_px: content_top_px,
    width_px: content_width_px,
    height_px: content_height_px,
  } = content_bounds;
  apply_container_to_padded_image_with_sources_and_anchor_and_alpha_outset_scale(
    image,
    container,
    ImageEffectSourceGeometry {
      paint_left_px: content_left_px,
      paint_top_px: content_top_px,
      paint_width_px: content_width_px,
      paint_height_px: content_height_px,
      shadow_anchor_left_px: content_left_px,
      shadow_anchor_top_px: content_top_px,
      shadow_anchor_width_px: content_width_px,
      shadow_anchor_height_px: content_height_px,
      anchor_left_px: content_left_px,
      anchor_top_px: content_top_px,
      anchor_width_px: content_width_px,
      anchor_height_px: content_height_px,
      ramp_left_px: content_left_px,
      ramp_top_px: content_top_px,
      ramp_width_px: content_width_px,
      ramp_height_px: content_height_px,
    },
    sources,
    alpha_outset_surface_scale,
  );
}

pub(crate) fn apply_container_to_padded_image_with_sources_and_anchor(
  image: &mut image::RgbaImage,
  container: &ImageEffectContainer,
  geometry: ImageEffectSourceGeometry,
  sources: ImageEffectSourceImages<'_>,
) {
  apply_container_to_padded_image_with_sources_and_anchor_and_alpha_outset_scale(
    image,
    container,
    geometry,
    sources,
    AlphaOutsetSurfaceScale::default(),
  );
}

fn apply_container_to_padded_image_with_sources_and_anchor_and_alpha_outset_scale(
  image: &mut image::RgbaImage,
  container: &ImageEffectContainer,
  geometry: ImageEffectSourceGeometry,
  sources: ImageEffectSourceImages<'_>,
  raster_context: impl Into<EffectRasterContext>,
) {
  let geometry = EffectGeometry {
    paint: PixelBounds {
      left: geometry.paint_left_px,
      top: geometry.paint_top_px,
      right: geometry.paint_left_px + geometry.paint_width_px,
      bottom: geometry.paint_top_px + geometry.paint_height_px,
    },
    shadow_anchor: PixelBounds {
      left: geometry.shadow_anchor_left_px,
      top: geometry.shadow_anchor_top_px,
      right: geometry.shadow_anchor_left_px + geometry.shadow_anchor_width_px,
      bottom: geometry.shadow_anchor_top_px + geometry.shadow_anchor_height_px,
    },
    anchor: PixelBounds {
      left: geometry.anchor_left_px,
      top: geometry.anchor_top_px,
      right: geometry.anchor_left_px + geometry.anchor_width_px,
      bottom: geometry.anchor_top_px + geometry.anchor_height_px,
    },
    ramp: PixelBounds {
      left: geometry.ramp_left_px,
      top: geometry.ramp_top_px,
      right: geometry.ramp_left_px + geometry.ramp_width_px,
      bottom: geometry.ramp_top_px + geometry.ramp_height_px,
    },
  };
  *image = apply_container_with_bounds(
    image,
    container,
    geometry,
    geometry,
    sources,
    raster_context,
  );
}

/// Executes the complete graph on an independently realized source texture.
/// All geometry, external-source bounds and effect lengths are expressed in
/// the same logical canvas; only image dimensions and sampling are device
/// pixels. Images must already be realized directly on this texture lattice.
pub(crate) fn apply_container_to_padded_image_with_sources_on_raster(
  image: &mut image::RgbaImage,
  container: &ImageEffectContainer,
  geometry: ImageEffectSourceGeometry,
  sources: ImageEffectSourceImages<'_>,
  scale: EffectRasterScale,
) -> Option<()> {
  if image.width() == 0
    || image.height() == 0
    || !scale.is_valid()
    || [
      sources.fill,
      sources.line,
      sources.fill_line,
      sources.children,
      sources.effect_mask,
      sources.reflection_paint,
    ]
    .into_iter()
    .flatten()
    .any(|source| source.dimensions() != image.dimensions())
  {
    return None;
  }
  apply_container_to_padded_image_with_sources_and_anchor_and_alpha_outset_scale(
    image,
    container,
    geometry,
    sources,
    EffectRasterContext {
      scale,
      alpha_outset: AlphaOutsetSurfaceScale::default(),
    },
  );
  Some(())
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct EffectOutputBounds {
  pub(crate) left_pt: f32,
  pub(crate) top_pt: f32,
  pub(crate) right_pt: f32,
  pub(crate) bottom_pt: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct EffectBitmapTarget {
  pub(crate) left_px: u32,
  pub(crate) top_px: u32,
  pub(crate) width_px: u32,
  pub(crate) height_px: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum EffectBitmapExtentRounding {
  Truncate,
  Nearest,
  /// Round to nearest while treating a floating representation of an exact
  /// half-pixel as a tie. This is reserved for fixed-output adapters whose
  /// independently quantized point extent can land microscopically below
  /// `.5` after conversion to device pixels.
  NearestTiesUp,
  Ceil,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum EffectBitmapOffsetRounding {
  Floor,
  Nearest,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct EffectBitmapTargetRounding {
  pub(crate) offset_x: EffectBitmapOffsetRounding,
  pub(crate) offset_y: EffectBitmapOffsetRounding,
  pub(crate) extent: EffectBitmapExtentRounding,
}

/// Maps an effect graph's independent output rectangle onto the working
/// bitmap which also contains the unchanged source needed to evaluate it.
/// Direct2D truncates the output extent after conversion to device pixels;
/// the continuous graph origin remains the placement coordinate.
pub(crate) fn effect_bitmap_target(
  output_bounds: EffectOutputBounds,
  working_bounds: EffectOutputBounds,
  pixels_per_point: f32,
  working_width_px: u32,
  working_height_px: u32,
) -> Option<EffectBitmapTarget> {
  effect_bitmap_target_with_rounding(
    output_bounds,
    working_bounds,
    pixels_per_point,
    working_width_px,
    working_height_px,
    EffectBitmapExtentRounding::Truncate,
  )
}

pub(crate) fn effect_bitmap_target_with_rounding(
  output_bounds: EffectOutputBounds,
  working_bounds: EffectOutputBounds,
  pixels_per_point: f32,
  working_width_px: u32,
  working_height_px: u32,
  extent_rounding: EffectBitmapExtentRounding,
) -> Option<EffectBitmapTarget> {
  effect_bitmap_target_with_rounding_modes(
    output_bounds,
    working_bounds,
    pixels_per_point,
    working_width_px,
    working_height_px,
    EffectBitmapTargetRounding {
      offset_x: EffectBitmapOffsetRounding::Floor,
      offset_y: EffectBitmapOffsetRounding::Floor,
      extent: extent_rounding,
    },
  )
}

pub(crate) fn effect_bitmap_target_with_rounding_modes(
  output_bounds: EffectOutputBounds,
  working_bounds: EffectOutputBounds,
  pixels_per_point: f32,
  working_width_px: u32,
  working_height_px: u32,
  rounding: EffectBitmapTargetRounding,
) -> Option<EffectBitmapTarget> {
  if !pixels_per_point.is_finite()
    || pixels_per_point <= 0.0
    || working_width_px == 0
    || working_height_px == 0
  {
    return None;
  }

  let round_extent = |value: f32| match rounding.extent {
    EffectBitmapExtentRounding::Truncate => value as u32,
    EffectBitmapExtentRounding::Nearest => value.round() as u32,
    EffectBitmapExtentRounding::NearestTiesUp => {
      let lower = value.floor();
      let fraction = value - lower;
      let half_tolerance = f32::EPSILON * value.abs().max(1.0) * 8.0;
      if (fraction - 0.5).abs() <= half_tolerance {
        (lower + 1.0) as u32
      } else {
        value.round() as u32
      }
    }
    EffectBitmapExtentRounding::Ceil => value.ceil() as u32,
  };
  let round_offset = |value: f32, mode: EffectBitmapOffsetRounding| match mode {
    EffectBitmapOffsetRounding::Floor => value.floor(),
    EffectBitmapOffsetRounding::Nearest => value.round(),
  };
  let output_width_px =
    round_extent(((output_bounds.right_pt - output_bounds.left_pt) * pixels_per_point).max(0.0));
  let output_height_px =
    round_extent(((output_bounds.bottom_pt - output_bounds.top_pt) * pixels_per_point).max(0.0));
  if output_width_px == 0 || output_height_px == 0 {
    return None;
  }

  let left_px = round_offset(
    (output_bounds.left_pt - working_bounds.left_pt) * pixels_per_point,
    rounding.offset_x,
  )
  .clamp(0.0, working_width_px.saturating_sub(1) as f32) as u32;
  let top_px = round_offset(
    (output_bounds.top_pt - working_bounds.top_pt) * pixels_per_point,
    rounding.offset_y,
  )
  .clamp(0.0, working_height_px.saturating_sub(1) as f32) as u32;
  let width_px = output_width_px.min(working_width_px - left_px);
  let height_px = output_height_px.min(working_height_px - top_px);
  (width_px > 0 && height_px > 0).then_some(EffectBitmapTarget {
    left_px,
    top_px,
    width_px,
    height_px,
  })
}

/// Clips the source-facing transparent border of a Word reflection surface.
///
/// The reflection alpha ramp starts at the edge nearest the source. A soft
/// Gaussian border may extend back into the authored `dist` gap, but it must
/// not cross the source edge. The far edge and both perpendicular borders
/// retain the fixed-output surface guard. This is expressed as a half-plane
/// so non-cardinal `dir`/`fadeDir` combinations do not need special cases.
pub(crate) fn wordprocessing_reflection_canvas_bounds(
  canvas: EffectOutputBounds,
  reflection: EffectOutputBounds,
  blur_radius_px: f32,
  distance_px: f32,
  direction_degrees: f32,
  fade_direction_degrees: f32,
) -> EffectOutputBounds {
  let fade = fade_direction_degrees.to_radians();
  let fade_x = fade.cos();
  let fade_y = fade.sin();
  let direction = direction_degrees.to_radians();
  let css_pixels_to_points = 72.0 / 96.0;
  let gap_pt =
    (distance_px * (direction.cos() * fade_x + direction.sin() * fade_y) * css_pixels_to_points)
      .max(0.0);
  // Direct2D's soft-border Gaussian output support is three standard
  // deviations on either side. Word clips the near support to the available
  // source/reflection gap.
  let near_support_pt = (blur_radius_px * 3.0 * css_pixels_to_points).min(gap_pt);
  let reflection_corners = [
    (reflection.left_pt, reflection.top_pt),
    (reflection.right_pt, reflection.top_pt),
    (reflection.right_pt, reflection.bottom_pt),
    (reflection.left_pt, reflection.bottom_pt),
  ];
  let minimum_projection = reflection_corners
    .iter()
    .map(|(x, y)| fade_x * *x + fade_y * *y)
    .fold(f32::INFINITY, f32::min)
    - near_support_pt;
  let mut polygon = vec![
    (canvas.left_pt, canvas.top_pt),
    (canvas.right_pt, canvas.top_pt),
    (canvas.right_pt, canvas.bottom_pt),
    (canvas.left_pt, canvas.bottom_pt),
  ];
  let mut clipped = Vec::with_capacity(6);
  for index in 0..polygon.len() {
    let current = polygon[index];
    let previous = polygon[(index + polygon.len() - 1) % polygon.len()];
    let current_distance = fade_x * current.0 + fade_y * current.1 - minimum_projection;
    let previous_distance = fade_x * previous.0 + fade_y * previous.1 - minimum_projection;
    let current_inside = current_distance >= 0.0;
    let previous_inside = previous_distance >= 0.0;
    if current_inside != previous_inside {
      let denominator = previous_distance - current_distance;
      let ratio = if denominator.abs() <= f32::EPSILON {
        0.0
      } else {
        previous_distance / denominator
      };
      clipped.push((
        previous.0 + (current.0 - previous.0) * ratio,
        previous.1 + (current.1 - previous.1) * ratio,
      ));
    }
    if current_inside {
      clipped.push(current);
    }
  }
  if clipped.is_empty() {
    return reflection;
  }
  polygon = clipped;
  EffectOutputBounds {
    left_pt: polygon
      .iter()
      .map(|point| point.0)
      .fold(f32::INFINITY, f32::min),
    top_pt: polygon
      .iter()
      .map(|point| point.1)
      .fold(f32::INFINITY, f32::min),
    right_pt: polygon
      .iter()
      .map(|point| point.0)
      .fold(f32::NEG_INFINITY, f32::max),
    bottom_pt: polygon
      .iter()
      .map(|point| point.1)
      .fold(f32::NEG_INFINITY, f32::max),
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct PixelBounds {
  left: f32,
  top: f32,
  right: f32,
  bottom: f32,
}

impl PixelBounds {
  fn width(self) -> f32 {
    (self.right - self.left).max(0.0)
  }

  fn height(self) -> f32 {
    (self.bottom - self.top).max(0.0)
  }

  fn outset(self, amount: f32) -> Self {
    if amount < 0.0 {
      let inset = (-amount).min(self.width() * 0.5).min(self.height() * 0.5);
      return Self {
        left: self.left + inset,
        top: self.top + inset,
        right: self.right - inset,
        bottom: self.bottom - inset,
      };
    }
    Self {
      left: self.left - amount,
      top: self.top - amount,
      right: self.right + amount,
      bottom: self.bottom + amount,
    }
  }

  fn union(self, other: Self) -> Self {
    Self {
      left: self.left.min(other.left),
      top: self.top.min(other.top),
      right: self.right.max(other.right),
      bottom: self.bottom.max(other.bottom),
    }
  }

  fn translated(self, x: f32, y: f32) -> Self {
    Self {
      left: self.left + x,
      top: self.top + y,
      right: self.right + x,
      bottom: self.bottom + y,
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct EffectGeometry {
  /// Bounds of pixels which can contribute alpha to the current branch.
  paint: PixelBounds,
  /// Logical rectangle used specifically as the outer-shadow transform
  /// origin. It normally equals `anchor`, but scene-hosted Word text retains
  /// the complete input-cell span between its laid-out foreground and its
  /// displaced effect source while reflection follows the projected cell.
  shadow_anchor: PixelBounds,
  /// Logical shape/text rectangle used by DrawingML alignment and percentage
  /// offsets. This deliberately does not collapse to the glyph ink box.
  anchor: PixelBounds,
  /// Rectangle along which a reflection's alpha ramp is measured. Shapes use
  /// their anchor box; Word text uses its DirectWrite default-baseline box,
  /// which is distinct from tight glyph ink and the paragraph line-height cell.
  ramp: PixelBounds,
}

impl EffectGeometry {
  fn union(self, other: Self) -> Self {
    Self {
      paint: self.paint.union(other.paint),
      shadow_anchor: self.shadow_anchor.union(other.shadow_anchor),
      anchor: self.anchor.union(other.anchor),
      ramp: self.ramp.union(other.ramp),
    }
  }

  fn translated(self, x: f32, y: f32) -> Self {
    Self {
      paint: self.paint.translated(x, y),
      shadow_anchor: self.shadow_anchor.translated(x, y),
      anchor: self.anchor.translated(x, y),
      ramp: self.ramp.translated(x, y),
    }
  }
}

fn reflection_source_geometry(
  reference: ReflectionReference,
  source: EffectGeometry,
  root_source: EffectGeometry,
) -> EffectGeometry {
  match reference {
    ReflectionReference::EffectInput => source,
    ReflectionReference::RootText => EffectGeometry {
      // W14 reflects the completed visible paint, but its `algn`, `dist`, and
      // fade coordinates remain relative to the owning text rather than a
      // glow/shadow branch's expanded output rectangle.
      paint: source.paint,
      shadow_anchor: root_source.shadow_anchor,
      anchor: root_source.anchor,
      ramp: root_source.ramp,
    },
    ReflectionReference::WordRunMetrics {
      ascent_px,
      ramp_extension_px,
    } => {
      let (top, bottom) = match ascent_px {
        Some(ascent) => (
          root_source.anchor.bottom - ascent,
          root_source.anchor.bottom,
        ),
        None => (root_source.paint.top, root_source.paint.bottom),
      };
      // GEL keeps completed paint horizontally and the owner's reference
      // vertically. A previous spatial sibling does not own the root Y range.
      let reference = PixelBounds {
        left: source.paint.left,
        right: source.paint.right,
        top,
        bottom,
      };
      // Word extends the fade reference before transforming it, independently
      // of the full-distance paint translation and the unchanged alignment pivot.
      let ramp = PixelBounds {
        bottom: reference.bottom + ramp_extension_px,
        ..reference
      };
      EffectGeometry {
        paint: source.paint,
        shadow_anchor: root_source.shadow_anchor,
        anchor: reference,
        ramp,
      }
    }
  }
}

/// Computes the authored effect graph's output range relative to the source
/// shape. Lengths are evaluated at DrawingML's 96-DPI bitmap baseline and
/// converted back to points for the bounded full-color raster.
pub(crate) fn container_output_bounds(
  container: &ImageEffectContainer,
  width_pt: f32,
  height_pt: f32,
) -> Option<EffectOutputBounds> {
  let source = EffectOutputBounds {
    left_pt: 0.0,
    top_pt: 0.0,
    right_pt: width_pt,
    bottom_pt: height_pt,
  };
  container_output_bounds_with_anchor(container, source, source)
}

/// Computes effect output bounds while keeping the painted source and the
/// logical alignment rectangle independent.
///
/// Text is the important counterexample: Win32 text extents describe
/// character cells while glyph outlines describe ink. DrawingML `algn`,
/// scale, skew, and relative offsets operate on the former; glow/blur alpha
/// originates from the latter. Shape callers which have one rectangle can use
/// [`container_output_bounds`].
pub(crate) fn container_output_bounds_with_anchor(
  container: &ImageEffectContainer,
  source: EffectOutputBounds,
  anchor: EffectOutputBounds,
) -> Option<EffectOutputBounds> {
  container_output_bounds_with_anchors(container, source, anchor, anchor)
}

/// Computes output bounds with independent outer-shadow and general effect
/// alignment rectangles. Most DrawingML callers use one rectangle for both;
/// scene-hosted Word text is the counterexample because its W14 shadow uses
/// the complete input-cell span while reflection follows the projected plane.
pub(crate) fn container_output_bounds_with_anchors(
  container: &ImageEffectContainer,
  source: EffectOutputBounds,
  anchor: EffectOutputBounds,
  shadow_anchor: EffectOutputBounds,
) -> Option<EffectOutputBounds> {
  container_output_bounds_with_anchors_and_ramp(container, source, anchor, shadow_anchor, anchor)
}

/// Computes output bounds with an independent reflection alpha-ramp box.
///
/// Word run effects measure `stPos`/`endPos` over the face's DirectWrite
/// default-baseline height while affine alignment uses the paragraph character
/// cell and paint comes from glyph or projected 3-D alpha. Shape callers
/// normally have one rectangle for all three roles and can use the simpler
/// wrappers above.
pub(crate) fn container_output_bounds_with_anchors_and_ramp(
  container: &ImageEffectContainer,
  source: EffectOutputBounds,
  anchor: EffectOutputBounds,
  shadow_anchor: EffectOutputBounds,
  ramp: EffectOutputBounds,
) -> Option<EffectOutputBounds> {
  container_output_bounds_with_sources(
    container,
    source,
    anchor,
    shadow_anchor,
    ramp,
    ImageEffectSourceBounds::default(),
  )
}

pub(crate) fn container_output_bounds_with_sources(
  container: &ImageEffectContainer,
  source: EffectOutputBounds,
  anchor: EffectOutputBounds,
  shadow_anchor: EffectOutputBounds,
  ramp: EffectOutputBounds,
  sources: ImageEffectSourceBounds,
) -> Option<EffectOutputBounds> {
  container_bounds_with_sources(
    container,
    source,
    anchor,
    shadow_anchor,
    ramp,
    sources,
    false,
  )
}

/// Continuous drawable bounds used before scene centering. Bitmap terminal
/// samples belong to allocation after projection, not to this geometric union.
/// Keep primitive source bounds and the actual filter support unchanged.
pub(crate) fn container_scene_bounds_with_sources(
  container: &ImageEffectContainer,
  source: EffectOutputBounds,
  anchor: EffectOutputBounds,
  shadow_anchor: EffectOutputBounds,
  ramp: EffectOutputBounds,
  sources: ImageEffectSourceBounds,
) -> Option<EffectOutputBounds> {
  container_bounds_with_sources(
    container,
    source,
    anchor,
    shadow_anchor,
    ramp,
    sources,
    true,
  )
}

fn container_bounds_with_sources(
  container: &ImageEffectContainer,
  source: EffectOutputBounds,
  anchor: EffectOutputBounds,
  shadow_anchor: EffectOutputBounds,
  ramp: EffectOutputBounds,
  sources: ImageEffectSourceBounds,
  scene_geometry: bool,
) -> Option<EffectOutputBounds> {
  let css_pixels_per_point = 96.0 / 72.0;
  let geometry = EffectGeometry {
    paint: PixelBounds {
      left: source.left_pt * css_pixels_per_point,
      top: source.top_pt * css_pixels_per_point,
      right: source.right_pt * css_pixels_per_point,
      bottom: source.bottom_pt * css_pixels_per_point,
    },
    shadow_anchor: PixelBounds {
      left: shadow_anchor.left_pt * css_pixels_per_point,
      top: shadow_anchor.top_pt * css_pixels_per_point,
      right: shadow_anchor.right_pt * css_pixels_per_point,
      bottom: shadow_anchor.bottom_pt * css_pixels_per_point,
    },
    anchor: PixelBounds {
      left: anchor.left_pt * css_pixels_per_point,
      top: anchor.top_pt * css_pixels_per_point,
      right: anchor.right_pt * css_pixels_per_point,
      bottom: anchor.bottom_pt * css_pixels_per_point,
    },
    ramp: PixelBounds {
      left: ramp.left_pt * css_pixels_per_point,
      top: ramp.top_pt * css_pixels_per_point,
      right: ramp.right_pt * css_pixels_per_point,
      bottom: ramp.bottom_pt * css_pixels_per_point,
    },
  };
  let output =
    effect_container_output_geometry(container, geometry, geometry, sources, scene_geometry)?.paint;
  Some(EffectOutputBounds {
    left_pt: output.left / css_pixels_per_point,
    top_pt: output.top / css_pixels_per_point,
    right_pt: output.right / css_pixels_per_point,
    bottom_pt: output.bottom / css_pixels_per_point,
  })
}

fn effect_container_output_geometry(
  container: &ImageEffectContainer,
  source: EffectGeometry,
  root_source: EffectGeometry,
  sources: impl Into<EffectSourceBounds>,
  scene_geometry: bool,
) -> Option<EffectGeometry> {
  let sources = sources.into();
  match container.kind {
    ImageEffectContainerKind::Tree => {
      let mut output = source;
      for effect in &container.effects {
        output =
          effect_output_geometry_for_domain(effect, output, root_source, sources, scene_geometry)?;
      }
      Some(output)
    }
    ImageEffectContainerKind::Sibling => {
      let mut effects = container.effects.iter();
      let first = effect_output_geometry_for_domain(
        effects.next()?,
        source,
        root_source,
        sources,
        scene_geometry,
      )?;
      effects.try_fold(first, |output, effect| {
        Some(output.union(effect_output_geometry_for_domain(
          effect,
          source,
          root_source,
          sources,
          scene_geometry,
        )?))
      })
    }
  }
}

fn effect_output_geometry(
  effect: &ImageEffect,
  source: EffectGeometry,
  root_source: EffectGeometry,
  sources: impl Into<EffectSourceBounds>,
) -> Option<EffectGeometry> {
  effect_output_geometry_for_domain(effect, source, root_source, sources.into(), false)
}

fn effect_output_geometry_for_domain(
  effect: &ImageEffect,
  source: EffectGeometry,
  root_source: EffectGeometry,
  sources: EffectSourceBounds,
  scene_geometry: bool,
) -> Option<EffectGeometry> {
  let allocation_padding = |padding| if scene_geometry { 0.0 } else { padding };
  match effect {
    ImageEffect::AlphaOutset(radius) => Some(EffectGeometry {
      paint: source.paint.outset(*radius),
      shadow_anchor: source.shadow_anchor.outset(*radius),
      anchor: source.anchor.outset(*radius),
      ramp: source.ramp.outset(*radius),
    }),
    ImageEffect::Blur {
      radius_px,
      grow_bounds,
    } => Some(if *grow_bounds {
      EffectGeometry {
        paint: source.paint.outset(*radius_px),
        shadow_anchor: source.shadow_anchor.outset(*radius_px),
        anchor: source.anchor.outset(*radius_px),
        ramp: source.ramp.outset(*radius_px),
      }
    } else {
      source
    }),
    ImageEffect::Glow {
      radius_px,
      bounds_radius_scale,
      bounds_radius_offset_px,
      ..
    } => {
      let radius = radius_px.mul_add(
        *bounds_radius_scale,
        allocation_padding(*bounds_radius_offset_px),
      );
      Some(EffectGeometry {
        paint: source.paint.outset(radius),
        // Glow expands painted alpha, not the owning shape/text alignment
        // rectangles. A following W14 shadow sees the expanded alpha while
        // retaining the original character cell for `algn`, scale and skew.
        shadow_anchor: source.shadow_anchor,
        anchor: source.anchor,
        ramp: source.ramp,
      })
    }
    ImageEffect::OuterShadow {
      blur_radius_px,
      distance_px,
      distance_length_scale,
      bounds_radius_scale,
      bounds_radius_offset_px,
      blur_kernel,
      direction_degrees,
      distance_mode,
      transform,
      alignment,
      ..
    } => {
      let direction = direction_degrees.to_radians();
      let distance = *distance_px * *distance_length_scale;
      let blur_radius = blur_radius_px.mul_add(
        *bounds_radius_scale,
        allocation_padding(*bounds_radius_offset_px),
      );
      let offset_x = direction.cos() * distance;
      let offset_y = direction.sin() * distance;
      let outset = |geometry: EffectGeometry| EffectGeometry {
        paint: geometry.paint.outset(blur_radius),
        shadow_anchor: geometry.shadow_anchor.outset(blur_radius),
        anchor: geometry.anchor.outset(blur_radius),
        ramp: geometry.ramp.outset(blur_radius),
      };
      // Ordinary DrawingML follows Direct2D's documented Shadow -> 2-D Affine
      // graph. Word's W14 wrapper is the stopping counterexample: the complete
      // flat-host `sx=sy x blurRad` Office matrix exposes an independent shadow
      // SMask whose vertical variance stays invariant under `sx=sy`, proving
      // that its physical blur follows the text-plane affine. Keep the bounds
      // order identical to the discrete order in `outer_shadow_image`.
      let transform_input = match blur_kernel {
        ShadowBlurKernel::Direct2dGaussian => outset(source),
        ShadowBlurKernel::WordTextBalanced { .. } => source,
      };
      let (transform_source, post_transform_offset) = match distance_mode {
        ShadowDistanceMode::PostTransformOffset => (transform_input, (offset_x, offset_y)),
        ShadowDistanceMode::PreTransformOffset => {
          (transform_input.translated(offset_x, offset_y), (0.0, 0.0))
        }
      };
      let transformed = transformed_effect_geometry_about(
        transform_source,
        *transform,
        *alignment,
        source.shadow_anchor,
      );
      let transformed = EffectGeometry {
        paint: transformed
          .paint
          .translated(post_transform_offset.0, post_transform_offset.1),
        shadow_anchor: transformed
          .shadow_anchor
          .translated(post_transform_offset.0, post_transform_offset.1),
        anchor: transformed
          .anchor
          .translated(post_transform_offset.0, post_transform_offset.1),
        ramp: transformed
          .ramp
          .translated(post_transform_offset.0, post_transform_offset.1),
      };
      Some(match blur_kernel {
        ShadowBlurKernel::Direct2dGaussian => transformed,
        ShadowBlurKernel::WordTextBalanced { .. } => {
          let alignment_radius = *blur_radius_px * *bounds_radius_scale;
          let alignment_shift = if scene_geometry {
            (0.0, 0.0)
          } else {
            word_text_balanced_output_alignment_shift(alignment_radius, *transform, *alignment)
          };
          outset(transformed).translated(alignment_shift.0, alignment_shift.1)
        }
      })
    }
    ImageEffect::Reflection(reflection) => {
      let direction = reflection.direction_degrees.to_radians();
      // Direct2D's soft Gaussian output grows by one kernel radius (`3σ`) on
      // every side. DrawingML names `blurRad` as that radius. Word may
      // independently normalize the sampled kernel, but retains this authored
      // output range for run-level reflection.
      let distance = reflection.distance_px * reflection.distance_length_scale;
      let offset_x = direction.cos() * distance;
      let offset_y = direction.sin() * distance;
      let mut transform_source =
        reflection_source_geometry(reflection.reference, source, root_source);
      let (alignment_bounds, post_transform_offset) = match reflection.distance_mode {
        ReflectionDistanceMode::PostTransformOffset => {
          (transform_source.anchor, (offset_x, offset_y))
        }
        ReflectionDistanceMode::AlignmentPivot => {
          // Word's run-reflection ramp includes the interval from the source
          // edge to the distance-shifted alignment pivot. This is observable
          // independently from paint: constant-opacity controls move painted
          // alpha by `(I - A) * dist`, while gradient controls extend the
          // transformed ramp by the untransformed distance. A completed glow
          // or shadow remains part of the reflected paint, but MS-DOCX defines
          // alignment and fade coordinates relative to the text. Do not let a
          // spatial sibling replace those root coordinate domains.
          transform_source.ramp = transform_source
            .ramp
            .union(transform_source.ramp.translated(offset_x, offset_y));
          (
            transform_source.anchor.translated(offset_x, offset_y),
            (0.0, 0.0),
          )
        }
      };
      let transformed = transformed_effect_geometry_about(
        transform_source,
        reflection.transform,
        reflection.alignment,
        alignment_bounds,
      );
      Some(EffectGeometry {
        // `stPos`/`endPos` measure opacity over the independent ramp, but a
        // fully transparent ramp tail does not reserve output allocation.
        // Office's isolated reflection masks retain only transformed source
        // paint plus their separately owned terminal samples.
        paint: transformed
          .paint
          .translated(post_transform_offset.0, post_transform_offset.1)
          .outset(reflection.blur_radius_px.mul_add(
            reflection.bounds_radius_scale,
            allocation_padding(reflection.bounds_radius_offset_px),
          )),
        shadow_anchor: transformed
          .shadow_anchor
          .translated(post_transform_offset.0, post_transform_offset.1),
        anchor: transformed
          .anchor
          .translated(post_transform_offset.0, post_transform_offset.1),
        ramp: transformed
          .ramp
          .translated(post_transform_offset.0, post_transform_offset.1),
      })
    }
    ImageEffect::RelativeOffset { offset_x, offset_y } => {
      let x = *offset_x * source.anchor.width();
      let y = *offset_y * source.anchor.height();
      Some(EffectGeometry {
        paint: source.paint.translated(x, y),
        shadow_anchor: source.shadow_anchor.translated(x, y),
        anchor: source.anchor.translated(x, y),
        ramp: source.ramp.translated(x, y),
      })
    }
    ImageEffect::Transform(transform) => {
      Some(transformed_effect_geometry(source, *transform, (0.0, 0.0)))
    }
    ImageEffect::Container(container) => {
      effect_container_output_geometry(container, source, root_source, sources, scene_geometry)
    }
    ImageEffect::Blend { container, .. } => Some(source.union(effect_container_output_geometry(
      container,
      source,
      root_source,
      sources,
      scene_geometry,
    )?)),
    ImageEffect::SourceReference(reference) => {
      let bounds = sources.get(*reference);
      Some(bounds.map_or(root_source, |bounds| EffectGeometry {
        paint: bounds,
        // A realized image replaces paint, not text alignment/fade ownership.
        ..root_source
      }))
    }
    // alphaMod uses the nested graph only as an alpha multiplier; its output
    // range remains the input range.
    ImageEffect::AlphaModulate(_) => Some(source),
    ImageEffect::AlphaBiLevel(_)
    | ImageEffect::AlphaCeiling
    | ImageEffect::AlphaFloor
    | ImageEffect::AlphaInverse(_)
    | ImageEffect::AlphaModulateFixed(_)
    | ImageEffect::AlphaReplace(_)
    | ImageEffect::BiLevel(_)
    | ImageEffect::ColorChange(_)
    | ImageEffect::ColorReplacement(_)
    | ImageEffect::Duotone(_, _)
    | ImageEffect::FillOverlay { .. }
    | ImageEffect::Fill(_)
    | ImageEffect::Grayscale
    | ImageEffect::Hsl { .. }
    | ImageEffect::Identity
    | ImageEffect::InnerShadow { .. }
    | ImageEffect::Luminance { .. }
    | ImageEffect::SoftEdge(_)
    | ImageEffect::Tint { .. } => Some(source),
  }
}

fn transformed_effect_bounds(
  source: PixelBounds,
  anchor: PixelBounds,
  transform: ImageEffectTransform,
  alignment: (f32, f32),
) -> PixelBounds {
  let anchor_x = anchor.left + anchor.width() * alignment.0;
  let anchor_y = anchor.top + anchor.height() * alignment.1;
  let point = |x: f32, y: f32| {
    let local_x = x - anchor_x;
    let local_y = y - anchor_y;
    (
      transform
        .scale_x
        .mul_add(local_x, transform.skew_x * local_y)
        + anchor_x
        + transform.shift_x_px,
      transform
        .skew_y
        .mul_add(local_x, transform.scale_y * local_y)
        + anchor_y
        + transform.shift_y_px,
    )
  };
  let corners = [
    point(source.left, source.top),
    point(source.right, source.top),
    point(source.right, source.bottom),
    point(source.left, source.bottom),
  ];
  PixelBounds {
    left: corners
      .iter()
      .map(|point| point.0)
      .fold(f32::INFINITY, f32::min),
    top: corners
      .iter()
      .map(|point| point.1)
      .fold(f32::INFINITY, f32::min),
    right: corners
      .iter()
      .map(|point| point.0)
      .fold(f32::NEG_INFINITY, f32::max),
    bottom: corners
      .iter()
      .map(|point| point.1)
      .fold(f32::NEG_INFINITY, f32::max),
  }
}

/// Reattaches Word's physical run-shadow blur range to the affine output.
///
/// Direct2D reports the affine input's transformed bounding box and the soft
/// Gaussian grows that box by one radius on every side. Word additionally
/// preserves the requested `algn` point while it maps the padded filter image
/// back to the run rectangle. The complete flat-host `sx=sy x blurRad` Office
/// matrix separates this output-origin term from both the affine and the
/// Gaussian: for left alignment the near edge grows by `scale * radius`, the
/// far edge grows by `(2 - scale) * radius`, and center alignment is the
/// stopping control. Expressing that rectangle mapping as `(I - A) * bias`
/// also extends the same contract to non-uniform scale and skew without an
/// axis-specific placement adjustment.
fn word_text_balanced_output_alignment_shift(
  blur_radius_px: f32,
  transform: ImageEffectTransform,
  alignment: (f32, f32),
) -> (f32, f32) {
  if !blur_radius_px.is_finite() || blur_radius_px <= f32::EPSILON {
    return (0.0, 0.0);
  }

  let bias_x = blur_radius_px * (1.0 - 2.0 * alignment.0);
  let bias_y = blur_radius_px * (1.0 - 2.0 * alignment.1);
  let transformed_bias_x = transform.scale_x.mul_add(bias_x, transform.skew_x * bias_y);
  let transformed_bias_y = transform.skew_y.mul_add(bias_x, transform.scale_y * bias_y);
  let shift = (bias_x - transformed_bias_x, bias_y - transformed_bias_y);
  if shift.0.is_finite() && shift.1.is_finite() {
    shift
  } else {
    (0.0, 0.0)
  }
}

fn transformed_effect_geometry(
  source: EffectGeometry,
  transform: ImageEffectTransform,
  alignment: (f32, f32),
) -> EffectGeometry {
  transformed_effect_geometry_about(source, transform, alignment, source.anchor)
}

fn transformed_effect_geometry_about(
  source: EffectGeometry,
  transform: ImageEffectTransform,
  alignment: (f32, f32),
  alignment_bounds: PixelBounds,
) -> EffectGeometry {
  EffectGeometry {
    paint: transformed_effect_bounds(source.paint, alignment_bounds, transform, alignment),
    shadow_anchor: transformed_effect_bounds(
      source.shadow_anchor,
      alignment_bounds,
      transform,
      alignment,
    ),
    anchor: transformed_effect_bounds(source.anchor, alignment_bounds, transform, alignment),
    ramp: transformed_effect_bounds(source.ramp, alignment_bounds, transform, alignment),
  }
}

/// Converts the parser's CSS-pixel DrawingML length baseline (96 DPI) to the
/// actual bounded shape-raster resolution.
pub(crate) fn scale_container_pixel_lengths(container: &mut ImageEffectContainer, scale: f32) {
  if !scale.is_finite() || scale <= 0.0 || (scale - 1.0).abs() <= f32::EPSILON {
    return;
  }
  for effect in &mut container.effects {
    match effect {
      ImageEffect::AlphaOutset(radius)
      | ImageEffect::SoftEdge(radius)
      | ImageEffect::Blur {
        radius_px: radius, ..
      } => *radius *= scale,
      ImageEffect::Glow {
        radius_px,
        bounds_radius_offset_px,
        ..
      } => {
        *radius_px *= scale;
        *bounds_radius_offset_px *= scale;
      }
      ImageEffect::InnerShadow {
        blur_radius_px,
        distance_px,
        ..
      } => {
        *blur_radius_px *= scale;
        *distance_px *= scale;
      }
      ImageEffect::OuterShadow {
        blur_radius_px,
        distance_px,
        bounds_radius_offset_px,
        transform,
        ..
      } => {
        *blur_radius_px *= scale;
        *distance_px *= scale;
        *bounds_radius_offset_px *= scale;
        transform.shift_x_px *= scale;
        transform.shift_y_px *= scale;
      }
      ImageEffect::Reflection(reflection) => {
        reflection.blur_radius_px *= scale;
        if let ReflectionReference::WordRunMetrics {
          ascent_px,
          ramp_extension_px,
        } = &mut reflection.reference
        {
          if let Some(ascent) = ascent_px {
            *ascent *= scale;
          }
          *ramp_extension_px *= scale;
        }
        reflection.bounds_radius_offset_px *= scale;
        reflection.distance_px *= scale;
        reflection.transform.shift_x_px *= scale;
        reflection.transform.shift_y_px *= scale;
      }
      ImageEffect::Transform(transform) => {
        transform.shift_x_px *= scale;
        transform.shift_y_px *= scale;
      }
      ImageEffect::AlphaModulate(container)
      | ImageEffect::Blend { container, .. }
      | ImageEffect::Container(container) => scale_container_pixel_lengths(container, scale),
      ImageEffect::AlphaBiLevel(_)
      | ImageEffect::AlphaCeiling
      | ImageEffect::AlphaFloor
      | ImageEffect::AlphaInverse(_)
      | ImageEffect::AlphaModulateFixed(_)
      | ImageEffect::AlphaReplace(_)
      | ImageEffect::BiLevel(_)
      | ImageEffect::ColorChange(_)
      | ImageEffect::ColorReplacement(_)
      | ImageEffect::Duotone(_, _)
      | ImageEffect::Grayscale
      | ImageEffect::Hsl { .. }
      | ImageEffect::Identity
      | ImageEffect::Luminance { .. }
      | ImageEffect::RelativeOffset { .. }
      | ImageEffect::SourceReference(_)
      | ImageEffect::Tint { .. } => {}
      ImageEffect::FillOverlay { fill, .. } | ImageEffect::Fill(fill) => {
        if let ImageEffectFill::Pattern { tile_px, .. } = fill {
          *tile_px *= scale;
        }
      }
    }
  }
}

pub(crate) fn scale_glow_filter_radius(container: &mut ImageEffectContainer, scale: f32) {
  if !scale.is_finite() || scale <= 0.0 || (scale - 1.0).abs() <= f32::EPSILON {
    return;
  }
  for effect in &mut container.effects {
    match effect {
      ImageEffect::Glow {
        raster_length_scale,
        ..
      } => *raster_length_scale *= scale,
      ImageEffect::AlphaModulate(container) | ImageEffect::Container(container) => {
        scale_glow_filter_radius(container, scale);
      }
      ImageEffect::Blend { container, .. } => {
        scale_glow_filter_radius(container, scale);
      }
      _ => {}
    }
  }
}

/// Scales only the outer-shadow blur kernel while retaining the authored
/// effect bounds. Office fixed output uses this distinction for chart
/// effects: the XObject canvas still reserves `blurRad`, but its sampled
/// shadow edge is narrower than that full extent.
pub(crate) fn scale_outer_shadow_filter_radius(container: &mut ImageEffectContainer, scale: f32) {
  if !scale.is_finite() || scale <= 0.0 || (scale - 1.0).abs() <= f32::EPSILON {
    return;
  }
  for effect in &mut container.effects {
    match effect {
      ImageEffect::OuterShadow {
        raster_length_scale,
        ..
      } => *raster_length_scale *= scale,
      ImageEffect::AlphaModulate(container)
      | ImageEffect::Blend { container, .. }
      | ImageEffect::Container(container) => {
        scale_outer_shadow_filter_radius(container, scale);
      }
      _ => {}
    }
  }
}

/// Quantizes an outer shadow's geometric support to a fixed raster device.
///
/// WPF's `CMilBlurEffectDuce::GetScaledRadius` converts the local blur radius
/// to `UINT` device pixels, while `CMilDropShadowEffectDuce::ApplyEffectSw`
/// converts each scaled offset component to `int`. Word's 200-DPI legacy
/// drawing surfaces expose the same integer boundary. Convert those device
/// values back to the DrawingML 96-DPI coordinate baseline so the existing
/// effect graph can continue to compose them with transforms and siblings.
pub(crate) fn quantize_outer_shadow_geometry_for_raster(
  container: &mut ImageEffectContainer,
  device_dpi: f32,
) {
  let device_pixels_per_css_pixel = device_dpi / crate::units::CSS_PIXELS_PER_INCH;
  if !device_pixels_per_css_pixel.is_finite() || device_pixels_per_css_pixel <= f32::EPSILON {
    return;
  }
  for effect in &mut container.effects {
    match effect {
      ImageEffect::OuterShadow {
        blur_radius_px,
        distance_px,
        distance_length_scale,
        bounds_radius_scale,
        direction_degrees,
        ..
      } => {
        let effective_radius_scale = *bounds_radius_scale;
        if effective_radius_scale.abs() > f32::EPSILON {
          let local_radius = (*blur_radius_px * effective_radius_scale).trunc();
          let device_radius = (local_radius * device_pixels_per_css_pixel).trunc();
          *blur_radius_px = device_radius / device_pixels_per_css_pixel / effective_radius_scale;
        }

        let effective_distance_scale = *distance_length_scale;
        if effective_distance_scale.abs() <= f32::EPSILON {
          *distance_px = 0.0;
          continue;
        }
        let direction = direction_degrees.to_radians();
        let effective_distance = *distance_px * effective_distance_scale;
        let offset_x = (direction.cos() * effective_distance * device_pixels_per_css_pixel).trunc()
          / device_pixels_per_css_pixel;
        let offset_y = (direction.sin() * effective_distance * device_pixels_per_css_pixel).trunc()
          / device_pixels_per_css_pixel;
        let quantized_distance = offset_x.hypot(offset_y);
        *distance_px = quantized_distance / effective_distance_scale.abs();
        if quantized_distance > f32::EPSILON {
          *direction_degrees = offset_y.atan2(offset_x).to_degrees().rem_euclid(360.0);
        }
      }
      ImageEffect::AlphaModulate(container)
      | ImageEffect::Blend { container, .. }
      | ImageEffect::Container(container) => {
        quantize_outer_shadow_geometry_for_raster(container, device_dpi);
      }
      _ => {}
    }
  }
}

/// Uses Word's complete WPG glow alpha graph.
///
/// A pre-registered Office matrix over eight radii and eight disjoint source
/// topologies makes built-in `glow(R)` byte-identical to the explicit chain
/// `alphaOutset(R/2) -> blur(R/2, grow=1)` in all 64 pairs.  The alpha-outset
/// stage is kept distinct from the later public Gaussian blur because their
/// numerical kernels are independently observable.
pub(crate) fn use_word_group_glow_profile(container: &mut ImageEffectContainer) {
  for effect in &mut container.effects {
    match effect {
      ImageEffect::Glow {
        spread_ratio,
        spread_kernel,
        spread_radius_rounding,
        blur_kernel,
        ..
      } => {
        *spread_ratio = 0.5;
        *spread_kernel = GlowSpreadKernel::AlphaOutset;
        *spread_radius_rounding = GlowSpreadRadiusRounding::Inward;
        *blur_kernel = GlowBlurKernel::WordGroupGaussian;
      }
      ImageEffect::AlphaModulate(container)
      | ImageEffect::Blend { container, .. }
      | ImageEffect::Container(container) => use_word_group_glow_profile(container),
      _ => {}
    }
  }
}

/// Uses Word's standalone WPS glow alpha graph.
///
/// Independent fixed-output radius, alpha, and source-coverage controls make
/// this the same explicit `alphaOutset(R/2) -> blur(R/2, grow=1)` topology as a
/// WPG glow, but with a distinct public Gaussian. The WPS Gaussian derives its
/// sigma from the unquantized authored radius rather than from its integer
/// support. Keeping it separate prevents WPG and shape-host calibration from
/// changing one another.
pub(crate) fn use_word_shape_glow_profile(container: &mut ImageEffectContainer) {
  for effect in &mut container.effects {
    match effect {
      ImageEffect::Glow {
        spread_ratio,
        spread_kernel,
        spread_radius_rounding,
        blur_kernel,
        ..
      } => {
        *spread_ratio = 0.5;
        *spread_kernel = GlowSpreadKernel::AlphaOutset;
        *spread_radius_rounding = GlowSpreadRadiusRounding::Inward;
        *blur_kernel = GlowBlurKernel::WordShapeGaussian;
      }
      ImageEffect::AlphaModulate(container)
      | ImageEffect::Blend { container, .. }
      | ImageEffect::Container(container) => use_word_shape_glow_profile(container),
      _ => {}
    }
  }
}

/// Binds independently visible W14 glows to a caller-realized coverage plane.
///
/// The reflected object contains a copy of the visible glow, but its Identity
/// paint still comes from the original flat image. Shadow-of-glow has a separate
/// source/filter owner and must not be rebound. Apply this to the generated W14
/// graph, not to arbitrary authored effect DAGs.
pub(crate) fn bind_wordprocessing_glow_mask(container: &mut ImageEffectContainer) {
  if matches!(
    container.effects.as_slice(),
    [
      ImageEffect::SourceReference(ImageEffectSourceReference::EffectMask),
      ImageEffect::Glow { .. },
    ]
  ) && container.kind == ImageEffectContainerKind::Tree
  {
    return;
  }
  if container.kind == ImageEffectContainerKind::Tree
    && matches!(
      container.effects.last(),
      Some(ImageEffect::OuterShadow { .. })
    )
  {
    return;
  }
  for effect in &mut container.effects {
    match effect {
      ImageEffect::Glow { blur_kernel, .. } => {
        *blur_kernel = GlowBlurKernel::WordShapeGaussian;
        let glow = std::mem::replace(effect, ImageEffect::Identity);
        *effect = ImageEffect::Container(ImageEffectContainer {
          kind: ImageEffectContainerKind::Tree,
          effects: vec![
            ImageEffect::SourceReference(ImageEffectSourceReference::EffectMask),
            glow,
          ],
        });
      }
      ImageEffect::AlphaModulate(nested)
      | ImageEffect::Blend {
        container: nested, ..
      }
      | ImageEffect::Container(nested) => bind_wordprocessing_glow_mask(nested),
      _ => {}
    }
  }
}

/// Bind only the painted Identity of Word's generated reflection source.
/// Nested shadow-of-glow Identity leaves must keep the original coverage,
/// as must the upright sibling branches. Binding is idempotent.
pub(crate) fn bind_wordprocessing_reflection_paint(container: &mut ImageEffectContainer) {
  for effect in &mut container.effects {
    let ImageEffect::Container(branch) = effect else {
      continue;
    };
    if branch.kind != ImageEffectContainerKind::Tree
      || !matches!(branch.effects.last(), Some(ImageEffect::Reflection(_)))
    {
      continue;
    }
    let Some(ImageEffect::Container(source)) = branch.effects.first_mut() else {
      continue;
    };
    if source.kind != ImageEffectContainerKind::Sibling {
      continue;
    }
    for effect in &mut source.effects {
      if matches!(effect, ImageEffect::Identity) {
        *effect = ImageEffect::SourceReference(ImageEffectSourceReference::ReflectionPaint);
      }
    }
  }
}

/// Applies the host shape orientation to effects whose `rotWithShape` value is
/// true (the DrawingML default for outer shadow and reflection).
pub(crate) fn rotate_container_with_shape(
  container: &mut ImageEffectContainer,
  rotation_degrees: f32,
) {
  if !rotation_degrees.is_finite() || rotation_degrees.abs() <= f32::EPSILON {
    return;
  }
  fn rotate_alignment(alignment: &mut (f32, f32), sin: f32, cos: f32) {
    let x = alignment.0 - 0.5;
    let y = alignment.1 - 0.5;
    *alignment = (
      cos.mul_add(x, -sin * y) + 0.5,
      sin.mul_add(x, cos * y) + 0.5,
    );
  }
  fn conjugate_transform(transform: &mut ImageEffectTransform, sin: f32, cos: f32) {
    let a = transform.scale_x;
    let b = transform.skew_x;
    let c = transform.skew_y;
    let d = transform.scale_y;
    let ra = cos.mul_add(a, -sin * c);
    let rb = cos.mul_add(b, -sin * d);
    let rc = sin.mul_add(a, cos * c);
    let rd = sin.mul_add(b, cos * d);
    transform.scale_x = ra.mul_add(cos, -rb * sin);
    transform.skew_x = ra.mul_add(sin, rb * cos);
    transform.skew_y = rc.mul_add(cos, -rd * sin);
    transform.scale_y = rc.mul_add(sin, rd * cos);
    let shift_x = transform.shift_x_px;
    let shift_y = transform.shift_y_px;
    transform.shift_x_px = cos.mul_add(shift_x, -sin * shift_y);
    transform.shift_y_px = sin.mul_add(shift_x, cos * shift_y);
  }
  fn visit(container: &mut ImageEffectContainer, rotation_degrees: f32, sin: f32, cos: f32) {
    for effect in &mut container.effects {
      match effect {
        ImageEffect::OuterShadow {
          direction_degrees,
          transform,
          alignment,
          rotate_with_shape: true,
          ..
        } => {
          *direction_degrees = (*direction_degrees + rotation_degrees).rem_euclid(360.0);
          conjugate_transform(transform, sin, cos);
          rotate_alignment(alignment, sin, cos);
        }
        ImageEffect::Reflection(reflection) if reflection.rotate_with_shape => {
          reflection.direction_degrees =
            (reflection.direction_degrees + rotation_degrees).rem_euclid(360.0);
          reflection.fade_direction_degrees =
            (reflection.fade_direction_degrees + rotation_degrees).rem_euclid(360.0);
          conjugate_transform(&mut reflection.transform, sin, cos);
          rotate_alignment(&mut reflection.alignment, sin, cos);
        }
        ImageEffect::AlphaModulate(child)
        | ImageEffect::Blend {
          container: child, ..
        }
        | ImageEffect::Container(child) => {
          visit(child, rotation_degrees, sin, cos);
        }
        _ => {}
      }
    }
  }

  let (sin, cos) = rotation_degrees.to_radians().sin_cos();
  visit(container, rotation_degrees, sin, cos);
}

pub(crate) fn raster_fill_image(
  data: &[u8],
  content_type: Option<&str>,
  effects: &[ImageEffect],
) -> Option<ImageEffectFill> {
  let raster_data = emf_wmf::decode_metafile_as_raster(data, content_type)
    .ok()
    .flatten()
    .map(|raster| raster.data);
  let image_data = raster_data.as_deref().unwrap_or(data);
  let mut image = image::load_from_memory(image_data).ok()?.to_rgba8();
  apply_to_image(&mut image, effects);
  Some(ImageEffectFill::Image(image))
}

fn apply_to_image(image: &mut image::RgbaImage, effects: &[ImageEffect]) {
  let bounds = PixelBounds {
    left: 0.0,
    top: 0.0,
    right: image.width() as f32,
    bottom: image.height() as f32,
  };
  apply_to_image_with_bounds(image, effects, bounds);
}

fn apply_to_image_with_bounds(
  image: &mut image::RgbaImage,
  effects: &[ImageEffect],
  content_bounds: PixelBounds,
) {
  let root_image = image.clone();
  let sources = ImageEffectSourceImages {
    fill: Some(&root_image),
    line: None,
    fill_line: Some(&root_image),
    children: None,
    effect_mask: None,
    reflection_paint: None,
    bounds: ImageEffectSourcePixelBounds::default(),
  };
  let geometry = EffectGeometry {
    paint: content_bounds,
    shadow_anchor: content_bounds,
    anchor: content_bounds,
    ramp: content_bounds,
  };
  apply_to_image_with_source_context(
    image,
    effects,
    geometry,
    geometry,
    sources,
    AlphaOutsetSurfaceScale::default(),
  );
}

fn apply_to_image_with_source_context(
  image: &mut image::RgbaImage,
  effects: &[ImageEffect],
  source_geometry: EffectGeometry,
  root_geometry: EffectGeometry,
  sources: ImageEffectSourceImages<'_>,
  raster_context: impl Into<EffectRasterContext>,
) {
  let raster_context = raster_context.into();
  let raster_scale = raster_context.scale;
  let alpha_outset_surface_scale = raster_context.alpha_outset;
  let mut current_geometry = source_geometry;
  for effect in effects {
    let effect_source = current_geometry;
    if let Some(output_geometry) =
      effect_output_geometry(effect, effect_source, root_geometry, sources.bounds)
    {
      current_geometry = output_geometry;
    }
    if let ImageEffect::Blur { radius_px, .. } = effect {
      if *radius_px > f32::EPSILON {
        *image = blur_rgba_premultiplied_xy(
          image,
          *radius_px * raster_scale.x,
          *radius_px * raster_scale.y,
        );
      }
      continue;
    }
    if let ImageEffect::AlphaModulate(container) = effect {
      let modulation = apply_container_with_bounds(
        image,
        container,
        effect_source,
        root_geometry,
        sources,
        raster_context,
      );
      for (pixel, modulation_pixel) in image.pixels_mut().zip(modulation.pixels()) {
        pixel.0[3] = ((u16::from(pixel.0[3]) * u16::from(modulation_pixel.0[3]) + 127) / 255) as u8;
      }
      continue;
    }
    if let ImageEffect::Blend {
      container,
      blend_mode,
    } = effect
    {
      let blended = apply_container_with_bounds(
        image,
        container,
        effect_source,
        root_geometry,
        sources,
        raster_context,
      );
      for (base, overlay) in image.pixels_mut().zip(blended.pixels()) {
        blend_rgba_pixel(base, overlay, *blend_mode);
      }
      continue;
    }
    if let ImageEffect::Container(container) = effect {
      *image = apply_container_with_bounds(
        image,
        container,
        effect_source,
        root_geometry,
        sources,
        raster_context,
      );
      continue;
    }
    if let ImageEffect::FillOverlay { fill, blend_mode } = effect {
      apply_fill_overlay_on_raster(image, fill, *blend_mode, effect_source.anchor, raster_scale);
      continue;
    }
    if let ImageEffect::Fill(fill) = effect {
      apply_fill_on_raster(image, fill, effect_source.anchor, raster_scale);
      continue;
    }
    if let ImageEffect::Glow {
      radius_px,
      raster_length_scale,
      spread_ratio,
      spread_kernel,
      spread_radius_rounding,
      blur_kernel,
      color,
      ..
    } = effect
    {
      *image = glow_image_on_raster(
        image,
        GlowImageOptions {
          radius_px: *radius_px * *raster_length_scale,
          spread_ratio: *spread_ratio,
          spread_kernel: *spread_kernel,
          spread_radius_rounding: *spread_radius_rounding,
          blur_kernel: *blur_kernel,
          color: *color,
          alpha_outset_surface_scale,
          color_alpha_mode: GlowColorAlphaMode::Straight,
        },
        raster_scale,
      );
      continue;
    }
    if matches!(effect, ImageEffect::Identity) {
      continue;
    }
    if let ImageEffect::SourceReference(reference) = effect {
      let referenced = match reference {
        ImageEffectSourceReference::Fill => sources.fill,
        ImageEffectSourceReference::Line => sources.line,
        ImageEffectSourceReference::FillLine => sources.fill_line,
        ImageEffectSourceReference::Children => sources.children,
        ImageEffectSourceReference::EffectMask => sources.effect_mask,
        ImageEffectSourceReference::ReflectionPaint => sources.reflection_paint,
      };
      if let Some(referenced) = referenced {
        image.clone_from(referenced);
      } else {
        for pixel in image.pixels_mut() {
          pixel.0 = [0; 4];
        }
      }
      continue;
    }
    if let ImageEffect::InnerShadow {
      blur_radius_px,
      distance_px,
      direction_degrees,
      color,
    } = effect
    {
      *image = inner_shadow_image_on_raster(
        image,
        *blur_radius_px,
        *distance_px,
        *direction_degrees,
        *color,
        raster_scale,
      );
      continue;
    }
    if let ImageEffect::OuterShadow {
      blur_radius_px,
      distance_px,
      raster_length_scale,
      distance_length_scale,
      blur_kernel,
      direction_degrees,
      distance_mode,
      transform,
      alignment,
      color,
      ..
    } = effect
    {
      *image = outer_shadow_image_with_scale(
        image,
        OuterShadowOptions {
          blur_radius_px: *blur_radius_px * *raster_length_scale,
          blur_kernel: *blur_kernel,
          distance_px: *distance_px * *distance_length_scale,
          direction_degrees: *direction_degrees,
          distance_mode: *distance_mode,
          transform: *transform,
          alignment: *alignment,
          color: *color,
          anchor_bounds: effect_source.shadow_anchor,
        },
        raster_scale,
      );
      continue;
    }
    if let ImageEffect::Reflection(effect) = effect {
      let coordinates = reflection_source_geometry(effect.reference, effect_source, root_geometry);
      *image = reflection_image_with_scale(
        image,
        *effect,
        coordinates.ramp,
        coordinates.anchor,
        coordinates.paint,
        raster_scale,
      );
      continue;
    }
    if let ImageEffect::RelativeOffset { offset_x, offset_y } = effect {
      *image = affine_image(
        image,
        ImageEffectTransform {
          scale_x: 1.0,
          scale_y: 1.0,
          skew_x: 0.0,
          skew_y: 0.0,
          shift_x_px: *offset_x * effect_source.anchor.width() * raster_scale.x,
          shift_y_px: *offset_y * effect_source.anchor.height() * raster_scale.y,
        },
      );
      continue;
    }
    if let ImageEffect::SoftEdge(radius_px) = effect {
      apply_soft_edge_on_raster(image, *radius_px, raster_scale);
      continue;
    }
    if let ImageEffect::Transform(transform) = effect {
      let mut transform = *transform;
      transform.shift_x_px += effect_source.anchor.left
        - transform.scale_x * effect_source.anchor.left
        - transform.skew_x * effect_source.anchor.top;
      transform.shift_y_px += effect_source.anchor.top
        - transform.skew_y * effect_source.anchor.left
        - transform.scale_y * effect_source.anchor.top;
      *image = affine_image(image, raster_scale.transform(transform));
      continue;
    }
    if let ImageEffect::AlphaOutset(radius_px) = effect {
      apply_alpha_outset(
        image,
        *radius_px * alpha_outset_surface_scale.x * raster_scale.x,
        *radius_px * alpha_outset_surface_scale.y * raster_scale.y,
      );
      continue;
    }
    for pixel in image.pixels_mut() {
      let [mut r, mut g, mut b, mut a] = pixel.0;
      match effect {
        ImageEffect::AlphaBiLevel(threshold) => {
          a = if a < *threshold { 0 } else { u8::MAX };
        }
        ImageEffect::AlphaCeiling => {
          if a > 0 {
            a = u8::MAX;
          }
        }
        ImageEffect::AlphaFloor => {
          if a < u8::MAX {
            a = 0;
          }
        }
        ImageEffect::AlphaInverse(color) => {
          a = u8::MAX - a;
          if let Some(color) = color {
            r = color.r;
            g = color.g;
            b = color.b;
          }
        }
        ImageEffect::AlphaModulate(_) => {
          unreachable!("alpha modulation handled as an image effect")
        }
        ImageEffect::AlphaModulateFixed(amount) => {
          a = (f32::from(a) * *amount).round().clamp(0.0, 255.0) as u8;
        }
        ImageEffect::AlphaOutset(_) => {
          unreachable!("alpha outset handled as an image effect")
        }
        ImageEffect::AlphaReplace(alpha) => a = *alpha,
        ImageEffect::BiLevel(threshold) => {
          let value = if srgb_luminance(r, g, b) >= *threshold {
            u8::MAX
          } else {
            0
          };
          r = value;
          g = value;
          b = value;
        }
        ImageEffect::Blur { .. } => unreachable!("blur handled as a whole-image effect"),
        ImageEffect::Blend { .. } => unreachable!("blend handled as a whole-image effect"),
        ImageEffect::ColorChange(effect)
          if channel_within_tolerance(r, effect.from.r, effect.tolerance)
            && channel_within_tolerance(g, effect.from.g, effect.tolerance)
            && channel_within_tolerance(b, effect.from.b, effect.tolerance)
            && (!effect.use_alpha || a == effect.from_alpha) =>
        {
          r = effect.to.r;
          g = effect.to.g;
          b = effect.to.b;
          if effect.use_alpha {
            a = effect.to_alpha;
          }
        }
        ImageEffect::ColorChange(_) => {}
        ImageEffect::ColorReplacement(color) => {
          r = color.r;
          g = color.g;
          b = color.b;
        }
        ImageEffect::Duotone(first, second) => {
          let luminance = libreoffice_luminance(r, g, b);
          r = duotone_component(luminance, first.r, second.r);
          g = duotone_component(luminance, first.g, second.g);
          b = duotone_component(luminance, first.b, second.b);
        }
        ImageEffect::Grayscale => {
          let luminance = srgb_luminance(r, g, b);
          r = luminance;
          g = luminance;
          b = luminance;
        }
        ImageEffect::Hsl {
          hue_degrees,
          saturation_offset,
          luminance_offset,
        } => {
          let mut hsl = HslColor::from_srgb8([r, g, b]);
          hsl.hue_degrees = (hsl.hue_degrees + *hue_degrees).rem_euclid(360.0);
          hsl.saturation = (hsl.saturation + *saturation_offset).clamp(0.0, 1.0);
          hsl.lightness = (hsl.lightness + *luminance_offset).clamp(0.0, 1.0);
          [r, g, b] = hsl.to_srgb8();
        }
        ImageEffect::Luminance {
          brightness,
          contrast,
        } => {
          if brightness.is_some() || contrast.is_some() {
            let brightness = brightness.unwrap_or(0);
            let contrast = contrast.unwrap_or(0);
            r = mso_brightness_contrast_component(r, brightness, contrast);
            g = mso_brightness_contrast_component(g, brightness, contrast);
            b = mso_brightness_contrast_component(b, brightness, contrast);
          }
        }
        ImageEffect::Tint {
          hue_degrees,
          amount,
        } => {
          let mut hsl = HslColor::from_srgb8([r, g, b]);
          let delta = (*hue_degrees - hsl.hue_degrees + 540.0).rem_euclid(360.0) - 180.0;
          hsl.hue_degrees = (hsl.hue_degrees + delta * *amount).rem_euclid(360.0);
          [r, g, b] = hsl.to_srgb8();
        }
        ImageEffect::FillOverlay { .. } => {
          unreachable!("fill overlay handled as an image effect")
        }
        ImageEffect::Fill(_) => unreachable!("fill handled as an image effect"),
        ImageEffect::Glow { .. } => unreachable!("glow handled as an image effect"),
        ImageEffect::Identity => unreachable!("identity handled as an image effect"),
        ImageEffect::InnerShadow { .. } => {
          unreachable!("inner shadow handled as an image effect")
        }
        ImageEffect::OuterShadow { .. } => {
          unreachable!("outer shadow handled as an image effect")
        }
        ImageEffect::Reflection(_) => {
          unreachable!("reflection handled as an image effect")
        }
        ImageEffect::SourceReference(_) => {
          unreachable!("source reference handled as an image effect")
        }
        ImageEffect::RelativeOffset { .. } => {
          unreachable!("relative offset handled as an image effect")
        }
        ImageEffect::SoftEdge(_) => unreachable!("soft edge handled as an image effect"),
        ImageEffect::Transform(_) => unreachable!("transform handled as an image effect"),
        ImageEffect::Container(_) => unreachable!("container handled as an image effect"),
      }
      pixel.0 = [r, g, b, a];
    }
  }
}

fn apply_fill_on_raster(
  image: &mut image::RgbaImage,
  fill: &ImageEffectFill,
  bounds: PixelBounds,
  scale: EffectRasterScale,
) {
  let width = image.width().max(1);
  let height = image.height().max(1);
  for y in 0..height {
    for x in 0..width {
      let alpha = image.get_pixel(x, y).0[3];
      let fill = sample_fill_at(
        fill,
        (x as f32 + 0.5) / scale.x - bounds.left,
        (y as f32 + 0.5) / scale.y - bounds.top,
        bounds.width(),
        bounds.height(),
      );
      let pixel = image.get_pixel_mut(x, y);
      pixel.0 = [
        fill.color.r,
        fill.color.g,
        fill.color.b,
        ((u16::from(alpha) * u16::from(fill.alpha) + 127) / 255) as u8,
      ];
    }
  }
}

#[cfg(test)]
fn apply_container(
  source: &image::RgbaImage,
  container: &ImageEffectContainer,
) -> image::RgbaImage {
  let bounds = PixelBounds {
    left: 0.0,
    top: 0.0,
    right: source.width() as f32,
    bottom: source.height() as f32,
  };
  let geometry = EffectGeometry {
    paint: bounds,
    shadow_anchor: bounds,
    anchor: bounds,
    ramp: bounds,
  };
  apply_container_with_bounds(
    source,
    container,
    geometry,
    geometry,
    ImageEffectSourceImages::default(),
    AlphaOutsetSurfaceScale::default(),
  )
}

fn apply_container_with_bounds<'a>(
  source: &'a image::RgbaImage,
  container: &ImageEffectContainer,
  source_geometry: EffectGeometry,
  root_geometry: EffectGeometry,
  sources: ImageEffectSourceImages<'a>,
  raster_context: impl Into<EffectRasterContext>,
) -> image::RgbaImage {
  let raster_context = raster_context.into();
  let sources = ImageEffectSourceImages {
    fill_line: sources.fill_line.or(Some(source)),
    ..sources
  };
  match container.kind {
    ImageEffectContainerKind::Tree => {
      let mut output = source.clone();
      apply_to_image_with_source_context(
        &mut output,
        &container.effects,
        source_geometry,
        root_geometry,
        sources,
        raster_context,
      );
      output
    }
    ImageEffectContainerKind::Sibling => {
      let mut output =
        image::RgbaImage::from_pixel(source.width(), source.height(), image::Rgba([0; 4]));
      let mut branch = source.clone();
      for effect in &container.effects {
        branch.clone_from(source);
        apply_to_image_with_source_context(
          &mut branch,
          std::slice::from_ref(effect),
          source_geometry,
          root_geometry,
          sources,
          raster_context,
        );
        composite_source_over(&mut output, &branch);
      }
      output
    }
  }
}

pub(crate) fn composite_source_over(destination: &mut image::RgbaImage, source: &image::RgbaImage) {
  debug_assert_eq!(destination.dimensions(), source.dimensions());
  for (destination, source) in destination.pixels_mut().zip(source.pixels()) {
    source_over(destination, source);
  }
}

/// Extends an authored paint surface with independently rendered coverage.
///
/// Word's static-3-D text effects retain the flat text paint as their color
/// source while adding the physical solid's antialiased silhouette to its
/// alpha. Pixels not reached by the flat paint inherit the coverage color;
/// overlapping pixels retain the authored flat color instead of leaking
/// material lighting into glow, shadow, or reflection.
pub(crate) fn composite_coverage_source_over_preserving_paint(
  paint: &mut image::RgbaImage,
  coverage: &image::RgbaImage,
) {
  debug_assert_eq!(paint.dimensions(), coverage.dimensions());
  for (paint, coverage) in paint.pixels_mut().zip(coverage.pixels()) {
    let coverage_alpha = u32::from(coverage.0[3]);
    let paint_alpha = u32::from(paint.0[3]);
    let inverse_coverage_alpha = u32::from(u8::MAX) - coverage_alpha;
    let output_alpha = coverage_alpha + (paint_alpha * inverse_coverage_alpha + 127) / 255;
    if output_alpha == 0 {
      paint.0 = [0; 4];
      continue;
    }
    if paint_alpha == 0 {
      paint.0[..3].copy_from_slice(&coverage.0[..3]);
    }
    paint.0[3] = output_alpha as u8;
  }
}

fn source_over(destination: &mut image::Rgba<u8>, source: &image::Rgba<u8>) {
  let source_alpha = u32::from(source.0[3]);
  let destination_alpha = u32::from(destination.0[3]);
  if source_alpha == 0 {
    if destination_alpha == 0 {
      destination.0 = [0; 4];
    }
    return;
  }
  if source_alpha == 255 || destination_alpha == 0 {
    *destination = *source;
    return;
  }

  // Keep alpha in 1/65025 units until after unpremultiplication. Dividing
  // color by an already rounded byte alpha can exceed 255 (even white over
  // white), wrapping on conversion to u8. These exact weights instead make
  // every color a convex combination; the largest numerator fits in u32.
  let source_weight = source_alpha * 255;
  let destination_weight = destination_alpha * (255 - source_alpha);
  let alpha_numerator = source_weight + destination_weight;
  for channel in 0..3 {
    let color_numerator = u32::from(source.0[channel]) * source_weight
      + u32::from(destination.0[channel]) * destination_weight;
    destination.0[channel] = ((color_numerator + alpha_numerator / 2) / alpha_numerator) as u8;
  }
  destination.0[3] = ((alpha_numerator + 127) / 255) as u8;
}

fn apply_fill_overlay_on_raster(
  image: &mut image::RgbaImage,
  fill: &ImageEffectFill,
  blend_mode: ImageEffectBlendMode,
  bounds: PixelBounds,
  scale: EffectRasterScale,
) {
  if matches!(fill, ImageEffectFill::None) {
    return;
  }
  let width = image.width().max(1);
  let height = image.height().max(1);
  for y in 0..height {
    for x in 0..width {
      let overlay = sample_fill_at(
        fill,
        (x as f32 + 0.5) / scale.x - bounds.left,
        (y as f32 + 0.5) / scale.y - bounds.top,
        bounds.width(),
        bounds.height(),
      );
      blend_fill_pixel(image.get_pixel_mut(x, y), overlay, blend_mode);
    }
  }
}

#[cfg(test)]
fn sample_fill(
  fill: &ImageEffectFill,
  x: u32,
  y: u32,
  width: u32,
  height: u32,
) -> ResolvedEffectColor {
  sample_fill_at(
    fill,
    x as f32 + 0.5,
    y as f32 + 0.5,
    width as f32,
    height as f32,
  )
}

fn sample_fill_at(
  fill: &ImageEffectFill,
  x: f32,
  y: f32,
  width: f32,
  height: f32,
) -> ResolvedEffectColor {
  let width = width.max(f32::EPSILON);
  let height = height.max(f32::EPSILON);
  match fill {
    ImageEffectFill::None => ResolvedEffectColor {
      color: RgbColor { r: 0, g: 0, b: 0 },
      alpha: 0,
    },
    ImageEffectFill::Solid(color) => *color,
    ImageEffectFill::Pattern {
      style,
      foreground,
      background,
      tile_px,
    } => {
      let tile_px = tile_px.max(f32::EPSILON);
      let hatch_x = (x.rem_euclid(tile_px) * 8.0 / tile_px).floor() as i32;
      let hatch_y = (y.rem_euclid(tile_px) * 8.0 / tile_px).floor() as i32;
      if style.is_foreground(hatch_x, hatch_y) {
        *foreground
      } else {
        *background
      }
    }
    ImageEffectFill::Image(image) => {
      let source_x = ((x / width).clamp(0.0, 1.0) * image.width() as f32)
        .floor()
        .min(image.width().saturating_sub(1) as f32) as u32;
      let source_y = ((y / height).clamp(0.0, 1.0) * image.height() as f32)
        .floor()
        .min(image.height().saturating_sub(1) as f32) as u32;
      let pixel = image.get_pixel(source_x, source_y).0;
      ResolvedEffectColor {
        color: RgbColor {
          r: pixel[0],
          g: pixel[1],
          b: pixel[2],
        },
        alpha: pixel[3],
      }
    }
    ImageEffectFill::Gradient {
      stops,
      kind,
      tile,
      flip,
    } => {
      let nx = x / width;
      let ny = y / height;
      let (nx, ny) = gradient_tile_point(nx, ny, *tile, *flip);
      let position = match kind {
        ImageEffectGradientKind::Linear(angle) => {
          let angle = angle.to_radians();
          let dx = angle.cos();
          let dy = angle.sin();
          let projections = [0.0, dx, dy, dx + dy];
          let minimum = projections.iter().copied().fold(f32::INFINITY, f32::min);
          let maximum = projections
            .iter()
            .copied()
            .fold(f32::NEG_INFINITY, f32::max);
          ((nx * dx + ny * dy - minimum) / (maximum - minimum).max(f32::EPSILON)).clamp(0.0, 1.0)
        }
        ImageEffectGradientKind::Circle(focus) => path_gradient_position(nx, ny, *focus, true),
        ImageEffectGradientKind::Rectangle(focus) => path_gradient_position(nx, ny, *focus, false),
      };
      sample_gradient(stops, position)
    }
  }
}

fn gradient_tile_point(
  x: f32,
  y: f32,
  tile: ImageEffectRelativeRect,
  flip: a::TileFlipValues,
) -> (f32, f32) {
  let left = tile.left;
  let top = tile.top;
  let tile_width = (1.0 - tile.left - tile.right).abs().max(f32::EPSILON);
  let tile_height = (1.0 - tile.top - tile.bottom).abs().max(f32::EPSILON);
  let tile_x = ((x - left) / tile_width).floor();
  let tile_y = ((y - top) / tile_height).floor();
  let mut local_x = (x - left).rem_euclid(tile_width) / tile_width;
  let mut local_y = (y - top).rem_euclid(tile_height) / tile_height;
  if matches!(
    flip,
    a::TileFlipValues::Horizontal | a::TileFlipValues::HorizontalAndVertical
  ) && tile_x.rem_euclid(2.0) >= 1.0
  {
    local_x = 1.0 - local_x;
  }
  if matches!(
    flip,
    a::TileFlipValues::Vertical | a::TileFlipValues::HorizontalAndVertical
  ) && tile_y.rem_euclid(2.0) >= 1.0
  {
    local_y = 1.0 - local_y;
  }
  (local_x, local_y)
}

fn path_gradient_position(x: f32, y: f32, focus: ImageEffectRelativeRect, circle: bool) -> f32 {
  if !path_gradient_contains(x, y, focus, 1.0, circle) {
    return 1.0;
  }
  if path_gradient_contains(x, y, focus, 0.0, circle) {
    return 0.0;
  }
  let mut outside = 0.0;
  let mut inside = 1.0;
  for _ in 0..14 {
    let middle = (outside + inside) / 2.0;
    if path_gradient_contains(x, y, focus, middle, circle) {
      inside = middle;
    } else {
      outside = middle;
    }
  }
  inside.clamp(0.0, 1.0)
}

fn path_gradient_contains(
  x: f32,
  y: f32,
  focus: ImageEffectRelativeRect,
  outer_ratio: f32,
  circle: bool,
) -> bool {
  let focus = normalize_image_effect_focus_rect(focus);
  let focus_width = 1.0 - focus.left - focus.right;
  let focus_height = 1.0 - focus.top - focus.bottom;
  let scale_x = focus_width + (1.0 - focus_width) * outer_ratio;
  let scale_y = focus_height + (1.0 - focus_height) * outer_ratio;
  let offset_x = focus.left * (1.0 - outer_ratio);
  let offset_y = focus.top * (1.0 - outer_ratio);
  if scale_x.abs() <= f32::EPSILON || scale_y.abs() <= f32::EPSILON {
    return (x - offset_x).abs() <= f32::EPSILON && (y - offset_y).abs() <= f32::EPSILON;
  }
  let base_x = (x - offset_x) / scale_x;
  let base_y = (y - offset_y) / scale_y;
  if circle {
    let x = (base_x - 0.5) * 2.0;
    let y = (base_y - 0.5) * 2.0;
    x.mul_add(x, y * y) <= 1.0
  } else {
    (0.0..=1.0).contains(&base_x) && (0.0..=1.0).contains(&base_y)
  }
}

fn normalize_image_effect_focus_rect(rect: ImageEffectRelativeRect) -> ImageEffectRelativeRect {
  let authored_right = 1.0 - rect.right;
  let authored_bottom = 1.0 - rect.bottom;
  let left = rect.left.min(authored_right);
  let top = rect.top.min(authored_bottom);
  let right = rect.left.max(authored_right);
  let bottom = rect.top.max(authored_bottom);
  ImageEffectRelativeRect {
    left,
    top,
    right: 1.0 - right,
    bottom: 1.0 - bottom,
  }
}

fn sample_gradient(stops: &[(f32, ResolvedEffectColor)], position: f32) -> ResolvedEffectColor {
  let Some(first) = stops.first() else {
    return ResolvedEffectColor {
      color: RgbColor { r: 0, g: 0, b: 0 },
      alpha: 0,
    };
  };
  let mut lower = first;
  for upper in stops.iter().skip(1) {
    if position < upper.0 {
      let span = upper.0 - lower.0;
      let amount = if span.abs() <= f32::EPSILON {
        1.0
      } else {
        ((position - lower.0) / span).clamp(0.0, 1.0)
      };
      return interpolate_color(lower.1, upper.1, amount);
    }
    lower = upper;
  }
  lower.1
}

fn interpolate_color(
  first: ResolvedEffectColor,
  second: ResolvedEffectColor,
  amount: f32,
) -> ResolvedEffectColor {
  let interpolate = |first: u8, second: u8| {
    (f32::from(first) + (f32::from(second) - f32::from(first)) * amount)
      .round()
      .clamp(0.0, 255.0) as u8
  };
  ResolvedEffectColor {
    color: RgbColor {
      r: interpolate(first.color.r, second.color.r),
      g: interpolate(first.color.g, second.color.g),
      b: interpolate(first.color.b, second.color.b),
    },
    alpha: interpolate(first.alpha, second.alpha),
  }
}

fn blend_fill_pixel(
  base: &mut image::Rgba<u8>,
  overlay: ResolvedEffectColor,
  mode: ImageEffectBlendMode,
) {
  let base_alpha = f32::from(base.0[3]) / 255.0;
  let overlay_alpha = f32::from(overlay.alpha) / 255.0;
  let output_alpha = overlay_alpha + base_alpha * (1.0 - overlay_alpha);
  if output_alpha <= f32::EPSILON {
    base.0 = [0; 4];
    return;
  }
  let overlay_channels = [overlay.color.r, overlay.color.g, overlay.color.b];
  for (channel, overlay_channel) in overlay_channels.into_iter().enumerate() {
    let backdrop = f32::from(base.0[channel]) / 255.0;
    let source = f32::from(overlay_channel) / 255.0;
    let blended = match mode {
      ImageEffectBlendMode::Over => source,
      ImageEffectBlendMode::Multiply => backdrop * source,
      ImageEffectBlendMode::Screen => backdrop + source - backdrop * source,
      ImageEffectBlendMode::Darken => backdrop.min(source),
      ImageEffectBlendMode::Lighten => backdrop.max(source),
    };
    let premultiplied = overlay_alpha * (1.0 - base_alpha) * source
      + overlay_alpha * base_alpha * blended
      + (1.0 - overlay_alpha) * base_alpha * backdrop;
    base.0[channel] = (premultiplied / output_alpha * 255.0)
      .round()
      .clamp(0.0, 255.0) as u8;
  }
  base.0[3] = (output_alpha * 255.0).round().clamp(0.0, 255.0) as u8;
}

fn blend_rgba_pixel(
  base: &mut image::Rgba<u8>,
  overlay: &image::Rgba<u8>,
  mode: ImageEffectBlendMode,
) {
  blend_fill_pixel(
    base,
    ResolvedEffectColor {
      color: RgbColor {
        r: overlay.0[0],
        g: overlay.0[1],
        b: overlay.0[2],
      },
      alpha: overlay.0[3],
    },
    mode,
  );
}

#[derive(Clone, Copy)]
struct GlowImageOptions {
  radius_px: f32,
  spread_ratio: f32,
  spread_kernel: GlowSpreadKernel,
  spread_radius_rounding: GlowSpreadRadiusRounding,
  blur_kernel: GlowBlurKernel,
  color: ResolvedEffectColor,
  alpha_outset_surface_scale: AlphaOutsetSurfaceScale,
  color_alpha_mode: GlowColorAlphaMode,
}

#[derive(Clone, Copy)]
enum GlowColorAlphaMode {
  Straight,
  BlackMatteAssociated,
}

/// Resolves one isolated glow directly into PDF's black-Matte sample model.
///
/// This is intentionally narrower than the general effect evaluator. An
/// associated surface cannot pass through its straight-alpha sibling/tree
/// compositors without being multiplied twice. Word's fixed-output flat-glow
/// branches contain exactly one terminal glow, so materialize that proven
/// boundary directly and leave every other effect graph on the ordinary path.
pub(crate) fn black_matte_associated_single_glow_surface(
  source: &image::RgbaImage,
  container: &ImageEffectContainer,
) -> Option<image::RgbaImage> {
  let [
    ImageEffect::Glow {
      radius_px,
      raster_length_scale,
      spread_ratio,
      spread_kernel,
      spread_radius_rounding,
      blur_kernel,
      color,
      ..
    },
  ] = container.effects.as_slice()
  else {
    return None;
  };
  Some(glow_image(
    source,
    GlowImageOptions {
      radius_px: *radius_px * *raster_length_scale,
      spread_ratio: *spread_ratio,
      spread_kernel: *spread_kernel,
      spread_radius_rounding: *spread_radius_rounding,
      blur_kernel: *blur_kernel,
      color: *color,
      alpha_outset_surface_scale: AlphaOutsetSurfaceScale::default(),
      color_alpha_mode: GlowColorAlphaMode::BlackMatteAssociated,
    },
  ))
}

fn glow_image(source: &image::RgbaImage, options: GlowImageOptions) -> image::RgbaImage {
  glow_image_on_raster(source, options, EffectRasterScale::default())
}

fn glow_image_on_raster(
  source: &image::RgbaImage,
  options: GlowImageOptions,
  raster_scale: EffectRasterScale,
) -> image::RgbaImage {
  let GlowImageOptions {
    radius_px,
    spread_ratio,
    spread_kernel,
    spread_radius_rounding,
    blur_kernel,
    color,
    alpha_outset_surface_scale,
    color_alpha_mode,
  } = options;
  let alpha_outset_surface_scale = AlphaOutsetSurfaceScale {
    x: alpha_outset_surface_scale.x * raster_scale.x,
    y: alpha_outset_surface_scale.y * raster_scale.y,
  };
  let spread_x = quantized_glow_spread_radius(
    radius_px * raster_scale.x,
    spread_ratio,
    spread_radius_rounding,
  );
  let spread_y = quantized_glow_spread_radius(
    radius_px * raster_scale.y,
    spread_ratio,
    spread_radius_rounding,
  );
  let alpha = image::GrayImage::from_fn(source.width(), source.height(), |x, y| {
    image::Luma([source.get_pixel(x, y).0[3]])
  });
  let glow_alpha = if radius_px > f32::EPSILON {
    match blur_kernel {
      GlowBlurKernel::Gaussian => {
        let spread = match spread_kernel {
          GlowSpreadKernel::Square => dilate_nontransparent_alpha_xy(&alpha, spread_x, spread_y),
          GlowSpreadKernel::Disk | GlowSpreadKernel::WordFlatAlphaOutset => {
            dilate_nontransparent_alpha_ellipse(&alpha, spread_x, spread_y)
          }
          GlowSpreadKernel::AlphaOutset => alpha_outset_mask(
            &alpha,
            radius_px * spread_ratio * alpha_outset_surface_scale.x,
            radius_px * spread_ratio * alpha_outset_surface_scale.y,
            true,
          ),
        };
        // Win2D's local official documentation defines the finite Gaussian
        // radius as three standard deviations.  The Office equivalence above
        // assigns half of R to this public blur, hence sigma = R / 6.
        blur_gray_xy(
          &spread,
          radius_px / 6.0 * raster_scale.x,
          radius_px / 6.0 * raster_scale.y,
        )
      }
      GlowBlurKernel::WordShapeGaussian => {
        let spread = match spread_kernel {
          GlowSpreadKernel::Square => dilate_nontransparent_alpha_xy(&alpha, spread_x, spread_y),
          GlowSpreadKernel::Disk | GlowSpreadKernel::WordFlatAlphaOutset => {
            dilate_nontransparent_alpha_ellipse(&alpha, spread_x, spread_y)
          }
          GlowSpreadKernel::AlphaOutset => alpha_outset_mask(
            &alpha,
            radius_px * spread_ratio * alpha_outset_surface_scale.x,
            radius_px * spread_ratio * alpha_outset_surface_scale.y,
            true,
          ),
        };
        let blur_radius_x =
          word_shape_glow_blur_device_radius(radius_px * alpha_outset_surface_scale.x);
        let blur_radius_y =
          word_shape_glow_blur_device_radius(radius_px * alpha_outset_surface_scale.y);
        finite_gaussian_blur_alpha_with_sigma(
          &spread,
          blur_radius_x,
          radius_px / 6.0 * alpha_outset_surface_scale.x,
          blur_radius_y,
          radius_px / 6.0 * alpha_outset_surface_scale.y,
        )
      }
      GlowBlurKernel::WordStatic3dGaussian => {
        let radius_x = radius_px * alpha_outset_surface_scale.x;
        let radius_y = radius_px * alpha_outset_surface_scale.y;
        let profile_x = word_static_3d_glow_axis_profile(radius_x);
        let profile_y = word_static_3d_glow_axis_profile(radius_y);
        let spread = match spread_kernel {
          GlowSpreadKernel::Square => dilate_nontransparent_alpha_xy(
            &alpha,
            profile_x.spread_radius_px.floor() as usize,
            profile_y.spread_radius_px.floor() as usize,
          ),
          GlowSpreadKernel::Disk | GlowSpreadKernel::WordFlatAlphaOutset => {
            dilate_nontransparent_alpha_ellipse(
              &alpha,
              profile_x.spread_radius_px.floor() as usize,
              profile_y.spread_radius_px.floor() as usize,
            )
          }
          GlowSpreadKernel::AlphaOutset => alpha_outset_mask(
            &alpha,
            profile_x.spread_radius_px,
            profile_y.spread_radius_px,
            true,
          ),
        };
        finite_gaussian_blur_alpha_with_sigma(
          &spread,
          profile_x.blur_support_px,
          profile_x.sigma_px,
          profile_y.blur_support_px,
          profile_y.sigma_px,
        )
      }
      GlowBlurKernel::WordGroupGaussian => {
        let spread = match spread_kernel {
          GlowSpreadKernel::Square => dilate_nontransparent_alpha_xy(&alpha, spread_x, spread_y),
          GlowSpreadKernel::Disk | GlowSpreadKernel::WordFlatAlphaOutset => {
            dilate_nontransparent_alpha_ellipse(&alpha, spread_x, spread_y)
          }
          GlowSpreadKernel::AlphaOutset => alpha_outset_mask(
            &alpha,
            radius_px * spread_ratio * alpha_outset_surface_scale.x,
            radius_px * spread_ratio * alpha_outset_surface_scale.y,
            true,
          ),
        };
        finite_gaussian_blur_alpha(
          &spread,
          word_group_public_blur_device_radius(radius_px * 0.5 * raster_scale.x),
          word_group_public_blur_device_radius(radius_px * 0.5 * raster_scale.y),
        )
      }
      #[cfg(test)]
      GlowBlurKernel::Stack => {
        // GlowPrimitive2D ceils the device radius before passing half through
        // integer morphology and Stack Blur constructors.
        let spread_radius_x = ((radius_px * raster_scale.x).ceil() as usize) / 2;
        let spread_radius_y = ((radius_px * raster_scale.y).ceil() as usize) / 2;
        let mut blurred = match spread_kernel {
          GlowSpreadKernel::Square => {
            dilate_nontransparent_alpha_xy(&alpha, spread_radius_x, spread_radius_y)
          }
          GlowSpreadKernel::Disk | GlowSpreadKernel::WordFlatAlphaOutset => {
            dilate_nontransparent_alpha_ellipse(&alpha, spread_radius_x, spread_radius_y)
          }
          GlowSpreadKernel::AlphaOutset => alpha_outset_mask(
            &alpha,
            radius_px * spread_ratio * alpha_outset_surface_scale.x,
            radius_px * spread_ratio * alpha_outset_surface_scale.y,
            true,
          ),
        };
        let width = blurred.width() as usize;
        let height = blurred.height() as usize;
        stack_blur_alpha_xy(
          blurred.as_mut(),
          width,
          height,
          spread_radius_x.max(2),
          spread_radius_y.max(2),
        );
        blurred
      }
    }
  } else {
    alpha
  };
  image::RgbaImage::from_fn(source.width(), source.height(), |x, y| {
    let glow_alpha = glow_alpha.get_pixel(x, y).0[0];
    let alpha = ((u16::from(glow_alpha) * u16::from(color.alpha) + 127) / 255) as u8;
    let rgb = match color_alpha_mode {
      GlowColorAlphaMode::Straight => [color.color.r, color.color.g, color.color.b],
      GlowColorAlphaMode::BlackMatteAssociated => {
        // Direct2D keeps the unattenuated A8 glow coverage until it resolves
        // the BGRA target. Office's complete 6-color x 2-host x 5-saturation
        // x 4-opacity matrix then pins one truncating conversion per color
        // component, while its separately exported SMask uses nearest
        // quantization above. Multiplying the already rounded SMask would
        // collapse those two independently observable device results.
        const COMPONENT_DENOMINATOR: u32 = u8::MAX as u32 * u8::MAX as u32;
        let component = |value: u8| {
          (u32::from(glow_alpha) * u32::from(value) * u32::from(color.alpha)
            / COMPONENT_DENOMINATOR) as u8
        };
        [
          component(color.color.r),
          component(color.color.g),
          component(color.color.b),
        ]
      }
    };
    image::Rgba([rgb[0], rgb[1], rgb[2], alpha])
  })
}

/// Quantizes the public blur support in Word's standalone WPS glow graph.
/// Office radius controls at both 200-DPI and 100-DPI surface tiers pin the
/// support to half the mapped glow radius, truncated toward zero.
fn word_shape_glow_blur_device_radius(radius_px: f32) -> usize {
  if radius_px.is_finite() {
    (radius_px.max(0.0) * 0.5).floor() as usize
  } else {
    0
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct WordStatic3dGlowAxisProfile {
  spread_radius_px: f32,
  blur_support_px: usize,
  sigma_px: f32,
}

/// Resolves the observable fixed-output profile of a projected Word glow.
///
/// ECMA defines alphaOutset as alpha-ceiling, alpha-blur, then alpha-ceiling.
/// Direct2D documents that Balanced Gaussian uses internal pre-scaling and
/// trilinear filtering but does not publish the reduced discrete kernel. The
/// exact-config 1/2/5pt radius controls separate the unchanged small tier from
/// the first large tier; six independent glyph topologies then select the
/// large tier's `R/3` outset and `2R/3` finite Gaussian on both training and
/// holdout inputs. Keep this host-specific: ordinary WPS and flat run glows
/// have independent edge-profile controls and retain `WordShapeGaussian`.
fn word_static_3d_glow_axis_profile(radius_px: f32) -> WordStatic3dGlowAxisProfile {
  let radius_px = if radius_px.is_finite() {
    radius_px.max(0.0)
  } else {
    0.0
  };
  let public_blur_radius_px = radius_px * 0.5;
  let boundary_tolerance = f32::EPSILON * public_blur_radius_px.abs().max(1.0) * 2.0;
  if public_blur_radius_px > DIRECT2D_BALANCED_BLUR_PRESCALE_STEP_PX + boundary_tolerance {
    let blur_radius_px = radius_px * (2.0 / 3.0);
    let nearest_support = blur_radius_px.round();
    let support_tolerance = f32::EPSILON * blur_radius_px.abs().max(1.0) * 2.0;
    let stable_support = if (blur_radius_px - nearest_support).abs() <= support_tolerance {
      nearest_support
    } else {
      blur_radius_px.ceil()
    };
    WordStatic3dGlowAxisProfile {
      spread_radius_px: radius_px / 3.0,
      blur_support_px: stable_support as usize,
      sigma_px: blur_radius_px / 3.0,
    }
  } else {
    WordStatic3dGlowAxisProfile {
      spread_radius_px: public_blur_radius_px,
      blur_support_px: word_shape_glow_blur_device_radius(radius_px),
      sigma_px: radius_px / 6.0,
    }
  }
}

/// Quantizes Word's public blur support after the authored length is mapped to
/// device pixels.  The corresponding Microsoft WPF implementation converts
/// the scaled radius to an unsigned integer, which truncates a positive value.
fn word_group_public_blur_device_radius(radius_px: f32) -> usize {
  if radius_px.is_finite() {
    radius_px.max(0.0).floor() as usize
  } else {
    0
  }
}

fn normalized_gaussian_kernel(radius: usize, sigma: f32) -> Vec<f32> {
  if radius == 0 || !sigma.is_finite() || sigma <= f32::EPSILON {
    return vec![1.0];
  }
  let sigma = f64::from(sigma);
  let mut kernel = Vec::with_capacity(radius.saturating_mul(2).saturating_add(1));
  let mut sum = 0.0_f64;
  for index in 0..=radius.saturating_mul(2) {
    let offset = index as f64 - radius as f64;
    let weight = (-(offset * offset) / (2.0 * sigma * sigma)).exp();
    kernel.push(weight);
    sum += weight;
  }
  kernel
    .into_iter()
    .map(|weight| (weight / sum) as f32)
    .collect()
}

fn direct2d_gaussian_blur_alpha(alpha: &image::GrayImage, blur_radius_px: f32) -> image::GrayImage {
  if !blur_radius_px.is_finite() || blur_radius_px <= f32::EPSILON {
    return alpha.clone();
  }
  let radius = blur_radius_px.floor() as usize;
  if radius == 0 {
    return alpha.clone();
  }
  floating_point_gaussian_blur_alpha(alpha, radius, direct2d_gaussian_sigma(blur_radius_px))
}

fn word_text_balanced_blur_alpha(
  alpha: &image::GrayImage,
  blur_support_radius_px: f32,
  prescale_divisor: u32,
) -> image::GrayImage {
  word_text_balanced_blur_alpha_xy(
    alpha,
    blur_support_radius_px,
    blur_support_radius_px,
    prescale_divisor,
  )
}

fn word_text_balanced_blur_alpha_xy(
  alpha: &image::GrayImage,
  blur_support_radius_x_px: f32,
  blur_support_radius_y_px: f32,
  prescale_divisor: u32,
) -> image::GrayImage {
  if (!blur_support_radius_x_px.is_finite() || blur_support_radius_x_px <= f32::EPSILON)
    && (!blur_support_radius_y_px.is_finite() || blur_support_radius_y_px <= f32::EPSILON)
  {
    return alpha.clone();
  }
  let prescale_divisor = prescale_divisor.max(1);
  let reduced_support_radius_x_px = blur_support_radius_x_px / prescale_divisor as f32;
  let reduced_support_radius_y_px = blur_support_radius_y_px / prescale_divisor as f32;
  let blur = |source: &image::GrayImage| {
    floating_point_gaussian_blur_alpha_xy(
      source,
      reduced_support_radius_x_px.floor() as usize,
      reduced_support_radius_x_px / 3.0,
      reduced_support_radius_y_px.floor() as usize,
      reduced_support_radius_y_px / 3.0,
    )
  };
  if prescale_divisor == 1 || alpha.width() == 0 || alpha.height() == 0 {
    return blur(alpha);
  }

  // Direct2D Balanced evaluates large kernels on an internally pre-scaled
  // image and reconstructs the result with linear filtering. Exact Word WPS
  // surface extents and the W14 native-shadow controls agree on inclusive
  // far-edge allocation: samples are counted over `(extent - 1) / divisor`
  // intervals, while the terminal half sample remains part of both mappings.
  let reduced_width = balanced_prescale_extent(alpha.width(), prescale_divisor);
  let reduced_height = balanced_prescale_extent(alpha.height(), prescale_divisor);
  let source_extent_x = alpha.width() as f32 - 0.5;
  let source_extent_y = alpha.height() as f32 - 0.5;
  let reduced = resize_gray_linear_hard(
    alpha,
    reduced_width,
    reduced_height,
    source_extent_x,
    source_extent_y,
    reduced_width as f32,
    reduced_height as f32,
  );
  let blurred = blur(&reduced);
  resize_gray_linear_hard(
    &blurred,
    alpha.width(),
    alpha.height(),
    reduced_width as f32,
    reduced_height as f32,
    source_extent_x,
    source_extent_y,
  )
}

fn balanced_prescale_extent(source_extent: u32, prescale_divisor: u32) -> u32 {
  if source_extent == 0 {
    0
  } else {
    source_extent.saturating_sub(1) / prescale_divisor.max(1) + 1
  }
}

fn resize_gray_linear_hard(
  source: &image::GrayImage,
  output_width: u32,
  output_height: u32,
  source_extent_x: f32,
  source_extent_y: f32,
  output_extent_x: f32,
  output_extent_y: f32,
) -> image::GrayImage {
  if source.width() == 0
    || source.height() == 0
    || output_width == 0
    || output_height == 0
    || !source_extent_x.is_finite()
    || source_extent_x <= f32::EPSILON
    || !source_extent_y.is_finite()
    || source_extent_y <= f32::EPSILON
    || !output_extent_x.is_finite()
    || output_extent_x <= f32::EPSILON
    || !output_extent_y.is_finite()
    || output_extent_y <= f32::EPSILON
  {
    return image::GrayImage::new(output_width, output_height);
  }
  let scale_x = source_extent_x / output_extent_x;
  let scale_y = source_extent_y / output_extent_y;
  image::GrayImage::from_fn(output_width, output_height, |x, y| {
    let source_x = (x as f32 + 0.5).mul_add(scale_x, -0.5);
    let source_y = (y as f32 + 0.5).mul_add(scale_y, -0.5);
    bilinear_sample_gray(source, source_x, source_y)
  })
}

fn floating_point_gaussian_blur_alpha(
  alpha: &image::GrayImage,
  radius: usize,
  sigma: f32,
) -> image::GrayImage {
  floating_point_gaussian_blur_alpha_xy(alpha, radius, sigma, radius, sigma)
}

fn floating_point_gaussian_blur_alpha_xy(
  alpha: &image::GrayImage,
  radius_x: usize,
  sigma_x: f32,
  radius_y: usize,
  sigma_y: f32,
) -> image::GrayImage {
  let kernel_x = normalized_gaussian_kernel(radius_x, sigma_x);
  let kernel_y = normalized_gaussian_kernel(radius_y, sigma_y);
  // A zero/invalid axis is an identity convolution on that axis, not a
  // reason to drop the other axis. Use the actual kernel support below.
  let radius_x = kernel_x.len() / 2;
  let radius_y = kernel_y.len() / 2;
  if radius_x == 0 && radius_y == 0 {
    return alpha.clone();
  }
  let width = alpha.width() as usize;
  let height = alpha.height() as usize;
  if width == 0 || height == 0 {
    return alpha.clone();
  }

  // Direct2D defines a finite kernel radius of three standard deviations.
  // Preserve floating-point precision between the two separable passes; an
  // intermediate A8 quantization removes the low outer taps that Word retains
  // in fixed output. Soft-border samples outside the effect surface are
  // transparent rather than clamped.
  let mut horizontal = vec![0.0_f32; width * height];
  for y in 0..height {
    for x in 0..width {
      let mut sum = 0.0_f32;
      for (sample_index, weight) in kernel_x.iter().copied().enumerate() {
        let source_x = x as isize + sample_index as isize - radius_x as isize;
        if (0..width as isize).contains(&source_x) {
          sum += f32::from(alpha.as_raw()[y * width + source_x as usize]) * weight;
        }
      }
      horizontal[y * width + x] = sum;
    }
  }

  let mut output = vec![0_u8; width * height];
  for y in 0..height {
    for x in 0..width {
      let mut sum = 0.0_f32;
      for (sample_index, weight) in kernel_y.iter().copied().enumerate() {
        let source_y = y as isize + sample_index as isize - radius_y as isize;
        if (0..height as isize).contains(&source_y) {
          sum += horizontal[source_y as usize * width + x] * weight;
        }
      }
      output[y * width + x] = sum.round().clamp(0.0, 255.0) as u8;
    }
  }

  image::GrayImage::from_raw(alpha.width(), alpha.height(), output)
    .expect("finite Gaussian output preserves the input dimensions")
}

fn finite_gaussian_blur_alpha(
  alpha: &image::GrayImage,
  radius_x: usize,
  radius_y: usize,
) -> image::GrayImage {
  finite_gaussian_blur_alpha_with_sigma(
    alpha,
    radius_x,
    radius_x as f32 / 3.0,
    radius_y,
    radius_y as f32 / 3.0,
  )
}

fn finite_gaussian_blur_alpha_with_sigma(
  alpha: &image::GrayImage,
  radius_x: usize,
  sigma_x: f32,
  radius_y: usize,
  sigma_y: f32,
) -> image::GrayImage {
  let width = alpha.width() as usize;
  let height = alpha.height() as usize;
  if width == 0 || height == 0 || (radius_x == 0 && radius_y == 0) {
    return alpha.clone();
  }

  let horizontal_kernel = normalized_gaussian_kernel(radius_x, sigma_x);
  let mut horizontal = vec![0_u8; width * height];
  for y in 0..height {
    for x in 0..width {
      let mut sum = 0.0_f32;
      for (sample_index, weight) in horizontal_kernel.iter().copied().enumerate() {
        let offset = sample_index as isize - radius_x as isize;
        let source_x = x as isize - offset;
        if (0..width as isize).contains(&source_x) {
          sum += f32::from(alpha.as_raw()[y * width + source_x as usize]) * weight;
        }
      }
      horizontal[y * width + x] = sum.round().clamp(0.0, 255.0) as u8;
    }
  }

  let vertical_kernel = normalized_gaussian_kernel(radius_y, sigma_y);
  let mut output = vec![0_u8; width * height];
  for y in 0..height {
    for x in 0..width {
      let mut sum = 0.0_f32;
      for (sample_index, weight) in vertical_kernel.iter().copied().enumerate() {
        let offset = sample_index as isize - radius_y as isize;
        let source_y = y as isize - offset;
        if (0..height as isize).contains(&source_y) {
          sum += f32::from(horizontal[source_y as usize * width + x]) * weight;
        }
      }
      output[y * width + x] = sum.round().clamp(0.0, 255.0) as u8;
    }
  }

  image::GrayImage::from_raw(alpha.width(), alpha.height(), output)
    .expect("finite Gaussian output preserves the input dimensions")
}

fn quantized_glow_spread_radius(
  radius_px: f32,
  spread_ratio: f32,
  rounding: GlowSpreadRadiusRounding,
) -> usize {
  let source_radius = radius_px.max(0.0);
  let spread_ratio = spread_ratio.clamp(0.0, 1.0);
  let radius = source_radius * spread_ratio;
  (match rounding {
    GlowSpreadRadiusRounding::Outward => radius.ceil(),
    GlowSpreadRadiusRounding::Inward => radius.floor(),
  }) as usize
}

pub(crate) fn stack_blur_alpha(alpha: &mut [u8], width: usize, height: usize, radius: usize) {
  stack_blur_alpha_xy(alpha, width, height, radius, radius);
}

fn stack_blur_alpha_xy(
  alpha: &mut [u8],
  width: usize,
  height: usize,
  radius_x: usize,
  radius_y: usize,
) {
  let radius_x = radius_x.min(254);
  let radius_y = radius_y.min(254);
  if (radius_x == 0 && radius_y == 0) || width == 0 || height == 0 {
    return;
  }
  let mut horizontal = vec![0_u8; alpha.len()];
  for y in 0..height {
    triangular_blur_line(
      &alpha[y * width..(y + 1) * width],
      &mut horizontal[y * width..(y + 1) * width],
      radius_x,
    );
  }
  let mut column = vec![0_u8; height];
  let mut blurred_column = vec![0_u8; height];
  for x in 0..width {
    for y in 0..height {
      column[y] = horizontal[y * width + x];
    }
    triangular_blur_line(&column, &mut blurred_column, radius_y);
    for y in 0..height {
      alpha[y * width + x] = blurred_column[y];
    }
  }
}

/// Applies the triangular kernel used by Stack Blur in linear time.
pub(crate) fn triangular_blur_line(input: &[u8], output: &mut [u8], radius: usize) {
  debug_assert_eq!(input.len(), output.len());
  if input.is_empty() {
    return;
  }
  let divisor = ((radius + 1) * (radius + 1)) as i64;
  let mut prefix = Vec::with_capacity(input.len() + 1);
  prefix.push(0_i64);
  for value in input {
    prefix.push(prefix.last().copied().unwrap_or_default() + i64::from(*value));
  }
  let range_sum = |start: usize, end: usize| prefix[end] - prefix[start];

  let mut weighted_sum = 0_i64;
  for (index, value) in input.iter().take(radius + 1).enumerate() {
    weighted_sum += i64::from(*value) * (radius + 1 - index) as i64;
  }
  for (center, output_value) in output.iter_mut().enumerate() {
    *output_value = (weighted_sum / divisor).clamp(0, 255) as u8;
    if center + 1 == input.len() {
      break;
    }
    let left_start = center.saturating_sub(radius);
    let left_end = center + 1;
    let right_start = center + 1;
    let right_end = (center + radius + 2).min(input.len());
    weighted_sum -= range_sum(left_start, left_end);
    weighted_sum += range_sum(right_start, right_end);
  }
}

#[cfg(test)]
fn dilate_nontransparent_alpha(alpha: &image::GrayImage, radius: usize) -> image::GrayImage {
  dilate_nontransparent_alpha_xy(alpha, radius, radius)
}

fn dilate_nontransparent_alpha_xy(
  alpha: &image::GrayImage,
  radius_x: usize,
  radius_y: usize,
) -> image::GrayImage {
  if radius_x == 0 && radius_y == 0 {
    return alpha.clone();
  }
  let width = alpha.width() as usize;
  let height = alpha.height() as usize;
  let integral_width = width + 1;
  let mut integral = vec![0_u64; integral_width * (height + 1)];
  for y in 0..height {
    let mut row_sum = 0_u64;
    for x in 0..width {
      row_sum += u64::from(alpha.get_pixel(x as u32, y as u32).0[0]);
      integral[(y + 1) * integral_width + x + 1] = integral[y * integral_width + x + 1] + row_sum;
    }
  }
  image::GrayImage::from_fn(alpha.width(), alpha.height(), |x, y| {
    let x = x as usize;
    let y = y as usize;
    let left = x.saturating_sub(radius_x);
    let top = y.saturating_sub(radius_y);
    let right = x.saturating_add(radius_x).saturating_add(1).min(width);
    let bottom = y.saturating_add(radius_y).saturating_add(1).min(height);
    let sum = integral[bottom * integral_width + right] + integral[top * integral_width + left]
      - integral[top * integral_width + right]
      - integral[bottom * integral_width + left];
    image::Luma([u8::from(sum > 0) * u8::MAX])
  })
}

fn dilate_nontransparent_alpha_disk(alpha: &image::GrayImage, radius: usize) -> image::GrayImage {
  if radius == 0 {
    return alpha.clone();
  }
  let width = alpha.width() as usize;
  let height = alpha.height() as usize;
  let radius = radius.min(width.max(height));
  let radius_squared = (radius as u128) * (radius as u128);
  let mut horizontal_radii = Vec::with_capacity(radius + 1);
  let mut horizontal_radius = radius;
  for delta_y in 0..=radius {
    let delta_y_squared = (delta_y as u128) * (delta_y as u128);
    while (horizontal_radius as u128) * (horizontal_radius as u128) + delta_y_squared
      > radius_squared
    {
      horizontal_radius -= 1;
    }
    horizontal_radii.push(horizontal_radius);
  }

  dilate_nontransparent_alpha_with_rows(alpha, &horizontal_radii)
}

fn dilate_nontransparent_alpha_ellipse(
  alpha: &image::GrayImage,
  radius_x: usize,
  radius_y: usize,
) -> image::GrayImage {
  if radius_x == radius_y {
    return dilate_nontransparent_alpha_disk(alpha, radius_x);
  }
  let vertical_support = radius_y.min(alpha.height().saturating_sub(1) as usize);
  let rx2 = (radius_x as u128) * (radius_x as u128);
  let ry2 = (radius_y as u128) * (radius_y as u128);
  let horizontal_radii = (0..=vertical_support)
    .map(|dy| {
      // Integer membership retains exact boundary points (e.g. 9,4 on a
      // 15-by-5 ellipse). sqrt followed by floor can lose one pixel there.
      let remaining = ry2 - (dy as u128) * (dy as u128);
      let mut low = 0;
      let mut high = radius_x.min(alpha.width() as usize);
      while low < high {
        let x = low + (high - low).div_ceil(2);
        let inside = match (
          (x as u128 * x as u128).checked_mul(ry2),
          rx2.checked_mul(remaining),
        ) {
          (Some(left), Some(right)) => left <= right,
          _ => (x as f64 / radius_x as f64).powi(2) + (dy as f64 / radius_y as f64).powi(2) <= 1.0,
        };
        if inside {
          low = x;
        } else {
          high = x - 1;
        }
      }
      low
    })
    .collect::<Vec<_>>();
  dilate_nontransparent_alpha_with_rows(alpha, &horizontal_radii)
}

fn dilate_nontransparent_alpha_with_rows(
  alpha: &image::GrayImage,
  horizontal_radii: &[usize],
) -> image::GrayImage {
  let width = alpha.width() as usize;
  let height = alpha.height() as usize;
  let radius = horizontal_radii.len().saturating_sub(1);

  let stride = width + 1;
  let mut differences = vec![0_i32; stride * height];
  for y in 0..height {
    let mut x = 0;
    while x < width {
      while x < width && alpha.get_pixel(x as u32, y as u32).0[0] == 0 {
        x += 1;
      }
      if x == width {
        break;
      }
      let run_start = x;
      while x < width && alpha.get_pixel(x as u32, y as u32).0[0] != 0 {
        x += 1;
      }
      let run_end = x;
      for delta_y in -(radius as isize)..=radius as isize {
        let destination_y = y as isize + delta_y;
        if !(0..height as isize).contains(&destination_y) {
          continue;
        }
        let horizontal_radius = horizontal_radii[delta_y.unsigned_abs()];
        let left = run_start.saturating_sub(horizontal_radius);
        let right = run_end.saturating_add(horizontal_radius).min(width);
        let row = destination_y as usize * stride;
        differences[row + left] += 1;
        differences[row + right] -= 1;
      }
    }
  }
  let mut output = image::GrayImage::new(alpha.width(), alpha.height());
  for y in 0..height {
    let mut coverage = 0_i32;
    for x in 0..width {
      coverage += differences[y * stride + x];
      if coverage > 0 {
        output.get_pixel_mut(x as u32, y as u32).0[0] = u8::MAX;
      }
    }
  }
  output
}

fn inner_shadow_image_on_raster(
  source: &image::RgbaImage,
  blur_radius_px: f32,
  distance_px: f32,
  direction_degrees: f32,
  color: ResolvedEffectColor,
  scale: EffectRasterScale,
) -> image::RgbaImage {
  let radians = direction_degrees.to_radians();
  let shifted = affine_image(
    source,
    ImageEffectTransform {
      scale_x: 1.0,
      scale_y: 1.0,
      skew_x: 0.0,
      skew_y: 0.0,
      // MS-OI29500 defines inner-shadow direction clockwise from the left.
      shift_x_px: -radians.cos() * distance_px * scale.x,
      shift_y_px: -radians.sin() * distance_px * scale.y,
    },
  );
  let shifted_alpha = image::GrayImage::from_fn(source.width(), source.height(), |x, y| {
    image::Luma([shifted.get_pixel(x, y).0[3]])
  });
  let shifted_alpha = if blur_radius_px > f32::EPSILON {
    blur_gray_xy(
      &shifted_alpha,
      blur_radius_px * scale.x,
      blur_radius_px * scale.y,
    )
  } else {
    shifted_alpha
  };
  image::RgbaImage::from_fn(source.width(), source.height(), |x, y| {
    let clip = source.get_pixel(x, y).0[3];
    let inset = (u16::from(clip) * u16::from(u8::MAX - shifted_alpha.get_pixel(x, y).0[0])
      / u16::from(u8::MAX)) as u8;
    let shadow = image::Rgba([
      color.color.r,
      color.color.g,
      color.color.b,
      ((u16::from(inset) * u16::from(color.alpha) + 127) / 255) as u8,
    ]);
    let mut output = *source.get_pixel(x, y);
    source_over(&mut output, &shadow);
    output
  })
}

struct OuterShadowOptions {
  blur_radius_px: f32,
  blur_kernel: ShadowBlurKernel,
  distance_px: f32,
  direction_degrees: f32,
  distance_mode: ShadowDistanceMode,
  transform: ImageEffectTransform,
  alignment: (f32, f32),
  color: ResolvedEffectColor,
  anchor_bounds: PixelBounds,
}

#[cfg(test)]
fn outer_shadow_image(source: &image::RgbaImage, options: OuterShadowOptions) -> image::RgbaImage {
  outer_shadow_image_with_scale(source, options, EffectRasterScale::default())
}

fn outer_shadow_image_with_scale(
  source: &image::RgbaImage,
  options: OuterShadowOptions,
  raster_scale: EffectRasterScale,
) -> image::RgbaImage {
  debug_assert!(raster_scale.is_valid());
  let OuterShadowOptions {
    blur_radius_px,
    blur_kernel,
    distance_px,
    direction_degrees,
    distance_mode,
    mut transform,
    alignment,
    color,
    anchor_bounds,
  } = options;
  let radians = direction_degrees.to_radians();
  // Preserve the requested point of the logical shape/text rectangle while
  // transforming painted pixels. A glyph ink box is not a substitute for the
  // character-cell rectangle used by DrawingML alignment.
  let anchor_x = anchor_bounds.left + anchor_bounds.width() * alignment.0;
  let anchor_y = anchor_bounds.top + anchor_bounds.height() * alignment.1;
  let distance_x = radians.cos() * distance_px;
  let distance_y = radians.sin() * distance_px;
  let (offset_x, offset_y) = match distance_mode {
    ShadowDistanceMode::PostTransformOffset => (distance_x, distance_y),
    ShadowDistanceMode::PreTransformOffset => (
      transform
        .scale_x
        .mul_add(distance_x, transform.skew_x * distance_y),
      transform
        .skew_y
        .mul_add(distance_x, transform.scale_y * distance_y),
    ),
  };
  let output_alignment_shift = match blur_kernel {
    ShadowBlurKernel::Direct2dGaussian => (0.0, 0.0),
    ShadowBlurKernel::WordTextBalanced { .. } => {
      word_text_balanced_output_alignment_shift(blur_radius_px, transform, alignment)
    }
  };
  transform.shift_x_px += anchor_x - transform.scale_x * anchor_x - transform.skew_x * anchor_y
    + offset_x
    + output_alignment_shift.0;
  transform.shift_y_px += anchor_y - transform.skew_y * anchor_x - transform.scale_y * anchor_y
    + offset_y
    + output_alignment_shift.1;
  let transform = raster_scale.transform(transform);
  let blur_radius_x_px = blur_radius_px * raster_scale.x;
  let blur_radius_y_px = blur_radius_px * raster_scale.y;
  let alpha = image::GrayImage::from_fn(source.width(), source.height(), |x, y| {
    image::Luma([source.get_pixel(x, y).0[3]])
  });
  let alpha = match blur_kernel {
    ShadowBlurKernel::Direct2dGaussian => {
      // Direct2D's documented drop-shadow graph feeds the source into the
      // Shadow effect first, then feeds that output into a 2-D affine effect.
      let alpha = if blur_radius_px > f32::EPSILON {
        if raster_scale == EffectRasterScale::default() {
          direct2d_gaussian_blur_alpha(&alpha, blur_radius_px)
        } else {
          floating_point_gaussian_blur_alpha_xy(
            &alpha,
            blur_radius_x_px.floor() as usize,
            direct2d_gaussian_sigma(blur_radius_x_px),
            blur_radius_y_px.floor() as usize,
            direct2d_gaussian_sigma(blur_radius_y_px),
          )
        }
      } else {
        alpha
      };
      affine_gray_image(&alpha, transform)
    }
    ShadowBlurKernel::WordTextBalanced { prescale_divisor } => {
      // W14 scale, skew, alignment and distance belong to the text plane. The
      // flat-host Office factorial exposes the complete shadow-only SMask and
      // pins its physical balanced blur after this affine, independently of
      // the later static-3-D camera projection.
      let alpha = affine_gray_image(&alpha, transform);
      if blur_radius_px > f32::EPSILON {
        if raster_scale == EffectRasterScale::default() {
          word_text_balanced_blur_alpha(&alpha, blur_radius_px, prescale_divisor)
        } else {
          word_text_balanced_blur_alpha_xy(
            &alpha,
            blur_radius_x_px,
            blur_radius_y_px,
            prescale_divisor,
          )
        }
      } else {
        alpha
      }
    }
  };
  image::RgbaImage::from_fn(source.width(), source.height(), |x, y| {
    image::Rgba([
      color.color.r,
      color.color.g,
      color.color.b,
      ((u16::from(alpha.get_pixel(x, y).0[0]) * u16::from(color.alpha) + 127) / 255) as u8,
    ])
  })
}

/// Converts DrawingML's authored shadow blur radius to Direct2D's Gaussian
/// standard deviation. Both Microsoft APIs describe the former as a radius;
/// Direct2D defines its finite kernel radius as three standard deviations.
fn direct2d_gaussian_sigma(blur_radius_px: f32) -> f32 {
  blur_radius_px / 3.0
}

/// Converts DrawingML reflection's radial blur extent to one separable-axis
/// Gaussian deviation.
///
/// This is deliberately distinct from Direct2D Shadow's documented
/// three-sigma support radius. Exact-config Word reflection sweeps at 48 pt
/// pin `sigma = radius / sqrt(2)` independently on both sides of zero blur;
/// the second moment of an isotropic two-dimensional Gaussian is
/// `E[x² + y²] = 2 sigma²`.
fn reflection_radius_gaussian_sigma(radius_px: f32) -> f32 {
  radius_px * std::f32::consts::FRAC_1_SQRT_2
}

#[cfg(test)]
fn reflection_image(
  source: &image::RgbaImage,
  effect: ImageReflectionEffect,
  ramp_bounds: PixelBounds,
  anchor_bounds: PixelBounds,
  paint_bounds: PixelBounds,
) -> image::RgbaImage {
  reflection_image_with_scale(
    source,
    effect,
    ramp_bounds,
    anchor_bounds,
    paint_bounds,
    EffectRasterScale::default(),
  )
}

fn reflection_image_with_scale(
  source: &image::RgbaImage,
  effect: ImageReflectionEffect,
  ramp_bounds: PixelBounds,
  anchor_bounds: PixelBounds,
  paint_bounds: PixelBounds,
  raster_scale: EffectRasterScale,
) -> image::RgbaImage {
  debug_assert!(raster_scale.is_valid());
  let direction = effect.direction_degrees.to_radians();
  let distance_px = effect.distance_px * effect.distance_length_scale;
  let offset_x = direction.cos() * distance_px;
  let offset_y = direction.sin() * distance_px;
  let (ramp_bounds, alignment_bounds, post_transform_offset) = match effect.distance_mode {
    ReflectionDistanceMode::PostTransformOffset => {
      (ramp_bounds, anchor_bounds, (offset_x, offset_y))
    }
    ReflectionDistanceMode::AlignmentPivot => (
      ramp_bounds.union(ramp_bounds.translated(offset_x, offset_y)),
      anchor_bounds.translated(offset_x, offset_y),
      (0.0, 0.0),
    ),
  };
  let reflected_text_bounds = transformed_effect_bounds(
    ramp_bounds,
    alignment_bounds,
    effect.transform,
    effect.alignment,
  )
  .translated(post_transform_offset.0, post_transform_offset.1);
  let (gradient_bounds, start_position, end_position) =
    if matches!(effect.reference, ReflectionReference::WordRunMetrics { .. }) {
      let reflected_paint = transformed_effect_bounds(
        paint_bounds,
        alignment_bounds,
        effect.transform,
        effect.alignment,
      )
      .translated(post_transform_offset.0, post_transform_offset.1);
      word_run_reflection_gradient(
        reflected_text_bounds,
        reflected_paint,
        effect.start_position,
        effect.end_position,
      )
    } else {
      (
        reflected_text_bounds,
        effect.start_position,
        effect.end_position,
      )
    };
  let anchor_x = alignment_bounds.left + alignment_bounds.width() * effect.alignment.0;
  let anchor_y = alignment_bounds.top + alignment_bounds.height() * effect.alignment.1;
  let mut transform = effect.transform;
  let authored_shift_x = transform.shift_x_px;
  let authored_shift_y = transform.shift_y_px;
  transform.shift_x_px = anchor_x - transform.scale_x * anchor_x - transform.skew_x * anchor_y
    + post_transform_offset.0
    + authored_shift_x;
  transform.shift_y_px = anchor_y - transform.skew_y * anchor_x - transform.scale_y * anchor_y
    + post_transform_offset.1
    + authored_shift_y;
  let mut reflected = affine_image(source, raster_scale.transform(transform));

  // MS-DOCX CT_Reflection defines fadeDir relative to the text, and stPos /
  // endPos as positions along that gradient ramp. Word text supplies its
  // DirectWrite default-baseline rectangle as the ramp box even when the
  // painted glyph ink is shorter and its paragraph line-height cell is taller.
  // Apply it in transformed reflection coordinates: applying the ramp before
  // a negative `sy` reverses the near-to-far fade. The two 11-point Office
  // counterexamples have about 21px of ink in a 40px line cell and use an
  // intermediate font-metric ramp for endPos=60% and endPos=45.5%.
  let fade = effect.fade_direction_degrees.to_radians();
  let fade_x = fade.cos();
  let fade_y = fade.sin();
  let corners = [
    (gradient_bounds.left, gradient_bounds.top),
    (gradient_bounds.right, gradient_bounds.top),
    (gradient_bounds.right, gradient_bounds.bottom),
    (gradient_bounds.left, gradient_bounds.bottom),
  ];
  let minimum = corners
    .iter()
    .map(|(x, y)| fade_x * *x + fade_y * *y)
    .fold(f32::INFINITY, f32::min);
  let maximum = corners
    .iter()
    .map(|(x, y)| fade_x * *x + fade_y * *y)
    .fold(f32::NEG_INFINITY, f32::max);
  let span = (maximum - minimum).max(f32::EPSILON);
  let raster_gradient_bounds = raster_scale.bounds(gradient_bounds);
  for (x, y, pixel) in reflected.enumerate_pixels_mut() {
    // The fade is a covector in logical text coordinates. Sampling it in
    // texture pixels without the inverse scale rotates non-axis-aligned
    // ramps and changes their endpoint ownership.
    let logical_x = (x as f32 + 0.5) / raster_scale.x;
    let logical_y = (y as f32 + 0.5) / raster_scale.y;
    let position = (fade_x * logical_x + fade_y * logical_y - minimum) / span;
    let opacity = effect_ramp(
      position,
      (start_position, effect.start_opacity),
      (end_position, effect.end_opacity),
    );
    // The Word creator fills a finite rectangle with this gradient and uses
    // it as an alpha mask before Gaussian blur. Gradient stop extension is
    // not permission to paint outside that rectangle. Source rectangles must
    // accompany external images so legitimate shadow paint is not clipped.
    let coverage = if matches!(effect.reference, ReflectionReference::WordRunMetrics { .. }) {
      let x = x as f32;
      let y = y as f32;
      let width = ((x + 1.0).min(raster_gradient_bounds.right)
        - x.max(raster_gradient_bounds.left))
      .clamp(0.0, 1.0);
      let height = ((y + 1.0).min(raster_gradient_bounds.bottom)
        - y.max(raster_gradient_bounds.top))
      .clamp(0.0, 1.0);
      width * height
    } else {
      1.0
    };
    pixel.0[3] = (f32::from(pixel.0[3]) * opacity * coverage)
      .round()
      .clamp(0.0, 255.0) as u8;
  }
  // `stA`/`endA` define the alpha-gradient reflection surface. Blur that
  // completed surface instead of multiplying a sharp ramp onto an already
  // blurred copy: the latter clips the soft tail exactly at `endPos`. Office
  // fixed output retains blur energy beyond the authored zero-alpha stop.
  // The Microsoft effect pipeline expresses this soft border as a Gaussian
  // standard deviation, so preserve premultiplied color while applying that
  // kernel rather than substituting LibreOffice's finite Stack Blur radius.
  if effect.blur_radius_px > f32::EPSILON {
    let blur_radius_px = effect.blur_radius_px * effect.raster_length_scale;
    blur_rgba_premultiplied_xy(
      &reflected,
      reflection_radius_gaussian_sigma(blur_radius_px * raster_scale.x),
      reflection_radius_gaussian_sigma(blur_radius_px * raster_scale.y),
    )
  } else {
    reflected
  }
}

/// Word's gradient is allocated over the completed reflected paint and ramp,
/// retaining the ramp's leading edge. Positions, not opacities, are remapped.
/// Keep the staged double arithmetic before the float gradient-stop boundary.
fn word_run_reflection_gradient(
  ramp: PixelBounds,
  paint: PixelBounds,
  start: f32,
  end: f32,
) -> (PixelBounds, f32, f32) {
  let domain = PixelBounds {
    top: ramp.top,
    ..ramp.union(paint)
  };
  let top = f64::from(ramp.top);
  let ramp_height = f64::from(ramp.bottom) - top;
  let domain_height = f64::from(domain.bottom) - top;
  if domain_height <= 0.0 {
    return (domain, start, end);
  }
  let remap =
    |position: f32| ((ramp_height * f64::from(position) + top - top) / domain_height) as f32;
  (domain, remap(start), remap(end))
}

fn effect_ramp(position: f32, first: (f32, f32), second: (f32, f32)) -> f32 {
  let (lower, upper) = if first.0 <= second.0 {
    (first, second)
  } else {
    (second, first)
  };
  if position <= lower.0 {
    return lower.1.clamp(0.0, 1.0);
  }
  if position >= upper.0 {
    return upper.1.clamp(0.0, 1.0);
  }
  let span = upper.0 - lower.0;
  if span <= f32::EPSILON {
    return upper.1.clamp(0.0, 1.0);
  }
  (lower.1 + (upper.1 - lower.1) * ((position - lower.0) / span)).clamp(0.0, 1.0)
}

fn blur_gray_xy(source: &image::GrayImage, sigma_x: f32, sigma_y: f32) -> image::GrayImage {
  if sigma_x == sigma_y {
    image::imageops::blur(source, sigma_x)
  } else {
    gaussian::blur_gray_xy(source, sigma_x, sigma_y)
  }
}

fn blur_rgba_premultiplied_xy(
  source: &image::RgbaImage,
  sigma_x: f32,
  sigma_y: f32,
) -> image::RgbaImage {
  let premultiplied = image::RgbaImage::from_fn(source.width(), source.height(), |x, y| {
    let pixel = source.get_pixel(x, y).0;
    let alpha = u16::from(pixel[3]);
    image::Rgba([
      ((u16::from(pixel[0]) * alpha + 127) / 255) as u8,
      ((u16::from(pixel[1]) * alpha + 127) / 255) as u8,
      ((u16::from(pixel[2]) * alpha + 127) / 255) as u8,
      pixel[3],
    ])
  });
  let blurred = if sigma_x == sigma_y {
    image::imageops::blur(&premultiplied, sigma_x)
  } else {
    gaussian::blur_associated_rgba_xy(&premultiplied, sigma_x, sigma_y)
  };
  image::RgbaImage::from_fn(source.width(), source.height(), |x, y| {
    let pixel = blurred.get_pixel(x, y).0;
    let alpha = u16::from(pixel[3]);
    let unpremultiply = |value: u8| {
      (u16::from(value) * 255 + alpha / 2)
        .checked_div(alpha)
        .unwrap_or_default()
        .min(255) as u8
    };
    image::Rgba([
      unpremultiply(pixel[0]),
      unpremultiply(pixel[1]),
      unpremultiply(pixel[2]),
      pixel[3],
    ])
  })
}

fn apply_soft_edge_on_raster(
  image: &mut image::RgbaImage,
  radius_px: f32,
  scale: EffectRasterScale,
) {
  if radius_px <= f32::EPSILON {
    return;
  }
  let alpha = image::GrayImage::from_fn(image.width(), image.height(), |x, y| {
    image::Luma([image.get_pixel(x, y).0[3]])
  });
  let rx = radius_px * scale.x;
  let ry = radius_px * scale.y;
  let eroded = erode_opaque_alpha_xy(
    &alpha,
    rx.ceil().max(1.0) as usize,
    ry.ceil().max(1.0) as usize,
  );
  let blurred = blur_gray_xy(&eroded, rx, ry);
  for ((pixel, original_alpha), blurred_alpha) in
    image.pixels_mut().zip(alpha.pixels()).zip(blurred.pixels())
  {
    pixel.0[3] =
      ((u16::from(original_alpha.0[0]) * u16::from(blurred_alpha.0[0]) + 127) / 255) as u8;
  }
}

fn erode_opaque_alpha_xy(
  alpha: &image::GrayImage,
  radius_x: usize,
  radius_y: usize,
) -> image::GrayImage {
  let width = alpha.width() as usize;
  let height = alpha.height() as usize;
  let integral_width = width + 1;
  let mut integral = vec![0_u64; integral_width * (height + 1)];
  for y in 0..height {
    let mut row_sum = 0_u64;
    for x in 0..width {
      row_sum += u64::from(alpha.get_pixel(x as u32, y as u32).0[0]);
      integral[(y + 1) * integral_width + x + 1] = integral[y * integral_width + x + 1] + row_sum;
    }
  }
  image::GrayImage::from_fn(alpha.width(), alpha.height(), |x, y| {
    let x = x as usize;
    let y = y as usize;
    if x < radius_x
      || y < radius_y
      || x.saturating_add(radius_x) >= width
      || y.saturating_add(radius_y) >= height
    {
      return image::Luma([0]);
    }
    let left = x - radius_x;
    let top = y - radius_y;
    let right = x + radius_x + 1;
    let bottom = y + radius_y + 1;
    let sum = integral[bottom * integral_width + right] + integral[top * integral_width + left]
      - integral[top * integral_width + right]
      - integral[bottom * integral_width + left];
    let area = (right - left) as u64 * (bottom - top) as u64;
    image::Luma([u8::from(sum == area * u64::from(u8::MAX)) * u8::MAX])
  })
}

fn affine_image(source: &image::RgbaImage, transform: ImageEffectTransform) -> image::RgbaImage {
  let determinant = transform
    .scale_x
    .mul_add(transform.scale_y, -transform.skew_x * transform.skew_y);
  if determinant.abs() <= f32::EPSILON {
    return image::RgbaImage::from_pixel(source.width(), source.height(), image::Rgba([0; 4]));
  }
  image::RgbaImage::from_fn(source.width(), source.height(), |x, y| {
    let destination_x = x as f32 + 0.5 - transform.shift_x_px;
    let destination_y = y as f32 + 0.5 - transform.shift_y_px;
    let source_x =
      (transform.scale_y * destination_x - transform.skew_x * destination_y) / determinant - 0.5;
    let source_y =
      (-transform.skew_y * destination_x + transform.scale_x * destination_y) / determinant - 0.5;
    bilinear_sample(source, source_x, source_y)
  })
}

fn affine_gray_image(
  source: &image::GrayImage,
  transform: ImageEffectTransform,
) -> image::GrayImage {
  let determinant = transform
    .scale_x
    .mul_add(transform.scale_y, -transform.skew_x * transform.skew_y);
  if determinant.abs() <= f32::EPSILON {
    return image::GrayImage::from_pixel(source.width(), source.height(), image::Luma([0]));
  }
  image::GrayImage::from_fn(source.width(), source.height(), |x, y| {
    let destination_x = x as f32 + 0.5 - transform.shift_x_px;
    let destination_y = y as f32 + 0.5 - transform.shift_y_px;
    let source_x =
      (transform.scale_y * destination_x - transform.skew_x * destination_y) / determinant - 0.5;
    let source_y =
      (-transform.skew_y * destination_x + transform.scale_x * destination_y) / determinant - 0.5;
    bilinear_sample_gray(source, source_x, source_y)
  })
}

fn bilinear_sample(source: &image::RgbaImage, x: f32, y: f32) -> image::Rgba<u8> {
  if x < -0.5 || y < -0.5 || x > source.width() as f32 - 0.5 || y > source.height() as f32 - 0.5 {
    return image::Rgba([0; 4]);
  }
  let x0 = x.floor() as i64;
  let y0 = y.floor() as i64;
  let x_amount = x - x.floor();
  let y_amount = y - y.floor();
  let sample = |sample_x: i64, sample_y: i64| {
    if sample_x < 0
      || sample_y < 0
      || sample_x >= i64::from(source.width())
      || sample_y >= i64::from(source.height())
    {
      [0; 4]
    } else {
      source.get_pixel(sample_x as u32, sample_y as u32).0
    }
  };
  let top_left = sample(x0, y0);
  let top_right = sample(x0 + 1, y0);
  let bottom_left = sample(x0, y0 + 1);
  let bottom_right = sample(x0 + 1, y0 + 1);
  let mut output = [0; 4];
  for channel in 0..4 {
    let top = f32::from(top_left[channel])
      + (f32::from(top_right[channel]) - f32::from(top_left[channel])) * x_amount;
    let bottom = f32::from(bottom_left[channel])
      + (f32::from(bottom_right[channel]) - f32::from(bottom_left[channel])) * x_amount;
    output[channel] = (top + (bottom - top) * y_amount).round().clamp(0.0, 255.0) as u8;
  }
  image::Rgba(output)
}

fn bilinear_sample_gray(source: &image::GrayImage, x: f32, y: f32) -> image::Luma<u8> {
  if x < -0.5 || y < -0.5 || x > source.width() as f32 - 0.5 || y > source.height() as f32 - 0.5 {
    return image::Luma([0]);
  }
  let x0 = x.floor() as i64;
  let y0 = y.floor() as i64;
  let x_amount = x - x.floor();
  let y_amount = y - y.floor();
  let sample = |sample_x: i64, sample_y: i64| {
    if sample_x < 0
      || sample_y < 0
      || sample_x >= i64::from(source.width())
      || sample_y >= i64::from(source.height())
    {
      0
    } else {
      source.get_pixel(sample_x as u32, sample_y as u32).0[0]
    }
  };
  let top_left = sample(x0, y0);
  let top_right = sample(x0 + 1, y0);
  let bottom_left = sample(x0, y0 + 1);
  let bottom_right = sample(x0 + 1, y0 + 1);
  let top = f32::from(top_left) + (f32::from(top_right) - f32::from(top_left)) * x_amount;
  let bottom =
    f32::from(bottom_left) + (f32::from(bottom_right) - f32::from(bottom_left)) * x_amount;
  image::Luma([(top + (bottom - top) * y_amount).round().clamp(0.0, 255.0) as u8])
}

fn alpha_outset_box_radii(radius: usize) -> [usize; 3] {
  let quotient = radius / 3;
  let remainder = radius % 3;
  let mut radii = [quotient; 3];
  for radius in radii.iter_mut().skip(3 - remainder) {
    *radius += 1;
  }
  radii
}

fn rounded_box_blur_axis(
  source: &image::GrayImage,
  radius: usize,
  horizontal: bool,
) -> image::GrayImage {
  if radius == 0 || source.width() == 0 || source.height() == 0 {
    return source.clone();
  }
  let radius = radius.min(u32::MAX as usize);
  let divisor = radius as u64 * 2 + 1;
  let mut output = image::GrayImage::new(source.width(), source.height());
  if horizontal {
    let length = source.width() as usize;
    for y in 0..source.height() {
      let mut sum = (0..=radius.min(length - 1))
        .map(|x| u64::from(source.get_pixel(x as u32, y).0[0]))
        .sum::<u64>();
      for x in 0..length {
        output.put_pixel(
          x as u32,
          y,
          image::Luma([((sum + divisor / 2) / divisor) as u8]),
        );
        if x >= radius {
          sum -= u64::from(source.get_pixel((x - radius) as u32, y).0[0]);
        }
        let incoming = x.saturating_add(radius).saturating_add(1);
        if incoming < length {
          sum += u64::from(source.get_pixel(incoming as u32, y).0[0]);
        }
      }
    }
  } else {
    let length = source.height() as usize;
    for x in 0..source.width() {
      let mut sum = (0..=radius.min(length - 1))
        .map(|y| u64::from(source.get_pixel(x, y as u32).0[0]))
        .sum::<u64>();
      for y in 0..length {
        output.put_pixel(
          x,
          y as u32,
          image::Luma([((sum + divisor / 2) / divisor) as u8]),
        );
        if y >= radius {
          sum -= u64::from(source.get_pixel(x, (y - radius) as u32).0[0]);
        }
        let incoming = y.saturating_add(radius).saturating_add(1);
        if incoming < length {
          sum += u64::from(source.get_pixel(x, incoming as u32).0[0]);
        }
      }
    }
  }
  output
}

fn alpha_outset_device_radius(radius_px: f32) -> usize {
  if !radius_px.is_finite() {
    return 0;
  }
  radius_px.abs().floor().min(u32::MAX as f32) as usize
}

fn alpha_outset_mask(
  source: &image::GrayImage,
  radius_x_px: f32,
  radius_y_px: f32,
  positive: bool,
) -> image::GrayImage {
  let mut output = image::GrayImage::from_fn(source.width(), source.height(), |x, y| {
    image::Luma([u8::from(source.get_pixel(x, y).0[0] > 0) * u8::MAX])
  });
  let radius_x = alpha_outset_device_radius(radius_x_px);
  let radius_y = alpha_outset_device_radius(radius_y_px);
  // Office's internal alpha blur is not the public DrawingML blur.  The
  // positive/negative K1-K8 calibration and blind K9-K16 holdouts agree on
  // three ascending horizontal boxes followed by three ascending vertical
  // boxes, with byte rounding after every pass.
  for radius in alpha_outset_box_radii(radius_x) {
    output = rounded_box_blur_axis(&output, radius, true);
  }
  for radius in alpha_outset_box_radii(radius_y) {
    output = rounded_box_blur_axis(&output, radius, false);
  }
  image::GrayImage::from_fn(source.width(), source.height(), |x, y| {
    let alpha = output.get_pixel(x, y).0[0];
    image::Luma([u8::from(if positive {
      alpha > 0
    } else {
      alpha == u8::MAX
    }) * u8::MAX])
  })
}

fn apply_alpha_outset(image: &mut image::RgbaImage, radius_x_px: f32, radius_y_px: f32) {
  if radius_x_px.abs() <= f32::EPSILON && radius_y_px.abs() <= f32::EPSILON {
    return;
  }
  let alpha = image::GrayImage::from_fn(image.width(), image.height(), |x, y| {
    image::Luma([image.get_pixel(x, y).0[3]])
  });
  let alpha = alpha_outset_mask(&alpha, radius_x_px, radius_y_px, radius_x_px > 0.0);
  for (pixel, alpha) in image.pixels_mut().zip(alpha.pixels()) {
    pixel.0[3] = alpha.0[0];
  }
}

pub(crate) fn office_alpha_modulate_amount(value: DrawingmlPercentageValue) -> f32 {
  // MS-OI29500 §20.1.8.6: Office wraps authored values beyond 100% while
  // retaining positive exact multiples as the schema default of 100%.
  let authored = value.as_drawingml_percent().max(0);
  let remainder = authored % 100_000;
  let office_value = if authored > 0 && remainder == 0 {
    100_000
  } else {
    remainder
  };
  office_value as f32 / 100_000.0
}

pub(crate) fn color_change_tolerance(content_type: Option<&str>) -> u8 {
  match content_type {
    Some("image/jpeg" | "image/jpg") => 15,
    Some("image/png" | "image/tiff" | "image/tif") => 1,
    Some("image/bmp" | "image/x-bmp") => 0,
    _ => 9,
  }
}

pub(crate) fn set_color_change_tolerance(effects: &mut [ImageEffect], tolerance: u8) {
  for effect in effects {
    match effect {
      ImageEffect::ColorChange(change) => change.tolerance = tolerance,
      ImageEffect::AlphaModulate(container)
      | ImageEffect::Blend { container, .. }
      | ImageEffect::Container(container) => {
        set_color_change_tolerance(&mut container.effects, tolerance);
      }
      _ => {}
    }
  }
}

fn channel_within_tolerance(actual: u8, expected: u8, tolerance: u8) -> bool {
  actual.abs_diff(expected) <= tolerance
}

fn srgb_luminance(r: u8, g: u8, b: u8) -> u8 {
  ((u32::from(r) * 2_126 + u32::from(g) * 7_152 + u32::from(b) * 722 + 5_000) / 10_000).min(255)
    as u8
}

fn libreoffice_luminance(r: u8, g: u8, b: u8) -> u8 {
  ((u32::from(b) * 29 + u32::from(g) * 151 + u32::from(r) * 76) >> 8) as u8
}

pub(crate) fn duotone_component(luminance: u8, first: u8, second: u8) -> u8 {
  let luminance = u16::from(luminance);
  ((u16::from(second) * luminance / u16::from(u8::MAX))
    + (u16::from(first) * (u16::from(u8::MAX) - luminance) / u16::from(u8::MAX))) as u8
}

fn mso_brightness_contrast_component(value: u8, brightness: i32, contrast: i32) -> u8 {
  let contrast = contrast.clamp(-100, 100) as f32;
  let slope = if contrast >= 0.0 {
    128.0 / (128.0 - 1.27 * contrast)
  } else {
    (128.0 + 1.27 * contrast) / 128.0
  };
  let offset = brightness.clamp(-100, 100) as f32 * 2.55;
  ((f32::from(value) + offset / 2.0 - 128.0) * slope + 128.0 + offset / 2.0)
    .round()
    .clamp(0.0, 255.0) as u8
}

#[cfg(test)]
mod tests {
  #[test]
  fn word_run_finite_mask_preserves_independently_bound_source_images() {
    use super::*;
    let root_bounds = PixelBounds {
      left: 0.0,
      top: 0.0,
      right: 4.0,
      bottom: 8.0,
    };
    let root = EffectGeometry {
      paint: root_bounds,
      anchor: root_bounds,
      shadow_anchor: root_bounds,
      ramp: root_bounds,
    };
    let source = image::RgbaImage::new(20, 12);
    let mut child = source.clone();
    child.put_pixel(13, 6, image::Rgba([90, 120, 150, 255]));
    let child_bounds = Some(ImageEffectContentBounds {
      left_px: 12.0,
      top_px: 4.0,
      width_px: 4.0,
      height_px: 4.0,
    });
    let ImageEffect::Reflection(mut effect) = reflection(&a::Reflection::default()) else {
      unreachable!()
    };
    effect.blur_radius_px = 0.0;
    effect.start_opacity = 1.0;
    effect.end_opacity = 1.0;
    effect.distance_px = 0.0;
    effect.transform.scale_y = 1.0;
    effect.reference = ReflectionReference::WordRunMetrics {
      ascent_px: Some(4.0),
      ramp_extension_px: 0.0,
    };
    for reference in [
      ImageEffectSourceReference::Fill,
      ImageEffectSourceReference::Line,
      ImageEffectSourceReference::FillLine,
      ImageEffectSourceReference::Children,
      ImageEffectSourceReference::EffectMask,
      ImageEffectSourceReference::ReflectionPaint,
    ] {
      let mut sources = ImageEffectSourceImages::default();
      match reference {
        ImageEffectSourceReference::Fill => {
          sources.fill = Some(&child);
          sources.bounds.fill = child_bounds;
        }
        ImageEffectSourceReference::Line => {
          sources.line = Some(&child);
          sources.bounds.line = child_bounds;
        }
        ImageEffectSourceReference::FillLine => {
          sources.fill_line = Some(&child);
          sources.bounds.fill_line = child_bounds;
        }
        ImageEffectSourceReference::Children => {
          sources.children = Some(&child);
          sources.bounds.children = child_bounds;
        }
        ImageEffectSourceReference::EffectMask => {
          sources.effect_mask = Some(&child);
          sources.bounds.effect_mask = child_bounds;
        }
        ImageEffectSourceReference::ReflectionPaint => {
          sources.reflection_paint = Some(&child);
          sources.bounds.reflection_paint = child_bounds;
        }
      }
      let graph = ImageEffectContainer {
        kind: ImageEffectContainerKind::Tree,
        effects: vec![
          ImageEffect::Container(ImageEffectContainer {
            kind: ImageEffectContainerKind::Tree,
            effects: vec![ImageEffect::SourceReference(reference)],
          }),
          ImageEffect::Reflection(effect),
        ],
      };
      let actual = apply_container_with_bounds(
        &source,
        &graph,
        root,
        root,
        sources,
        AlphaOutsetSurfaceScale::default(),
      );
      assert_eq!(
        actual.get_pixel(13, 6).0,
        [90, 120, 150, 255],
        "{reference:?}"
      );
      sources.bounds = ImageEffectSourcePixelBounds::default();
      let unbound = apply_container_with_bounds(
        &source,
        &graph,
        root,
        root,
        sources,
        AlphaOutsetSurfaceScale::default(),
      );
      assert_eq!(
        unbound.get_pixel(13, 6).0[3],
        0,
        "missing source rectangle must be detectable"
      );
    }
  }

  #[test]
  fn word_run_reflection_gradient_is_a_finite_surface_before_blur() {
    use super::*;
    let source = image::RgbaImage::from_pixel(12, 12, image::Rgba([90, 120, 150, 255]));
    let paint = PixelBounds {
      left: 0.0,
      top: 0.0,
      right: 12.0,
      bottom: 12.0,
    };
    let ramp = PixelBounds {
      top: 4.0,
      bottom: 8.0,
      ..paint
    };
    let effect = ImageReflectionEffect {
      blur_radius_px: 0.0,
      raster_length_scale: 1.0,
      bounds_radius_scale: 1.0,
      bounds_radius_offset_px: 0.0,
      start_opacity: 1.0,
      start_position: 0.0,
      end_opacity: 1.0,
      end_position: 1.0,
      fade_direction_degrees: 90.0,
      distance_px: 0.0,
      distance_length_scale: 1.0,
      distance_mode: ReflectionDistanceMode::PostTransformOffset,
      reference: ReflectionReference::WordRunMetrics {
        ascent_px: Some(4.0),
        ramp_extension_px: 0.0,
      },
      direction_degrees: 90.0,
      transform: ImageEffectTransform {
        scale_x: 1.0,
        scale_y: 1.0,
        skew_x: 0.0,
        skew_y: 0.0,
        shift_x_px: 0.0,
        shift_y_px: 0.0,
      },
      alignment: (0.0, 1.0),
      rotate_with_shape: false,
    };
    let reflected = reflection_image(&source, effect, ramp, paint, paint);
    for (x, y, pixel) in reflected.enumerate_pixels() {
      assert_eq!(
        pixel.0[3],
        if y < 4 { 0 } else { 255 },
        "finite gradient ({x}, {y})"
      );
    }
    let legacy = reflection_image(
      &source,
      ImageReflectionEffect {
        reference: ReflectionReference::EffectInput,
        ..effect
      },
      ramp,
      paint,
      paint,
    );
    assert_eq!(legacy, source);
    let blurred = reflection_image(
      &source,
      ImageReflectionEffect {
        blur_radius_px: 2.0,
        ..effect
      },
      ramp,
      paint,
      paint,
    );
    assert!(
      blurred.get_pixel(6, 3).0[3] > 0,
      "blur must follow the finite mask, not be clipped afterwards"
    );
    assert!(blurred.get_pixel(6, 3).0[3] < 255);
  }

  #[test]
  fn word_run_reflection_gradient_remaps_positions_without_moving_the_ramp() {
    use super::*;
    let ramp = PixelBounds {
      left: 10.0,
      top: 20.0,
      right: 40.0,
      bottom: 60.0,
    };
    let paint = PixelBounds {
      left: 0.0,
      top: 5.0,
      right: 50.0,
      bottom: 100.0,
    };
    let (domain, start, end) = word_run_reflection_gradient(ramp, paint, 0.25, 0.75);
    assert_eq!(domain, PixelBounds { top: 20.0, ..paint });
    assert_eq!((start, end), (0.125, 0.375));
    // A shorter paint box must not shorten the reference or remap its stops.
    let contained = PixelBounds {
      left: 15.0,
      top: 30.0,
      right: 35.0,
      bottom: 50.0,
    };
    assert_eq!(
      word_run_reflection_gradient(ramp, contained, 0.25, 0.75),
      (ramp, 0.25, 0.75)
    );
    // Translation changes neither the span nor the normalized stop positions.
    let shifted = word_run_reflection_gradient(
      ramp.translated(7.0, 12.0),
      paint.translated(7.0, 12.0),
      0.25,
      0.75,
    );
    assert_eq!(shifted, (domain.translated(7.0, 12.0), start, end));
  }

  #[test]
  fn word_run_reflection_binding_preserves_device_reference_and_fallback() {
    use super::*;
    let run = WordRunReflectionBinding::new(36.0, Some((286.0, 81.0)), 600.0);
    assert!((run.distance_scale - 2.548_951_1).abs() < 0.000001);
    assert_eq!(run.ascent_px, Some(45.76));
    let empty = WordRunReflectionBinding::new(36.0, None, 600.0);
    assert_eq!(empty.distance_scale, 0.5);
    let root = EffectGeometry {
      paint: PixelBounds {
        left: 10.0,
        top: 20.0,
        right: 40.0,
        bottom: 53.0,
      },
      anchor: PixelBounds {
        left: 0.0,
        top: 0.0,
        right: 50.0,
        bottom: 50.0,
      },
      shadow_anchor: PixelBounds {
        left: 1.0,
        top: 2.0,
        right: 3.0,
        bottom: 4.0,
      },
      ramp: PixelBounds {
        left: 0.0,
        top: 6.0,
        right: 50.0,
        bottom: 50.0,
      },
    };
    let completed = EffectGeometry {
      paint: PixelBounds {
        left: -15.0,
        top: -70.0,
        right: 90.0,
        bottom: 70.0,
      },
      ..root
    };
    for (ascent_px, top, bottom) in [(Some(30.0), 20.0, 50.0), (None, 20.0, 53.0)] {
      let source = reflection_source_geometry(
        ReflectionReference::WordRunMetrics {
          ascent_px,
          ramp_extension_px: 5.0,
        },
        completed,
        root,
      );
      assert_eq!(source.paint, completed.paint);
      assert_eq!(
        source.anchor,
        PixelBounds {
          left: -15.0,
          right: 90.0,
          top,
          bottom
        }
      );
      assert_eq!(
        source.ramp,
        PixelBounds {
          bottom: bottom + 5.0,
          ..source.anchor
        }
      );
      assert_eq!(source.shadow_anchor, root.shadow_anchor);
    }
  }

  #[test]
  fn referenced_bounds_follow_nested_sources_without_replacing_text_anchors() {
    let root = EffectOutputBounds {
      left_pt: 0.0,
      top_pt: 0.0,
      right_pt: 12.0,
      bottom_pt: 24.0,
    };
    let child = EffectOutputBounds {
      left_pt: 30.0,
      top_pt: 60.0,
      right_pt: 36.0,
      bottom_pt: 72.0,
    };
    for reference in [
      ImageEffectSourceReference::Fill,
      ImageEffectSourceReference::Line,
      ImageEffectSourceReference::FillLine,
      ImageEffectSourceReference::Children,
      ImageEffectSourceReference::EffectMask,
      ImageEffectSourceReference::ReflectionPaint,
    ] {
      let mut sources = ImageEffectSourceBounds::default();
      match reference {
        ImageEffectSourceReference::Fill => sources.fill = Some(child),
        ImageEffectSourceReference::Line => sources.line = Some(child),
        ImageEffectSourceReference::FillLine => sources.fill_line = Some(child),
        ImageEffectSourceReference::Children => sources.children = Some(child),
        ImageEffectSourceReference::EffectMask => sources.effect_mask = Some(child),
        ImageEffectSourceReference::ReflectionPaint => sources.reflection_paint = Some(child),
      }
      let graph = ImageEffectContainer {
        kind: ImageEffectContainerKind::Tree,
        effects: vec![
          ImageEffect::Container(ImageEffectContainer {
            kind: ImageEffectContainerKind::Tree,
            effects: vec![ImageEffect::SourceReference(reference)],
          }),
          ImageEffect::RelativeOffset {
            offset_x: 0.5,
            offset_y: 0.5,
          },
        ],
      };
      let actual =
        super::container_output_bounds_with_sources(&graph, root, root, root, root, sources)
          .unwrap();
      // Offset is half the ROOT text cell, not half the referenced paint.
      assert_eq!(
        actual,
        EffectOutputBounds {
          left_pt: 36.0,
          top_pt: 72.0,
          right_pt: 42.0,
          bottom_pt: 84.0,
        }
      );
      let legacy =
        container_output_bounds_with_anchors_and_ramp(&graph, root, root, root, root).unwrap();
      assert_eq!(
        legacy,
        EffectOutputBounds {
          left_pt: 6.0,
          top_pt: 12.0,
          right_pt: 18.0,
          bottom_pt: 36.0,
        }
      );
    }
  }

  use image::{Rgba, RgbaImage};

  use super::{
    EffectBitmapExtentRounding, EffectBitmapOffsetRounding, EffectBitmapTarget,
    EffectBitmapTargetRounding, EffectGeometry, EffectOutputBounds, GlowSpreadRadiusRounding,
    ImageEffect, ImageEffectBlendMode, ImageEffectColorResolver, ImageEffectContainer,
    ImageEffectContainerKind, ImageEffectFill, ImageEffectGradientKind, ImageEffectRelativeRect,
    ImageEffectSourceBounds, ImageEffectSourceGeometry, ImageEffectSourceImages,
    ImageEffectSourcePixelBounds, ImageEffectSourceReference, ImageEffectSourceRequirements,
    ImageEffectTransform, ImageReflectionEffect, PixelBounds, ReflectionDistanceMode,
    ReflectionReference, ResolvedEffectColor, ShadowBlurKernel, ShadowDistanceMode,
    WordprocessingTextEffectHost, WordprocessingTextGlow, apply_container_to_padded_image,
    apply_container_to_padded_image_with_sources,
    apply_container_to_padded_image_with_sources_and_anchor, apply_to_image,
    composite_coverage_source_over_preserving_paint, container_output_bounds,
    container_output_bounds_with_anchor, container_output_bounds_with_anchors,
    container_output_bounds_with_anchors_and_ramp, dpi_compensate_linear_hard,
    effect_bitmap_target_with_rounding_modes, effect_output_geometry,
    effective_backdrop_blur_radius_px, from_effect_dag, from_effect_list,
    from_wordprocessing_text_effects, mso_brightness_contrast_component,
    quantize_outer_shadow_geometry_for_raster, quantized_glow_spread_radius, reflection,
    reflection_image, rotate_container_with_shape, sample_fill, source_requirements,
    split_wordprocessing_shadow_of_glow_dpi_stages, suppress_soft_edge,
    unchanged_foreground_backdrop, wordprocessing_reflection_canvas_bounds,
  };
  use crate::model::RgbColor;
  use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
  use ooxmlsdk::units::DrawingmlPercentageValue;

  struct NoColorResolver;

  #[test]
  fn outer_shadow_output_translation_reaches_pixels_and_bounds() {
    for kernel in [
      ShadowBlurKernel::Direct2dGaussian,
      ShadowBlurKernel::WordTextBalanced {
        prescale_divisor: 1,
      },
    ] {
      let effect = ImageEffect::OuterShadow {
        blur_radius_px: 0.0,
        distance_px: 0.0,
        raster_length_scale: 1.0,
        distance_length_scale: 1.0,
        bounds_radius_scale: 1.0,
        bounds_radius_offset_px: 0.0,
        blur_kernel: kernel,
        direction_degrees: 0.0,
        distance_mode: ShadowDistanceMode::PreTransformOffset,
        transform: ImageEffectTransform {
          scale_x: 1.0,
          scale_y: 1.0,
          skew_x: 0.0,
          skew_y: 0.0,
          shift_x_px: 0.0,
          shift_y_px: 0.0,
        },
        alignment: (0.5, 0.5),
        rotate_with_shape: false,
        color: ResolvedEffectColor {
          color: RgbColor { r: 0, g: 0, b: 0 },
          alpha: 255,
        },
      };
      let mut container = ImageEffectContainer {
        kind: ImageEffectContainerKind::Sibling,
        effects: vec![effect],
      };
      super::translate_outer_shadow_outputs(&mut container, (2.0, -1.0));
      let mut source = RgbaImage::new(8, 8);
      source.put_pixel(2, 3, Rgba([255; 4]));
      apply_to_image(&mut source, &container.effects);
      assert_eq!(source.get_pixel(4, 2).0, [0, 0, 0, 255]);
      assert_eq!(source.pixels().filter(|p| p.0[3] != 0).count(), 1);
      // Bounds use point units, whereas authored effect lengths use CSS px.
      let bounds = EffectOutputBounds {
        left_pt: 1.5,
        top_pt: 2.25,
        right_pt: 2.25,
        bottom_pt: 3.0,
      };
      let actual = container_output_bounds_with_anchor(&container, bounds, bounds).unwrap();
      assert_eq!(
        actual,
        EffectOutputBounds {
          left_pt: 3.0,
          top_pt: 1.5,
          right_pt: 3.75,
          bottom_pt: 2.25
        }
      );
    }
  }

  #[test]
  fn source_over_unpremultiplies_before_quantizing_alpha() {
    let mut white = Rgba([255, 255, 255, 12]);
    super::source_over(&mut white, &Rgba([255, 255, 255, 12]));
    assert_eq!(white.0, [255, 255, 255, 23]);

    // Native shadow/foreground overlap captured in GDB: the previous
    // rounded-alpha divisor yielded red=256, which wrapped to zero.
    let mut shadow = Rgba([255, 255, 255, 83]);
    super::source_over(&mut shadow, &Rgba([255, 199, 160, 128]));
    assert_eq!(shadow.0, [255, 213, 183, 169]);
  }

  #[test]
  fn source_over_preserves_constant_color_for_every_alpha_pair() {
    for source_alpha in 0..=255 {
      for destination_alpha in 0..=255 {
        for rgb in [[255, 255, 255], [0, 0, 0], [7, 121, 254]] {
          let mut destination = Rgba([rgb[0], rgb[1], rgb[2], destination_alpha]);
          super::source_over(
            &mut destination,
            &Rgba([rgb[0], rgb[1], rgb[2], source_alpha]),
          );
          if source_alpha == 0 && destination_alpha == 0 {
            assert_eq!(destination.0, [0; 4]);
          } else {
            assert_eq!(&destination.0[..3], &rgb);
          }
        }
      }
    }
  }

  #[test]
  fn source_over_matches_exact_rational_reference_for_every_alpha_pair() {
    // Independent rational form: normalize only at the final channel store.
    // u64 and remainder-based rounding avoid reproducing byte arithmetic.
    let round = |numerator: u64, denominator: u64| {
      (numerator / denominator + u64::from(2 * (numerator % denominator) >= denominator)) as u8
    };
    for source_alpha in 0..=255_u8 {
      for destination_alpha in 0..=255_u8 {
        let sa = u64::from(source_alpha);
        let da = u64::from(destination_alpha);
        let alpha = 255 * (sa + da) - sa * da;
        for (source_rgb, destination_rgb) in [
          ([0, 127, 255], [255, 128, 0]),
          ([255, 199, 160], [255, 255, 255]),
          ([1, 254, 43], [253, 2, 178]),
        ] {
          let mut destination = Rgba([
            destination_rgb[0],
            destination_rgb[1],
            destination_rgb[2],
            destination_alpha,
          ]);
          super::source_over(
            &mut destination,
            &Rgba([source_rgb[0], source_rgb[1], source_rgb[2], source_alpha]),
          );
          if alpha == 0 {
            assert_eq!(destination.0, [0; 4]);
            continue;
          }
          assert_eq!(destination.0[3], round(alpha, 255));
          for channel in 0..3 {
            let numerator = u64::from(source_rgb[channel]) * sa * 255
              + u64::from(destination_rgb[channel]) * da * (255 - sa);
            assert_eq!(destination.0[channel], round(numerator, alpha));
            assert!(destination.0[channel] >= source_rgb[channel].min(destination_rgb[channel]));
            assert!(destination.0[channel] <= source_rgb[channel].max(destination_rgb[channel]));
          }
        }
      }
    }
  }

  #[test]
  fn coverage_source_over_extends_alpha_without_relighting_existing_paint() {
    let mut paint = RgbaImage::from_vec(3, 1, vec![10, 20, 30, 128, 0, 0, 0, 0, 7, 8, 9, 64])
      .expect("three paint pixels");
    let coverage = RgbaImage::from_vec(
      3,
      1,
      vec![200, 210, 220, 128, 40, 50, 60, 96, 70, 80, 90, 0],
    )
    .expect("three coverage pixels");

    composite_coverage_source_over_preserving_paint(&mut paint, &coverage);

    assert_eq!(paint.get_pixel(0, 0).0, [10, 20, 30, 192]);
    assert_eq!(paint.get_pixel(1, 0).0, [40, 50, 60, 96]);
    assert_eq!(paint.get_pixel(2, 0).0, [7, 8, 9, 64]);
  }

  #[test]
  fn bitmap_target_rounds_crop_offsets_independently_by_axis() {
    let target = effect_bitmap_target_with_rounding_modes(
      EffectOutputBounds {
        left_pt: 10.6,
        top_pt: 10.6,
        right_pt: 30.6,
        bottom_pt: 20.6,
      },
      EffectOutputBounds {
        left_pt: 0.0,
        top_pt: 0.0,
        right_pt: 40.0,
        bottom_pt: 30.0,
      },
      1.0,
      40,
      30,
      EffectBitmapTargetRounding {
        offset_x: EffectBitmapOffsetRounding::Floor,
        offset_y: EffectBitmapOffsetRounding::Nearest,
        extent: EffectBitmapExtentRounding::Nearest,
      },
    )
    .expect("positive independently rounded crop target");

    assert_eq!(
      target,
      EffectBitmapTarget {
        left_px: 10,
        top_px: 11,
        width_px: 20,
        height_px: 10,
      }
    );
  }

  #[test]
  fn simple_outer_shadow_translation_profiles_only_identity_affine_branches() {
    let shadow = ImageEffect::OuterShadow {
      blur_radius_px: 8.0,
      distance_px: 12.0,
      raster_length_scale: 0.5,
      distance_length_scale: 0.25,
      bounds_radius_scale: 2.0,
      bounds_radius_offset_px: 1.0,
      blur_kernel: ShadowBlurKernel::Direct2dGaussian,
      direction_degrees: 45.0,
      distance_mode: ShadowDistanceMode::PostTransformOffset,
      transform: ImageEffectTransform {
        scale_x: 1.0,
        scale_y: 1.0,
        skew_x: 0.0,
        skew_y: 0.0,
        shift_x_px: 0.0,
        shift_y_px: 0.0,
      },
      alignment: (0.0, 0.0),
      rotate_with_shape: false,
      color: ResolvedEffectColor {
        color: RgbColor { r: 0, g: 0, b: 0 },
        alpha: u8::MAX,
      },
    };
    let effects = ImageEffectContainer {
      kind: ImageEffectContainerKind::Sibling,
      effects: vec![shadow.clone()],
    };
    let translation = super::simple_outer_shadow_translation(&effects).unwrap();
    assert_eq!(translation.blur_radius_px, 17.0);
    assert!((translation.offset_x_px - 1.5 * std::f32::consts::SQRT_2).abs() < 0.001);
    assert!((translation.offset_y_px - 1.5 * std::f32::consts::SQRT_2).abs() < 0.001);
    let mut scaled = shadow;
    let ImageEffect::OuterShadow { transform, .. } = &mut scaled else {
      unreachable!()
    };
    transform.scale_x = 0.75;
    let effects = ImageEffectContainer {
      kind: ImageEffectContainerKind::Sibling,
      effects: vec![scaled],
    };
    assert!(super::simple_outer_shadow_translation(&effects).is_none());
  }

  #[test]
  fn outer_shadow_filter_scaling_does_not_mutate_distance_or_output_bounds() {
    let mut effects = ImageEffectContainer {
      kind: ImageEffectContainerKind::Tree,
      effects: vec![ImageEffect::OuterShadow {
        blur_radius_px: 8.0,
        distance_px: 12.0,
        raster_length_scale: 0.5,
        distance_length_scale: 0.25,
        bounds_radius_scale: 2.0,
        bounds_radius_offset_px: 1.0,
        blur_kernel: ShadowBlurKernel::Direct2dGaussian,
        direction_degrees: 0.0,
        distance_mode: ShadowDistanceMode::PostTransformOffset,
        transform: ImageEffectTransform {
          scale_x: 1.0,
          scale_y: 1.0,
          skew_x: 0.0,
          skew_y: 0.0,
          shift_x_px: 0.0,
          shift_y_px: 0.0,
        },
        alignment: (0.5, 0.5),
        rotate_with_shape: false,
        color: ResolvedEffectColor {
          color: RgbColor { r: 0, g: 0, b: 0 },
          alpha: u8::MAX,
        },
      }],
    };
    let bounds_before = container_output_bounds(&effects, 10.0, 10.0).unwrap();

    super::scale_outer_shadow_filter_radius(&mut effects, 0.25);

    let bounds_after = container_output_bounds(&effects, 10.0, 10.0).unwrap();
    assert_eq!(bounds_after, bounds_before);
    assert_eq!(effective_backdrop_blur_radius_px(&effects), 1.0);
    let ImageEffect::OuterShadow {
      blur_radius_px,
      distance_px,
      raster_length_scale,
      distance_length_scale,
      bounds_radius_scale,
      bounds_radius_offset_px,
      ..
    } = &effects.effects[0]
    else {
      unreachable!();
    };
    assert_eq!(*blur_radius_px, 8.0);
    assert_eq!(*distance_px, 12.0);
    assert_eq!(*raster_length_scale, 0.125);
    assert_eq!(*distance_length_scale, 0.25);
    assert_eq!(*bounds_radius_scale, 2.0);
    assert_eq!(*bounds_radius_offset_px, 1.0);
  }

  #[test]
  fn fixed_raster_outer_shadow_bounds_use_integer_device_radius_and_offsets() {
    let mut effects = ImageEffectContainer {
      kind: ImageEffectContainerKind::Sibling,
      effects: vec![
        ImageEffect::Identity,
        ImageEffect::OuterShadow {
          blur_radius_px: 4.0,
          distance_px: 4.0,
          raster_length_scale: 0.25,
          distance_length_scale: 1.0,
          bounds_radius_scale: 1.0,
          bounds_radius_offset_px: 0.0,
          blur_kernel: ShadowBlurKernel::Direct2dGaussian,
          direction_degrees: 45.0,
          distance_mode: ShadowDistanceMode::PostTransformOffset,
          transform: ImageEffectTransform {
            scale_x: 1.0,
            scale_y: 1.0,
            skew_x: 0.0,
            skew_y: 0.0,
            shift_x_px: 0.0,
            shift_y_px: 0.0,
          },
          alignment: (0.0, 0.0),
          rotate_with_shape: true,
          color: ResolvedEffectColor {
            color: RgbColor { r: 0, g: 0, b: 0 },
            alpha: 255,
          },
        },
      ],
    };
    let source = EffectOutputBounds {
      left_pt: 0.0,
      top_pt: 0.0,
      right_pt: 100.0,
      bottom_pt: 40.0,
    };
    let continuous = container_output_bounds_with_anchor(&effects, source, source).unwrap();

    quantize_outer_shadow_geometry_for_raster(&mut effects, 200.0);
    let quantized = container_output_bounds_with_anchor(&effects, source, source).unwrap();

    assert!((continuous.right_pt - 105.121_32).abs() < 0.001);
    // 4 CSS pixels become floor(4 * 200/96) = 8 device pixels;
    // each 45-degree offset component becomes floor(4/sqrt(2) * 200/96)
    // = 5 device pixels. At 200 DPI the far-edge expansion is therefore
    // (8 + 5) * 72/200 = 4.68pt.
    assert!((quantized.right_pt - 104.68).abs() < 0.001);
  }

  #[test]
  fn outer_shadow_affine_transforms_the_blurred_output_bounds() {
    let effects = ImageEffectContainer {
      kind: ImageEffectContainerKind::Tree,
      effects: vec![ImageEffect::OuterShadow {
        blur_radius_px: 4.0,
        distance_px: 0.0,
        raster_length_scale: 1.0,
        distance_length_scale: 1.0,
        bounds_radius_scale: 1.0,
        bounds_radius_offset_px: 0.0,
        blur_kernel: ShadowBlurKernel::Direct2dGaussian,
        direction_degrees: 0.0,
        distance_mode: ShadowDistanceMode::PostTransformOffset,
        transform: ImageEffectTransform {
          scale_x: 0.5,
          scale_y: 0.5,
          skew_x: 0.0,
          skew_y: 0.0,
          shift_x_px: 0.0,
          shift_y_px: 0.0,
        },
        alignment: (0.0, 0.0),
        rotate_with_shape: true,
        color: ResolvedEffectColor {
          color: RgbColor { r: 0, g: 0, b: 0 },
          alpha: 255,
        },
      }],
    };

    let bounds = container_output_bounds(&effects, 100.0, 40.0).expect("shadow output bounds");

    // Four CSS pixels are three points. The Shadow effect grows the source
    // by that amount before the following 0.5 affine, so both the source and
    // its soft border are halved. An affine-first/outset-second calculation
    // would incorrectly produce [-3, -3, 53, 23].
    assert!((bounds.left_pt + 1.5).abs() < 0.000_1);
    assert!((bounds.top_pt + 1.5).abs() < 0.000_1);
    assert!((bounds.right_pt - 51.5).abs() < 0.000_1);
    assert!((bounds.bottom_pt - 21.5).abs() < 0.000_1);
  }

  #[test]
  fn word_text_shadow_blur_expands_the_affine_output_bounds() {
    let effects = ImageEffectContainer {
      kind: ImageEffectContainerKind::Tree,
      effects: vec![ImageEffect::OuterShadow {
        blur_radius_px: 4.0,
        distance_px: 0.0,
        raster_length_scale: 1.0,
        distance_length_scale: 1.0,
        bounds_radius_scale: 1.0,
        bounds_radius_offset_px: 0.0,
        blur_kernel: ShadowBlurKernel::WordTextBalanced {
          prescale_divisor: 1,
        },
        direction_degrees: 0.0,
        distance_mode: ShadowDistanceMode::PreTransformOffset,
        transform: ImageEffectTransform {
          scale_x: 0.5,
          scale_y: 0.5,
          skew_x: 0.0,
          skew_y: 0.0,
          shift_x_px: 0.0,
          shift_y_px: 0.0,
        },
        alignment: (0.0, 0.0),
        rotate_with_shape: true,
        color: ResolvedEffectColor {
          color: RgbColor { r: 0, g: 0, b: 0 },
          alpha: 255,
        },
      }],
    };

    let bounds = container_output_bounds(&effects, 100.0, 40.0).expect("shadow output bounds");

    // W14 first halves the text plane, then grows that result by the four-CSS-
    // pixel (three-point) physical blur support. Top-left alignment keeps half
    // a radius on each near edge and the remaining one-and-a-half radii on the
    // far edges. This is the stopping counterexample to both blur-before-
    // affine bounds and a symmetrically reattached padded output rectangle.
    assert!((bounds.left_pt + 1.5).abs() < 0.000_1);
    assert!((bounds.top_pt + 1.5).abs() < 0.000_1);
    assert!((bounds.right_pt - 54.5).abs() < 0.000_1);
    assert!((bounds.bottom_pt - 24.5).abs() < 0.000_1);
  }

  #[test]
  fn word_text_shadow_aligns_the_padded_blur_output_rectangle() {
    let effects = ImageEffectContainer {
      kind: ImageEffectContainerKind::Tree,
      effects: vec![ImageEffect::OuterShadow {
        blur_radius_px: 4.0,
        distance_px: 0.0,
        raster_length_scale: 1.0,
        distance_length_scale: 1.0,
        bounds_radius_scale: 1.0,
        bounds_radius_offset_px: 0.0,
        blur_kernel: ShadowBlurKernel::WordTextBalanced {
          prescale_divisor: 1,
        },
        direction_degrees: 0.0,
        distance_mode: ShadowDistanceMode::PreTransformOffset,
        transform: ImageEffectTransform {
          scale_x: 0.4,
          scale_y: 0.4,
          skew_x: 0.0,
          skew_y: 0.0,
          shift_x_px: 0.0,
          shift_y_px: 0.0,
        },
        alignment: (0.0, 0.5),
        rotate_with_shape: true,
        color: ResolvedEffectColor {
          color: RgbColor { r: 0, g: 0, b: 0 },
          alpha: 255,
        },
      }],
    };

    let bounds = container_output_bounds(&effects, 100.0, 40.0).expect("shadow output bounds");

    // Four CSS pixels are three points. With `algn=l`, Word keeps 0.4 of
    // that radius before the scaled input rectangle and the remaining 1.6
    // radii after it. Vertical center alignment is the zero-shift control.
    assert!((bounds.left_pt + 1.2).abs() < 0.000_1);
    assert!((bounds.top_pt - 9.0).abs() < 0.000_1);
    assert!((bounds.right_pt - 44.8).abs() < 0.000_1);
    assert!((bounds.bottom_pt - 31.0).abs() < 0.000_1);
  }

  #[test]
  fn effect_list_backdrop_can_leave_an_identity_foreground_unrasterized() {
    let container = ImageEffectContainer {
      kind: ImageEffectContainerKind::Sibling,
      effects: vec![
        ImageEffect::OuterShadow {
          blur_radius_px: 0.0,
          distance_px: 2.0,
          raster_length_scale: 1.0,
          distance_length_scale: 1.0,
          bounds_radius_scale: 1.0,
          bounds_radius_offset_px: 0.0,
          blur_kernel: ShadowBlurKernel::Direct2dGaussian,
          direction_degrees: 0.0,
          distance_mode: ShadowDistanceMode::PostTransformOffset,
          transform: ImageEffectTransform {
            scale_x: 1.0,
            scale_y: 1.0,
            skew_x: 0.0,
            skew_y: 0.0,
            shift_x_px: 0.0,
            shift_y_px: 0.0,
          },
          alignment: (0.5, 0.5),
          rotate_with_shape: false,
          color: ResolvedEffectColor {
            color: RgbColor { r: 1, g: 2, b: 3 },
            alpha: 255,
          },
        },
        ImageEffect::Container(ImageEffectContainer {
          kind: ImageEffectContainerKind::Tree,
          effects: vec![ImageEffect::Identity],
        }),
      ],
    };

    let backdrop = unchanged_foreground_backdrop(&container).expect("separable backdrop");
    assert_eq!(backdrop.kind, ImageEffectContainerKind::Sibling);
    assert!(matches!(
      backdrop.effects.as_slice(),
      [ImageEffect::OuterShadow { .. }]
    ));
  }

  #[test]
  fn backdrop_blur_radius_uses_the_effective_filter_kernel() {
    let color = ResolvedEffectColor {
      color: RgbColor { r: 1, g: 2, b: 3 },
      alpha: 255,
    };
    let glow = ImageEffectContainer {
      kind: ImageEffectContainerKind::Sibling,
      effects: vec![ImageEffect::Glow {
        radius_px: 32.0 / 3.0,
        raster_length_scale: 1.0,
        bounds_radius_scale: 1.0,
        bounds_radius_offset_px: 0.0,
        spread_ratio: 0.5,
        spread_kernel: super::GlowSpreadKernel::Square,
        spread_radius_rounding: super::GlowSpreadRadiusRounding::Outward,
        blur_kernel: super::GlowBlurKernel::Stack,
        color,
      }],
    };
    assert!((effective_backdrop_blur_radius_px(&glow) - 16.0 / 3.0).abs() < 0.0001);

    let shadow = ImageEffectContainer {
      kind: ImageEffectContainerKind::Sibling,
      effects: vec![ImageEffect::OuterShadow {
        blur_radius_px: 12.0,
        distance_px: 0.0,
        raster_length_scale: 0.5,
        distance_length_scale: 1.0,
        bounds_radius_scale: 1.0,
        bounds_radius_offset_px: 0.0,
        blur_kernel: ShadowBlurKernel::Direct2dGaussian,
        direction_degrees: 0.0,
        distance_mode: ShadowDistanceMode::PostTransformOffset,
        transform: ImageEffectTransform {
          scale_x: 1.0,
          scale_y: 1.0,
          skew_x: 0.0,
          skew_y: 0.0,
          shift_x_px: 0.0,
          shift_y_px: 0.0,
        },
        alignment: (0.5, 0.5),
        rotate_with_shape: false,
        color,
      }],
    };
    assert!((effective_backdrop_blur_radius_px(&shadow) - 6.0).abs() < f32::EPSILON);
  }

  #[test]
  fn word_text_backdrop_can_leave_a_direct_identity_foreground_unrasterized() {
    let container = ImageEffectContainer {
      kind: ImageEffectContainerKind::Sibling,
      effects: vec![
        ImageEffect::Glow {
          radius_px: 2.0,
          raster_length_scale: 1.0,
          bounds_radius_scale: 1.0,
          bounds_radius_offset_px: 0.0,
          spread_ratio: 0.5,
          spread_kernel: super::GlowSpreadKernel::Square,
          spread_radius_rounding: super::GlowSpreadRadiusRounding::Outward,
          blur_kernel: super::GlowBlurKernel::Stack,
          color: ResolvedEffectColor {
            color: RgbColor { r: 1, g: 2, b: 3 },
            alpha: 255,
          },
        },
        ImageEffect::Identity,
      ],
    };

    let backdrop = unchanged_foreground_backdrop(&container).expect("separable backdrop");
    assert_eq!(backdrop.kind, ImageEffectContainerKind::Sibling);
    assert!(matches!(
      backdrop.effects.as_slice(),
      [ImageEffect::Glow { .. }]
    ));
  }

  #[test]
  fn word_flat_text_glow_uses_radial_alpha_outset_then_finite_gaussian() {
    let effects = from_wordprocessing_text_effects(
      Some(WordprocessingTextGlow {
        radius_px: 12.0,
        raster_length_scale: 1.0,
        geometry_length_scale: 1.0,
        color: ResolvedEffectColor {
          color: RgbColor { r: 1, g: 2, b: 3 },
          alpha: 255,
        },
      }),
      None,
      None,
      WordprocessingTextEffectHost::FlatText,
    )
    .expect("word text glow");

    assert!(matches!(
      effects.effects.as_slice(),
      [
        ImageEffect::Glow {
          spread_ratio,
          spread_kernel,
          spread_radius_rounding,
          blur_kernel,
          ..
        },
        ImageEffect::Identity
      ] if (*spread_ratio - 0.5).abs() <= f32::EPSILON
        && *spread_kernel == super::GlowSpreadKernel::WordFlatAlphaOutset
        && *spread_radius_rounding == super::GlowSpreadRadiusRounding::Inward
        && *blur_kernel == super::GlowBlurKernel::WordShapeGaussian
    ));
  }

  #[test]
  fn word_static_3d_effect_normalization_matches_office_nodes() {
    // Direct Office Gaussian nodes, same configured source/options. The glow
    // node receives half the normalized full radius; shadow blur and distance
    // independently pass through the same normalizer with another intercept.
    for (font, glow_half_device, shadow_device) in [
      (18.0_f64, 16.099_249_663_669_514, 31.825_856_812_315_585),
      (24.0, 19.579_229_170_446_09, 38.836_536_174_955_356),
      (36.0, 25.841_055_841_349_526, 51.451_455_006_702_524),
      (48.0, 31.494_299_228_649_254, 62.840_337_231_481_24),
    ] {
      let scale = (font.powf(0.7) / 72.0_f64.powf(0.7)) as f32;
      let radius_px = 10.0 * 96.0 / 72.0;
      let glow = super::word_static_3d_effect_length_scale(radius_px, scale, 1.0);
      let shadow = super::word_static_3d_effect_length_scale(radius_px, scale, 0.4);
      assert!((f64::from(radius_px * glow) * 600.0 / 96.0 / 2.0 - glow_half_device).abs() < 1e-5);
      assert!((f64::from(radius_px * shadow) * 600.0 / 96.0 - shadow_device).abs() < 1e-5);
      let distance_px = 62.0 * 96.0 / 72.0;
      let distance = super::word_static_3d_effect_length_scale(distance_px, scale, 0.4);
      let expected = (62.0 * 600.0 / 72.0 - 0.4) * font.powf(0.7) / 72.0_f64.powf(0.7) + 0.4;
      assert!((f64::from(distance_px * distance) * 600.0 / 96.0 - expected).abs() < 5e-5);
    }
  }

  #[test]
  fn word_static_3d_effect_normalization_has_device_space_boundaries() {
    for floor in [0.4, 1.0] {
      let threshold = floor * 96.0 / 600.0;
      for scale in [0.25, 0.5, 1.0, 2.0] {
        for radius in [0.0, threshold * 0.5, threshold] {
          assert_eq!(
            super::word_static_3d_effect_length_scale(radius, scale, floor),
            0.0
          );
        }
        for radius in [threshold + 0.001, 1.0, 10.0, 100.0] {
          let actual = radius * super::word_static_3d_effect_length_scale(radius, scale, floor);
          let expected = (radius - threshold) * scale + threshold;
          assert!((actual - expected).abs() < 2e-5);
        }
      }
    }
  }

  #[test]
  fn word_static_3d_glow_separates_normalized_support_from_bitmap_guard() {
    let scale = (36.0_f64.powf(0.7) / 72.0_f64.powf(0.7)) as f32;
    let glow = WordprocessingTextGlow {
      radius_px: 10.0 * 96.0 / 72.0,
      raster_length_scale: scale,
      geometry_length_scale: scale,
      color: ResolvedEffectColor {
        color: RgbColor { r: 1, g: 2, b: 3 },
        alpha: 255,
      },
    };
    let effect = from_wordprocessing_text_effects(
      Some(glow),
      None,
      None,
      WordprocessingTextEffectHost::Static3d,
    )
    .unwrap();
    let bounds = super::container_output_bounds(&effect, 20.0, 30.0).unwrap();
    let radius_pt = 51.682_111_682_699_05 * 72.0 / 600.0;
    let ImageEffect::Glow {
      radius_px,
      raster_length_scale,
      bounds_radius_scale,
      bounds_radius_offset_px,
      ..
    } = effect.effects[0]
    else {
      panic!("expected independent glow branch");
    };
    // The actual Office node fixes the normalized support, not the surface
    // allocator's terminal samples. In particular, never feed the latter back
    // into the sampled radius when preserving the bitmap allocation contract.
    assert!((f64::from(radius_px * raster_length_scale) * 72.0 / 96.0 - radius_pt).abs() < 1e-5);
    assert_eq!(raster_length_scale, bounds_radius_scale);
    assert_eq!(bounds_radius_offset_px, 2.0 * 96.0 / 200.0);
    let allocation_radius_pt = radius_pt + 2.0 * 72.0 / 200.0;
    assert!((f64::from(bounds.left_pt) + allocation_radius_pt).abs() < 1e-5);
    assert!((f64::from(bounds.bottom_pt) - 30.0 - allocation_radius_pt).abs() < 1e-5);
    let flat = from_wordprocessing_text_effects(
      Some(glow),
      None,
      None,
      WordprocessingTextEffectHost::FlatText,
    )
    .unwrap();
    assert!(
      matches!(flat.effects[0], ImageEffect::Glow { bounds_radius_scale, bounds_radius_offset_px, .. }
      if bounds_radius_scale == scale && bounds_radius_offset_px == 96.0 / 200.0)
    );
  }

  #[test]
  fn word_static_3d_glow_retains_the_internal_alpha_blur_outset() {
    let effects = from_wordprocessing_text_effects(
      Some(WordprocessingTextGlow {
        radius_px: 12.0,
        raster_length_scale: 1.0,
        geometry_length_scale: 1.0,
        color: ResolvedEffectColor {
          color: RgbColor { r: 1, g: 2, b: 3 },
          alpha: 255,
        },
      }),
      None,
      None,
      WordprocessingTextEffectHost::Static3d,
    )
    .expect("static-3-D word text glow");

    assert!(matches!(
      effects.effects.as_slice(),
      [
        ImageEffect::Glow {
          spread_kernel: super::GlowSpreadKernel::AlphaOutset,
          blur_kernel: super::GlowBlurKernel::WordStatic3dGaussian,
          ..
        },
        ImageEffect::Identity
      ]
    ));
  }

  #[test]
  fn word_static_3d_glow_keeps_small_tier_and_resolves_first_balanced_tier() {
    let small = super::word_static_3d_glow_axis_profile(4.182_77);
    assert!((small.spread_radius_px - 2.091_385).abs() < 0.000_001);
    assert_eq!(small.blur_support_px, 2);
    assert!((small.sigma_px - 0.697_128_3).abs() < 0.000_001);

    let large = super::word_static_3d_glow_axis_profile(10.456_924);
    assert!((large.spread_radius_px - 3.485_641_5).abs() < 0.000_001);
    assert_eq!(large.blur_support_px, 7);
    assert!((large.sigma_px - 2.323_760_7).abs() < 0.000_001);

    let exact_boundary =
      super::word_static_3d_glow_axis_profile(super::DIRECT2D_BALANCED_BLUR_PRESCALE_STEP_PX * 2.0);
    assert!((exact_boundary.spread_radius_px - 3.84).abs() < f32::EPSILON);
    assert_eq!(exact_boundary.blur_support_px, 3);
    assert!((exact_boundary.sigma_px - 1.28).abs() < f32::EPSILON);
  }

  #[test]
  fn direct2d_balanced_prescale_tiers_keep_exact_boundaries() {
    let cases = [
      (36_575, 1),
      (36_576, 1),
      (36_577, 2),
      (73_151, 2),
      (73_152, 2),
      (73_153, 3),
      (109_727, 3),
      (109_728, 3),
      (109_729, 4),
    ];
    for (radius_emu, expected) in cases {
      assert_eq!(
        super::direct2d_balanced_blur_prescale_divisor(radius_emu as f32 / 9_525.0),
        expected,
        "radius_emu={radius_emu}"
      );
    }

    let natural_groupshapes_radius_px =
      10.0 * crate::units::CSS_PIXELS_PER_INCH / crate::units::POINTS_PER_INCH * 0.615_572_7;
    assert_eq!(
      super::direct2d_balanced_blur_prescale_divisor(natural_groupshapes_radius_px),
      3
    );
  }

  #[test]
  fn word_text_balanced_blur_prescales_inclusive_far_edge() {
    assert_eq!(super::balanced_prescale_extent(595, 3), 199);
    assert_eq!(super::balanced_prescale_extent(391, 3), 131);

    let source = image::GrayImage::from_fn(31, 19, |x, y| {
      image::Luma([if (8..=22).contains(&x) && (5..=13).contains(&y) {
        u8::MAX
      } else {
        0
      }])
    });
    let full_resolution = super::word_text_balanced_blur_alpha(&source, 8.207_636, 1);
    let prescaled = super::word_text_balanced_blur_alpha(&source, 8.207_636, 3);

    assert_eq!(prescaled.dimensions(), source.dimensions());
    assert_ne!(prescaled, full_resolution);
    assert!(prescaled.pixels().any(|pixel| pixel.0[0] != 0));
  }

  #[test]
  fn word_text_shadow_uses_balanced_a8_profile() {
    let effects = from_wordprocessing_text_effects(
      None,
      Some(super::WordprocessingTextShadow {
        blur_radius_px: 12.0,
        distance_px: 0.0,
        raster_length_scale: 0.25,
        geometry_length_scale: 0.75,
        direction_degrees: 0.0,
        scale_x: 1.0,
        scale_y: 1.0,
        skew_x_degrees: 0.0,
        skew_y_degrees: 0.0,
        alignment: (0.5, 0.5),
        color: ResolvedEffectColor {
          color: RgbColor { r: 1, g: 2, b: 3 },
          alpha: 255,
        },
      }),
      None,
      WordprocessingTextEffectHost::FlatText,
    )
    .expect("word text shadow");

    assert!(matches!(
      effects.effects.as_slice(),
      [
        ImageEffect::OuterShadow {
          raster_length_scale,
          distance_length_scale,
          bounds_radius_scale,
          bounds_radius_offset_px,
          blur_kernel: ShadowBlurKernel::WordTextBalanced {
            prescale_divisor: 1,
          },
          ..
        },
        ImageEffect::Identity
      ] if (*raster_length_scale - 0.25).abs() <= f32::EPSILON
        && (*distance_length_scale - 0.75).abs() <= f32::EPSILON
        && (*bounds_radius_scale - 0.75).abs() <= f32::EPSILON
        && (*bounds_radius_offset_px - 0.96).abs() <= f32::EPSILON
    ));
  }

  #[test]
  fn word_text_shadow_consumes_glow_before_the_shadow_filter() {
    let effects = from_wordprocessing_text_effects(
      Some(WordprocessingTextGlow {
        radius_px: 12.0,
        raster_length_scale: 0.25,
        geometry_length_scale: 0.25,
        color: ResolvedEffectColor {
          color: RgbColor { r: 1, g: 2, b: 3 },
          alpha: 153,
        },
      }),
      Some(super::WordprocessingTextShadow {
        blur_radius_px: 12.0,
        distance_px: 20.0,
        raster_length_scale: 0.25,
        geometry_length_scale: 0.25,
        direction_degrees: 180.0,
        scale_x: 0.7,
        scale_y: 0.7,
        skew_x_degrees: 0.0,
        skew_y_degrees: 0.0,
        alignment: (0.0, 0.5),
        color: ResolvedEffectColor {
          color: RgbColor { r: 4, g: 5, b: 6 },
          alpha: 102,
        },
      }),
      None,
      WordprocessingTextEffectHost::FlatText,
    )
    .expect("word text glow and shadow");

    let [
      ImageEffect::Container(shadow_branch),
      ImageEffect::Glow { .. },
      ImageEffect::Identity,
    ] = effects.effects.as_slice()
    else {
      panic!("expected shadow-of-glow, visible glow and foreground branches");
    };
    assert_eq!(shadow_branch.kind, ImageEffectContainerKind::Tree);
    assert!(matches!(
      shadow_branch.effects.as_slice(),
      [
        ImageEffect::Container(ImageEffectContainer {
          kind: ImageEffectContainerKind::Sibling,
          effects: glow_source,
        }),
        ImageEffect::OuterShadow {
          bounds_radius_offset_px,
          blur_kernel: ShadowBlurKernel::WordTextBalanced {
            prescale_divisor: 1,
          },
          ..
        }
      ] if matches!(
        glow_source.as_slice(),
        [ImageEffect::Glow { .. }, ImageEffect::Identity]
      ) && (*bounds_radius_offset_px - 0.24).abs() <= f32::EPSILON
    ));

    let shadow_only_branch = ImageEffectContainer {
      kind: ImageEffectContainerKind::Sibling,
      effects: vec![effects.effects[0].clone()],
    };
    let (source_stage, target_stage) =
      split_wordprocessing_shadow_of_glow_dpi_stages(&shadow_only_branch)
        .expect("word shadow-of-glow stages");
    assert!(matches!(
      source_stage.effects.as_slice(),
      [ImageEffect::Glow { .. }, ImageEffect::Identity]
    ));
    assert!(matches!(
      target_stage.effects.as_slice(),
      [ImageEffect::OuterShadow { .. }]
    ));
    assert!(split_wordprocessing_shadow_of_glow_dpi_stages(&effects).is_none());
  }

  #[test]
  fn linear_hard_dpi_compensation_uses_four_premultiplied_taps() {
    let mut source = RgbaImage::from_pixel(2, 2, Rgba([0, 255, 0, 0]));
    source.get_pixel_mut(0, 0).0 = [200, 100, 50, 255];

    let resolved = dpi_compensate_linear_hard(&source, 200.0 / 72.0, 100.0 / 72.0, 1, 1)
      .expect("valid DPI compensation");

    assert_eq!(resolved.get_pixel(0, 0).0, [200, 100, 50, 64]);
  }

  #[test]
  fn linear_hard_dpi_compensation_keeps_same_dpi_pixels_exact() {
    let source = RgbaImage::from_fn(2, 2, |x, y| {
      Rgba([
        (x * 80 + y * 20) as u8,
        (x * 30 + y * 90) as u8,
        (x * 10 + y * 40) as u8,
        (x * 70 + y * 50 + 65) as u8,
      ])
    });

    let resolved = dpi_compensate_linear_hard(&source, 200.0 / 72.0, 200.0 / 72.0, 2, 2)
      .expect("same-DPI realization");

    assert_eq!(resolved, source);
  }

  #[test]
  fn word_text_reflection_consumes_each_complete_visible_effect_combination() {
    let glow = WordprocessingTextGlow {
      radius_px: 12.0,
      raster_length_scale: 0.25,
      geometry_length_scale: 0.25,
      color: ResolvedEffectColor {
        color: RgbColor { r: 1, g: 2, b: 3 },
        alpha: 153,
      },
    };
    let shadow = super::WordprocessingTextShadow {
      blur_radius_px: 12.0,
      distance_px: 20.0,
      raster_length_scale: 0.25,
      geometry_length_scale: 0.25,
      direction_degrees: 180.0,
      scale_x: 0.7,
      scale_y: 0.7,
      skew_x_degrees: 0.0,
      skew_y_degrees: 0.0,
      alignment: (0.0, 0.5),
      color: ResolvedEffectColor {
        color: RgbColor { r: 4, g: 5, b: 6 },
        alpha: 102,
      },
    };
    let reflection = super::WordprocessingTextReflection {
      blur_radius_px: 0.0,
      raster_length_scale: 1.0,
      geometry_length_scale: 0.25,
      start_opacity: 0.5,
      start_position: 0.0,
      end_opacity: 0.0,
      end_position: 1.0,
      distance_px: 4.0,
      distance_length_scale: 1.0,
      direction_degrees: 90.0,
      fade_direction_degrees: 90.0,
      scale_x: 1.0,
      scale_y: -1.0,
      skew_x_degrees: 0.0,
      skew_y_degrees: 0.0,
      alignment: (0.5, 1.0),
    };

    for (has_glow, has_shadow) in [(false, false), (true, false), (false, true), (true, true)] {
      let effects = from_wordprocessing_text_effects(
        has_glow.then_some(glow),
        has_shadow.then_some(shadow),
        Some(reflection),
        WordprocessingTextEffectHost::FlatText,
      )
      .expect("word text reflection");
      let [ImageEffect::Container(reflection_branch), upright @ ..] = effects.effects.as_slice()
      else {
        panic!("expected reflected and upright branches");
      };
      assert_eq!(reflection_branch.kind, ImageEffectContainerKind::Tree);
      let [
        ImageEffect::Container(reflected_source),
        ImageEffect::Reflection(reflection_effect),
      ] = reflection_branch.effects.as_slice()
      else {
        panic!("expected completed visible source followed by reflection");
      };
      assert_eq!(reflected_source.kind, ImageEffectContainerKind::Sibling);
      assert_eq!(reflected_source.effects.as_slice(), upright);
      assert!(matches!(upright.last(), Some(ImageEffect::Identity)));
      assert_eq!(reflection_effect.bounds_radius_scale, 0.25);
      let expected_terminal_bounds = if has_glow || has_shadow { 0.0 } else { 0.16 };
      assert!(
        (reflection_effect.bounds_radius_offset_px - expected_terminal_bounds).abs()
          <= f32::EPSILON
      );

      let backdrop = unchanged_foreground_backdrop(&effects).expect("separable foreground");
      assert_eq!(
        backdrop.effects.as_slice(),
        &effects.effects[..effects.effects.len() - 1]
      );
    }
  }

  #[test]
  fn static_3d_reflection_keeps_the_projected_flat_foreground_source() {
    let effects = from_wordprocessing_text_effects(
      Some(WordprocessingTextGlow {
        radius_px: 12.0,
        raster_length_scale: 0.25,
        geometry_length_scale: 0.25,
        color: ResolvedEffectColor {
          color: RgbColor { r: 1, g: 2, b: 3 },
          alpha: 153,
        },
      }),
      Some(super::WordprocessingTextShadow {
        blur_radius_px: 12.0,
        distance_px: 20.0,
        raster_length_scale: 0.25,
        geometry_length_scale: 0.25,
        direction_degrees: 180.0,
        scale_x: 0.7,
        scale_y: 0.7,
        skew_x_degrees: 0.0,
        skew_y_degrees: 0.0,
        alignment: (0.0, 0.5),
        color: ResolvedEffectColor {
          color: RgbColor { r: 4, g: 5, b: 6 },
          alpha: 102,
        },
      }),
      Some(super::WordprocessingTextReflection {
        blur_radius_px: 0.0,
        raster_length_scale: 1.0,
        geometry_length_scale: 0.25,
        start_opacity: 0.5,
        start_position: 0.0,
        end_opacity: 0.0,
        end_position: 1.0,
        distance_px: 4.0,
        distance_length_scale: 1.0,
        direction_degrees: 90.0,
        fade_direction_degrees: 90.0,
        scale_x: 1.0,
        scale_y: -1.0,
        skew_x_degrees: 0.0,
        skew_y_degrees: 0.0,
        alignment: (0.5, 1.0),
      }),
      WordprocessingTextEffectHost::Static3d,
    )
    .expect("word text effects");

    let ImageEffect::Container(reflection_branch) = &effects.effects[0] else {
      panic!("expected reflection branch");
    };
    let ImageEffect::Container(reflected_source) = &reflection_branch.effects[0] else {
      panic!("expected reflected source");
    };
    assert!(matches!(
      reflected_source.effects.last(),
      Some(ImageEffect::Identity)
    ));
    assert!(matches!(
      effects.effects.last(),
      Some(ImageEffect::Identity)
    ));
    let ImageEffect::Container(shadow_branch) = &reflected_source.effects[0] else {
      panic!("expected shadow branch");
    };
    let ImageEffect::Container(glow_source) = &shadow_branch.effects[0] else {
      panic!("expected glow source");
    };
    assert!(matches!(
      glow_source.effects.last(),
      Some(ImageEffect::Identity)
    ));
    let mut bound = effects.clone();
    super::bind_wordprocessing_glow_mask(&mut bound);
    let ImageEffect::Container(reflection) = &bound.effects[0] else {
      panic!("reflection");
    };
    let ImageEffect::Container(reflected) = &reflection.effects[0] else {
      panic!("source");
    };
    // Binding covers both visible copies; the shadow-of-glow graph and both
    // original painted Identity leaves remain byte-for-byte equivalent.
    assert_eq!(bound.effects[1], effects.effects[1]);
    assert_eq!(reflected.effects[0], reflected_source.effects[0]);
    assert_eq!(reflected.effects[1], bound.effects[2]);
    assert_eq!(reflected.effects.last(), Some(&ImageEffect::Identity));
    assert_eq!(bound.effects.last(), Some(&ImageEffect::Identity));
    assert!(matches!(&bound.effects[2], ImageEffect::Container(masked)
    if matches!(masked.effects.as_slice(), [
      ImageEffect::SourceReference(ImageEffectSourceReference::EffectMask),
      ImageEffect::Glow { blur_kernel: super::GlowBlurKernel::WordShapeGaussian, .. }
    ])));
    let before_binding = bound.clone();
    super::bind_wordprocessing_reflection_paint(&mut bound);
    let once = bound.clone();
    super::bind_wordprocessing_reflection_paint(&mut bound);
    assert_eq!(bound, once);
    assert_eq!(bound.effects[1..], before_binding.effects[1..]);
    let ImageEffect::Container(reflection) = &bound.effects[0] else {
      unreachable!()
    };
    let ImageEffect::Container(reflected) = &reflection.effects[0] else {
      unreachable!()
    };
    // The nested shadow-of-glow Identity still sees root coverage. Only the
    // separate painted leaf changes; upright branches are byte-for-byte equal.
    assert_eq!(reflected.effects[0], reflected_source.effects[0]);
    assert_eq!(
      reflected.effects.last(),
      Some(&ImageEffect::SourceReference(
        ImageEffectSourceReference::ReflectionPaint,
      ))
    );
    assert!(super::source_requirements(&bound).reflection_paint);
  }

  #[test]
  fn drawingml_shadow_blur_radius_maps_to_direct2d_standard_deviation() {
    assert_eq!(super::direct2d_gaussian_sigma(0.0), 0.0);
    assert!((super::direct2d_gaussian_sigma(6.0) - 2.0).abs() < f32::EPSILON);
  }

  #[test]
  fn glow_mask_pixels_do_not_replace_identity_paint() {
    let mut graph = from_wordprocessing_text_effects(
      Some(WordprocessingTextGlow {
        radius_px: 0.0,
        raster_length_scale: 1.0,
        geometry_length_scale: 1.0,
        color: ResolvedEffectColor {
          color: RgbColor { r: 0, g: 0, b: 255 },
          alpha: 255,
        },
      }),
      None,
      None,
      WordprocessingTextEffectHost::Static3d,
    )
    .unwrap();
    super::bind_wordprocessing_glow_mask(&mut graph);
    let once = graph.clone();
    super::bind_wordprocessing_glow_mask(&mut graph);
    assert_eq!(graph, once);
    assert!(source_requirements(&graph).effect_mask);
    let mut paint = RgbaImage::new(3, 1);
    paint.put_pixel(0, 0, Rgba([255, 0, 0, 255]));
    let mut mask = RgbaImage::new(3, 1);
    mask.put_pixel(2, 0, Rgba([17, 91, 63, 255]));
    for present in [false, true] {
      let mut actual = paint.clone();
      apply_container_to_padded_image_with_sources(
        &mut actual,
        &graph,
        0.0,
        0.0,
        3.0,
        1.0,
        ImageEffectSourceImages {
          effect_mask: present.then_some(&mask),
          ..Default::default()
        },
      );
      assert_eq!(actual.get_pixel(0, 0).0, [255, 0, 0, 255]);
      assert_eq!(actual.get_pixel(1, 0).0, [0; 4]);
      assert_eq!(
        actual.get_pixel(2, 0).0,
        if present { [0, 0, 255, 255] } else { [0; 4] }
      );
    }
  }

  #[test]
  fn drawingml_reflection_radial_blur_maps_to_per_axis_deviation() {
    assert_eq!(super::reflection_radius_gaussian_sigma(0.0), 0.0);
    assert!(
      (super::reflection_radius_gaussian_sigma(2.0) - std::f32::consts::SQRT_2).abs()
        < f32::EPSILON
    );
  }

  #[test]
  fn direct2d_gaussian_retains_every_sample_inside_three_sigma() {
    let mut source = image::GrayImage::new(9, 9);
    for y in 0..source.height() {
      source.put_pixel(4, y, image::Luma([u8::MAX]));
    }

    let blurred = super::direct2d_gaussian_blur_alpha(&source, 2.099_737_6);
    assert_eq!(
      blurred
        .rows()
        .nth(4)
        .unwrap()
        .map(|value| value[0])
        .collect::<Vec<_>>(),
      [0, 0, 2, 52, 145, 52, 2, 0, 0,]
    );

    let subpixel = super::direct2d_gaussian_blur_alpha(&source, 0.999);
    assert_eq!(subpixel, source);
  }

  #[test]
  fn black_matte_glow_quantizes_color_before_its_independent_soft_mask() {
    let coverages = [0, 1, 2, 127, 128, 254, 255];
    let source = RgbaImage::from_fn(coverages.len() as u32, 1, |x, _| {
      Rgba([17, 33, 65, coverages[x as usize]])
    });
    let glow = ImageEffectContainer {
      kind: ImageEffectContainerKind::Sibling,
      effects: vec![ImageEffect::Glow {
        radius_px: 0.0,
        raster_length_scale: 1.0,
        bounds_radius_scale: 1.0,
        bounds_radius_offset_px: 0.0,
        spread_ratio: 0.0,
        spread_kernel: super::GlowSpreadKernel::AlphaOutset,
        spread_radius_rounding: super::GlowSpreadRadiusRounding::Outward,
        blur_kernel: super::GlowBlurKernel::WordShapeGaussian,
        color: ResolvedEffectColor {
          color: RgbColor {
            r: 255,
            g: 111,
            b: 0,
          },
          alpha: 102,
        },
      }],
    };

    let associated =
      super::black_matte_associated_single_glow_surface(&source, &glow).expect("one isolated glow");
    assert_eq!(
      associated.pixels().map(|pixel| pixel.0).collect::<Vec<_>>(),
      [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 1],
        [50, 22, 0, 51],
        [51, 22, 0, 51],
        [101, 44, 0, 102],
        [102, 44, 0, 102],
      ]
    );
    assert!(
      super::black_matte_associated_single_glow_surface(
        &source,
        &ImageEffectContainer {
          kind: ImageEffectContainerKind::Sibling,
          effects: vec![ImageEffect::Identity],
        },
      )
      .is_none()
    );
  }

  #[test]
  fn outer_shadow_runs_direct2d_shadow_before_affine_transform() {
    let source = image::RgbaImage::from_fn(9, 9, |x, y| {
      let alpha = if (3..=5).contains(&x) && (3..=5).contains(&y) {
        if x == 4 && y == 4 { u8::MAX } else { 192 }
      } else if (2..=6).contains(&x) && (2..=6).contains(&y) {
        64
      } else {
        0
      };
      image::Rgba([0, 0, 0, alpha])
    });
    let transform = ImageEffectTransform {
      scale_x: 1.0,
      scale_y: 1.0,
      skew_x: 0.0,
      skew_y: 0.0,
      shift_x_px: 0.5,
      shift_y_px: 0.0,
    };
    let actual = super::outer_shadow_image(
      &source,
      super::OuterShadowOptions {
        blur_radius_px: 2.099_737_6,
        blur_kernel: ShadowBlurKernel::Direct2dGaussian,
        distance_px: 0.5,
        direction_degrees: 0.0,
        distance_mode: ShadowDistanceMode::PostTransformOffset,
        transform: ImageEffectTransform {
          shift_x_px: 0.0,
          ..transform
        },
        alignment: (0.0, 0.0),
        color: ResolvedEffectColor {
          color: RgbColor { r: 0, g: 0, b: 0 },
          alpha: u8::MAX,
        },
        anchor_bounds: PixelBounds {
          left: 0.0,
          top: 0.0,
          right: 9.0,
          bottom: 9.0,
        },
      },
    );
    let actual_alpha =
      image::GrayImage::from_fn(9, 9, |x, y| image::Luma([actual.get_pixel(x, y).0[3]]));
    let source_alpha =
      image::GrayImage::from_fn(9, 9, |x, y| image::Luma([source.get_pixel(x, y).0[3]]));
    let blurred = super::direct2d_gaussian_blur_alpha(&source_alpha, 2.099_737_6);
    let documented_graph = super::affine_gray_image(&blurred, transform);

    let translated_source = super::affine_gray_image(&source_alpha, transform);
    let reversed_graph = super::direct2d_gaussian_blur_alpha(&translated_source, 2.099_737_6);

    assert_eq!(actual_alpha, documented_graph);
    assert_ne!(actual_alpha, reversed_graph);
  }

  #[test]
  fn word_text_shadow_runs_affine_before_balanced_blur() {
    let source = image::RgbaImage::from_fn(9, 9, |x, y| {
      let alpha = if (3..=5).contains(&x) && (3..=5).contains(&y) {
        if x == 4 && y == 4 { u8::MAX } else { 192 }
      } else if (2..=6).contains(&x) && (2..=6).contains(&y) {
        64
      } else {
        0
      };
      image::Rgba([0, 0, 0, alpha])
    });
    let transform = ImageEffectTransform {
      scale_x: 1.0,
      scale_y: 1.0,
      skew_x: 0.0,
      skew_y: 0.0,
      shift_x_px: 0.5,
      shift_y_px: 0.0,
    };
    let actual = super::outer_shadow_image(
      &source,
      super::OuterShadowOptions {
        blur_radius_px: 2.099_737_6,
        blur_kernel: ShadowBlurKernel::WordTextBalanced {
          prescale_divisor: 1,
        },
        distance_px: 0.5,
        direction_degrees: 0.0,
        distance_mode: ShadowDistanceMode::PreTransformOffset,
        transform: ImageEffectTransform {
          shift_x_px: 0.0,
          ..transform
        },
        alignment: (0.0, 0.0),
        color: ResolvedEffectColor {
          color: RgbColor { r: 0, g: 0, b: 0 },
          alpha: u8::MAX,
        },
        anchor_bounds: PixelBounds {
          left: 0.0,
          top: 0.0,
          right: 9.0,
          bottom: 9.0,
        },
      },
    );
    let actual_alpha =
      image::GrayImage::from_fn(9, 9, |x, y| image::Luma([actual.get_pixel(x, y).0[3]]));
    let source_alpha =
      image::GrayImage::from_fn(9, 9, |x, y| image::Luma([source.get_pixel(x, y).0[3]]));
    let transformed = super::affine_gray_image(&source_alpha, transform);
    let office_graph = super::word_text_balanced_blur_alpha(&transformed, 2.099_737_6, 1);

    let blurred = super::word_text_balanced_blur_alpha(&source_alpha, 2.099_737_6, 1);
    let direct2d_order = super::affine_gray_image(&blurred, transform);

    assert_eq!(actual_alpha, office_graph);
    assert_ne!(actual_alpha, direct2d_order);
  }

  #[test]
  fn glow_spread_radius_rounding_is_an_explicit_host_policy() {
    assert_eq!(
      quantized_glow_spread_radius(5.5, 0.5, GlowSpreadRadiusRounding::Outward),
      3
    );
    assert_eq!(
      quantized_glow_spread_radius(5.5, 0.5, GlowSpreadRadiusRounding::Inward),
      2
    );
    assert_eq!(
      quantized_glow_spread_radius(4.0, 0.5, GlowSpreadRadiusRounding::Inward),
      2
    );
  }

  #[test]
  fn word_group_glow_uses_the_pinned_alpha_outset_then_blur_graph() {
    let color = ResolvedEffectColor {
      color: RgbColor { r: 1, g: 2, b: 3 },
      alpha: 255,
    };
    let mut effects = ImageEffectContainer {
      kind: ImageEffectContainerKind::Sibling,
      effects: vec![ImageEffect::Glow {
        radius_px: 15.0,
        raster_length_scale: 1.0,
        bounds_radius_scale: 1.0,
        bounds_radius_offset_px: 0.0,
        spread_ratio: 1.0 / 3.0,
        spread_kernel: super::GlowSpreadKernel::Disk,
        spread_radius_rounding: super::GlowSpreadRadiusRounding::Outward,
        blur_kernel: super::GlowBlurKernel::Stack,
        color,
      }],
    };

    super::use_word_group_glow_profile(&mut effects);

    assert!(matches!(
      effects.effects.as_slice(),
      [ImageEffect::Glow {
        spread_ratio,
        spread_kernel: super::GlowSpreadKernel::AlphaOutset,
        spread_radius_rounding: super::GlowSpreadRadiusRounding::Inward,
        blur_kernel: super::GlowBlurKernel::WordGroupGaussian,
        ..
      }] if (*spread_ratio - 0.5).abs() <= f32::EPSILON
    ));
  }

  #[test]
  fn word_shape_glow_uses_the_pinned_alpha_outset_then_finite_gaussian_graph() {
    let color = ResolvedEffectColor {
      color: RgbColor { r: 1, g: 2, b: 3 },
      alpha: 255,
    };
    let mut effects = ImageEffectContainer {
      kind: ImageEffectContainerKind::Sibling,
      effects: vec![ImageEffect::Glow {
        radius_px: 15.0,
        raster_length_scale: 1.0,
        bounds_radius_scale: 1.0,
        bounds_radius_offset_px: 0.0,
        spread_ratio: 1.0 / 3.0,
        spread_kernel: super::GlowSpreadKernel::Disk,
        spread_radius_rounding: super::GlowSpreadRadiusRounding::Outward,
        blur_kernel: super::GlowBlurKernel::Stack,
        color,
      }],
    };

    super::use_word_shape_glow_profile(&mut effects);

    assert!(matches!(
      effects.effects.as_slice(),
      [ImageEffect::Glow {
        spread_ratio,
        spread_kernel: super::GlowSpreadKernel::AlphaOutset,
        spread_radius_rounding: super::GlowSpreadRadiusRounding::Inward,
        blur_kernel: super::GlowBlurKernel::WordShapeGaussian,
        ..
      }] if (*spread_ratio - 0.5).abs() <= f32::EPSILON
    ));

    // Office's 200-DPI radius controls from 0.12pt through the 5.76pt tier
    // boundary expose the integer support independently of the Gaussian
    // sigma and alpha-outset footprint.
    for (radius_px, expected) in [
      (1.0 / 3.0, 0),
      (25.0 / 9.0, 1),
      (50.0 / 9.0, 2),
      (25.0 / 3.0, 4),
      (100.0 / 9.0, 5),
      (125.0 / 9.0, 6),
      (575.0 / 36.0, 7),
      (16.0, 8),
    ] {
      assert_eq!(
        quantized_glow_spread_radius(radius_px, 0.5, super::GlowSpreadRadiusRounding::Inward,),
        expected,
      );
    }
  }

  #[test]
  fn word_shape_gaussian_matches_the_office_radius_five_edge_profile() {
    let mapped_radius = 5.0 * 200.0 / 72.0;
    assert_eq!(super::word_shape_glow_blur_device_radius(mapped_radius), 6);

    // This is the straight edge after the independently calibrated
    // alphaOutset stage. The expected values are the 100%-alpha Office PDF
    // samples from the radius-five control, including both zero outer taps.
    let spread = image::GrayImage::from_fn(32, 1, |x, _| image::Luma([u8::from(x >= 8) * u8::MAX]));
    let actual =
      super::finite_gaussian_blur_alpha_with_sigma(&spread, 6, mapped_radius / 6.0, 0, 0.0);
    assert_eq!(
      &actual.as_raw()[..15],
      &[
        0, 0, 2, 6, 16, 35, 65, 105, 150, 190, 220, 239, 249, 253, 255
      ]
    );
  }

  #[test]
  fn word_shape_gaussian_scales_each_axis_support_at_the_half_terminal_sample() {
    let pixels_per_point = 100.0 / 72.0;
    let vertical_surface_scale = 86.5 / 86.0;
    let below_transition = 10.02 * pixels_per_point;
    let above_transition = 10.03 * pixels_per_point;
    assert_eq!(
      super::word_shape_glow_blur_device_radius(below_transition * vertical_surface_scale),
      6
    );
    assert_eq!(
      super::word_shape_glow_blur_device_radius(above_transition * vertical_surface_scale),
      7
    );

    // The r=10.03 Office surface owns seven transparent support pixels before
    // the solid alphaOutset edge, so x=0 is the first exported surface sample.
    let spread = image::GrayImage::from_fn(32, 1, |x, _| image::Luma([u8::from(x >= 7) * u8::MAX]));
    let actual = super::finite_gaussian_blur_alpha_with_sigma(
      &spread,
      7,
      above_transition / 6.0 * vertical_surface_scale,
      0,
      0.0,
    );
    assert_eq!(
      &actual.as_raw()[..15],
      &[
        0, 2, 7, 17, 36, 66, 106, 149, 189, 219, 238, 248, 253, 255, 255
      ]
    );
  }

  #[test]
  fn word_group_public_blur_matches_the_office_finite_gaussian_control() {
    assert_eq!(super::word_group_public_blur_device_radius(0.0), 0);
    assert_eq!(super::word_group_public_blur_device_radius(3.999_999), 3);
    assert_eq!(super::word_group_public_blur_device_radius(4.0), 4);

    // Office training control B4/l3.  The source was independently exported
    // through clrRepl, alphaCeiling, and alphaFloor controls before the blur
    // target was inspected.  The frozen model then passed 32 dense holdouts.
    let mut source = image::GrayImage::new(11, 11);
    for (x, y, alpha) in [
      (4, 4, 75),
      (5, 4, 152),
      (6, 4, 75),
      (4, 5, 174),
      (5, 5, 222),
      (6, 5, 61),
      (4, 6, 73),
      (5, 6, 52),
    ] {
      source.put_pixel(x, y, image::Luma([alpha]));
    }
    let expected = [
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
      0, 0, 0, 1, 2, 2, 2, 1, 0, 0, 0, //
      0, 0, 2, 5, 9, 11, 8, 4, 1, 0, 0, //
      0, 1, 5, 14, 26, 30, 23, 12, 4, 1, 0, //
      0, 2, 9, 26, 47, 54, 40, 20, 6, 1, 0, //
      0, 2, 11, 31, 54, 61, 44, 21, 6, 1, 0, //
      0, 2, 8, 24, 41, 44, 31, 14, 4, 1, 0, //
      0, 1, 4, 12, 20, 21, 14, 6, 2, 0, 0, //
      0, 0, 1, 4, 6, 6, 4, 2, 0, 0, 0, //
      0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, //
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let actual = super::finite_gaussian_blur_alpha(&source, 4, 4);
    assert_eq!(actual.as_raw(), expected.as_slice());
  }

  #[test]
  fn alpha_outset_internal_blur_splits_each_axis_into_ascending_boxes() {
    for (radius, expected) in [
      (0, [0, 0, 0]),
      (1, [0, 0, 1]),
      (2, [0, 1, 1]),
      (3, [1, 1, 1]),
      (4, [1, 1, 2]),
      (5, [1, 2, 2]),
      (8, [2, 3, 3]),
      (16, [5, 5, 6]),
    ] {
      assert_eq!(super::alpha_outset_box_radii(radius), expected);
    }
    assert_eq!(super::alpha_outset_device_radius(8.999_999), 8);
    assert_eq!(super::alpha_outset_device_radius(9.0), 9);
    assert_eq!(super::alpha_outset_device_radius(-9.999), 9);
  }

  #[test]
  fn alpha_outset_rounds_each_box_pass_and_handles_axes_independently() {
    let line = image::GrayImage::from_raw(3, 1, vec![254, 0, 0]).unwrap();
    assert_eq!(
      super::rounded_box_blur_axis(&line, 1, true).into_raw(),
      vec![85, 85, 0]
    );

    let single = image::GrayImage::from_fn(9, 9, |x, y| {
      image::Luma([u8::from(x == 4 && y == 4) * u8::MAX])
    });
    let anisotropic = super::alpha_outset_mask(&single, 1.999, 2.001, true);
    assert_eq!(
      anisotropic.pixels().filter(|pixel| pixel.0[0] > 0).count(),
      15
    );

    let rectangle = image::GrayImage::from_fn(9, 9, |x, y| {
      image::Luma([u8::from((2..=6).contains(&x) && (2..=6).contains(&y)) * u8::MAX])
    });
    let positive = super::alpha_outset_mask(&rectangle, 1.0, 1.0, true);
    let negative = super::alpha_outset_mask(&rectangle, -1.0, -1.0, false);
    assert_eq!(positive.pixels().filter(|pixel| pixel.0[0] > 0).count(), 49);
    assert_eq!(negative.pixels().filter(|pixel| pixel.0[0] > 0).count(), 9);
  }

  #[test]
  fn glow_spread_uses_a_separable_square_morphology_kernel() {
    let source = image::GrayImage::from_fn(5, 5, |x, y| {
      image::Luma([u8::from(x == 2 && y == 2) * u8::MAX])
    });
    let dilated = super::dilate_nontransparent_alpha(&source, 1);

    for y in 1..=3 {
      for x in 1..=3 {
        assert_eq!(dilated.get_pixel(x, y).0[0], u8::MAX);
      }
    }
    assert_eq!(dilated.get_pixel(0, 0).0[0], 0);
    assert_eq!(dilated.get_pixel(4, 4).0[0], 0);
  }

  #[test]
  fn disk_morphology_is_radial_between_diamond_and_square_topologies() {
    let source = image::GrayImage::from_fn(9, 9, |x, y| {
      image::Luma([u8::from(x == 4 && y == 4) * u8::MAX])
    });
    let dilated = super::dilate_nontransparent_alpha_disk(&source, 3);

    assert_eq!(dilated.get_pixel(7, 4).0[0], u8::MAX);
    assert_eq!(dilated.get_pixel(6, 6).0[0], u8::MAX);
    assert_eq!(dilated.get_pixel(7, 5).0[0], 0);
    assert_eq!(dilated.get_pixel(7, 7).0[0], 0);
  }

  #[test]
  fn direct2d_shadow_preserves_axis_aligned_source_coverage() {
    let source = image::RgbaImage::from_fn(7, 7, |x, y| {
      let alpha = if (2..=4).contains(&x) && (2..=4).contains(&y) {
        if x == 3 && y == 3 { u8::MAX } else { 192 }
      } else if (1..=5).contains(&x) && (1..=5).contains(&y) {
        64
      } else {
        0
      };
      image::Rgba([255, 255, 255, alpha])
    });
    let shadow = super::outer_shadow_image(
      &source,
      super::OuterShadowOptions {
        blur_radius_px: 0.999,
        blur_kernel: ShadowBlurKernel::Direct2dGaussian,
        distance_px: 0.0,
        direction_degrees: 0.0,
        distance_mode: ShadowDistanceMode::PostTransformOffset,
        transform: ImageEffectTransform {
          scale_x: 1.0,
          scale_y: 1.0,
          skew_x: 0.0,
          skew_y: 0.0,
          shift_x_px: 0.0,
          shift_y_px: 0.0,
        },
        alignment: (0.5, 0.5),
        color: ResolvedEffectColor {
          color: RgbColor { r: 0, g: 0, b: 0 },
          alpha: u8::MAX,
        },
        anchor_bounds: PixelBounds {
          left: 0.0,
          top: 0.0,
          right: 7.0,
          bottom: 7.0,
        },
      },
    );
    assert_eq!(shadow.get_pixel(1, 3).0[3], 64);
    assert_eq!(shadow.get_pixel(2, 2).0[3], 192);
    assert_eq!(shadow.get_pixel(3, 3).0[3], u8::MAX);
    assert_eq!(shadow.get_pixel(5, 3).0[3], 64);
  }

  #[test]
  fn stack_blur_alpha_uses_the_finite_triangular_kernel() {
    let mut alpha = vec![0; 11 * 11];
    alpha[5 * 11 + 5] = 255;
    super::stack_blur_alpha(&mut alpha, 11, 11, 2);

    // A radius-two Stack Blur is the separable [1, 2, 3, 2, 1] / 9
    // triangle. Unlike a Gaussian approximation, it is exactly zero beyond
    // two pixels from this impulse.
    assert_eq!(alpha[5 * 11 + 5], 28);
    assert_eq!(alpha[5 * 11 + 6], 18);
    assert_eq!(alpha[5 * 11 + 7], 9);
    assert_eq!(alpha[5 * 11 + 8], 0);
  }

  #[test]
  fn word_text_glow_bounds_follow_the_host_owned_terminal_samples() {
    let glow = WordprocessingTextGlow {
      radius_px: 12.0,
      raster_length_scale: 0.25,
      geometry_length_scale: 0.25,
      color: ResolvedEffectColor {
        color: RgbColor { r: 1, g: 2, b: 3 },
        alpha: 255,
      },
    };
    let flat_effects = from_wordprocessing_text_effects(
      Some(glow),
      None,
      None,
      WordprocessingTextEffectHost::FlatText,
    )
    .expect("flat word text glow");
    let static_3d_effects = from_wordprocessing_text_effects(
      Some(glow),
      None,
      None,
      WordprocessingTextEffectHost::Static3d,
    )
    .expect("static-3-D word text glow");

    let flat_bounds = container_output_bounds(&flat_effects, 30.0, 10.0).expect("flat glow bounds");
    // The runtime kernel is 12 * 0.25 = 3 CSS pixels. A flat text surface
    // retains another 0.48 CSS pixels (one sample at 200 DPI), so the total
    // 3.48 CSS pixels map to 2.61 points.
    assert!((flat_bounds.left_pt + 2.61).abs() < 0.001);
    assert!((flat_bounds.top_pt + 2.61).abs() < 0.001);
    assert!((flat_bounds.right_pt - 32.61).abs() < 0.001);
    assert!((flat_bounds.bottom_pt - 12.61).abs() < 0.001);

    let static_3d_bounds =
      container_output_bounds(&static_3d_effects, 30.0, 10.0).expect("static-3-D glow bounds");
    // The 600-DPI intercept normalizes 12 CSS pixels to
    // (12 - 0.16) * 0.25 + 0.16 = 3.12 CSS pixels. The independent
    // two-sample allocation guard adds 0.96 CSS pixels, not kernel support.
    let allocation_radius_pt = ((12.0 - 0.16) * 0.25 + 0.16 + 0.96) * 72.0 / 96.0;
    assert!((static_3d_bounds.left_pt + allocation_radius_pt).abs() < 0.001);
    assert!((static_3d_bounds.top_pt + allocation_radius_pt).abs() < 0.001);
    assert!((static_3d_bounds.right_pt - 30.0 - allocation_radius_pt).abs() < 0.001);
    assert!((static_3d_bounds.bottom_pt - 10.0 - allocation_radius_pt).abs() < 0.001);

    let source = EffectOutputBounds {
      left_pt: 0.0,
      top_pt: 0.0,
      right_pt: 30.0,
      bottom_pt: 10.0,
    };
    let scene = super::container_scene_bounds_with_sources(
      &static_3d_effects,
      source,
      source,
      source,
      source,
      super::ImageEffectSourceBounds::default(),
    )
    .unwrap();
    let continuous_radius_pt = ((12.0 - 0.16) * 0.25 + 0.16) * 72.0 / 96.0;
    assert!((scene.top_pt + continuous_radius_pt).abs() < 0.00001);
    assert!((scene.bottom_pt - 10.0 - continuous_radius_pt).abs() < 0.00001);
    // A scene query cannot mutate the later bitmap allocation contract.
    assert_eq!(
      container_output_bounds(&static_3d_effects, 30.0, 10.0),
      Some(static_3d_bounds)
    );
  }

  #[test]
  fn text_shadow_affine_alignment_uses_the_character_cell_not_tight_ink() {
    let effects = ImageEffectContainer {
      kind: ImageEffectContainerKind::Tree,
      effects: vec![ImageEffect::OuterShadow {
        blur_radius_px: 0.0,
        distance_px: 0.0,
        raster_length_scale: 1.0,
        distance_length_scale: 1.0,
        bounds_radius_scale: 1.0,
        bounds_radius_offset_px: 0.0,
        blur_kernel: ShadowBlurKernel::Direct2dGaussian,
        direction_degrees: 0.0,
        distance_mode: ShadowDistanceMode::PreTransformOffset,
        transform: ImageEffectTransform {
          scale_x: 1.0,
          scale_y: -0.3,
          skew_x: 0.0,
          skew_y: 0.0,
          shift_x_px: 0.0,
          shift_y_px: 0.0,
        },
        alignment: (0.0, 1.0),
        rotate_with_shape: false,
        color: ResolvedEffectColor {
          color: RgbColor { r: 1, g: 2, b: 3 },
          alpha: 255,
        },
      }],
    };
    let ink = EffectOutputBounds {
      left_pt: 10.0,
      top_pt: 20.0,
      right_pt: 30.0,
      bottom_pt: 40.0,
    };
    let character_cell = EffectOutputBounds {
      left_pt: 0.0,
      top_pt: 0.0,
      right_pt: 100.0,
      bottom_pt: 80.0,
    };

    let bounds =
      container_output_bounds_with_anchor(&effects, ink, character_cell).expect("shadow bounds");

    // Bottom alignment fixes the logical line-box y=80. Tight-ink alignment
    // would instead produce y=40..46 and is the counterexample this API must
    // reject.
    assert!((bounds.left_pt - 10.0).abs() < 0.001);
    assert!((bounds.top_pt - 92.0).abs() < 0.001);
    assert!((bounds.right_pt - 30.0).abs() < 0.001);
    assert!((bounds.bottom_pt - 98.0).abs() < 0.001);
  }

  #[test]
  fn reflection_canvas_near_support_is_clipped_by_the_source_gap() {
    let canvas = EffectOutputBounds {
      left_pt: -10.0,
      top_pt: -10.0,
      right_pt: 110.0,
      bottom_pt: 110.0,
    };
    let reflection = EffectOutputBounds {
      left_pt: 0.0,
      top_pt: 0.0,
      right_pt: 100.0,
      bottom_pt: 100.0,
    };

    let touching =
      wordprocessing_reflection_canvas_bounds(canvas, reflection, 2.0, 0.0, 90.0, 90.0);
    assert!(touching.top_pt.abs() < 0.001);
    assert!((touching.bottom_pt - 110.0).abs() < 0.001);

    let separated =
      wordprocessing_reflection_canvas_bounds(canvas, reflection, 2.0, 20.0, 90.0, 90.0);
    // Two CSS pixels are 1.5pt standard deviation, so the soft border may
    // consume 4.5pt of the 15pt source/reflection gap.
    assert!((separated.top_pt + 4.5).abs() < 0.001);
    assert!((separated.bottom_pt - 110.0).abs() < 0.001);
  }

  #[test]
  fn reflection_uses_schema_alpha_ramp_defaults() {
    let ImageEffect::Reflection(effect) = reflection(&a::Reflection::default()) else {
      panic!("reflection effect");
    };

    assert_eq!(effect.start_opacity, 1.0);
    assert_eq!(effect.start_position, 0.0);
    assert_eq!(effect.end_opacity, 0.0);
    assert_eq!(effect.end_position, 1.0);
  }

  impl ImageEffectColorResolver for NoColorResolver {
    fn alpha_inverse(&self, _: &a::AlphaInverseChoice) -> Option<ResolvedEffectColor> {
      None
    }

    fn color_from(&self, _: &a::ColorFromChoice) -> Option<ResolvedEffectColor> {
      None
    }

    fn color_to(&self, _: &a::ColorToChoice) -> Option<ResolvedEffectColor> {
      None
    }

    fn color_replacement(&self, _: &a::ColorReplacementChoice) -> Option<ResolvedEffectColor> {
      None
    }

    fn duotone(&self, _: &a::DuotoneChoice) -> Option<ResolvedEffectColor> {
      None
    }

    fn solid_fill(&self, _: &a::SolidFillChoice) -> Option<ResolvedEffectColor> {
      None
    }

    fn gradient_stop(&self, _: &a::GradientStopChoice) -> Option<ResolvedEffectColor> {
      None
    }

    fn foreground(&self, _: &a::ForegroundColorChoice) -> Option<ResolvedEffectColor> {
      None
    }

    fn background(&self, _: &a::BackgroundColorChoice) -> Option<ResolvedEffectColor> {
      None
    }

    fn glow(&self, _: &a::GlowChoice) -> Option<ResolvedEffectColor> {
      None
    }

    fn inner_shadow(&self, _: &a::InnerShadowChoice) -> Option<ResolvedEffectColor> {
      None
    }

    fn outer_shadow(&self, _: &a::OuterShadowChoice) -> Option<ResolvedEffectColor> {
      None
    }

    fn preset_shadow(&self, _: &a::PresetShadowChoice) -> Option<ResolvedEffectColor> {
      Some(ResolvedEffectColor {
        color: RgbColor {
          r: 10,
          g: 20,
          b: 30,
        },
        alpha: 128,
      })
    }
  }

  #[test]
  fn empty_effect_list_has_no_runtime_pipeline() {
    let effects = from_effect_list(&a::EffectList::default(), None, &NoColorResolver);

    assert!(effects.effects.is_empty());
    assert!(container_output_bounds(&effects, 100.0, 80.0).is_none());
  }

  #[test]
  fn alpha_modulate_uses_nested_effect_alpha() {
    let mut image = RgbaImage::from_pixel(1, 1, Rgba([10, 20, 30, 128]));
    apply_to_image(
      &mut image,
      &[ImageEffect::AlphaModulate(ImageEffectContainer {
        kind: ImageEffectContainerKind::Tree,
        effects: vec![ImageEffect::AlphaModulateFixed(0.5)],
      })],
    );

    // Nested alpha is round(128 * 0.5) = 64; alphaMod then multiplies the
    // source alpha by that effect alpha.
    assert_eq!(image.get_pixel(0, 0).0, [10, 20, 30, 32]);
  }

  #[test]
  fn sibling_container_applies_each_branch_to_the_parent() {
    let mut image = RgbaImage::from_pixel(1, 1, Rgba([10, 20, 30, 128]));
    apply_to_image(
      &mut image,
      &[ImageEffect::AlphaModulate(ImageEffectContainer {
        kind: ImageEffectContainerKind::Sibling,
        effects: vec![
          ImageEffect::AlphaModulateFixed(0.5),
          ImageEffect::AlphaModulateFixed(0.5),
        ],
      })],
    );

    // Both sibling branches see alpha 128 and produce alpha 64. Source-over
    // composition yields alpha 112, which modulates the parent to 56.
    assert_eq!(image.get_pixel(0, 0).0[3], 56);
  }

  #[test]
  fn preset_shadow_uses_the_ecma_outer_shadow_transform() {
    let list = a::EffectList {
      preset_shadow: Some(Box::new(a::PresetShadow {
        preset: a::PresetShadowValues::TopLeftLargeDropShadow,
        preset_shadow_choice: Some(a::PresetShadowChoice::RgbColorModelPercentage(
          a::RgbColorModelPercentage::default(),
        )),
        ..a::PresetShadow::default()
      })),
      ..a::EffectList::default()
    };
    let effects = from_effect_list(&list, None, &NoColorResolver);
    let bounds = container_output_bounds(&effects, 100.0, 80.0).unwrap();

    assert!((bounds.left_pt + 25.0).abs() < 0.01);
    assert!((bounds.top_pt + 20.0).abs() < 0.01);
    assert!((bounds.right_pt - 100.0).abs() < 0.01);
    assert!((bounds.bottom_pt - 80.0).abs() < 0.01);
  }

  #[test]
  fn alpha_outset_expands_and_inset_erodes_the_silhouette() {
    let mut expanded = RgbaImage::from_pixel(5, 5, Rgba([1, 2, 3, 0]));
    expanded.get_pixel_mut(2, 2).0[3] = u8::MAX;
    apply_to_image(&mut expanded, &[ImageEffect::AlphaOutset(1.0)]);
    assert!(expanded.get_pixel(1, 2).0[3] > 0);

    let mut eroded = RgbaImage::from_pixel(5, 5, Rgba([1, 2, 3, u8::MAX]));
    eroded.get_pixel_mut(0, 0).0[3] = 0;
    apply_to_image(&mut eroded, &[ImageEffect::AlphaOutset(-1.0)]);
    assert_eq!(eroded.get_pixel(0, 1).0[3], 0);
    assert_eq!(eroded.get_pixel(2, 2).0[3], u8::MAX);
  }

  #[test]
  fn fill_overlay_composites_the_resolved_fill_over_the_bitmap() {
    let mut image = RgbaImage::from_pixel(1, 1, Rgba([100, 150, 200, u8::MAX]));
    apply_to_image(
      &mut image,
      &[ImageEffect::FillOverlay {
        fill: ImageEffectFill::Solid(ResolvedEffectColor {
          color: RgbColor {
            r: 200,
            g: 100,
            b: 50,
          },
          alpha: 128,
        }),
        blend_mode: ImageEffectBlendMode::Over,
      }],
    );

    assert_eq!(image.get_pixel(0, 0).0, [150, 125, 125, u8::MAX]);
  }

  #[test]
  fn path_gradient_uses_fill_to_focus_and_drawingml_focus_to_outer_stop_direction() {
    let fill = ImageEffectFill::Gradient {
      stops: vec![
        (
          0.0,
          ResolvedEffectColor {
            color: RgbColor { r: 255, g: 0, b: 0 },
            alpha: 255,
          },
        ),
        (
          1.0,
          ResolvedEffectColor {
            color: RgbColor { r: 0, g: 0, b: 255 },
            alpha: 255,
          },
        ),
      ],
      kind: ImageEffectGradientKind::Rectangle(ImageEffectRelativeRect {
        left: 0.5,
        top: 0.5,
        right: 0.5,
        bottom: 0.5,
      }),
      tile: ImageEffectRelativeRect::default(),
      flip: a::TileFlipValues::None,
    };

    let edge = sample_fill(&fill, 0, 2, 5, 5).color;
    let focus = sample_fill(&fill, 2, 2, 5, 5).color;
    assert!(edge.b > edge.r);
    assert!(focus.r > 250);
  }

  #[test]
  fn tile_rectangle_repeats_and_flips_alternate_tiles() {
    let fill = ImageEffectFill::Gradient {
      stops: vec![
        (
          0.0,
          ResolvedEffectColor {
            color: RgbColor { r: 0, g: 0, b: 0 },
            alpha: 255,
          },
        ),
        (
          1.0,
          ResolvedEffectColor {
            color: RgbColor {
              r: 255,
              g: 255,
              b: 255,
            },
            alpha: 255,
          },
        ),
      ],
      kind: ImageEffectGradientKind::Linear(0.0),
      tile: ImageEffectRelativeRect {
        left: 0.5,
        top: 0.0,
        right: 0.0,
        bottom: 0.0,
      },
      flip: a::TileFlipValues::Horizontal,
    };

    let left = sample_fill(&fill, 0, 0, 8, 1).color.r;
    let right = sample_fill(&fill, 7, 0, 8, 1).color.r;
    assert!(left > 180);
    assert!(right > 180);
  }

  #[test]
  fn relative_and_affine_offsets_leave_transparent_uncovered_pixels() {
    let mut relative = RgbaImage::from_pixel(4, 1, Rgba([1, 2, 3, 255]));
    apply_to_image(
      &mut relative,
      &[ImageEffect::RelativeOffset {
        offset_x: 0.5,
        offset_y: 0.0,
      }],
    );
    assert_eq!(relative.get_pixel(0, 0).0[3], 0);
    assert_eq!(relative.get_pixel(3, 0).0[3], 255);

    let mut transformed = RgbaImage::from_pixel(3, 1, Rgba([4, 5, 6, 255]));
    apply_to_image(
      &mut transformed,
      &[ImageEffect::Transform(ImageEffectTransform {
        scale_x: 1.0,
        scale_y: 1.0,
        skew_x: 0.0,
        skew_y: 0.0,
        shift_x_px: 1.0,
        shift_y_px: 0.0,
      })],
    );
    assert_eq!(transformed.get_pixel(0, 0).0[3], 0);
    assert_eq!(transformed.get_pixel(2, 0).0[3], 255);
  }

  #[test]
  fn colored_container_effects_produce_their_own_effect_branch() {
    let color = ResolvedEffectColor {
      color: RgbColor {
        r: 20,
        g: 40,
        b: 60,
      },
      alpha: 255,
    };
    let mut outer = RgbaImage::from_pixel(5, 1, Rgba([100, 110, 120, 0]));
    outer.get_pixel_mut(1, 0).0[3] = 255;
    apply_to_image(
      &mut outer,
      &[ImageEffect::OuterShadow {
        blur_radius_px: 0.0,
        distance_px: 2.0,
        raster_length_scale: 1.0,
        distance_length_scale: 1.0,
        bounds_radius_scale: 1.0,
        bounds_radius_offset_px: 0.0,
        blur_kernel: ShadowBlurKernel::Direct2dGaussian,
        direction_degrees: 0.0,
        distance_mode: ShadowDistanceMode::PostTransformOffset,
        transform: ImageEffectTransform {
          scale_x: 1.0,
          scale_y: 1.0,
          skew_x: 0.0,
          skew_y: 0.0,
          shift_x_px: 0.0,
          shift_y_px: 0.0,
        },
        alignment: (0.5, 1.0),
        rotate_with_shape: false,
        color,
      }],
    );
    assert_eq!(outer.get_pixel(3, 0).0, [20, 40, 60, 255]);
    assert_eq!(outer.get_pixel(1, 0).0[3], 0);

    let mut reflected = RgbaImage::from_pixel(1, 3, Rgba([10, 20, 30, 0]));
    reflected.get_pixel_mut(0, 0).0[3] = 255;
    apply_to_image(
      &mut reflected,
      &[ImageEffect::Reflection(ImageReflectionEffect {
        blur_radius_px: 0.0,
        raster_length_scale: 1.0,
        bounds_radius_scale: 1.0,
        bounds_radius_offset_px: 0.0,
        start_opacity: 1.0,
        start_position: 0.0,
        end_opacity: 1.0,
        end_position: 1.0,
        fade_direction_degrees: 90.0,
        distance_px: 0.0,
        distance_length_scale: 1.0,
        distance_mode: ReflectionDistanceMode::PostTransformOffset,
        reference: ReflectionReference::EffectInput,
        direction_degrees: 0.0,
        transform: ImageEffectTransform {
          scale_x: 1.0,
          scale_y: -1.0,
          skew_x: 0.0,
          skew_y: 0.0,
          shift_x_px: 0.0,
          shift_y_px: 0.0,
        },
        alignment: (0.5, 0.5),
        rotate_with_shape: false,
      })],
    );
    assert_eq!(reflected.get_pixel(0, 2).0, [10, 20, 30, 255]);
  }

  #[test]
  fn reflection_soft_blur_reserves_one_kernel_radius_on_each_side() {
    let effects = ImageEffectContainer {
      kind: ImageEffectContainerKind::Tree,
      effects: vec![ImageEffect::Reflection(ImageReflectionEffect {
        blur_radius_px: 12.0,
        raster_length_scale: 1.0,
        bounds_radius_scale: 1.0,
        bounds_radius_offset_px: 0.0,
        start_opacity: 1.0,
        start_position: 0.0,
        end_opacity: 0.0,
        end_position: 1.0,
        fade_direction_degrees: 90.0,
        distance_px: 3.0,
        distance_length_scale: 1.0,
        distance_mode: ReflectionDistanceMode::PostTransformOffset,
        reference: ReflectionReference::EffectInput,
        direction_degrees: 0.0,
        transform: ImageEffectTransform {
          scale_x: 1.0,
          scale_y: 1.0,
          skew_x: 0.0,
          skew_y: 0.0,
          shift_x_px: 0.0,
          shift_y_px: 0.0,
        },
        alignment: (0.5, 0.5),
        rotate_with_shape: false,
      })],
    };

    let bounds = container_output_bounds(&effects, 100.0, 80.0).expect("reflection bounds");

    assert!((bounds.left_pt + 6.75).abs() < 0.001);
    assert!((bounds.top_pt + 9.0).abs() < 0.001);
    assert!((bounds.right_pt - 111.25).abs() < 0.001);
    assert!((bounds.bottom_pt - 89.0).abs() < 0.001);
  }

  #[test]
  fn reflection_output_bounds_do_not_allocate_the_independent_alpha_ramp() {
    let effects = ImageEffectContainer {
      kind: ImageEffectContainerKind::Tree,
      effects: vec![ImageEffect::Reflection(ImageReflectionEffect {
        blur_radius_px: 0.0,
        raster_length_scale: 1.0,
        bounds_radius_scale: 1.0,
        bounds_radius_offset_px: 0.0,
        start_opacity: 0.5,
        start_position: 0.0,
        end_opacity: 0.0,
        end_position: 0.5,
        fade_direction_degrees: 90.0,
        distance_px: 0.0,
        distance_length_scale: 1.0,
        distance_mode: ReflectionDistanceMode::PostTransformOffset,
        reference: ReflectionReference::EffectInput,
        direction_degrees: 90.0,
        transform: ImageEffectTransform {
          scale_x: 1.0,
          scale_y: -1.0,
          skew_x: 0.0,
          skew_y: 0.0,
          shift_x_px: 0.0,
          shift_y_px: 0.0,
        },
        alignment: (0.0, 1.0),
        rotate_with_shape: false,
      })],
    };
    let paint = EffectOutputBounds {
      left_pt: 0.0,
      top_pt: 0.0,
      right_pt: 30.0,
      bottom_pt: 10.0,
    };
    let character_cell = EffectOutputBounds {
      left_pt: 0.0,
      top_pt: -20.0,
      right_pt: 30.0,
      bottom_pt: 10.0,
    };
    let text_metric_ramp = EffectOutputBounds {
      left_pt: 0.0,
      top_pt: -10.0,
      right_pt: 30.0,
      bottom_pt: 10.0,
    };

    let with_text_metric_ramp = container_output_bounds_with_anchors_and_ramp(
      &effects,
      paint,
      character_cell,
      character_cell,
      text_metric_ramp,
    )
    .unwrap();
    let with_character_cell_ramp =
      container_output_bounds_with_anchors(&effects, paint, character_cell, character_cell)
        .unwrap();

    assert!((with_text_metric_ramp.top_pt - 10.0).abs() < 0.001);
    assert!((with_text_metric_ramp.bottom_pt - 20.0).abs() < 0.001);
    assert_eq!(with_text_metric_ramp, with_character_cell_ramp);
  }

  #[test]
  fn word_reflection_distance_moves_the_alignment_pivot_and_extends_the_ramp() {
    let bounds = PixelBounds {
      left: 0.0,
      top: 0.0,
      right: 10.0,
      bottom: 10.0,
    };
    let source = EffectGeometry {
      paint: bounds,
      shadow_anchor: bounds,
      anchor: bounds,
      ramp: bounds,
    };
    let effect = |distance_mode| {
      ImageEffect::Reflection(ImageReflectionEffect {
        blur_radius_px: 0.0,
        raster_length_scale: 1.0,
        bounds_radius_scale: 1.0,
        bounds_radius_offset_px: 0.0,
        start_opacity: 1.0,
        start_position: 0.0,
        end_opacity: 1.0,
        end_position: 1.0,
        fade_direction_degrees: 90.0,
        distance_px: 2.0,
        distance_length_scale: 1.0,
        distance_mode,
        reference: ReflectionReference::RootText,
        direction_degrees: 90.0,
        transform: ImageEffectTransform {
          scale_x: 1.0,
          scale_y: -1.0,
          skew_x: 0.0,
          skew_y: 0.0,
          shift_x_px: 0.0,
          shift_y_px: 0.0,
        },
        alignment: (0.5, 1.0),
        rotate_with_shape: false,
      })
    };

    let generic = effect_output_geometry(
      &effect(ReflectionDistanceMode::PostTransformOffset),
      source,
      source,
      ImageEffectSourceBounds::default(),
    )
    .unwrap();
    let word = effect_output_geometry(
      &effect(ReflectionDistanceMode::AlignmentPivot),
      source,
      source,
      ImageEffectSourceBounds::default(),
    )
    .unwrap();

    assert!((generic.paint.top - 12.0).abs() < 0.000_1);
    assert!((generic.paint.bottom - 22.0).abs() < 0.000_1);
    assert!((generic.ramp.top - 12.0).abs() < 0.000_1);
    assert!((generic.ramp.bottom - 22.0).abs() < 0.000_1);
    assert!((word.paint.top - 14.0).abs() < 0.000_1);
    assert!((word.paint.bottom - 24.0).abs() < 0.000_1);
    assert!((word.ramp.top - 12.0).abs() < 0.000_1);
    assert!((word.ramp.bottom - 24.0).abs() < 0.000_1);

    let expanded = PixelBounds {
      left: -2.0,
      top: -4.0,
      right: 12.0,
      bottom: 13.0,
    };
    let completed_shadow_source = EffectGeometry {
      paint: expanded,
      shadow_anchor: expanded,
      anchor: expanded,
      ramp: expanded,
    };
    let word_with_shadow = effect_output_geometry(
      &effect(ReflectionDistanceMode::AlignmentPivot),
      completed_shadow_source,
      source,
      ImageEffectSourceBounds::default(),
    )
    .unwrap();
    // The completed shadow remains reflected paint, while the affine anchor
    // and opacity ramp stay attached to the root text coordinate domains.
    assert!((word_with_shadow.paint.top - 11.0).abs() < 0.000_1);
    assert!((word_with_shadow.paint.bottom - 28.0).abs() < 0.000_1);
    assert!((word_with_shadow.anchor.top - 14.0).abs() < 0.000_1);
    assert!((word_with_shadow.anchor.bottom - 24.0).abs() < 0.000_1);
    assert!((word_with_shadow.ramp.top - 12.0).abs() < 0.000_1);
    assert!((word_with_shadow.ramp.bottom - 24.0).abs() < 0.000_1);

    // Coordinate ownership and distance order are independent. The completed
    // shadow is still reflected when the text root uses post-transform distance;
    // changing distance mode must not silently substitute the shadow's anchor.
    let post_transform_text = effect_output_geometry(
      &effect(ReflectionDistanceMode::PostTransformOffset),
      completed_shadow_source,
      source,
      ImageEffectSourceBounds::default(),
    )
    .unwrap();
    assert!((post_transform_text.paint.top - 9.0).abs() < 0.000_1);
    assert!((post_transform_text.paint.bottom - 26.0).abs() < 0.000_1);
    assert!((post_transform_text.anchor.top - 12.0).abs() < 0.000_1);
    assert!((post_transform_text.anchor.bottom - 22.0).abs() < 0.000_1);
    assert!((post_transform_text.ramp.top - 12.0).abs() < 0.000_1);
    assert!((post_transform_text.ramp.bottom - 22.0).abs() < 0.000_1);

    let ImageEffect::Reflection(mut input_reflection) =
      effect(ReflectionDistanceMode::PostTransformOffset)
    else {
      unreachable!();
    };
    input_reflection.reference = ReflectionReference::EffectInput;
    let post_transform_input = effect_output_geometry(
      &ImageEffect::Reflection(input_reflection),
      completed_shadow_source,
      source,
      ImageEffectSourceBounds::default(),
    )
    .unwrap();
    assert!((post_transform_input.paint.top - 15.0).abs() < 0.000_1);
    assert!((post_transform_input.paint.bottom - 32.0).abs() < 0.000_1);
    assert!((post_transform_input.ramp.top - 15.0).abs() < 0.000_1);
    assert!((post_transform_input.ramp.bottom - 32.0).abs() < 0.000_1);

    let mut image = RgbaImage::from_pixel(1, 30, Rgba([0; 4]));
    image.get_pixel_mut(0, 2).0 = [20, 30, 40, 255];
    image.get_pixel_mut(0, 3).0 = [20, 30, 40, 255];
    let alpha_centroid = |image: &RgbaImage| {
      let (weighted, alpha) =
        image
          .enumerate_pixels()
          .fold((0.0_f32, 0.0_f32), |(weighted, alpha), (_, y, pixel)| {
            let sample = f32::from(pixel.0[3]);
            (weighted + (y as f32 + 0.5) * sample, alpha + sample)
          });
      weighted / alpha
    };
    let generic_image = reflection_image(
      &image,
      match effect(ReflectionDistanceMode::PostTransformOffset) {
        ImageEffect::Reflection(effect) => effect,
        _ => unreachable!(),
      },
      bounds,
      bounds,
      bounds,
    );
    let word_image = reflection_image(
      &image,
      match effect(ReflectionDistanceMode::AlignmentPivot) {
        ImageEffect::Reflection(effect) => effect,
        _ => unreachable!(),
      },
      bounds,
      bounds,
      bounds,
    );
    assert!((alpha_centroid(&word_image) - alpha_centroid(&generic_image) - 2.0).abs() < 0.001);
  }

  #[test]
  fn reflection_blur_converts_the_authored_radius_to_gaussian_sigma() {
    let mut source = RgbaImage::from_pixel(11, 11, Rgba([0; 4]));
    source.get_pixel_mut(5, 5).0 = [90, 120, 150, 255];
    let bounds = PixelBounds {
      left: 0.0,
      top: 0.0,
      right: 11.0,
      bottom: 11.0,
    };
    let reflected = reflection_image(
      &source,
      ImageReflectionEffect {
        blur_radius_px: 2.0,
        raster_length_scale: 1.0,
        bounds_radius_scale: 1.0,
        bounds_radius_offset_px: 0.0,
        start_opacity: 1.0,
        start_position: 0.0,
        end_opacity: 1.0,
        end_position: 1.0,
        fade_direction_degrees: 90.0,
        distance_px: 0.0,
        distance_length_scale: 1.0,
        distance_mode: ReflectionDistanceMode::PostTransformOffset,
        reference: ReflectionReference::EffectInput,
        direction_degrees: 0.0,
        transform: ImageEffectTransform {
          scale_x: 1.0,
          scale_y: 1.0,
          skew_x: 0.0,
          skew_y: 0.0,
          shift_x_px: 0.0,
          shift_y_px: 0.0,
        },
        alignment: (0.5, 0.5),
        rotate_with_shape: false,
      },
      bounds,
      bounds,
      bounds,
    );

    assert!(reflected.get_pixel(5, 5).0[3] > reflected.get_pixel(6, 5).0[3]);
    assert!(reflected.get_pixel(6, 5).0[3] > 0);
    assert_eq!(reflected.get_pixel(7, 5).0[3], 0);
  }

  #[test]
  fn reflection_blurs_the_completed_alpha_ramp() {
    let source = RgbaImage::from_pixel(1, 9, Rgba([90, 120, 150, 255]));
    let bounds = PixelBounds {
      left: 0.0,
      top: 0.0,
      right: 1.0,
      bottom: 9.0,
    };
    let reflected = reflection_image(
      &source,
      ImageReflectionEffect {
        blur_radius_px: 2.0,
        raster_length_scale: 1.0,
        bounds_radius_scale: 1.0,
        bounds_radius_offset_px: 0.0,
        start_opacity: 1.0,
        start_position: 0.0,
        end_opacity: 0.0,
        end_position: 0.5,
        fade_direction_degrees: 90.0,
        distance_px: 0.0,
        distance_length_scale: 1.0,
        distance_mode: ReflectionDistanceMode::PostTransformOffset,
        reference: ReflectionReference::EffectInput,
        direction_degrees: 0.0,
        transform: ImageEffectTransform {
          scale_x: 1.0,
          scale_y: 1.0,
          skew_x: 0.0,
          skew_y: 0.0,
          shift_x_px: 0.0,
          shift_y_px: 0.0,
        },
        alignment: (0.5, 0.5),
        rotate_with_shape: false,
      },
      bounds,
      bounds,
      bounds,
    );

    // The authored ramp reaches zero at y=4.5. Blurring the completed
    // reflection surface carries a Gaussian soft tail into that zero sample;
    // the correctly bounded `r / 3` kernel does not reach another pixel.
    assert!(reflected.get_pixel(0, 4).0[3] > 0);
    assert_eq!(reflected.get_pixel(0, 5).0[3], 0);
  }

  #[test]
  fn vertically_flipped_reflection_fades_away_from_the_source_edge() {
    let mut reflected = RgbaImage::from_pixel(1, 4, Rgba([10, 20, 30, 255]));
    apply_to_image(
      &mut reflected,
      &[ImageEffect::Reflection(ImageReflectionEffect {
        blur_radius_px: 0.0,
        raster_length_scale: 1.0,
        bounds_radius_scale: 1.0,
        bounds_radius_offset_px: 0.0,
        start_opacity: 1.0,
        start_position: 0.0,
        end_opacity: 0.0,
        end_position: 1.0,
        fade_direction_degrees: 90.0,
        distance_px: 0.0,
        distance_length_scale: 1.0,
        distance_mode: ReflectionDistanceMode::PostTransformOffset,
        reference: ReflectionReference::EffectInput,
        direction_degrees: 90.0,
        transform: ImageEffectTransform {
          scale_x: 1.0,
          scale_y: -1.0,
          skew_x: 0.0,
          skew_y: 0.0,
          shift_x_px: 0.0,
          shift_y_px: 0.0,
        },
        alignment: (0.5, 0.5),
        rotate_with_shape: false,
      })],
    );

    assert!(reflected.get_pixel(0, 0).0[3] > reflected.get_pixel(0, 3).0[3]);
  }

  #[test]
  fn reflection_fade_positions_use_explicit_text_metric_box_not_ink_or_line_cell() {
    let effects = ImageEffectContainer {
      kind: ImageEffectContainerKind::Tree,
      effects: vec![ImageEffect::Reflection(ImageReflectionEffect {
        blur_radius_px: 0.0,
        raster_length_scale: 1.0,
        bounds_radius_scale: 1.0,
        bounds_radius_offset_px: 0.0,
        start_opacity: 1.0,
        start_position: 0.0,
        end_opacity: 0.0,
        end_position: 0.5,
        fade_direction_degrees: 90.0,
        distance_px: 0.0,
        distance_length_scale: 1.0,
        distance_mode: ReflectionDistanceMode::PostTransformOffset,
        reference: ReflectionReference::EffectInput,
        direction_degrees: 0.0,
        transform: ImageEffectTransform {
          scale_x: 1.0,
          scale_y: 1.0,
          skew_x: 0.0,
          skew_y: 0.0,
          shift_x_px: 0.0,
          shift_y_px: 0.0,
        },
        alignment: (0.0, 0.0),
        rotate_with_shape: false,
      })],
    };
    let mut image = RgbaImage::from_pixel(1, 10, Rgba([10, 20, 30, 0]));
    for y in 2..6 {
      image.get_pixel_mut(0, y).0[3] = 255;
    }

    apply_container_to_padded_image_with_sources_and_anchor(
      &mut image,
      &effects,
      ImageEffectSourceGeometry {
        paint_left_px: 0.0,
        paint_top_px: 2.0,
        paint_width_px: 1.0,
        paint_height_px: 4.0,
        shadow_anchor_left_px: 0.0,
        shadow_anchor_top_px: 0.0,
        shadow_anchor_width_px: 1.0,
        shadow_anchor_height_px: 10.0,
        anchor_left_px: 0.0,
        anchor_top_px: 0.0,
        anchor_width_px: 1.0,
        anchor_height_px: 10.0,
        ramp_left_px: 0.0,
        ramp_top_px: 1.0,
        ramp_width_px: 1.0,
        ramp_height_px: 8.0,
      },
      ImageEffectSourceImages::default(),
    );

    assert!(image.get_pixel(0, 4).0[3] > 0);
    assert_eq!(image.get_pixel(0, 5).0[3], 0);
  }

  #[test]
  fn premultiplied_blur_does_not_leak_hidden_rgb() {
    let mut image = RgbaImage::from_pixel(3, 1, Rgba([255, 0, 0, 0]));
    image.get_pixel_mut(1, 0).0 = [0, 0, 255, 255];
    apply_to_image(
      &mut image,
      &[ImageEffect::Blur {
        radius_px: 1.0,
        grow_bounds: true,
      }],
    );
    let edge = image.get_pixel(0, 0).0;
    assert!(edge[2] > edge[0]);
  }

  #[test]
  fn effect_dag_preserves_tree_order() {
    let dag = a::EffectDag {
      r#type: Some(a::EffectContainerValues::Tree),
      effect_dag_choice: vec![
        a::EffectDagChoice::AlphaReplace(a::AlphaReplace {
          alpha: DrawingmlPercentageValue::Decimal(50_000),
        }),
        a::EffectDagChoice::AlphaModulationFixed(a::AlphaModulationFixed {
          amount: Some(DrawingmlPercentageValue::Decimal(50_000)),
        }),
      ],
      ..a::EffectDag::default()
    };
    let effects = from_effect_dag(&dag, None, &NoColorResolver);
    assert_eq!(effects.kind, ImageEffectContainerKind::Tree);

    let image = RgbaImage::from_pixel(1, 1, Rgba([10, 20, 30, 255]));
    let image = super::apply_container(&image, &effects);
    assert_eq!(image.get_pixel(0, 0).0[3], 64);
  }

  #[test]
  fn effect_dag_named_reference_cycle_terminates() {
    let dag = a::EffectDag {
      r#type: Some(a::EffectContainerValues::Tree),
      name: Some("self".into()),
      effect_dag_choice: vec![a::EffectDagChoice::Effect(a::Effect {
        reference: Some("self".into()),
      })],
    };
    let effects = from_effect_dag(&dag, None, &NoColorResolver);
    let [ImageEffect::Container(referenced)] = effects.effects.as_slice() else {
      panic!("self reference should lower to one finite container");
    };
    assert!(referenced.effects.is_empty());
  }

  #[test]
  fn built_in_effect_references_select_separate_host_sources() {
    let container = ImageEffectContainer {
      kind: ImageEffectContainerKind::Sibling,
      effects: vec![
        ImageEffect::SourceReference(ImageEffectSourceReference::Fill),
        ImageEffect::SourceReference(ImageEffectSourceReference::Line),
      ],
    };
    assert_eq!(
      source_requirements(&container),
      ImageEffectSourceRequirements {
        fill: true,
        line: true,
        fill_line: false,
        children: false,
        effect_mask: false,
        reflection_paint: false,
      }
    );

    let mut combined = RgbaImage::from_pixel(2, 1, Rgba([0; 4]));
    let mut fill = combined.clone();
    fill.get_pixel_mut(0, 0).0 = [255, 0, 0, 255];
    let mut line = combined.clone();
    line.get_pixel_mut(1, 0).0 = [0, 0, 255, 255];
    apply_container_to_padded_image_with_sources(
      &mut combined,
      &container,
      0.0,
      0.0,
      2.0,
      1.0,
      ImageEffectSourceImages {
        fill: Some(&fill),
        line: Some(&line),
        fill_line: None,
        children: None,
        effect_mask: None,
        reflection_paint: None,
        bounds: ImageEffectSourcePixelBounds::default(),
      },
    );

    assert_eq!(combined.get_pixel(0, 0).0, [255, 0, 0, 255]);
    assert_eq!(combined.get_pixel(1, 0).0, [0, 0, 255, 255]);
  }

  #[test]
  fn rotate_with_shape_rotates_shadow_direction_and_alignment() {
    let mut container = ImageEffectContainer {
      kind: ImageEffectContainerKind::Tree,
      effects: vec![ImageEffect::OuterShadow {
        blur_radius_px: 0.0,
        distance_px: 1.0,
        raster_length_scale: 1.0,
        distance_length_scale: 1.0,
        bounds_radius_scale: 1.0,
        bounds_radius_offset_px: 0.0,
        blur_kernel: ShadowBlurKernel::Direct2dGaussian,
        direction_degrees: 0.0,
        distance_mode: ShadowDistanceMode::PostTransformOffset,
        transform: ImageEffectTransform {
          scale_x: 2.0,
          scale_y: 1.0,
          skew_x: 0.0,
          skew_y: 0.0,
          shift_x_px: 0.0,
          shift_y_px: 0.0,
        },
        alignment: (1.0, 0.5),
        rotate_with_shape: true,
        color: ResolvedEffectColor {
          color: RgbColor { r: 0, g: 0, b: 0 },
          alpha: 255,
        },
      }],
    };
    rotate_container_with_shape(&mut container, 90.0);
    let [
      ImageEffect::OuterShadow {
        direction_degrees,
        transform,
        alignment,
        ..
      },
    ] = container.effects.as_slice()
    else {
      panic!("expected one outer shadow");
    };
    assert!((*direction_degrees - 90.0).abs() < 0.001);
    assert!((transform.scale_x - 1.0).abs() < 0.001);
    assert!((transform.scale_y - 2.0).abs() < 0.001);
    assert!((alignment.0 - 0.5).abs() < 0.001);
    assert!((alignment.1 - 1.0).abs() < 0.001);
  }

  #[test]
  fn office_washout_uses_mso_split_brightness_formula() {
    // LibreOffice Bitmap::Adjust(..., msoBrightness=true) documents Office's
    // half-before/half-after ordering. The canonical bright=70,
    // contrast=-70 washout maps a near-black channel to 206, not the 217
    // produced by LO's ordinary luminance-first formula.
    assert_eq!(mso_brightness_contrast_component(1, 70, -70), 206);
  }

  #[test]
  fn padded_container_keeps_transformed_shadow_outside_source_bounds() {
    let color = ResolvedEffectColor {
      color: RgbColor { r: 5, g: 6, b: 7 },
      alpha: 255,
    };
    let container = ImageEffectContainer {
      kind: ImageEffectContainerKind::Tree,
      effects: vec![ImageEffect::OuterShadow {
        blur_radius_px: 0.0,
        distance_px: 3.0,
        raster_length_scale: 1.0,
        distance_length_scale: 1.0,
        bounds_radius_scale: 1.0,
        bounds_radius_offset_px: 0.0,
        blur_kernel: ShadowBlurKernel::Direct2dGaussian,
        direction_degrees: 0.0,
        distance_mode: ShadowDistanceMode::PostTransformOffset,
        transform: ImageEffectTransform {
          scale_x: 1.0,
          scale_y: 1.0,
          skew_x: 0.0,
          skew_y: 0.0,
          shift_x_px: 0.0,
          shift_y_px: 0.0,
        },
        alignment: (0.5, 0.5),
        rotate_with_shape: false,
        color,
      }],
    };
    let mut image = RgbaImage::from_pixel(7, 1, Rgba([0; 4]));
    image.get_pixel_mut(1, 0).0 = [100, 110, 120, 255];
    apply_container_to_padded_image(&mut image, &container, 1.0, 0.0, 1.0, 1.0);
    assert_eq!(image.get_pixel(4, 0).0, [5, 6, 7, 255]);

    let bounds = container_output_bounds(&container, 0.75, 0.75).unwrap();
    assert!(bounds.right_pt > 2.9);
  }

  #[test]
  fn outer_shadow_uses_its_independent_alignment_rectangle() {
    let color = ResolvedEffectColor {
      color: RgbColor { r: 5, g: 6, b: 7 },
      alpha: 255,
    };
    let container = ImageEffectContainer {
      kind: ImageEffectContainerKind::Tree,
      effects: vec![ImageEffect::OuterShadow {
        blur_radius_px: 0.0,
        distance_px: 0.0,
        raster_length_scale: 1.0,
        distance_length_scale: 1.0,
        bounds_radius_scale: 1.0,
        bounds_radius_offset_px: 0.0,
        blur_kernel: ShadowBlurKernel::Direct2dGaussian,
        direction_degrees: 0.0,
        distance_mode: ShadowDistanceMode::PostTransformOffset,
        transform: ImageEffectTransform {
          scale_x: 1.0,
          scale_y: 0.5,
          skew_x: 0.0,
          skew_y: 0.0,
          shift_x_px: 0.0,
          shift_y_px: 0.0,
        },
        alignment: (0.5, 0.5),
        rotate_with_shape: false,
        color,
      }],
    };
    let source = EffectOutputBounds {
      left_pt: 0.0,
      top_pt: 0.0,
      right_pt: 10.0,
      bottom_pt: 10.0,
    };
    let shadow_anchor = EffectOutputBounds {
      left_pt: 0.0,
      top_pt: -10.0,
      right_pt: 10.0,
      bottom_pt: 0.0,
    };

    let bounds =
      container_output_bounds_with_anchors(&container, source, source, shadow_anchor).unwrap();
    assert!((bounds.top_pt + 2.5).abs() < 0.001);
    assert!((bounds.bottom_pt - 2.5).abs() < 0.001);

    let mut image = RgbaImage::from_pixel(1, 12, Rgba([0; 4]));
    image.get_pixel_mut(0, 8).0[3] = 255;
    image.get_pixel_mut(0, 9).0[3] = 255;
    apply_container_to_padded_image_with_sources_and_anchor(
      &mut image,
      &container,
      ImageEffectSourceGeometry {
        paint_left_px: 0.0,
        paint_top_px: 8.0,
        paint_width_px: 1.0,
        paint_height_px: 2.0,
        shadow_anchor_left_px: 0.0,
        shadow_anchor_top_px: -4.0,
        shadow_anchor_width_px: 1.0,
        shadow_anchor_height_px: 8.0,
        anchor_left_px: 0.0,
        anchor_top_px: 0.0,
        anchor_width_px: 1.0,
        anchor_height_px: 12.0,
        ramp_left_px: 0.0,
        ramp_top_px: 0.0,
        ramp_width_px: 1.0,
        ramp_height_px: 12.0,
      },
      ImageEffectSourceImages::default(),
    );
    assert_eq!(image.get_pixel(0, 4).0, [5, 6, 7, 255]);
    assert_eq!(image.get_pixel(0, 7).0[3], 0);
  }

  #[test]
  fn reflection_keeps_the_general_anchor_when_shadow_anchor_differs() {
    let container = ImageEffectContainer {
      kind: ImageEffectContainerKind::Tree,
      effects: vec![ImageEffect::Reflection(ImageReflectionEffect {
        blur_radius_px: 0.0,
        raster_length_scale: 1.0,
        bounds_radius_scale: 1.0,
        bounds_radius_offset_px: 0.0,
        start_opacity: 1.0,
        start_position: 0.0,
        end_opacity: 1.0,
        end_position: 1.0,
        fade_direction_degrees: 90.0,
        distance_px: 0.0,
        distance_length_scale: 1.0,
        distance_mode: ReflectionDistanceMode::PostTransformOffset,
        reference: ReflectionReference::EffectInput,
        direction_degrees: 0.0,
        transform: ImageEffectTransform {
          scale_x: 1.0,
          scale_y: -1.0,
          skew_x: 0.0,
          skew_y: 0.0,
          shift_x_px: 0.0,
          shift_y_px: 0.0,
        },
        alignment: (0.5, 0.5),
        rotate_with_shape: false,
      })],
    };
    let mut image = RgbaImage::from_pixel(1, 12, Rgba([0; 4]));
    image.get_pixel_mut(0, 2).0 = [10, 20, 30, 255];
    image.get_pixel_mut(0, 3).0 = [10, 20, 30, 255];
    apply_container_to_padded_image_with_sources_and_anchor(
      &mut image,
      &container,
      ImageEffectSourceGeometry {
        paint_left_px: 0.0,
        paint_top_px: 2.0,
        paint_width_px: 1.0,
        paint_height_px: 2.0,
        shadow_anchor_left_px: 0.0,
        shadow_anchor_top_px: -4.0,
        shadow_anchor_width_px: 1.0,
        shadow_anchor_height_px: 8.0,
        anchor_left_px: 0.0,
        anchor_top_px: 0.0,
        anchor_width_px: 1.0,
        anchor_height_px: 12.0,
        ramp_left_px: 0.0,
        ramp_top_px: 0.0,
        ramp_width_px: 1.0,
        ramp_height_px: 12.0,
      },
      ImageEffectSourceImages::default(),
    );
    assert_eq!(image.get_pixel(0, 8).0, [10, 20, 30, 255]);
    assert_eq!(image.get_pixel(0, 9).0, [10, 20, 30, 255]);
  }

  #[test]
  fn three_d_precedence_removes_soft_edge_from_nested_effect_graphs() {
    let mut container = ImageEffectContainer {
      kind: ImageEffectContainerKind::Tree,
      effects: vec![
        ImageEffect::SoftEdge(4.0),
        ImageEffect::Container(ImageEffectContainer {
          kind: ImageEffectContainerKind::Sibling,
          effects: vec![ImageEffect::SoftEdge(2.0), ImageEffect::Grayscale],
        }),
      ],
    };

    suppress_soft_edge(&mut container);

    assert_eq!(
      container.effects,
      vec![ImageEffect::Container(ImageEffectContainer {
        kind: ImageEffectContainerKind::Sibling,
        effects: vec![ImageEffect::Grayscale],
      })]
    );
  }
}
