use ooxmlsdk::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main as x14;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SheetConditionCatalog {
  pub(crate) conditional_formats: Vec<ConditionalFormatModel>,
  pub(crate) data_validations: Vec<DataValidationModel>,
  pub(crate) validations_disable_prompts: bool,
  pub(crate) validation_window: Option<(u32, u32)>,
  pub(crate) extension_conditions: SheetConditionExtensionCatalog,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ConditionalFormatModel {
  pub(crate) sequence_of_references: Vec<String>,
  pub(crate) pivot: bool,
  pub(crate) rules: Vec<ConditionalFormatRuleModel>,
  pub(crate) has_extensions: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ConditionalFormatRuleModel {
  pub(crate) rule_type: x::ConditionalFormatValues,
  pub(crate) format_id: Option<u32>,
  pub(crate) priority: i32,
  pub(crate) stop_if_true: bool,
  pub(crate) operator: Option<x::ConditionalFormattingOperatorValues>,
  pub(crate) text: Option<String>,
  pub(crate) time_period: Option<x::TimePeriodValues>,
  pub(crate) rank: Option<u32>,
  pub(crate) std_dev: Option<i32>,
  pub(crate) above_average: bool,
  pub(crate) percent: bool,
  pub(crate) bottom: bool,
  pub(crate) equal_average: bool,
  pub(crate) formulas: Vec<String>,
  pub(crate) has_color_scale: bool,
  pub(crate) has_data_bar: bool,
  pub(crate) icon_set: Option<IconSetModel>,
  pub(crate) has_extensions: bool,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum IconSetType {
  ThreeArrows,
  ThreeArrowsGray,
  ThreeFlags,
  ThreeTrafficLights1,
  ThreeTrafficLights2,
  ThreeSigns,
  ThreeSymbols,
  ThreeSymbols2,
  FourArrows,
  FourArrowsGray,
  FourRedToBlack,
  FourRating,
  FourTrafficLights,
  FiveArrows,
  FiveArrowsGray,
  FiveRating,
  FiveQuarters,
  ThreeStars,
  ThreeTriangles,
  FiveBoxes,
  NoIcons,
}

impl IconSetType {
  pub(crate) const fn icon_count(self) -> usize {
    match self {
      Self::FourArrows
      | Self::FourArrowsGray
      | Self::FourRedToBlack
      | Self::FourRating
      | Self::FourTrafficLights => 4,
      Self::FiveArrows
      | Self::FiveArrowsGray
      | Self::FiveRating
      | Self::FiveQuarters
      | Self::FiveBoxes => 5,
      Self::NoIcons => 0,
      _ => 3,
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum IconSetThresholdType {
  Number,
  Percent,
  Maximum,
  Minimum,
  Formula,
  Percentile,
  AutomaticMinimum,
  AutomaticMaximum,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct IconSetThresholdModel {
  pub(crate) threshold_type: IconSetThresholdType,
  pub(crate) value: Option<String>,
  pub(crate) greater_than_or_equal: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct IconSetCustomIcon {
  pub(crate) icon_set: IconSetType,
  pub(crate) icon_index: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct IconSetModel {
  pub(crate) icon_set: IconSetType,
  pub(crate) show_value: bool,
  pub(crate) reverse: bool,
  pub(crate) thresholds: Vec<IconSetThresholdModel>,
  pub(crate) custom_icons: Option<Vec<Option<IconSetCustomIcon>>>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DataValidationModel {
  pub(crate) validation_type: Option<x::DataValidationValues>,
  pub(crate) error_style: Option<x::DataValidationErrorStyleValues>,
  pub(crate) ime_mode: Option<x::DataValidationImeModeValues>,
  pub(crate) operator: Option<x::DataValidationOperatorValues>,
  pub(crate) allow_blank: bool,
  pub(crate) no_drop_down: bool,
  pub(crate) show_input_message: bool,
  pub(crate) show_error_message: bool,
  pub(crate) error_title: Option<String>,
  pub(crate) error: Option<String>,
  pub(crate) prompt_title: Option<String>,
  pub(crate) prompt: Option<String>,
  pub(crate) sequence_of_references: Vec<String>,
  pub(crate) list: Option<String>,
  pub(crate) formula1: Option<String>,
  pub(crate) formula2: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SheetConditionExtensionCatalog {
  pub(crate) conditional_formats: Vec<ExtendedConditionalFormatModel>,
  pub(crate) data_validations: Vec<ExtendedDataValidationModel>,
  pub(crate) sparkline_groups: Vec<SparklineGroupModel>,
  pub(crate) ignored_errors: Vec<IgnoredErrorsModel>,
  pub(crate) slicer_refs: usize,
  pub(crate) protected_ranges: usize,
  pub(crate) web_extensions: usize,
  pub(crate) timeline_refs: usize,
  pub(crate) unknown_extensions: usize,
  pub(crate) uri_text_len: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ExtendedConditionalFormatModel {
  pub(crate) sequence_of_references: Vec<String>,
  pub(crate) pivot: bool,
  pub(crate) rules: Vec<ExtendedConditionalFormatRuleModel>,
  pub(crate) has_extensions: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ExtendedConditionalFormatRuleModel {
  pub(crate) rule_type: Option<x::ConditionalFormatValues>,
  pub(crate) priority: Option<i32>,
  pub(crate) stop_if_true: bool,
  pub(crate) boolean_flags: usize,
  pub(crate) operator: Option<x::ConditionalFormattingOperatorValues>,
  pub(crate) text: Option<String>,
  pub(crate) time_period: Option<x::TimePeriodValues>,
  pub(crate) rank: Option<u32>,
  pub(crate) std_dev: Option<i32>,
  pub(crate) id: Option<String>,
  pub(crate) formulas: Vec<String>,
  pub(crate) has_color_scale: bool,
  pub(crate) has_data_bar: bool,
  pub(crate) icon_set: Option<IconSetModel>,
  pub(crate) has_differential_format: bool,
  pub(crate) has_extensions: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ExtendedDataValidationModel {
  pub(crate) validation_type: Option<x::DataValidationValues>,
  pub(crate) error_style: Option<x::DataValidationErrorStyleValues>,
  pub(crate) ime_mode: Option<x::DataValidationImeModeValues>,
  pub(crate) operator: Option<x::DataValidationOperatorValues>,
  pub(crate) allow_blank: bool,
  pub(crate) no_drop_down: bool,
  pub(crate) show_input_message: bool,
  pub(crate) show_error_message: bool,
  pub(crate) error_title: Option<String>,
  pub(crate) error: Option<String>,
  pub(crate) prompt_title: Option<String>,
  pub(crate) prompt: Option<String>,
  pub(crate) sequence_of_references: Vec<String>,
  pub(crate) formula1: Option<String>,
  pub(crate) formula2: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SparklineGroupModel {
  pub(crate) sparklines: usize,
  pub(crate) formula: Option<String>,
  pub(crate) sparkline_formula_text_len: usize,
  pub(crate) reference_text_len: usize,
  pub(crate) color_count: usize,
  pub(crate) flag_count: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct IgnoredErrorsModel {
  pub(crate) ignored_errors: usize,
  pub(crate) flag_count: usize,
  pub(crate) reference_text_len: usize,
  pub(crate) has_extensions: bool,
}

impl SheetConditionCatalog {
  pub(crate) fn from_worksheet(worksheet: &x::Worksheet) -> Self {
    // worksheetfragment.cxx DataValidationsContext. Formula strings stay with
    // the sheet condition catalog; evaluation is delegated to ooxmlsdk-formula.
    let data_validations = worksheet.data_validations.as_ref();
    Self {
      conditional_formats: worksheet
        .conditional_formatting
        .iter()
        .map(ConditionalFormatModel::from_conditional_format)
        .collect(),
      data_validations: data_validations
        .map(|validations| {
          validations
            .data_validation
            .iter()
            .map(DataValidationModel::from_data_validation)
            .collect()
        })
        .unwrap_or_default(),
      validations_disable_prompts: data_validations
        .and_then(|validations| validations.disable_prompts)
        .is_some_and(|value| value.as_bool()),
      validation_window: data_validations
        .and_then(|validations| Some((validations.x_window?, validations.y_window?))),
      extension_conditions: SheetConditionExtensionCatalog::from_worksheet(worksheet),
    }
  }
}

impl SheetConditionExtensionCatalog {
  fn from_worksheet(worksheet: &x::Worksheet) -> Self {
    let mut catalog = Self::default();
    if let Some(extensions) = worksheet.worksheet_extension_list.as_ref() {
      for extension in &extensions.worksheet_extension {
        catalog.uri_text_len += extension.uri.len();
        match extension.worksheet_extension_choice.as_ref() {
          Some(x::WorksheetExtensionChoice::ConditionalFormattings(formattings)) => {
            catalog.conditional_formats.extend(
              formattings
                .conditional_formatting
                .iter()
                .map(ExtendedConditionalFormatModel::from_conditional_format),
            );
          }
          Some(x::WorksheetExtensionChoice::DataValidations(validations)) => {
            catalog.data_validations.extend(
              validations
                .data_validation
                .iter()
                .map(ExtendedDataValidationModel::from_data_validation),
            );
          }
          Some(x::WorksheetExtensionChoice::SparklineGroups(groups)) => {
            catalog.sparkline_groups.extend(
              groups
                .sparkline_group
                .iter()
                .map(SparklineGroupModel::from_sparkline_group),
            );
          }
          Some(x::WorksheetExtensionChoice::SlicerList(slicers)) => {
            catalog.slicer_refs += slicers.slicer_ref.len();
          }
          Some(x::WorksheetExtensionChoice::ProtectedRanges(ranges)) => {
            catalog.protected_ranges += ranges.protected_range.len();
          }
          Some(x::WorksheetExtensionChoice::IgnoredErrors(errors)) => {
            catalog
              .ignored_errors
              .push(IgnoredErrorsModel::from_ignored_errors(errors));
          }
          Some(x::WorksheetExtensionChoice::WebExtensions(web_extensions)) => {
            catalog.web_extensions += web_extensions.web_extension.len();
          }
          Some(x::WorksheetExtensionChoice::TimelineReferences(timelines)) => {
            catalog.timeline_refs += timelines.timeline_reference.len();
          }
          Some(x::WorksheetExtensionChoice::XmlAny(value)) => {
            catalog.unknown_extensions += 1;
            catalog.uri_text_len += value.len();
          }
          None => {}
        }
      }
    }
    catalog
  }
}

impl ConditionalFormatModel {
  fn from_conditional_format(format: &x::ConditionalFormatting) -> Self {
    Self {
      sequence_of_references: format.sequence_of_references.clone().unwrap_or_default(),
      pivot: format.pivot.is_some_and(|value| value.as_bool()),
      rules: format
        .conditional_formatting_rule
        .iter()
        .map(ConditionalFormatRuleModel::from_rule)
        .collect(),
      has_extensions: format.extension_list.is_some(),
    }
  }
}

impl ConditionalFormatRuleModel {
  fn from_rule(rule: &x::ConditionalFormattingRule) -> Self {
    Self {
      rule_type: rule.r#type,
      format_id: rule.format_id,
      priority: rule.priority,
      stop_if_true: rule.stop_if_true.is_some_and(|value| value.as_bool()),
      operator: rule.operator,
      text: rule.text.clone(),
      time_period: rule.time_period,
      rank: rule.rank,
      std_dev: rule.std_dev,
      above_average: rule.above_average.is_none_or(|value| value.as_bool()),
      percent: rule.percent.is_some_and(|value| value.as_bool()),
      bottom: rule.bottom.is_some_and(|value| value.as_bool()),
      equal_average: rule.equal_average.is_some_and(|value| value.as_bool()),
      formulas: rule
        .formula
        .iter()
        .filter_map(|formula| formula.xml_content.clone())
        .collect(),
      has_color_scale: rule.color_scale.is_some(),
      has_data_bar: rule.data_bar.is_some(),
      icon_set: rule.icon_set.as_ref().map(IconSetModel::from_base),
      has_extensions: rule.conditional_formatting_rule_extension_list.is_some(),
    }
  }
}

impl IconSetModel {
  fn from_base(icon_set: &x::IconSet) -> Self {
    Self {
      // ECMA-376 CT_IconSet and LibreOffice condformatbuffer.cxx both define
      // the omitted base-format iconSet attribute as 3TrafficLights1. The x14
      // extension has a different 3Arrows default and is handled separately.
      icon_set: icon_set
        .icon_set_value
        .unwrap_or(x::IconSetValues::ThreeTrafficLights1)
        .into(),
      show_value: icon_set.show_value.is_none_or(|value| value.as_bool()),
      reverse: icon_set.reverse.is_some_and(|value| value.as_bool()),
      thresholds: icon_set
        .conditional_format_value_object
        .iter()
        .map(|threshold| IconSetThresholdModel {
          threshold_type: threshold.r#type.into(),
          value: threshold.val.clone(),
          greater_than_or_equal: threshold
            .greater_than_or_equal
            .is_none_or(|value| value.as_bool()),
        })
        .collect(),
      custom_icons: None,
    }
  }

  fn from_extended(icon_set: &x14::IconSet) -> Self {
    let custom = icon_set.custom.is_some_and(|value| value.as_bool());
    Self {
      icon_set: icon_set.icon_set_types.unwrap_or_default().into(),
      show_value: icon_set.show_value.is_none_or(|value| value.as_bool()),
      reverse: icon_set.reverse.is_some_and(|value| value.as_bool()),
      thresholds: icon_set
        .conditional_formatting_value_object
        .iter()
        .map(|threshold| IconSetThresholdModel {
          threshold_type: threshold.r#type.into(),
          value: threshold
            .formula
            .clone()
            .or_else(|| threshold.value.clone()),
          greater_than_or_equal: threshold
            .greater_than_or_equal
            .is_none_or(|value| value.as_bool()),
        })
        .collect(),
      custom_icons: custom.then(|| {
        icon_set
          .conditional_formatting_icon
          .iter()
          .map(|icon| {
            let icon_set: IconSetType = icon.icon_set.into();
            (icon_set != IconSetType::NoIcons).then_some(IconSetCustomIcon {
              icon_set,
              icon_index: icon.icon_id as usize,
            })
          })
          .collect()
      }),
    }
  }
}

impl From<x::IconSetValues> for IconSetType {
  fn from(value: x::IconSetValues) -> Self {
    match value {
      x::IconSetValues::ThreeArrows => Self::ThreeArrows,
      x::IconSetValues::ThreeArrowsGray => Self::ThreeArrowsGray,
      x::IconSetValues::ThreeFlags => Self::ThreeFlags,
      x::IconSetValues::ThreeTrafficLights1 => Self::ThreeTrafficLights1,
      x::IconSetValues::ThreeTrafficLights2 => Self::ThreeTrafficLights2,
      x::IconSetValues::ThreeSigns => Self::ThreeSigns,
      x::IconSetValues::ThreeSymbols => Self::ThreeSymbols,
      x::IconSetValues::ThreeSymbols2 => Self::ThreeSymbols2,
      x::IconSetValues::FourArrows => Self::FourArrows,
      x::IconSetValues::FourArrowsGray => Self::FourArrowsGray,
      x::IconSetValues::FourRedToBlack => Self::FourRedToBlack,
      x::IconSetValues::FourRating => Self::FourRating,
      x::IconSetValues::FourTrafficLights => Self::FourTrafficLights,
      x::IconSetValues::FiveArrows => Self::FiveArrows,
      x::IconSetValues::FiveArrowsGray => Self::FiveArrowsGray,
      x::IconSetValues::FiveRating => Self::FiveRating,
      x::IconSetValues::FiveQuarters => Self::FiveQuarters,
    }
  }
}

impl From<x14::IconSetTypeValues> for IconSetType {
  fn from(value: x14::IconSetTypeValues) -> Self {
    match value {
      x14::IconSetTypeValues::ThreeArrows => Self::ThreeArrows,
      x14::IconSetTypeValues::ThreeArrowsGray => Self::ThreeArrowsGray,
      x14::IconSetTypeValues::ThreeFlags => Self::ThreeFlags,
      x14::IconSetTypeValues::ThreeTrafficLights1 => Self::ThreeTrafficLights1,
      x14::IconSetTypeValues::ThreeTrafficLights2 => Self::ThreeTrafficLights2,
      x14::IconSetTypeValues::ThreeSigns => Self::ThreeSigns,
      x14::IconSetTypeValues::ThreeSymbols => Self::ThreeSymbols,
      x14::IconSetTypeValues::ThreeSymbols2 => Self::ThreeSymbols2,
      x14::IconSetTypeValues::FourArrows => Self::FourArrows,
      x14::IconSetTypeValues::FourArrowsGray => Self::FourArrowsGray,
      x14::IconSetTypeValues::FourRedToBlack => Self::FourRedToBlack,
      x14::IconSetTypeValues::FourRating => Self::FourRating,
      x14::IconSetTypeValues::FourTrafficLights => Self::FourTrafficLights,
      x14::IconSetTypeValues::FiveArrows => Self::FiveArrows,
      x14::IconSetTypeValues::FiveArrowsGray => Self::FiveArrowsGray,
      x14::IconSetTypeValues::FiveRating => Self::FiveRating,
      x14::IconSetTypeValues::FiveQuarters => Self::FiveQuarters,
      x14::IconSetTypeValues::ThreeStars => Self::ThreeStars,
      x14::IconSetTypeValues::ThreeTriangles => Self::ThreeTriangles,
      x14::IconSetTypeValues::FiveBoxes => Self::FiveBoxes,
      x14::IconSetTypeValues::NoIcons => Self::NoIcons,
    }
  }
}

impl From<x::ConditionalFormatValueObjectValues> for IconSetThresholdType {
  fn from(value: x::ConditionalFormatValueObjectValues) -> Self {
    match value {
      x::ConditionalFormatValueObjectValues::Number => Self::Number,
      x::ConditionalFormatValueObjectValues::Percent => Self::Percent,
      x::ConditionalFormatValueObjectValues::Max => Self::Maximum,
      x::ConditionalFormatValueObjectValues::Min => Self::Minimum,
      x::ConditionalFormatValueObjectValues::Formula => Self::Formula,
      x::ConditionalFormatValueObjectValues::Percentile => Self::Percentile,
    }
  }
}

impl From<x14::ConditionalFormattingValueObjectTypeValues> for IconSetThresholdType {
  fn from(value: x14::ConditionalFormattingValueObjectTypeValues) -> Self {
    match value {
      x14::ConditionalFormattingValueObjectTypeValues::Numeric => Self::Number,
      x14::ConditionalFormattingValueObjectTypeValues::Percent => Self::Percent,
      x14::ConditionalFormattingValueObjectTypeValues::Max => Self::Maximum,
      x14::ConditionalFormattingValueObjectTypeValues::Min => Self::Minimum,
      x14::ConditionalFormattingValueObjectTypeValues::Formula => Self::Formula,
      x14::ConditionalFormattingValueObjectTypeValues::Percentile => Self::Percentile,
      x14::ConditionalFormattingValueObjectTypeValues::AutoMin => Self::AutomaticMinimum,
      x14::ConditionalFormattingValueObjectTypeValues::AutoMax => Self::AutomaticMaximum,
    }
  }
}

impl DataValidationModel {
  fn from_data_validation(validation: &x::DataValidation) -> Self {
    Self {
      validation_type: validation.r#type,
      error_style: validation.error_style,
      ime_mode: validation.ime_mode,
      operator: validation.operator,
      allow_blank: validation.allow_blank.is_some_and(|value| value.as_bool()),
      no_drop_down: validation
        .show_drop_down
        .is_some_and(|value| value.as_bool()),
      show_input_message: validation
        .show_input_message
        .is_some_and(|value| value.as_bool()),
      show_error_message: validation
        .show_error_message
        .is_some_and(|value| value.as_bool()),
      error_title: validation.error_title.clone(),
      error: validation.error.clone(),
      prompt_title: validation.prompt_title.clone(),
      prompt: validation.prompt.clone(),
      sequence_of_references: validation.sequence_of_references.clone(),
      list: validation.list.clone(),
      formula1: validation
        .formula1
        .as_ref()
        .and_then(|formula| formula.xml_content.clone()),
      formula2: validation
        .formula2
        .as_ref()
        .and_then(|formula| formula.xml_content.clone()),
    }
  }
}

impl ExtendedConditionalFormatModel {
  fn from_conditional_format(format: &x14::ConditionalFormatting) -> Self {
    Self {
      sequence_of_references: format.reference_sequence.clone().unwrap_or_default(),
      pivot: format.pivot.is_some_and(|value| value.as_bool()),
      rules: format
        .conditional_formatting_rule
        .iter()
        .map(ExtendedConditionalFormatRuleModel::from_rule)
        .collect(),
      has_extensions: format.extension_list.is_some(),
    }
  }
}

impl ExtendedConditionalFormatRuleModel {
  fn from_rule(rule: &x14::ConditionalFormattingRule) -> Self {
    Self {
      rule_type: rule.r#type,
      priority: rule.priority,
      stop_if_true: rule.stop_if_true.is_some_and(|value| value.as_bool()),
      boolean_flags: bool_attr_count([
        rule.above_average,
        rule.percent,
        rule.bottom,
        rule.equal_average,
        rule.active_present,
      ]),
      operator: rule.operator,
      text: rule.text.clone(),
      time_period: rule.time_period,
      rank: rule.rank,
      std_dev: rule.standard_deviation,
      id: rule.id.clone(),
      formulas: rule.formula.to_vec(),
      has_color_scale: rule.color_scale.is_some(),
      has_data_bar: rule.data_bar.is_some(),
      icon_set: rule.icon_set.as_ref().map(IconSetModel::from_extended),
      has_differential_format: rule.differential_type.is_some(),
      has_extensions: rule.extension_list.is_some(),
    }
  }
}

impl ExtendedDataValidationModel {
  fn from_data_validation(validation: &x14::DataValidation) -> Self {
    Self {
      validation_type: validation.r#type,
      error_style: validation.error_style,
      ime_mode: validation.ime_mode,
      operator: validation.operator,
      allow_blank: validation.allow_blank.is_some_and(|value| value.as_bool()),
      no_drop_down: validation
        .show_drop_down
        .is_some_and(|value| value.as_bool()),
      show_input_message: validation
        .show_input_message
        .is_some_and(|value| value.as_bool()),
      show_error_message: validation
        .show_error_message
        .is_some_and(|value| value.as_bool()),
      error_title: validation.error_title.clone(),
      error: validation.error.clone(),
      prompt_title: validation.prompt_title.clone(),
      prompt: validation.prompt.clone(),
      sequence_of_references: validation.reference_sequence.clone(),
      formula1: validation
        .data_validation_forumla1
        .as_ref()
        .map(|formula| formula.formula.clone()),
      formula2: validation
        .data_validation_forumla2
        .as_ref()
        .map(|formula| formula.formula.clone()),
    }
  }
}

impl SparklineGroupModel {
  fn from_sparkline_group(group: &x14::SparklineGroup) -> Self {
    Self {
      sparklines: group.sparklines.sparkline.len(),
      formula: group.formula.clone(),
      sparkline_formula_text_len: group
        .sparklines
        .sparkline
        .iter()
        .map(|sparkline| {
          sparkline
            .formula
            .as_ref()
            .as_ref()
            .map_or(0, |value| value.len())
        })
        .sum(),
      reference_text_len: group
        .sparklines
        .sparkline
        .iter()
        .flat_map(|sparkline| sparkline.reference_sequence.iter())
        .map(|reference| reference.len())
        .sum(),
      color_count: usize::from(group.series_color.is_some())
        + usize::from(group.negative_color.is_some())
        + usize::from(group.axis_color.is_some())
        + usize::from(group.markers_color.is_some())
        + usize::from(group.first_marker_color.is_some())
        + usize::from(group.last_marker_color.is_some())
        + usize::from(group.high_marker_color.is_some())
        + usize::from(group.low_marker_color.is_some()),
      flag_count: usize::from(group.manual_max.is_some())
        + usize::from(group.manual_min.is_some())
        + usize::from(group.line_weight.is_some())
        + usize::from(group.r#type.is_some())
        + usize::from(group.display_empty_cells_as.is_some())
        + usize::from(group.min_axis_type.is_some())
        + usize::from(group.max_axis_type.is_some())
        + bool_attr_count([
          group.date_axis,
          group.markers,
          group.high,
          group.low,
          group.first,
          group.last,
          group.negative,
          group.display_x_axis,
          group.display_hidden,
          group.right_to_left,
        ]),
    }
  }
}

impl IgnoredErrorsModel {
  fn from_ignored_errors(errors: &x14::IgnoredErrors) -> Self {
    Self {
      ignored_errors: errors.ignored_error.len(),
      flag_count: errors
        .ignored_error
        .iter()
        .map(|error| {
          bool_attr_count([
            error.eval_error,
            error.two_digit_text_year,
            error.number_stored_as_text,
            error.formula,
            error.formula_range,
            error.unlocked_formula,
            error.empty_cell_reference,
            error.list_data_validation,
            error.calculated_column,
          ])
        })
        .sum(),
      reference_text_len: errors
        .ignored_error
        .iter()
        .flat_map(|error| error.reference_sequence.iter())
        .map(|reference| reference.len())
        .sum(),
      has_extensions: errors.extension_list.is_some(),
    }
  }
}

fn bool_attr_count<const N: usize>(
  values: [Option<ooxmlsdk::simple_type::BooleanValue>; N],
) -> usize {
  values
    .into_iter()
    .filter(|value| value.is_some_and(|value| value.as_bool()))
    .count()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn base_icon_set_preserves_threshold_inclusivity() {
    let icon_set = x::IconSet {
      icon_set_value: Some(x::IconSetValues::ThreeArrows),
      conditional_format_value_object: vec![
        x::ConditionalFormatValueObject {
          r#type: x::ConditionalFormatValueObjectValues::Percent,
          val: Some("0".to_string()),
          ..Default::default()
        },
        x::ConditionalFormatValueObject {
          r#type: x::ConditionalFormatValueObjectValues::Number,
          val: Some("0".to_string()),
          ..Default::default()
        },
        x::ConditionalFormatValueObject {
          r#type: x::ConditionalFormatValueObjectValues::Number,
          val: Some("3".to_string()),
          greater_than_or_equal: Some(ooxmlsdk::simple_type::BooleanValue::Zero),
          ..Default::default()
        },
      ],
      ..Default::default()
    };

    let model = IconSetModel::from_base(&icon_set);
    assert_eq!(model.icon_set, IconSetType::ThreeArrows);
    assert_eq!(model.thresholds.len(), 3);
    assert!(model.thresholds[1].greater_than_or_equal);
    assert!(!model.thresholds[2].greater_than_or_equal);
  }

  #[test]
  fn base_icon_set_uses_the_schema_traffic_light_default() {
    let model = IconSetModel::from_base(&x::IconSet::default());
    assert_eq!(model.icon_set, IconSetType::ThreeTrafficLights1);
  }

  #[test]
  fn extended_custom_icon_set_preserves_no_icon_slots() {
    let icon_set = x14::IconSet {
      icon_set_types: Some(x14::IconSetTypeValues::FiveQuarters),
      custom: Some(ooxmlsdk::simple_type::BooleanValue::One),
      conditional_formatting_icon: vec![
        x14::ConditionalFormattingIcon {
          icon_set: x14::IconSetTypeValues::ThreeArrowsGray,
          icon_id: 0,
        },
        x14::ConditionalFormattingIcon {
          icon_set: x14::IconSetTypeValues::NoIcons,
          icon_id: 0,
        },
      ],
      ..Default::default()
    };

    let model = IconSetModel::from_extended(&icon_set);
    let custom = model.custom_icons.expect("custom icon map");
    assert_eq!(
      custom[0],
      Some(IconSetCustomIcon {
        icon_set: IconSetType::ThreeArrowsGray,
        icon_index: 0,
      })
    );
    assert_eq!(custom[1], None);
  }
}
