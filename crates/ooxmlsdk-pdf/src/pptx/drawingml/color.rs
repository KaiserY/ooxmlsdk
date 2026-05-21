use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use ooxmlsdk::schemas::schemas_openxmlformats_org_presentationml_2006_main as p;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Color {
  RgbHex(String),
  Scheme(SchemeColor),
  Preset(PresetColor),
  System(SystemColor),
}

impl Color {
  pub(crate) fn from_solid_fill_choice(choice: &a::SolidFillChoice) -> Option<Self> {
    match choice {
      a::SolidFillChoice::RgbColorModelHex(color) => Some(Self::RgbHex(color.val.clone())),
      a::SolidFillChoice::SchemeColor(color) => {
        Some(Self::Scheme(SchemeColor { value: color.val }))
      }
      a::SolidFillChoice::PresetColor(color) => {
        Some(Self::Preset(PresetColor { value: color.val }))
      }
      a::SolidFillChoice::SystemColor(color) => Some(Self::System(SystemColor {
        value: color.val,
        last_color: color.last_color.clone(),
      })),
      a::SolidFillChoice::RgbColorModelPercentage(_) | a::SolidFillChoice::HslColor(_) => None,
    }
  }

  pub(crate) fn from_background_style_reference_choice(
    choice: &p::BackgroundStyleReferenceChoice,
  ) -> Option<Self> {
    match choice {
      p::BackgroundStyleReferenceChoice::RgbColorModelHex(color) => {
        Some(Self::RgbHex(color.val.clone()))
      }
      p::BackgroundStyleReferenceChoice::SchemeColor(color) => {
        Some(Self::Scheme(SchemeColor { value: color.val }))
      }
      p::BackgroundStyleReferenceChoice::PresetColor(color) => {
        Some(Self::Preset(PresetColor { value: color.val }))
      }
      p::BackgroundStyleReferenceChoice::SystemColor(color) => Some(Self::System(SystemColor {
        value: color.val,
        last_color: color.last_color.clone(),
      })),
      p::BackgroundStyleReferenceChoice::RgbColorModelPercentage(_)
      | p::BackgroundStyleReferenceChoice::HslColor(_) => None,
    }
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SchemeColor {
  pub(crate) value: a::SchemeColorValues,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PresetColor {
  pub(crate) value: a::PresetColorValues,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SystemColor {
  pub(crate) value: a::SystemColorValues,
  pub(crate) last_color: Option<String>,
}
