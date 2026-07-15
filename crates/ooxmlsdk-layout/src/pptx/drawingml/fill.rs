use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use super::color::Color;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FillProperties {
  pub(crate) kind: FillKind,
  pub(crate) placeholder_color: Option<Color>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum FillKind {
  None,
  /// Paint the portion of the slide background directly behind the shape.
  ///
  /// PresentationML `p:sp/@useBgFill` is not equivalent to `a:noFill`: it
  /// retains a distinct, page-relative fill mode through layout lowering.
  SlideBackground,
  Solid(Option<Color>),
  Group,
  Gradient(std::boxed::Box<a::GradientFill>),
  Blip(std::boxed::Box<a::BlipFill>),
  Pattern(std::boxed::Box<a::PatternFill>),
}

impl FillProperties {
  pub(crate) fn from_dml_fill_properties(properties: &a::FillProperties) -> Option<Self> {
    Some(match properties.fill_properties_choice.as_ref()? {
      a::FillPropertiesChoice::NoFill(_) => Self {
        kind: FillKind::None,
        placeholder_color: None,
      },
      a::FillPropertiesChoice::SolidFill(fill) => Self {
        kind: FillKind::Solid(color_from_solid_fill(fill)),
        placeholder_color: None,
      },
      a::FillPropertiesChoice::GradientFill(fill) => Self {
        kind: FillKind::Gradient(fill.clone()),
        placeholder_color: None,
      },
      a::FillPropertiesChoice::BlipFill(fill) => Self {
        kind: FillKind::Blip(fill.clone()),
        placeholder_color: None,
      },
      a::FillPropertiesChoice::PatternFill(fill) => Self {
        kind: FillKind::Pattern(fill.clone()),
        placeholder_color: None,
      },
      a::FillPropertiesChoice::GroupFill => Self {
        kind: FillKind::Group,
        placeholder_color: None,
      },
    })
  }

  pub(crate) fn from_fill_style_choice(choice: &a::FillStyleListChoice) -> Self {
    match choice {
      a::FillStyleListChoice::NoFill(_) => Self {
        kind: FillKind::None,
        placeholder_color: None,
      },
      a::FillStyleListChoice::SolidFill(fill) => Self {
        kind: FillKind::Solid(color_from_solid_fill(fill)),
        placeholder_color: None,
      },
      a::FillStyleListChoice::GradientFill(fill) => Self {
        kind: FillKind::Gradient(fill.clone()),
        placeholder_color: None,
      },
      a::FillStyleListChoice::BlipFill(fill) => Self {
        kind: FillKind::Blip(fill.clone()),
        placeholder_color: None,
      },
      a::FillStyleListChoice::PatternFill(fill) => Self {
        kind: FillKind::Pattern(fill.clone()),
        placeholder_color: None,
      },
      a::FillStyleListChoice::GroupFill => Self {
        kind: FillKind::Group,
        placeholder_color: None,
      },
    }
  }

  pub(crate) fn from_background_fill_style_choice(
    choice: &a::BackgroundFillStyleListChoice,
  ) -> Self {
    match choice {
      a::BackgroundFillStyleListChoice::NoFill(_) => Self {
        kind: FillKind::None,
        placeholder_color: None,
      },
      a::BackgroundFillStyleListChoice::SolidFill(fill) => Self {
        kind: FillKind::Solid(color_from_solid_fill(fill)),
        placeholder_color: None,
      },
      a::BackgroundFillStyleListChoice::GradientFill(fill) => Self {
        kind: FillKind::Gradient(fill.clone()),
        placeholder_color: None,
      },
      a::BackgroundFillStyleListChoice::BlipFill(fill) => Self {
        kind: FillKind::Blip(fill.clone()),
        placeholder_color: None,
      },
      a::BackgroundFillStyleListChoice::PatternFill(fill) => Self {
        kind: FillKind::Pattern(fill.clone()),
        placeholder_color: None,
      },
      a::BackgroundFillStyleListChoice::GroupFill => Self {
        kind: FillKind::Group,
        placeholder_color: None,
      },
    }
  }

  pub(crate) fn from_table_cell_properties_choice(choice: &a::TableCellPropertiesChoice) -> Self {
    match choice {
      a::TableCellPropertiesChoice::NoFill(_) => Self {
        kind: FillKind::None,
        placeholder_color: None,
      },
      a::TableCellPropertiesChoice::SolidFill(fill) => Self {
        kind: FillKind::Solid(color_from_solid_fill(fill)),
        placeholder_color: None,
      },
      a::TableCellPropertiesChoice::GradientFill(fill) => Self {
        kind: FillKind::Gradient(fill.clone()),
        placeholder_color: None,
      },
      a::TableCellPropertiesChoice::BlipFill(fill) => Self {
        kind: FillKind::Blip(fill.clone()),
        placeholder_color: None,
      },
      a::TableCellPropertiesChoice::PatternFill(fill) => Self {
        kind: FillKind::Pattern(fill.clone()),
        placeholder_color: None,
      },
      a::TableCellPropertiesChoice::GroupFill => Self {
        kind: FillKind::Group,
        placeholder_color: None,
      },
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
