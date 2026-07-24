use ooxmlsdk::schemas::{
  schemas_openxmlformats_org_drawingml_2006_main as a,
  schemas_openxmlformats_org_presentationml_2006_main as p,
};

use super::color::Color;
use super::fill::FillProperties;

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct EffectProperties {
  /// Direct `effectLst`/`effectDag` presence replaces the referenced effect
  /// style even when the direct list is empty.
  pub(crate) clears_inherited: bool,
  /// Preserve the typed source container so ordered effects and named DAG
  /// references are not collapsed while the fixed-output lowering grows.
  pub(crate) effect_list: Option<a::EffectList>,
  pub(crate) effect_dag: Option<a::EffectDag>,
  pub(crate) inner_shadow: Option<EffectShadowProperties>,
  pub(crate) outer_shadow: Option<EffectShadowProperties>,
  pub(crate) blur: Option<EffectBlurProperties>,
  pub(crate) fill_overlay: Option<EffectFillOverlayProperties>,
  pub(crate) glow: Option<EffectGlowProperties>,
  pub(crate) reflection: Option<EffectReflectionProperties>,
  pub(crate) preset_shadow: Option<EffectPresetShadowProperties>,
  pub(crate) soft_edge: Option<EffectSoftEdgeProperties>,
  pub(crate) effect_names: Vec<&'static str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct EffectBlurProperties {
  pub(crate) radius_emu: Option<i64>,
  pub(crate) grow: Option<bool>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct EffectFillOverlayProperties {
  pub(crate) blend_mode: a::BlendModeValues,
  pub(crate) fill: Option<FillProperties>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct EffectReflectionProperties {
  pub(crate) blur_radius_emu: Option<i64>,
  pub(crate) start_opacity: Option<f32>,
  pub(crate) start_position: Option<f32>,
  pub(crate) end_opacity: Option<f32>,
  pub(crate) end_position: Option<f32>,
  pub(crate) distance_emu: Option<i64>,
  pub(crate) direction: Option<i32>,
  pub(crate) fade_direction: Option<i32>,
  pub(crate) scale_x: Option<i32>,
  pub(crate) scale_y: Option<i32>,
  pub(crate) horizontal_skew: Option<i32>,
  pub(crate) vertical_skew: Option<i32>,
  pub(crate) alignment: Option<a::RectangleAlignmentValues>,
  pub(crate) rotate_with_shape: Option<bool>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct EffectPresetShadowProperties {
  pub(crate) preset: a::PresetShadowValues,
  pub(crate) distance_emu: Option<i64>,
  pub(crate) direction: Option<i32>,
  pub(crate) color: Option<Color>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct EffectShadowProperties {
  pub(crate) distance_emu: Option<i64>,
  pub(crate) direction: Option<i32>,
  pub(crate) blur_radius_emu: Option<i64>,
  pub(crate) scale_x: Option<i32>,
  pub(crate) scale_y: Option<i32>,
  pub(crate) horizontal_skew: Option<i32>,
  pub(crate) vertical_skew: Option<i32>,
  pub(crate) alignment: Option<a::RectangleAlignmentValues>,
  pub(crate) rotate_with_shape: Option<bool>,
  pub(crate) color: Option<Color>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct EffectGlowProperties {
  pub(crate) radius_emu: Option<i64>,
  pub(crate) color: Option<Color>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct EffectSoftEdgeProperties {
  pub(crate) radius_emu: i64,
}

impl EffectProperties {
  pub(crate) fn from_effect_style(style: &a::EffectStyle) -> Self {
    let mut properties = style
      .effect_style_choice
      .as_ref()
      .map(Self::from_effect_style_choice)
      .unwrap_or_default();
    if style.scene3_d_type.is_some() {
      properties.push_effect_name("scene3d");
    }
    if style.shape3_d_type.is_some() {
      properties.push_effect_name("sp3d");
    }
    properties
  }

  pub(crate) fn from_pml_shape_properties_choice(choice: &p::ShapePropertiesChoice3) -> Self {
    let mut properties = match choice {
      p::ShapePropertiesChoice3::EffectList(list) => Self::from_effect_list(list),
      p::ShapePropertiesChoice3::EffectDag(dag) => Self::from_effect_dag(dag),
    };
    properties.clears_inherited = true;
    properties
  }

  pub(crate) fn from_effect_style_choice(choice: &a::EffectStyleChoice) -> Self {
    match choice {
      a::EffectStyleChoice::EffectList(list) => Self::from_effect_list(list),
      a::EffectStyleChoice::EffectDag(dag) => Self::from_effect_dag(dag),
    }
  }

  fn from_effect_list(list: &a::EffectList) -> Self {
    let mut properties = Self {
      effect_list: Some(list.clone()),
      ..Self::default()
    };
    if let Some(blur) = &list.blur {
      properties.blur = Some(EffectBlurProperties::from_dml(blur));
      properties.push_effect_name("blur");
    }
    if let Some(fill_overlay) = &list.fill_overlay {
      properties.fill_overlay = Some(EffectFillOverlayProperties::from_dml(fill_overlay));
      properties.push_effect_name("fillOverlay");
    }
    if let Some(glow) = &list.glow {
      properties.glow = Some(EffectGlowProperties::from_dml(glow));
      properties.push_effect_name("glow");
    }
    if let Some(shadow) = &list.inner_shadow {
      properties.inner_shadow = Some(EffectShadowProperties::from_inner_shadow(shadow));
      properties.push_effect_name("innerShdw");
    }
    if let Some(shadow) = &list.outer_shadow {
      properties.outer_shadow = Some(EffectShadowProperties::from_outer_shadow(shadow));
      properties.push_effect_name("outerShdw");
    }
    if let Some(preset_shadow) = &list.preset_shadow {
      properties.preset_shadow = Some(EffectPresetShadowProperties::from_dml(preset_shadow));
      properties.push_effect_name("prstShdw");
    }
    if let Some(reflection) = &list.reflection {
      properties.reflection = Some(EffectReflectionProperties::from_dml(reflection));
      properties.push_effect_name("reflection");
    }
    if let Some(soft_edge) = &list.soft_edge {
      properties.soft_edge = Some(EffectSoftEdgeProperties::from_dml(soft_edge));
      properties.push_effect_name("softEdge");
    }
    properties
  }

  fn from_effect_dag(dag: &a::EffectDag) -> Self {
    let mut properties = Self {
      effect_dag: Some(dag.clone()),
      ..Self::default()
    };
    properties.push_effect_name("effectDag");
    for choice in &dag.effect_dag_choice {
      properties.merge_from(&Self::from_effect_dag_choice(choice));
    }
    properties
  }

  fn from_effect_dag_choice(choice: &a::EffectDagChoice) -> Self {
    let mut properties = Self::default();
    match choice {
      a::EffectDagChoice::EffectContainer(container) => {
        properties.push_effect_name("cont");
        for child in &container.effect_container_choice {
          properties.merge_from(&Self::from_effect_container_choice(child));
        }
      }
      a::EffectDagChoice::Glow(glow) => {
        properties.glow = Some(EffectGlowProperties::from_dml(glow));
        properties.push_effect_name("glow");
      }
      a::EffectDagChoice::InnerShadow(shadow) => {
        properties.inner_shadow = Some(EffectShadowProperties::from_inner_shadow(shadow));
        properties.push_effect_name("innerShdw");
      }
      a::EffectDagChoice::OuterShadow(shadow) => {
        properties.outer_shadow = Some(EffectShadowProperties::from_outer_shadow(shadow));
        properties.push_effect_name("outerShdw");
      }
      a::EffectDagChoice::SoftEdge(soft_edge) => {
        properties.soft_edge = Some(EffectSoftEdgeProperties::from_dml(soft_edge));
        properties.push_effect_name("softEdge");
      }
      a::EffectDagChoice::Blur(blur) => {
        properties.blur = Some(EffectBlurProperties::from_dml(blur));
        properties.push_effect_name("blur");
      }
      a::EffectDagChoice::FillOverlay(fill_overlay) => {
        properties.fill_overlay = Some(EffectFillOverlayProperties::from_dml(fill_overlay));
        properties.push_effect_name("fillOverlay");
      }
      a::EffectDagChoice::PresetShadow(preset_shadow) => {
        properties.preset_shadow = Some(EffectPresetShadowProperties::from_dml(preset_shadow));
        properties.push_effect_name("prstShdw");
      }
      a::EffectDagChoice::Reflection(reflection) => {
        properties.reflection = Some(EffectReflectionProperties::from_dml(reflection));
        properties.push_effect_name("reflection");
      }
      _ => properties.push_effect_name("effect"),
    }
    properties
  }

  fn from_effect_container_choice(choice: &a::EffectContainerChoice) -> Self {
    let mut properties = Self::default();
    match choice {
      a::EffectContainerChoice::EffectContainer(container) => {
        properties.push_effect_name("cont");
        for child in &container.effect_container_choice {
          properties.merge_from(&Self::from_effect_container_choice(child));
        }
      }
      a::EffectContainerChoice::Glow(glow) => {
        properties.glow = Some(EffectGlowProperties::from_dml(glow));
        properties.push_effect_name("glow");
      }
      a::EffectContainerChoice::InnerShadow(shadow) => {
        properties.inner_shadow = Some(EffectShadowProperties::from_inner_shadow(shadow));
        properties.push_effect_name("innerShdw");
      }
      a::EffectContainerChoice::OuterShadow(shadow) => {
        properties.outer_shadow = Some(EffectShadowProperties::from_outer_shadow(shadow));
        properties.push_effect_name("outerShdw");
      }
      a::EffectContainerChoice::SoftEdge(soft_edge) => {
        properties.soft_edge = Some(EffectSoftEdgeProperties::from_dml(soft_edge));
        properties.push_effect_name("softEdge");
      }
      a::EffectContainerChoice::Blur(blur) => {
        properties.blur = Some(EffectBlurProperties::from_dml(blur));
        properties.push_effect_name("blur");
      }
      a::EffectContainerChoice::FillOverlay(fill_overlay) => {
        properties.fill_overlay = Some(EffectFillOverlayProperties::from_dml(fill_overlay));
        properties.push_effect_name("fillOverlay");
      }
      a::EffectContainerChoice::PresetShadow(preset_shadow) => {
        properties.preset_shadow = Some(EffectPresetShadowProperties::from_dml(preset_shadow));
        properties.push_effect_name("prstShdw");
      }
      a::EffectContainerChoice::Reflection(reflection) => {
        properties.reflection = Some(EffectReflectionProperties::from_dml(reflection));
        properties.push_effect_name("reflection");
      }
      _ => properties.push_effect_name("effect"),
    }
    properties
  }

  pub(crate) fn merge_from(&mut self, source: &Self) {
    if source.effect_list.is_some() || source.effect_dag.is_some() {
      self.effect_list = source.effect_list.clone();
      self.effect_dag = source.effect_dag.clone();
    }
    if source.inner_shadow.is_some() {
      self.inner_shadow = source.inner_shadow.clone();
    }
    if source.outer_shadow.is_some() {
      self.outer_shadow = source.outer_shadow.clone();
    }
    if source.blur.is_some() {
      self.blur = source.blur.clone();
    }
    if source.fill_overlay.is_some() {
      self.fill_overlay = source.fill_overlay.clone();
    }
    if source.glow.is_some() {
      self.glow = source.glow.clone();
    }
    if source.reflection.is_some() {
      self.reflection = source.reflection.clone();
    }
    if source.preset_shadow.is_some() {
      self.preset_shadow = source.preset_shadow.clone();
    }
    if source.soft_edge.is_some() {
      self.soft_edge = source.soft_edge.clone();
    }
    if !source.effect_names.is_empty() {
      self.effect_names = source.effect_names.clone();
    }
  }

  pub(crate) fn has_effect(&self) -> bool {
    self
      .inner_shadow
      .as_ref()
      .is_some_and(|shadow| shadow.is_used())
      || self
        .outer_shadow
        .as_ref()
        .is_some_and(|shadow| shadow.is_used())
      || self.blur.is_some()
      || self.fill_overlay.is_some()
      || self.glow.as_ref().is_some_and(|glow| glow.is_used())
      || self.reflection.is_some()
      || self.preset_shadow.is_some()
      || self
        .soft_edge
        .as_ref()
        .is_some_and(|soft_edge| soft_edge.is_used())
      || !self.effect_names.is_empty()
  }

  fn push_effect_name(&mut self, name: &'static str) {
    if !self.effect_names.contains(&name) {
      self.effect_names.push(name);
    }
  }
}

impl EffectBlurProperties {
  fn from_dml(source: &a::Blur) -> Self {
    Self {
      radius_emu: source.radius.map(|value| value.to_emu()),
      grow: source.grow.map(|value| value.as_bool()),
    }
  }
}

impl EffectFillOverlayProperties {
  fn from_dml(source: &a::FillOverlay) -> Self {
    Self {
      blend_mode: source.blend,
      fill: source
        .fill_overlay_choice
        .as_ref()
        .map(FillProperties::from_fill_overlay_choice),
    }
  }
}

impl EffectReflectionProperties {
  fn from_dml(source: &a::Reflection) -> Self {
    Self {
      blur_radius_emu: source.blur_radius.map(|value| value.to_emu()),
      start_opacity: source.start_opacity.map(|value| value.as_ratio() as f32),
      start_position: source.start_position.map(|value| value.as_ratio() as f32),
      end_opacity: source.end_alpha.map(|value| value.as_ratio() as f32),
      end_position: source.end_position.map(|value| value.as_ratio() as f32),
      distance_emu: source.distance.map(|value| value.to_emu()),
      direction: source.direction,
      fade_direction: source.fade_direction,
      scale_x: source
        .horizontal_ratio
        .map(|value| value.as_drawingml_percent()),
      scale_y: source
        .vertical_ratio
        .map(|value| value.as_drawingml_percent()),
      horizontal_skew: source.horizontal_skew,
      vertical_skew: source.vertical_skew,
      alignment: source.alignment,
      rotate_with_shape: source.rotate_with_shape.map(|value| value.as_bool()),
    }
  }
}

impl EffectPresetShadowProperties {
  fn from_dml(source: &a::PresetShadow) -> Self {
    Self {
      preset: source.preset,
      distance_emu: source.distance.map(|value| value.to_emu()),
      direction: source.direction,
      color: source
        .preset_shadow_choice
        .as_ref()
        .and_then(Color::from_preset_shadow_choice),
    }
  }
}

impl EffectShadowProperties {
  fn is_used(&self) -> bool {
    self.distance_emu.is_some()
      || self.direction.is_some()
      || self.blur_radius_emu.is_some()
      || self.scale_x.is_some()
      || self.scale_y.is_some()
      || self.horizontal_skew.is_some()
      || self.vertical_skew.is_some()
      || self.rotate_with_shape.is_some()
      || self.color.is_some()
  }

  fn from_inner_shadow(source: &a::InnerShadow) -> Self {
    Self {
      distance_emu: source.distance.map(|value| value.to_emu()),
      direction: source.direction,
      blur_radius_emu: source.blur_radius.map(|value| value.to_emu()),
      scale_x: None,
      scale_y: None,
      horizontal_skew: None,
      vertical_skew: None,
      alignment: None,
      rotate_with_shape: None,
      color: source
        .inner_shadow_choice
        .as_ref()
        .and_then(Color::from_inner_shadow_choice),
    }
  }

  fn from_outer_shadow(source: &a::OuterShadow) -> Self {
    Self {
      distance_emu: source.distance.map(|value| value.to_emu()),
      direction: source.direction,
      blur_radius_emu: source.blur_radius.map(|value| value.to_emu()),
      scale_x: source
        .horizontal_ratio
        .map(|value| value.as_drawingml_percent()),
      scale_y: source
        .vertical_ratio
        .map(|value| value.as_drawingml_percent()),
      horizontal_skew: source.horizontal_skew,
      vertical_skew: source.vertical_skew,
      alignment: source.alignment,
      rotate_with_shape: source.rotate_with_shape.map(|value| value.as_bool()),
      color: source
        .outer_shadow_choice
        .as_ref()
        .and_then(Color::from_outer_shadow_choice),
    }
  }
}

impl EffectGlowProperties {
  fn is_used(&self) -> bool {
    self.radius_emu.is_some() || self.color.is_some()
  }

  fn from_dml(source: &a::Glow) -> Self {
    Self {
      radius_emu: source.radius.map(|value| value.to_emu()),
      color: source
        .glow_choice
        .as_ref()
        .and_then(Color::from_glow_choice),
    }
  }
}

impl EffectSoftEdgeProperties {
  fn is_used(&self) -> bool {
    self.radius_emu >= 0
  }

  fn from_dml(source: &a::SoftEdge) -> Self {
    Self {
      radius_emu: source.radius.to_emu(),
    }
  }
}
