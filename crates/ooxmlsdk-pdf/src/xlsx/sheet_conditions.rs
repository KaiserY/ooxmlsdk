use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SheetConditionCatalog {
  pub(crate) conditional_formats: Vec<ConditionalFormatModel>,
  pub(crate) data_validations: Vec<DataValidationModel>,
  pub(crate) validations_disable_prompts: bool,
  pub(crate) validation_window: Option<(u32, u32)>,
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
  pub(crate) formulas: Vec<String>,
  pub(crate) has_color_scale: bool,
  pub(crate) has_data_bar: bool,
  pub(crate) has_icon_set: bool,
  pub(crate) has_extensions: bool,
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

impl SheetConditionCatalog {
  pub(crate) fn from_worksheet(worksheet: &x::Worksheet) -> Self {
    // Source: LibreOffice sc/source/filter/oox/condformatbuffer.cxx and
    // worksheetfragment.cxx DataValidationsContext. Formula strings are kept
    // here until the FormulaParser/FormulaBuffer owner exists.
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
    }
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
      formulas: rule
        .formula
        .iter()
        .filter_map(|formula| formula.0.xml_content.clone())
        .collect(),
      has_color_scale: rule.color_scale.is_some(),
      has_data_bar: rule.data_bar.is_some(),
      has_icon_set: rule.icon_set.is_some(),
      has_extensions: rule.conditional_formatting_rule_extension_list.is_some(),
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
        .and_then(|formula| formula.0.xml_content.clone()),
      formula2: validation
        .formula2
        .as_ref()
        .and_then(|formula| formula.0.xml_content.clone()),
    }
  }
}
