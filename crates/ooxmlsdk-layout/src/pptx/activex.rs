use std::collections::HashMap;

use olecfsdk::cfb::CompoundFile;
use olecfsdk::forms::{
  AlignedValue, CommandButtonControl, CountOfBytesWithCompressionFlag, FmFontEffects, FmString,
  LabelControl, MorphDataControl, OleColor, OleColorType, ScrollBarControl, SpinButtonControl,
  TextProps, VariousPropertiesBitfield,
};
use ooxmlsdk::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart;
use ooxmlsdk::parts::presentation_document::PresentationDocument;
use ooxmlsdk::parts::slide_part::SlidePart;
use ooxmlsdk::schemas::schemas_microsoft_com_office_2006_active_x::{
  ActiveXControlData, PersistenceValues,
};
use ooxmlsdk::sdk::{SdkPart, SdkType};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ActiveXControlKind {
  TextBox,
  ListBox,
  ComboBox,
  CheckBox,
  OptionButton,
  ToggleButton,
  CommandButton,
  Label,
  ScrollBar,
  SpinButton,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ActiveXControlState {
  pub(crate) kind: ActiveXControlKind,
  pub(crate) back_color: Option<[u8; 3]>,
  pub(crate) fore_color: Option<[u8; 3]>,
  pub(crate) opaque_background: bool,
  pub(crate) caption: Option<String>,
  pub(crate) value: Option<String>,
  pub(crate) font_family: Option<String>,
  pub(crate) font_size_pt: Option<f32>,
  pub(crate) bold: bool,
  pub(crate) italic: bool,
}

impl ActiveXControlState {
  pub(crate) fn preview_palette_override(&self) -> Option<[[u8; 3]; 2]> {
    self
      .opaque_background
      .then_some([self.back_color?, [255, 255, 255]])
  }
}

pub(crate) fn collect_slide_active_x_controls(
  package: &PresentationDocument,
  slide_part: &SlidePart,
) -> HashMap<String, ActiveXControlState> {
  slide_part
    .related_parts_of_type::<_, EmbeddedControlPersistencePart>(package)
    .filter_map(|related| {
      parse_control_part(package, related.part())
        .map(|state| (related.relationship_id().to_string(), state))
    })
    .collect()
}

fn parse_control_part(
  package: &PresentationDocument,
  part: &EmbeddedControlPersistencePart,
) -> Option<ActiveXControlState> {
  let control_data = ActiveXControlData::from_bytes(part.data_to_vec(package)?.as_slice()).ok()?;
  let kind = control_kind(&control_data.active_x_control_class_id)?;
  let binary = part
    .embedded_control_persistence_binary_data_parts(package)
    .next()?
    .data_to_vec(package)?;
  let contents = match control_data.persistence {
    PersistenceValues::PersistStorage => CompoundFile::from_bytes(&binary)
      .ok()?
      .stream("Contents")?
      .to_vec(),
    PersistenceValues::PersistStream | PersistenceValues::PersistStreamInit => binary,
    PersistenceValues::PersistPropertyBag => return None,
  };

  match kind {
    ActiveXControlKind::CommandButton => {
      state_from_command_button(kind, CommandButtonControl::from_bytes(&contents).ok()?)
    }
    ActiveXControlKind::Label => state_from_label(kind, LabelControl::from_bytes(&contents).ok()?),
    ActiveXControlKind::ScrollBar => {
      state_from_scroll_bar(kind, ScrollBarControl::from_bytes(&contents).ok()?)
    }
    ActiveXControlKind::SpinButton => {
      state_from_spin_button(kind, SpinButtonControl::from_bytes(&contents).ok()?)
    }
    _ => state_from_morph(kind, MorphDataControl::from_bytes(&contents).ok()?),
  }
}

fn control_kind(class_id: &str) -> Option<ActiveXControlKind> {
  match class_id
    .trim_matches(['{', '}'])
    .to_ascii_uppercase()
    .as_str()
  {
    "8BD21D10-EC42-11CE-9E0D-00AA006002F3" => Some(ActiveXControlKind::TextBox),
    "8BD21D20-EC42-11CE-9E0D-00AA006002F3" => Some(ActiveXControlKind::ListBox),
    "8BD21D30-EC42-11CE-9E0D-00AA006002F3" => Some(ActiveXControlKind::ComboBox),
    "8BD21D40-EC42-11CE-9E0D-00AA006002F3" => Some(ActiveXControlKind::CheckBox),
    "8BD21D50-EC42-11CE-9E0D-00AA006002F3" => Some(ActiveXControlKind::OptionButton),
    "8BD21D60-EC42-11CE-9E0D-00AA006002F3" => Some(ActiveXControlKind::ToggleButton),
    "D7053240-CE69-11CD-A777-00DD01143C57" => Some(ActiveXControlKind::CommandButton),
    "978C9E23-D4B0-11CE-BF2D-00AA003F40D0" => Some(ActiveXControlKind::Label),
    "DFD181E0-5E2F-11CE-A449-00AA004A803D" => Some(ActiveXControlKind::ScrollBar),
    "79176FB0-B7F2-11CE-97EF-00AA006D2776" => Some(ActiveXControlKind::SpinButton),
    _ => None,
  }
}

fn state_from_morph(
  kind: ActiveXControlKind,
  control: MorphDataControl,
) -> Option<ActiveXControlState> {
  let caption = decode_fm_string(
    control.data_block.caption.as_ref(),
    control.extra_data_block.caption.as_ref(),
  );
  let value = decode_fm_string(
    control.data_block.value.as_ref(),
    control.extra_data_block.value.as_ref(),
  );
  let various = control
    .data_block
    .various_property_bits
    .as_ref()
    .map(|value| value.value);
  Some(state(
    kind,
    control
      .data_block
      .back_color
      .as_ref()
      .map(|value| value.value),
    control
      .data_block
      .fore_color
      .as_ref()
      .map(|value| value.value),
    various,
    caption,
    value,
    Some(&control.text_props),
  ))
}

fn state_from_command_button(
  kind: ActiveXControlKind,
  control: CommandButtonControl,
) -> Option<ActiveXControlState> {
  let caption = decode_fm_string(
    control.data_block.caption.as_ref(),
    control.extra_data_block.caption.as_ref(),
  );
  Some(state(
    kind,
    control
      .data_block
      .back_color
      .as_ref()
      .map(|value| value.value),
    control
      .data_block
      .fore_color
      .as_ref()
      .map(|value| value.value),
    control
      .data_block
      .various_property_bits
      .as_ref()
      .map(|value| value.value),
    caption,
    None,
    Some(&control.text_props),
  ))
}

fn state_from_label(
  kind: ActiveXControlKind,
  control: LabelControl,
) -> Option<ActiveXControlState> {
  let caption = decode_fm_string(
    control.data_block.caption.as_ref(),
    control.extra_data_block.caption.as_ref(),
  );
  Some(state(
    kind,
    control
      .data_block
      .back_color
      .as_ref()
      .map(|value| value.value),
    control
      .data_block
      .fore_color
      .as_ref()
      .map(|value| value.value),
    control
      .data_block
      .various_property_bits
      .as_ref()
      .map(|value| value.value),
    caption,
    None,
    Some(&control.text_props),
  ))
}

fn state_from_scroll_bar(
  kind: ActiveXControlKind,
  control: ScrollBarControl,
) -> Option<ActiveXControlState> {
  let back_color = control
    .data_block
    .back_color
    .as_ref()
    .map(|value| value.value)
    .or_else(|| OleColor::from_raw(0x8000_000f).ok());
  Some(state(
    kind,
    back_color,
    control
      .data_block
      .fore_color
      .as_ref()
      .map(|value| value.value),
    control
      .data_block
      .various_property_bits
      .as_ref()
      .map(|value| value.value),
    None,
    None,
    None,
  ))
}

fn state_from_spin_button(
  kind: ActiveXControlKind,
  control: SpinButtonControl,
) -> Option<ActiveXControlState> {
  let back_color = control
    .data_block
    .back_color
    .as_ref()
    .map(|value| value.value)
    .or_else(|| OleColor::from_raw(0x8000_000f).ok());
  Some(state(
    kind,
    back_color,
    control
      .data_block
      .fore_color
      .as_ref()
      .map(|value| value.value),
    control
      .data_block
      .various_property_bits
      .as_ref()
      .map(|value| value.value),
    None,
    None,
    None,
  ))
}

fn state(
  kind: ActiveXControlKind,
  back_color: Option<OleColor>,
  fore_color: Option<OleColor>,
  various: Option<VariousPropertiesBitfield>,
  caption: Option<String>,
  value: Option<String>,
  text_props: Option<&TextProps>,
) -> ActiveXControlState {
  let effects = text_props
    .and_then(|text_props| text_props.data_block.font_effects.as_ref())
    .map(|value| value.value)
    .unwrap_or_else(FmFontEffects::empty);
  ActiveXControlState {
    kind,
    back_color: back_color.and_then(resolve_ole_color),
    fore_color: fore_color.and_then(resolve_ole_color),
    opaque_background: various
      .is_none_or(|value| value.contains(VariousPropertiesBitfield::BACK_STYLE)),
    caption,
    value,
    font_family: text_props.and_then(|text_props| {
      decode_fm_string(
        text_props.data_block.font_name.as_ref(),
        text_props.extra_data_block.font_name.as_ref(),
      )
    }),
    font_size_pt: text_props
      .and_then(|text_props| text_props.data_block.font_height.as_ref())
      .map(|value| value.value as f32 / 20.0),
    bold: effects.contains(FmFontEffects::BOLD)
      || text_props
        .and_then(|text_props| text_props.data_block.font_weight.as_ref())
        .is_some_and(|value| value.value >= 700),
    italic: effects.contains(FmFontEffects::ITALIC),
  }
}

fn decode_fm_string(
  descriptor: Option<&AlignedValue<CountOfBytesWithCompressionFlag>>,
  value: Option<&FmString>,
) -> Option<String> {
  value?.decode(descriptor?.value).ok()
}

fn resolve_ole_color(color: OleColor) -> Option<[u8; 3]> {
  if let Some((red, green, blue)) = color.rgb_components() {
    return Some([red, green, blue]);
  }
  match color.color_type {
    OleColorType::SystemPalette => match color.palette_index()? {
      0x05 => Some([255, 255, 255]),
      0x06 | 0x08 | 0x12 => Some([0, 0, 0]),
      0x0f => Some([240, 240, 240]),
      _ => None,
    },
    OleColorType::Default
    | OleColorType::PaletteEntry
    | OleColorType::RgbColor
    | OleColorType::Compatibility(_) => None,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn class_ids_are_case_and_brace_insensitive() {
    assert_eq!(
      control_kind("{8bd21d60-ec42-11ce-9e0d-00aa006002f3}"),
      Some(ActiveXControlKind::ToggleButton)
    );
    assert_eq!(
      control_kind("D7053240-CE69-11CD-A777-00DD01143C57"),
      Some(ActiveXControlKind::CommandButton)
    );
    assert_eq!(
      control_kind("{DFD181E0-5E2F-11CE-A449-00AA004A803D}"),
      Some(ActiveXControlKind::ScrollBar)
    );
    assert_eq!(
      control_kind("{79176FB0-B7F2-11CE-97EF-00AA006D2776}"),
      Some(ActiveXControlKind::SpinButton)
    );
    assert_eq!(control_kind("{00000000-0000-0000-0000-000000000000}"), None);
  }

  #[test]
  fn ole_system_colors_use_the_office_control_palette() {
    assert_eq!(
      resolve_ole_color(OleColor::from_raw(0x8000_000f).unwrap()),
      Some([240, 240, 240])
    );
    assert_eq!(
      resolve_ole_color(OleColor::from_raw(0x0211_2233).unwrap()),
      Some([0x33, 0x22, 0x11])
    );
  }
}
