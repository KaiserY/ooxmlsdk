use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use super::color::Color;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct LineProperties {
  pub(crate) fill: LineFill,
  pub(crate) width_emu: Option<i64>,
  pub(crate) placeholder_color: Option<Color>,
  pub(crate) source_outline: Option<Box<a::Outline>>,
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

    // A line can override only its decorations or stroke style while keeping
    // the theme's fill and width (ECMA-376 Part 1, 20.1.2.2.24/20.1.8.57).
    if fill == LineFill::Unspecified
      && outline.width.is_none()
      && outline.cap_type.is_none()
      && outline.compound_line_type.is_none()
      && outline.alignment.is_none()
      && outline.outline_choice2.is_none()
      && outline.outline_choice3.is_none()
      && outline.head_end.is_none()
      && outline.tail_end.is_none()
      && outline.line_properties_extension_list.is_none()
    {
      None
    } else {
      Some(Self {
        fill,
        width_emu: outline.width.map(i64::from),
        placeholder_color: None,
        source_outline: Some(Box::new(outline.clone())),
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
