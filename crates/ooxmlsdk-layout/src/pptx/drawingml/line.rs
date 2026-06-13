use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use super::color::Color;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct LineProperties {
  pub(crate) fill: LineFill,
  pub(crate) width_emu: Option<i64>,
  pub(crate) placeholder_color: Option<Color>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum LineFill {
  Unspecified,
  None,
  Solid(Option<Color>),
  Gradient(std::boxed::Box<a::GradientFill>),
  Pattern(std::boxed::Box<a::PatternFill>),
}

impl LineProperties {
  pub(crate) fn from_dml_outline(outline: &a::Outline) -> Option<Self> {
    let fill = match outline.outline_choice1.as_ref() {
      Some(a::OutlineChoice::NoFill(_)) => LineFill::None,
      Some(a::OutlineChoice::SolidFill(fill)) => LineFill::Solid(color_from_solid_fill(fill)),
      Some(a::OutlineChoice::GradientFill(fill)) => LineFill::Gradient(fill.clone()),
      Some(a::OutlineChoice::PatternFill(fill)) => LineFill::Pattern(fill.clone()),
      None => LineFill::Unspecified,
    };

    if fill == LineFill::Unspecified && outline.width.is_none() {
      None
    } else {
      Some(Self {
        fill,
        width_emu: outline.width.map(i64::from),
        placeholder_color: None,
      })
    }
  }

  pub(crate) fn with_placeholder_color(mut self, placeholder_color: Option<Color>) -> Self {
    self.placeholder_color = placeholder_color;
    self
  }
}

fn color_from_solid_fill(fill: &a::SolidFill) -> Option<Color> {
  Color::from_solid_fill_choice(fill.solid_fill_choice.as_ref()?)
}
