use std::io::Cursor;

use image::codecs::png::PngEncoder;
use image::{ColorType, ImageEncoder};
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use ooxmlsdk::units::DrawingmlPercentageValue;

use crate::model::RgbColor;
use crate::render::emf_wmf;

use super::color_math::HslColor;

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
    spread_ratio: f32,
    spread_kernel: GlowSpreadKernel,
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
    direction_degrees: f32,
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
  Diamond,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ImageEffectSourceReference {
  Fill,
  Line,
  FillLine,
  Children,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct ImageEffectSourceImages<'a> {
  pub(crate) fill: Option<&'a image::RgbaImage>,
  pub(crate) line: Option<&'a image::RgbaImage>,
  pub(crate) fill_line: Option<&'a image::RgbaImage>,
  pub(crate) children: Option<&'a image::RgbaImage>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct ImageEffectSourceRequirements {
  pub(crate) fill: bool,
  pub(crate) line: bool,
  pub(crate) fill_line: bool,
  pub(crate) children: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ImageEffectContainer {
  pub(crate) kind: ImageEffectContainerKind,
  pub(crate) effects: Vec<ImageEffect>,
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ImageReflectionEffect {
  blur_radius_px: f32,
  start_opacity: f32,
  start_position: f32,
  end_opacity: f32,
  end_position: f32,
  fade_direction_degrees: f32,
  distance_px: f32,
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
  pub(crate) color: ResolvedEffectColor,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct WordprocessingTextShadow {
  pub(crate) blur_radius_px: f32,
  pub(crate) distance_px: f32,
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
  pub(crate) start_opacity: f32,
  pub(crate) start_position: f32,
  pub(crate) end_opacity: f32,
  pub(crate) end_position: f32,
  pub(crate) distance_px: f32,
  pub(crate) direction_degrees: f32,
  pub(crate) fade_direction_degrees: f32,
  pub(crate) scale_x: f32,
  pub(crate) scale_y: f32,
  pub(crate) skew_x_degrees: f32,
  pub(crate) skew_y_degrees: f32,
  pub(crate) alignment: (f32, f32),
}

pub(crate) fn from_wordprocessing_text_effects(
  glow: Option<WordprocessingTextGlow>,
  shadow: Option<WordprocessingTextShadow>,
  reflection: Option<WordprocessingTextReflection>,
) -> Option<ImageEffectContainer> {
  let mut branches = Vec::new();
  if let Some(glow) = glow {
    branches.push(ImageEffect::Glow {
      radius_px: glow.radius_px,
      spread_ratio: 0.5,
      spread_kernel: GlowSpreadKernel::Square,
      color: glow.color,
    });
  }
  if let Some(shadow) = shadow {
    branches.push(ImageEffect::OuterShadow {
      blur_radius_px: shadow.blur_radius_px,
      distance_px: shadow.distance_px,
      direction_degrees: shadow.direction_degrees,
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
    });
  }
  if let Some(reflection) = reflection {
    branches.push(ImageEffect::Reflection(ImageReflectionEffect {
      blur_radius_px: reflection.blur_radius_px,
      start_opacity: reflection.start_opacity,
      start_position: reflection.start_position,
      end_opacity: reflection.end_opacity,
      end_position: reflection.end_position,
      fade_direction_degrees: reflection.fade_direction_degrees,
      distance_px: reflection.distance_px,
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
    }));
  }
  if branches.is_empty() {
    return None;
  }
  branches.push(ImageEffect::Identity);
  Some(ImageEffectContainer {
    kind: ImageEffectContainerKind::Sibling,
    effects: branches,
  })
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
    spread_ratio: 1.0 / 3.0,
    spread_kernel: GlowSpreadKernel::Square,
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
    direction_degrees,
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
    start_opacity: effect
      .start_opacity
      .map(|value| value.as_ratio() as f32)
      .unwrap_or(0.0),
    start_position: effect
      .start_position
      .map(|value| value.as_ratio() as f32)
      .unwrap_or(1.0),
    end_opacity: effect
      .end_alpha
      .map(|value| value.as_ratio() as f32)
      .unwrap_or(1.0),
    end_position: effect
      .end_position
      .map(|value| value.as_ratio() as f32)
      .unwrap_or(0.0),
    fade_direction_degrees: effect.fade_direction.unwrap_or(5_400_000) as f32 / 60_000.0,
    distance_px: effect
      .distance
      .map(|value| value.to_emu() as f32 / 9_525.0)
      .unwrap_or_default(),
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
  *image = apply_container_with_bounds(
    image,
    container,
    content_bounds,
    content_bounds,
    ImageEffectSourceImages::default(),
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
  let content_bounds = PixelBounds {
    left: content_left_px,
    top: content_top_px,
    right: content_left_px + content_width_px,
    bottom: content_top_px + content_height_px,
  };
  *image = apply_container_with_bounds(image, container, content_bounds, content_bounds, sources);
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct EffectOutputBounds {
  pub(crate) left_pt: f32,
  pub(crate) top_pt: f32,
  pub(crate) right_pt: f32,
  pub(crate) bottom_pt: f32,
}

#[derive(Clone, Copy)]
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

/// Computes the authored effect graph's output range relative to the source
/// shape. Lengths are evaluated at DrawingML's 96-DPI bitmap baseline and
/// converted back to points for the bounded full-color raster.
pub(crate) fn container_output_bounds(
  container: &ImageEffectContainer,
  width_pt: f32,
  height_pt: f32,
) -> Option<EffectOutputBounds> {
  let css_pixels_per_point = 96.0 / 72.0;
  let source = PixelBounds {
    left: 0.0,
    top: 0.0,
    right: width_pt * css_pixels_per_point,
    bottom: height_pt * css_pixels_per_point,
  };
  let output = effect_container_output_bounds(container, source, source)?;
  Some(EffectOutputBounds {
    left_pt: output.left / css_pixels_per_point,
    top_pt: output.top / css_pixels_per_point,
    right_pt: output.right / css_pixels_per_point,
    bottom_pt: output.bottom / css_pixels_per_point,
  })
}

fn effect_container_output_bounds(
  container: &ImageEffectContainer,
  source: PixelBounds,
  root_source: PixelBounds,
) -> Option<PixelBounds> {
  match container.kind {
    ImageEffectContainerKind::Tree => {
      let mut output = source;
      for effect in &container.effects {
        output = effect_output_bounds(effect, output, root_source)?;
      }
      Some(output)
    }
    ImageEffectContainerKind::Sibling => {
      let mut effects = container.effects.iter();
      let first = effect_output_bounds(effects.next()?, source, root_source)?;
      effects.try_fold(first, |output, effect| {
        Some(output.union(effect_output_bounds(effect, source, root_source)?))
      })
    }
  }
}

fn effect_output_bounds(
  effect: &ImageEffect,
  source: PixelBounds,
  root_source: PixelBounds,
) -> Option<PixelBounds> {
  match effect {
    ImageEffect::AlphaOutset(radius) => Some(source.outset(*radius)),
    ImageEffect::Blur {
      radius_px,
      grow_bounds,
    } => Some(if *grow_bounds {
      source.outset(*radius_px)
    } else {
      source
    }),
    ImageEffect::Glow { radius_px, .. } => Some(source.outset(*radius_px)),
    ImageEffect::OuterShadow {
      blur_radius_px,
      distance_px,
      direction_degrees,
      transform,
      alignment,
      ..
    } => {
      let transformed = transformed_effect_bounds(source, *transform, *alignment);
      let direction = direction_degrees.to_radians();
      Some(
        transformed
          .translated(
            direction.cos() * *distance_px,
            direction.sin() * *distance_px,
          )
          .outset(*blur_radius_px),
      )
    }
    ImageEffect::Reflection(reflection) => {
      let transformed =
        transformed_effect_bounds(source, reflection.transform, reflection.alignment);
      let direction = reflection.direction_degrees.to_radians();
      Some(
        transformed
          .translated(
            direction.cos() * reflection.distance_px,
            direction.sin() * reflection.distance_px,
          )
          .outset(reflection.blur_radius_px),
      )
    }
    ImageEffect::RelativeOffset { offset_x, offset_y } => {
      Some(source.translated(*offset_x * source.width(), *offset_y * source.height()))
    }
    ImageEffect::Transform(transform) => {
      Some(transformed_effect_bounds(source, *transform, (0.0, 0.0)))
    }
    ImageEffect::Container(container) => {
      effect_container_output_bounds(container, source, root_source)
    }
    ImageEffect::Blend { container, .. } => Some(source.union(effect_container_output_bounds(
      container,
      source,
      root_source,
    )?)),
    ImageEffect::SourceReference(_) => Some(root_source),
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
  transform: ImageEffectTransform,
  alignment: (f32, f32),
) -> PixelBounds {
  let anchor_x = source.left + source.width() * alignment.0;
  let anchor_y = source.top + source.height() * alignment.1;
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
      ImageEffect::Glow { radius_px, .. } => *radius_px *= scale,
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
        transform,
        ..
      } => {
        *blur_radius_px *= scale;
        *distance_px *= scale;
        transform.shift_x_px *= scale;
        transform.shift_y_px *= scale;
      }
      ImageEffect::Reflection(reflection) => {
        reflection.blur_radius_px *= scale;
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
      ImageEffect::Glow { radius_px, .. } => *radius_px *= scale,
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
  };
  apply_to_image_with_source_context(image, effects, content_bounds, content_bounds, sources);
}

fn apply_to_image_with_source_context(
  image: &mut image::RgbaImage,
  effects: &[ImageEffect],
  content_bounds: PixelBounds,
  root_bounds: PixelBounds,
  sources: ImageEffectSourceImages<'_>,
) {
  let mut current_bounds = content_bounds;
  for effect in effects {
    let effect_source_bounds = current_bounds;
    if let Some(output_bounds) = effect_output_bounds(effect, effect_source_bounds, root_bounds) {
      current_bounds = output_bounds;
    }
    if let ImageEffect::Blur { radius_px, .. } = effect {
      if *radius_px > f32::EPSILON {
        *image = blur_rgba_premultiplied(image, *radius_px);
      }
      continue;
    }
    if let ImageEffect::AlphaModulate(container) = effect {
      let modulation =
        apply_container_with_bounds(image, container, effect_source_bounds, root_bounds, sources);
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
      let blended =
        apply_container_with_bounds(image, container, effect_source_bounds, root_bounds, sources);
      for (base, overlay) in image.pixels_mut().zip(blended.pixels()) {
        blend_rgba_pixel(base, overlay, *blend_mode);
      }
      continue;
    }
    if let ImageEffect::Container(container) = effect {
      *image =
        apply_container_with_bounds(image, container, effect_source_bounds, root_bounds, sources);
      continue;
    }
    if let ImageEffect::FillOverlay { fill, blend_mode } = effect {
      apply_fill_overlay(image, fill, *blend_mode, effect_source_bounds);
      continue;
    }
    if let ImageEffect::Fill(fill) = effect {
      apply_fill(image, fill, effect_source_bounds);
      continue;
    }
    if let ImageEffect::Glow {
      radius_px,
      spread_ratio,
      spread_kernel,
      color,
    } = effect
    {
      *image = glow_image(image, *radius_px, *spread_ratio, *spread_kernel, *color);
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
      *image = inner_shadow_image(
        image,
        *blur_radius_px,
        *distance_px,
        *direction_degrees,
        *color,
      );
      continue;
    }
    if let ImageEffect::OuterShadow {
      blur_radius_px,
      distance_px,
      direction_degrees,
      transform,
      alignment,
      color,
      ..
    } = effect
    {
      *image = outer_shadow_image(
        image,
        *blur_radius_px,
        *distance_px,
        *direction_degrees,
        *transform,
        *alignment,
        *color,
        effect_source_bounds,
      );
      continue;
    }
    if let ImageEffect::Reflection(effect) = effect {
      *image = reflection_image(image, *effect, effect_source_bounds);
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
          shift_x_px: *offset_x * effect_source_bounds.width(),
          shift_y_px: *offset_y * effect_source_bounds.height(),
        },
      );
      continue;
    }
    if let ImageEffect::SoftEdge(radius_px) = effect {
      apply_soft_edge(image, *radius_px);
      continue;
    }
    if let ImageEffect::Transform(transform) = effect {
      let mut transform = *transform;
      transform.shift_x_px += effect_source_bounds.left
        - transform.scale_x * effect_source_bounds.left
        - transform.skew_x * effect_source_bounds.top;
      transform.shift_y_px += effect_source_bounds.top
        - transform.skew_y * effect_source_bounds.left
        - transform.scale_y * effect_source_bounds.top;
      *image = affine_image(image, transform);
      continue;
    }
    if let ImageEffect::AlphaOutset(radius_px) = effect {
      apply_alpha_outset(image, *radius_px);
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

fn apply_fill(image: &mut image::RgbaImage, fill: &ImageEffectFill, bounds: PixelBounds) {
  let width = image.width().max(1);
  let height = image.height().max(1);
  for y in 0..height {
    for x in 0..width {
      let alpha = image.get_pixel(x, y).0[3];
      let fill = sample_fill_at(
        fill,
        x as f32 + 0.5 - bounds.left,
        y as f32 + 0.5 - bounds.top,
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
  apply_container_with_bounds(
    source,
    container,
    bounds,
    bounds,
    ImageEffectSourceImages::default(),
  )
}

fn apply_container_with_bounds<'a>(
  source: &'a image::RgbaImage,
  container: &ImageEffectContainer,
  content_bounds: PixelBounds,
  root_bounds: PixelBounds,
  sources: ImageEffectSourceImages<'a>,
) -> image::RgbaImage {
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
        content_bounds,
        root_bounds,
        sources,
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
          content_bounds,
          root_bounds,
          sources,
        );
        for (destination, source) in output.pixels_mut().zip(branch.pixels()) {
          source_over(destination, source);
        }
      }
      output
    }
  }
}

fn source_over(destination: &mut image::Rgba<u8>, source: &image::Rgba<u8>) {
  let source_alpha = u32::from(source.0[3]);
  let destination_alpha = u32::from(destination.0[3]);
  let inverse_source_alpha = u32::from(u8::MAX) - source_alpha;
  let output_alpha = source_alpha + (destination_alpha * inverse_source_alpha + 127) / 255;
  if output_alpha == 0 {
    destination.0 = [0; 4];
    return;
  }
  for channel in 0..3 {
    let source_premultiplied = u32::from(source.0[channel]) * source_alpha;
    let destination_premultiplied =
      u32::from(destination.0[channel]) * destination_alpha * inverse_source_alpha / 255;
    destination.0[channel] =
      ((source_premultiplied + destination_premultiplied + output_alpha / 2) / output_alpha) as u8;
  }
  destination.0[3] = output_alpha as u8;
}

fn apply_fill_overlay(
  image: &mut image::RgbaImage,
  fill: &ImageEffectFill,
  blend_mode: ImageEffectBlendMode,
  bounds: PixelBounds,
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
        x as f32 + 0.5 - bounds.left,
        y as f32 + 0.5 - bounds.top,
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

fn glow_image(
  source: &image::RgbaImage,
  radius_px: f32,
  spread_ratio: f32,
  spread_kernel: GlowSpreadKernel,
  color: ResolvedEffectColor,
) -> image::RgbaImage {
  let alpha = image::GrayImage::from_fn(source.width(), source.height(), |x, y| {
    image::Luma([source.get_pixel(x, y).0[3]])
  });
  let glow_alpha = if radius_px > f32::EPSILON {
    // Office fixed output splits the halo between an opaque spread and a
    // Gaussian fringe. Its stored SMask reaches transparency at roughly
    // three standard deviations from the spread boundary, so the authored
    // fringe extent is the Gaussian support rather than sigma itself.
    let spread_radius = radius_px * spread_ratio.clamp(0.0, 1.0);
    let spread_radius = spread_radius.ceil() as usize;
    let dilated = match spread_kernel {
      GlowSpreadKernel::Square => dilate_nontransparent_alpha(&alpha, spread_radius),
      GlowSpreadKernel::Diamond => dilate_nontransparent_alpha_diamond(&alpha, spread_radius),
    };
    image::imageops::blur(&dilated, radius_px / 6.0)
  } else {
    alpha
  };
  image::RgbaImage::from_fn(source.width(), source.height(), |x, y| {
    let alpha =
      ((u16::from(glow_alpha.get_pixel(x, y).0[0]) * u16::from(color.alpha) + 127) / 255) as u8;
    image::Rgba([color.color.r, color.color.g, color.color.b, alpha])
  })
}

pub(crate) fn stack_blur_alpha(alpha: &mut [u8], width: usize, height: usize, radius: usize) {
  let radius = radius.min(254);
  if radius == 0 || width == 0 || height == 0 {
    return;
  }
  let mut horizontal = vec![0_u8; alpha.len()];
  for y in 0..height {
    triangular_blur_line(
      &alpha[y * width..(y + 1) * width],
      &mut horizontal[y * width..(y + 1) * width],
      radius,
    );
  }
  let mut column = vec![0_u8; height];
  let mut blurred_column = vec![0_u8; height];
  for x in 0..width {
    for y in 0..height {
      column[y] = horizontal[y * width + x];
    }
    triangular_blur_line(&column, &mut blurred_column, radius);
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

fn dilate_nontransparent_alpha_diamond(
  alpha: &image::GrayImage,
  radius: usize,
) -> image::GrayImage {
  if radius == 0 {
    return alpha.clone();
  }
  let width = alpha.width() as usize;
  let height = alpha.height() as usize;
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
        let horizontal_radius = radius - delta_y.unsigned_abs();
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

fn dilate_nontransparent_alpha(alpha: &image::GrayImage, radius: usize) -> image::GrayImage {
  if radius == 0 {
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
    let left = x.saturating_sub(radius);
    let top = y.saturating_sub(radius);
    let right = x.saturating_add(radius).saturating_add(1).min(width);
    let bottom = y.saturating_add(radius).saturating_add(1).min(height);
    let sum = integral[bottom * integral_width + right] + integral[top * integral_width + left]
      - integral[top * integral_width + right]
      - integral[bottom * integral_width + left];
    image::Luma([u8::from(sum > 0) * u8::MAX])
  })
}

fn inner_shadow_image(
  source: &image::RgbaImage,
  blur_radius_px: f32,
  distance_px: f32,
  direction_degrees: f32,
  color: ResolvedEffectColor,
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
      shift_x_px: -radians.cos() * distance_px,
      shift_y_px: -radians.sin() * distance_px,
    },
  );
  let shifted_alpha = image::GrayImage::from_fn(source.width(), source.height(), |x, y| {
    image::Luma([shifted.get_pixel(x, y).0[3]])
  });
  let shifted_alpha = if blur_radius_px > f32::EPSILON {
    image::imageops::blur(&shifted_alpha, blur_radius_px)
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

fn outer_shadow_image(
  source: &image::RgbaImage,
  blur_radius_px: f32,
  distance_px: f32,
  direction_degrees: f32,
  mut transform: ImageEffectTransform,
  alignment: (f32, f32),
  color: ResolvedEffectColor,
  content_bounds: PixelBounds,
) -> image::RgbaImage {
  let radians = direction_degrees.to_radians();
  // The default DrawingML alignment is bottom center. Preserve that point
  // while applying the authored scale/skew matrix, then apply dist/dir.
  let anchor_x = content_bounds.left + content_bounds.width() * alignment.0;
  let anchor_y = content_bounds.top + content_bounds.height() * alignment.1;
  transform.shift_x_px = anchor_x - transform.scale_x * anchor_x - transform.skew_x * anchor_y
    + radians.cos() * distance_px;
  transform.shift_y_px = anchor_y - transform.skew_y * anchor_x - transform.scale_y * anchor_y
    + radians.sin() * distance_px;
  let transformed = affine_image(source, transform);
  let alpha = image::GrayImage::from_fn(source.width(), source.height(), |x, y| {
    image::Luma([transformed.get_pixel(x, y).0[3]])
  });
  let alpha = if blur_radius_px > f32::EPSILON {
    image::imageops::blur(&alpha, stack_blur_equivalent_sigma(blur_radius_px))
  } else {
    alpha
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

/// LibreOffice renders DrawingML shadows with a Stack Blur radius. `image`
/// accepts a Gaussian standard deviation instead, so passing the radius
/// directly produces a much wider and lighter fringe. A Stack Blur kernel is
/// triangular and has variance `radius * (radius + 2) / 6`.
fn stack_blur_equivalent_sigma(radius_px: f32) -> f32 {
  (radius_px * (radius_px + 2.0) / 6.0).sqrt()
}

fn reflection_image(
  source: &image::RgbaImage,
  effect: ImageReflectionEffect,
  content_bounds: PixelBounds,
) -> image::RgbaImage {
  let fade = effect.fade_direction_degrees.to_radians();
  let fade_x = fade.cos();
  let fade_y = fade.sin();
  let width = content_bounds.width();
  let height = content_bounds.height();
  let projections = [
    0.0,
    fade_x * width,
    fade_y * height,
    fade_x * width + fade_y * height,
  ];
  let minimum = projections.iter().copied().fold(f32::INFINITY, f32::min);
  let maximum = projections
    .iter()
    .copied()
    .fold(f32::NEG_INFINITY, f32::max);
  let span = (maximum - minimum).max(f32::EPSILON);
  let mut faded = source.clone();
  for (x, y, pixel) in faded.enumerate_pixels_mut() {
    let position = (fade_x * (x as f32 + 0.5 - content_bounds.left)
      + fade_y * (y as f32 + 0.5 - content_bounds.top)
      - minimum)
      / span;
    let opacity = effect_ramp(
      position,
      (effect.start_position, effect.start_opacity),
      (effect.end_position, effect.end_opacity),
    );
    pixel.0[3] = (f32::from(pixel.0[3]) * opacity).round().clamp(0.0, 255.0) as u8;
  }

  let direction = effect.direction_degrees.to_radians();
  let anchor_x = content_bounds.left + width * effect.alignment.0;
  let anchor_y = content_bounds.top + height * effect.alignment.1;
  let mut transform = effect.transform;
  transform.shift_x_px = anchor_x - transform.scale_x * anchor_x - transform.skew_x * anchor_y
    + direction.cos() * effect.distance_px;
  transform.shift_y_px = anchor_y - transform.skew_y * anchor_x - transform.scale_y * anchor_y
    + direction.sin() * effect.distance_px;
  let reflected = affine_image(&faded, transform);
  if effect.blur_radius_px > f32::EPSILON {
    blur_rgba_premultiplied(&reflected, effect.blur_radius_px)
  } else {
    reflected
  }
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

fn blur_rgba_premultiplied(source: &image::RgbaImage, radius_px: f32) -> image::RgbaImage {
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
  let blurred = image::imageops::blur(&premultiplied, radius_px);
  image::RgbaImage::from_fn(source.width(), source.height(), |x, y| {
    let pixel = blurred.get_pixel(x, y).0;
    let alpha = u16::from(pixel[3]);
    if alpha == 0 {
      image::Rgba([0; 4])
    } else {
      image::Rgba([
        ((u16::from(pixel[0]) * 255 + alpha / 2) / alpha).min(255) as u8,
        ((u16::from(pixel[1]) * 255 + alpha / 2) / alpha).min(255) as u8,
        ((u16::from(pixel[2]) * 255 + alpha / 2) / alpha).min(255) as u8,
        pixel[3],
      ])
    }
  })
}

fn apply_soft_edge(image: &mut image::RgbaImage, radius_px: f32) {
  if radius_px <= f32::EPSILON {
    return;
  }
  let alpha = image::GrayImage::from_fn(image.width(), image.height(), |x, y| {
    image::Luma([image.get_pixel(x, y).0[3]])
  });
  let radius = radius_px.ceil().max(1.0) as usize;
  let eroded = erode_opaque_alpha(&alpha, radius);
  let blurred = image::imageops::blur(&eroded, radius_px);
  for ((pixel, original_alpha), blurred_alpha) in
    image.pixels_mut().zip(alpha.pixels()).zip(blurred.pixels())
  {
    pixel.0[3] =
      ((u16::from(original_alpha.0[0]) * u16::from(blurred_alpha.0[0]) + 127) / 255) as u8;
  }
}

fn erode_opaque_alpha(alpha: &image::GrayImage, radius: usize) -> image::GrayImage {
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
    if x < radius
      || y < radius
      || x.saturating_add(radius) >= width
      || y.saturating_add(radius) >= height
    {
      return image::Luma([0]);
    }
    let left = x - radius;
    let top = y - radius;
    let right = x + radius + 1;
    let bottom = y + radius + 1;
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

fn apply_alpha_outset(image: &mut image::RgbaImage, radius_px: f32) {
  if radius_px.abs() <= f32::EPSILON {
    return;
  }
  let mut alpha = image::GrayImage::from_fn(image.width(), image.height(), |x, y| {
    image::Luma([u8::from(image.get_pixel(x, y).0[3] > 0) * u8::MAX])
  });
  alpha = image::imageops::blur(&alpha, radius_px.abs());
  for (pixel, blurred) in image.pixels_mut().zip(alpha.pixels()) {
    pixel.0[3] = if radius_px > 0.0 {
      u8::from(blurred.0[0] > 0) * u8::MAX
    } else {
      u8::from(blurred.0[0] == u8::MAX) * u8::MAX
    };
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
  use image::{Rgba, RgbaImage};

  use super::{
    ImageEffect, ImageEffectBlendMode, ImageEffectColorResolver, ImageEffectContainer,
    ImageEffectContainerKind, ImageEffectFill, ImageEffectGradientKind, ImageEffectRelativeRect,
    ImageEffectSourceImages, ImageEffectSourceReference, ImageEffectSourceRequirements,
    ImageEffectTransform, ImageReflectionEffect, ResolvedEffectColor,
    apply_container_to_padded_image, apply_container_to_padded_image_with_sources, apply_to_image,
    container_output_bounds, from_effect_dag, from_effect_list, mso_brightness_contrast_component,
    rotate_container_with_shape, sample_fill, source_requirements, suppress_soft_edge,
  };
  use crate::model::RgbColor;
  use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
  use ooxmlsdk::units::DrawingmlPercentageValue;

  struct NoColorResolver;

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
    assert_eq!(eroded.get_pixel(4, 4).0[3], u8::MAX);
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
    assert!(edge.b > 250);
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
        direction_degrees: 0.0,
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
        start_opacity: 1.0,
        start_position: 0.0,
        end_opacity: 1.0,
        end_position: 1.0,
        fade_direction_degrees: 90.0,
        distance_px: 0.0,
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
        direction_degrees: 0.0,
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
