use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use super::color::Color;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FillProperties {
  pub(crate) kind: FillKind,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum FillKind {
  None,
  Solid(Option<Color>),
  Group,
  Gradient(std::boxed::Box<a::GradientFill>),
  Blip(std::boxed::Box<a::BlipFill>),
  Pattern(std::boxed::Box<a::PatternFill>),
}

impl FillProperties {
  pub(crate) fn from_fill_style_choice(choice: &a::FillStyleListChoice) -> Self {
    match choice {
      a::FillStyleListChoice::NoFill(_) => Self {
        kind: FillKind::None,
      },
      a::FillStyleListChoice::SolidFill(fill) => Self {
        kind: FillKind::Solid(color_from_solid_fill(fill)),
      },
      a::FillStyleListChoice::GradientFill(fill) => Self {
        kind: FillKind::Gradient(fill.clone()),
      },
      a::FillStyleListChoice::BlipFill(fill) => Self {
        kind: FillKind::Blip(fill.clone()),
      },
      a::FillStyleListChoice::PatternFill(fill) => Self {
        kind: FillKind::Pattern(fill.clone()),
      },
      a::FillStyleListChoice::GroupFill => Self {
        kind: FillKind::Group,
      },
    }
  }

  pub(crate) fn from_background_fill_style_choice(
    choice: &a::BackgroundFillStyleListChoice,
  ) -> Self {
    match choice {
      a::BackgroundFillStyleListChoice::NoFill(_) => Self {
        kind: FillKind::None,
      },
      a::BackgroundFillStyleListChoice::SolidFill(fill) => Self {
        kind: FillKind::Solid(color_from_solid_fill(fill)),
      },
      a::BackgroundFillStyleListChoice::GradientFill(fill) => Self {
        kind: FillKind::Gradient(fill.clone()),
      },
      a::BackgroundFillStyleListChoice::BlipFill(fill) => Self {
        kind: FillKind::Blip(fill.clone()),
      },
      a::BackgroundFillStyleListChoice::PatternFill(fill) => Self {
        kind: FillKind::Pattern(fill.clone()),
      },
      a::BackgroundFillStyleListChoice::GroupFill => Self {
        kind: FillKind::Group,
      },
    }
  }
}

fn color_from_solid_fill(fill: &a::SolidFill) -> Option<Color> {
  Color::from_solid_fill_choice(fill.solid_fill_choice.as_ref()?)
}
