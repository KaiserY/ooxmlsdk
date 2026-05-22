use ooxmlsdk::schemas::{
  schemas_openxmlformats_org_drawingml_2006_main as a,
  schemas_openxmlformats_org_presentationml_2006_main as p,
};

use super::color::Color;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct EffectProperties {
  pub(crate) shadow: Option<EffectShadowProperties>,
  pub(crate) glow: Option<EffectGlowProperties>,
  pub(crate) soft_edge: Option<EffectSoftEdgeProperties>,
  pub(crate) effect_names: Vec<&'static str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct EffectShadowProperties {
  pub(crate) kind: EffectShadowKind,
  pub(crate) distance_emu: Option<i64>,
  pub(crate) direction: Option<i32>,
  pub(crate) blur_radius_emu: Option<i64>,
  pub(crate) scale_x: Option<i32>,
  pub(crate) scale_y: Option<i32>,
  pub(crate) color: Option<Color>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum EffectShadowKind {
  Inner,
  Outer,
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
    match choice {
      p::ShapePropertiesChoice3::EffectList(list) => Self::from_effect_list(list),
      p::ShapePropertiesChoice3::EffectDag(dag) => Self::from_effect_dag(dag),
    }
  }

  pub(crate) fn from_effect_style_choice(choice: &a::EffectStyleChoice) -> Self {
    match choice {
      a::EffectStyleChoice::EffectList(list) => Self::from_effect_list(list),
      a::EffectStyleChoice::EffectDag(dag) => Self::from_effect_dag(dag),
    }
  }

  fn from_effect_list(list: &a::EffectList) -> Self {
    let mut properties = Self::default();
    if list.blur.is_some() {
      properties.push_effect_name("blur");
    }
    if list.fill_overlay.is_some() {
      properties.push_effect_name("fillOverlay");
    }
    if let Some(glow) = &list.glow {
      properties.glow = Some(EffectGlowProperties::from_dml(glow));
      properties.push_effect_name("glow");
    }
    if let Some(shadow) = &list.inner_shadow {
      properties.shadow = Some(EffectShadowProperties::from_inner_shadow(shadow));
      properties.push_effect_name("innerShdw");
    }
    if let Some(shadow) = &list.outer_shadow {
      properties.shadow = Some(EffectShadowProperties::from_outer_shadow(shadow));
      properties.push_effect_name("outerShdw");
    }
    if list.preset_shadow.is_some() {
      properties.push_effect_name("prstShdw");
    }
    if list.reflection.is_some() {
      properties.push_effect_name("reflection");
    }
    if let Some(soft_edge) = &list.soft_edge {
      properties.soft_edge = Some(EffectSoftEdgeProperties::from_dml(soft_edge));
      properties.push_effect_name("softEdge");
    }
    properties
  }

  fn from_effect_dag(dag: &a::EffectDag) -> Self {
    let mut properties = Self::default();
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
        properties.shadow = Some(EffectShadowProperties::from_inner_shadow(shadow));
        properties.push_effect_name("innerShdw");
      }
      a::EffectDagChoice::OuterShadow(shadow) => {
        properties.shadow = Some(EffectShadowProperties::from_outer_shadow(shadow));
        properties.push_effect_name("outerShdw");
      }
      a::EffectDagChoice::SoftEdge(soft_edge) => {
        properties.soft_edge = Some(EffectSoftEdgeProperties::from_dml(soft_edge));
        properties.push_effect_name("softEdge");
      }
      a::EffectDagChoice::Blur(_) => properties.push_effect_name("blur"),
      a::EffectDagChoice::FillOverlay(_) => properties.push_effect_name("fillOverlay"),
      a::EffectDagChoice::PresetShadow(_) => properties.push_effect_name("prstShdw"),
      a::EffectDagChoice::Reflection(_) => properties.push_effect_name("reflection"),
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
        properties.shadow = Some(EffectShadowProperties::from_inner_shadow(shadow));
        properties.push_effect_name("innerShdw");
      }
      a::EffectContainerChoice::OuterShadow(shadow) => {
        properties.shadow = Some(EffectShadowProperties::from_outer_shadow(shadow));
        properties.push_effect_name("outerShdw");
      }
      a::EffectContainerChoice::SoftEdge(soft_edge) => {
        properties.soft_edge = Some(EffectSoftEdgeProperties::from_dml(soft_edge));
        properties.push_effect_name("softEdge");
      }
      a::EffectContainerChoice::Blur(_) => properties.push_effect_name("blur"),
      a::EffectContainerChoice::FillOverlay(_) => properties.push_effect_name("fillOverlay"),
      a::EffectContainerChoice::PresetShadow(_) => properties.push_effect_name("prstShdw"),
      a::EffectContainerChoice::Reflection(_) => properties.push_effect_name("reflection"),
      _ => properties.push_effect_name("effect"),
    }
    properties
  }

  pub(crate) fn merge_from(&mut self, source: &Self) {
    if source.shadow.is_some() {
      self.shadow = source.shadow.clone();
    }
    if source.glow.is_some() {
      self.glow = source.glow.clone();
    }
    if source.soft_edge.is_some() {
      self.soft_edge = source.soft_edge.clone();
    }
    if !source.effect_names.is_empty() {
      self.effect_names = source.effect_names.clone();
    }
  }

  pub(crate) fn has_effect(&self) -> bool {
    self.shadow.as_ref().is_some_and(|shadow| shadow.is_used())
      || self.glow.as_ref().is_some_and(|glow| glow.is_used())
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

impl EffectShadowProperties {
  fn is_used(&self) -> bool {
    matches!(self.kind, EffectShadowKind::Inner | EffectShadowKind::Outer)
      || self.distance_emu.is_some()
      || self.direction.is_some()
      || self.blur_radius_emu.is_some()
      || self.scale_x.is_some()
      || self.scale_y.is_some()
      || self.color.is_some()
  }

  fn from_inner_shadow(source: &a::InnerShadow) -> Self {
    Self {
      kind: EffectShadowKind::Inner,
      distance_emu: source.distance.map(|value| value.to_emu()),
      direction: source.direction,
      blur_radius_emu: source.blur_radius.map(|value| value.to_emu()),
      scale_x: None,
      scale_y: None,
      color: source
        .inner_shadow_choice
        .as_ref()
        .and_then(Color::from_inner_shadow_choice),
    }
  }

  fn from_outer_shadow(source: &a::OuterShadow) -> Self {
    Self {
      kind: EffectShadowKind::Outer,
      distance_emu: source.distance.map(|value| value.to_emu()),
      direction: source.direction,
      blur_radius_emu: source.blur_radius.map(|value| value.to_emu()),
      scale_x: source
        .horizontal_ratio
        .map(|value| value.as_drawingml_percent()),
      scale_y: source
        .vertical_ratio
        .map(|value| value.as_drawingml_percent()),
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
