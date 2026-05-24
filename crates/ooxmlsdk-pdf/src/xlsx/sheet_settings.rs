use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SheetSettingsCatalog {
  pub(crate) properties: SheetPropertiesModel,
  pub(crate) protection: SheetProtectionModel,
  pub(crate) auto_filter: Option<AutoFilterModel>,
  pub(crate) sort_state: Option<SortStateModel>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SheetPropertiesModel {
  pub(crate) code_name: Option<String>,
  pub(crate) filter_mode: bool,
  pub(crate) published: Option<bool>,
  pub(crate) sync_horizontal: Option<bool>,
  pub(crate) sync_vertical: Option<bool>,
  pub(crate) sync_reference: Option<String>,
  pub(crate) transition_evaluation: Option<bool>,
  pub(crate) transition_entry: Option<bool>,
  pub(crate) enable_format_conditions_calculation: Option<bool>,
  pub(crate) has_tab_color: bool,
  pub(crate) outline: OutlinePropertiesModel,
  pub(crate) page_setup: PageSetupPropertiesModel,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct OutlinePropertiesModel {
  pub(crate) apply_styles: bool,
  pub(crate) summary_below: Option<bool>,
  pub(crate) summary_right: Option<bool>,
  pub(crate) show_outline_symbols: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct PageSetupPropertiesModel {
  pub(crate) auto_page_breaks: Option<bool>,
  pub(crate) fit_to_page: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SheetProtectionModel {
  pub(crate) has_password: bool,
  pub(crate) has_hash: bool,
  pub(crate) algorithm_name: Option<String>,
  pub(crate) spin_count: Option<u32>,
  pub(crate) sheet: bool,
  pub(crate) objects: bool,
  pub(crate) scenarios: bool,
  pub(crate) locked_options: usize,
  pub(crate) unlocked_selection: bool,
  pub(crate) locked_selection: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct AutoFilterModel {
  pub(crate) reference: Option<String>,
  pub(crate) filter_columns: usize,
  pub(crate) sort_state: Option<SortStateModel>,
  pub(crate) has_extensions: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SortStateModel {
  pub(crate) reference: String,
  pub(crate) column_sort: bool,
  pub(crate) case_sensitive: bool,
  pub(crate) sort_method: Option<x::SortMethodValues>,
  pub(crate) has_sort_condition: bool,
  pub(crate) has_extensions: bool,
}

impl SheetSettingsCatalog {
  pub(crate) fn from_worksheet(worksheet: &x::Worksheet) -> Self {
    // Source: LibreOffice sc/source/filter/oox/worksheetsettings.cxx and
    // autofilterbuffer.cxx. These settings are imported before worksheet
    // finalization and table auto-filter application.
    Self {
      properties: worksheet
        .sheet_properties
        .as_deref()
        .map(SheetPropertiesModel::from_sheet_properties)
        .unwrap_or_default(),
      protection: worksheet
        .sheet_protection
        .as_ref()
        .map(SheetProtectionModel::from_sheet_protection)
        .unwrap_or_default(),
      auto_filter: worksheet
        .auto_filter
        .as_deref()
        .map(AutoFilterModel::from_auto_filter),
      sort_state: worksheet
        .sort_state
        .as_deref()
        .map(SortStateModel::from_sort_state),
    }
  }
}

impl SheetPropertiesModel {
  fn from_sheet_properties(properties: &x::SheetProperties) -> Self {
    Self {
      code_name: properties.code_name.clone(),
      filter_mode: properties.filter_mode.is_some_and(|value| value.as_bool()),
      published: properties.published.map(|value| value.as_bool()),
      sync_horizontal: properties.sync_horizontal.map(|value| value.as_bool()),
      sync_vertical: properties.sync_vertical.map(|value| value.as_bool()),
      sync_reference: properties.sync_reference.clone(),
      transition_evaluation: properties
        .transition_evaluation
        .map(|value| value.as_bool()),
      transition_entry: properties.transition_entry.map(|value| value.as_bool()),
      enable_format_conditions_calculation: properties
        .enable_format_conditions_calculation
        .map(|value| value.as_bool()),
      has_tab_color: properties.tab_color.is_some(),
      outline: properties
        .outline_properties
        .as_ref()
        .map(OutlinePropertiesModel::from_outline_properties)
        .unwrap_or_default(),
      page_setup: properties
        .page_setup_properties
        .as_ref()
        .map(PageSetupPropertiesModel::from_page_setup_properties)
        .unwrap_or_default(),
    }
  }
}

impl OutlinePropertiesModel {
  fn from_outline_properties(outline: &x::OutlineProperties) -> Self {
    Self {
      apply_styles: outline.apply_styles.is_some_and(|value| value.as_bool()),
      summary_below: outline.summary_below.map(|value| value.as_bool()),
      summary_right: outline.summary_right.map(|value| value.as_bool()),
      show_outline_symbols: outline.show_outline_symbols.map(|value| value.as_bool()),
    }
  }
}

impl PageSetupPropertiesModel {
  fn from_page_setup_properties(properties: &x::PageSetupProperties) -> Self {
    Self {
      auto_page_breaks: properties.auto_page_breaks.map(|value| value.as_bool()),
      fit_to_page: properties.fit_to_page.is_some_and(|value| value.as_bool()),
    }
  }
}

impl SheetProtectionModel {
  fn from_sheet_protection(protection: &x::SheetProtection) -> Self {
    let lock_flags = [
      protection.format_cells,
      protection.format_columns,
      protection.format_rows,
      protection.insert_columns,
      protection.insert_rows,
      protection.insert_hyperlinks,
      protection.delete_columns,
      protection.delete_rows,
      protection.sort,
      protection.auto_filter,
      protection.pivot_tables,
    ];
    Self {
      has_password: protection.password.is_some(),
      has_hash: protection.hash_value.is_some(),
      algorithm_name: protection.algorithm_name.clone(),
      spin_count: protection.spin_count,
      sheet: protection.sheet.is_some_and(|value| value.as_bool()),
      objects: protection.objects.is_some_and(|value| value.as_bool()),
      scenarios: protection.scenarios.is_some_and(|value| value.as_bool()),
      locked_options: lock_flags
        .iter()
        .filter(|value| value.is_some_and(|value| value.as_bool()))
        .count(),
      unlocked_selection: protection
        .select_unlocked_cells
        .is_some_and(|value| value.as_bool()),
      locked_selection: protection
        .select_locked_cells
        .is_some_and(|value| value.as_bool()),
    }
  }
}

impl AutoFilterModel {
  fn from_auto_filter(auto_filter: &x::AutoFilter) -> Self {
    Self {
      reference: auto_filter.reference.clone(),
      filter_columns: auto_filter.filter_column.len(),
      sort_state: auto_filter
        .sort_state
        .as_deref()
        .map(SortStateModel::from_sort_state),
      has_extensions: auto_filter.extension_list.is_some(),
    }
  }
}

impl SortStateModel {
  fn from_sort_state(sort_state: &x::SortState) -> Self {
    Self {
      reference: sort_state.reference.clone(),
      column_sort: sort_state.column_sort.is_some_and(|value| value.as_bool()),
      case_sensitive: sort_state
        .case_sensitive
        .is_some_and(|value| value.as_bool()),
      sort_method: sort_state.sort_method,
      has_sort_condition: sort_state.sort_state_choice.is_some(),
      has_extensions: sort_state.extension_list.is_some(),
    }
  }
}
