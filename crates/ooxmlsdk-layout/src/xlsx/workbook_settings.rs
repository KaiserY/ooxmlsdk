use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct WorkbookGlobals {
  pub(crate) settings: WorkbookSettings,
  pub(crate) views: Vec<WorkbookViewModel>,
  pub(crate) calculation: CalculationSettings,
  pub(crate) custom_workbook_views: usize,
  pub(crate) external_references: usize,
  pub(crate) function_groups: usize,
  pub(crate) pivot_caches: usize,
  pub(crate) web_publish_objects: usize,
  pub(crate) file_recovery_properties: usize,
  pub(crate) has_workbook_extensions: bool,
  pub(crate) ole_size_reference: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct WorkbookSettings {
  pub(crate) date_1904: bool,
  pub(crate) date_compatibility: Option<bool>,
  pub(crate) show_objects: Option<x::ObjectDisplayValues>,
  pub(crate) save_external_link_values: Option<bool>,
  pub(crate) update_links: Option<x::UpdateLinksBehaviorValues>,
  pub(crate) code_name: Option<String>,
  pub(crate) default_theme_version: Option<u32>,
  pub(crate) has_file_version: bool,
  pub(crate) file_application_name: Option<String>,
  pub(crate) read_only_recommended: bool,
  pub(crate) workbook_protection: WorkbookProtectionModel,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct WorkbookProtectionModel {
  pub(crate) lock_structure: bool,
  pub(crate) lock_windows: bool,
  pub(crate) lock_revision: bool,
  pub(crate) has_workbook_password: bool,
  pub(crate) has_revision_password: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct WorkbookViewModel {
  pub(crate) visibility: Option<x::VisibilityValues>,
  pub(crate) minimized: bool,
  pub(crate) show_horizontal_scroll: Option<bool>,
  pub(crate) show_vertical_scroll: Option<bool>,
  pub(crate) show_sheet_tabs: Option<bool>,
  pub(crate) first_sheet: Option<u32>,
  pub(crate) active_tab: Option<u32>,
  pub(crate) tab_ratio: Option<u32>,
  pub(crate) window_width: Option<u32>,
  pub(crate) window_height: Option<u32>,
  pub(crate) has_extensions: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct CalculationSettings {
  pub(crate) calculation_id: Option<u32>,
  pub(crate) calculation_mode: Option<x::CalculateModeValues>,
  pub(crate) reference_mode: Option<x::ReferenceModeValues>,
  pub(crate) full_calculation_on_load: bool,
  pub(crate) force_full_calculation: bool,
  pub(crate) iterate: bool,
  pub(crate) iterate_count: Option<u32>,
  pub(crate) iterate_delta: Option<f64>,
  pub(crate) full_precision: Option<bool>,
  pub(crate) calculation_completed: Option<bool>,
  pub(crate) calculation_on_save: Option<bool>,
  pub(crate) concurrent_calculation: Option<bool>,
  pub(crate) concurrent_manual_count: Option<u32>,
}

impl WorkbookGlobals {
  pub(crate) fn from_workbook(workbook: &x::Workbook) -> Self {
    Self {
      settings: WorkbookSettings::from_workbook(workbook),
      views: workbook
        .book_views
        .as_ref()
        .map(|views| {
          views
            .workbook_view
            .iter()
            .map(WorkbookViewModel::from_workbook_view)
            .collect()
        })
        .unwrap_or_default(),
      calculation: workbook
        .calculation_properties
        .as_ref()
        .map(CalculationSettings::from_calculation_properties)
        .unwrap_or_default(),
      custom_workbook_views: workbook
        .custom_workbook_views
        .as_ref()
        .map_or(0, |views| views.custom_workbook_view.len()),
      external_references: workbook
        .external_references
        .as_ref()
        .map_or(0, |references| references.external_reference.len()),
      function_groups: workbook
        .function_groups
        .as_ref()
        .map_or(0, |groups| groups.function_group.len()),
      pivot_caches: workbook
        .pivot_caches
        .as_ref()
        .map_or(0, |caches| caches.pivot_cache.len()),
      web_publish_objects: workbook
        .web_publish_objects
        .as_ref()
        .map_or(0, |objects| objects.web_publish_object.len()),
      file_recovery_properties: workbook.file_recovery_properties.len(),
      has_workbook_extensions: workbook.workbook_extension_list.is_some(),
      ole_size_reference: workbook
        .ole_size
        .as_ref()
        .map(|ole_size| ole_size.reference.clone()),
    }
  }

  pub(crate) fn active_tab(&self) -> Option<u32> {
    self.views.first().and_then(|view| view.active_tab)
  }
}

impl WorkbookSettings {
  fn from_workbook(workbook: &x::Workbook) -> Self {
    let properties = workbook.workbook_properties.as_ref();
    Self {
      date_1904: properties
        .and_then(|properties| properties.date1904)
        .is_some_and(|value| value.as_bool()),
      date_compatibility: properties
        .and_then(|properties| properties.date_compatibility)
        .map(|value| value.as_bool()),
      show_objects: properties.and_then(|properties| properties.show_objects),
      save_external_link_values: properties
        .and_then(|properties| properties.save_external_link_values)
        .map(|value| value.as_bool()),
      update_links: properties.and_then(|properties| properties.update_links),
      code_name: properties.and_then(|properties| properties.code_name.clone()),
      default_theme_version: properties.and_then(|properties| properties.default_theme_version),
      has_file_version: workbook.file_version.is_some(),
      file_application_name: workbook
        .file_version
        .as_ref()
        .and_then(|version| version.application_name.clone()),
      read_only_recommended: workbook
        .file_sharing
        .as_ref()
        .and_then(|sharing| sharing.read_only_recommended)
        .is_some_and(|value| value.as_bool()),
      workbook_protection: workbook
        .workbook_protection
        .as_ref()
        .map(WorkbookProtectionModel::from_workbook_protection)
        .unwrap_or_default(),
    }
  }
}

impl WorkbookProtectionModel {
  fn from_workbook_protection(protection: &x::WorkbookProtection) -> Self {
    Self {
      lock_structure: protection
        .lock_structure
        .is_some_and(|value| value.as_bool()),
      lock_windows: protection.lock_windows.is_some_and(|value| value.as_bool()),
      lock_revision: protection
        .lock_revision
        .is_some_and(|value| value.as_bool()),
      has_workbook_password: protection.workbook_password.is_some()
        || protection.workbook_hash_value.is_some(),
      has_revision_password: protection.revisions_password.is_some()
        || protection.revisions_hash_value.is_some(),
    }
  }
}

impl WorkbookViewModel {
  fn from_workbook_view(view: &x::WorkbookView) -> Self {
    Self {
      visibility: view.visibility,
      minimized: view.minimized.is_some_and(|value| value.as_bool()),
      show_horizontal_scroll: view.show_horizontal_scroll.map(|value| value.as_bool()),
      show_vertical_scroll: view.show_vertical_scroll.map(|value| value.as_bool()),
      show_sheet_tabs: view.show_sheet_tabs.map(|value| value.as_bool()),
      first_sheet: view.first_sheet,
      active_tab: view.active_tab,
      tab_ratio: view.tab_ratio,
      window_width: view.window_width,
      window_height: view.window_height,
      has_extensions: view.extension_list.is_some(),
    }
  }
}

impl CalculationSettings {
  fn from_calculation_properties(properties: &x::CalculationProperties) -> Self {
    Self {
      calculation_id: properties.calculation_id,
      calculation_mode: properties.calculation_mode,
      reference_mode: properties.reference_mode,
      full_calculation_on_load: properties
        .full_calculation_on_load
        .is_some_and(|value| value.as_bool()),
      force_full_calculation: properties
        .force_full_calculation
        .is_some_and(|value| value.as_bool()),
      iterate: properties.iterate.is_some_and(|value| value.as_bool()),
      iterate_count: properties.iterate_count,
      iterate_delta: properties.iterate_delta,
      full_precision: properties.full_precision.map(|value| value.as_bool()),
      calculation_completed: properties
        .calculation_completed
        .map(|value| value.as_bool()),
      calculation_on_save: properties.calculation_on_save.map(|value| value.as_bool()),
      concurrent_calculation: properties
        .concurrent_calculation
        .map(|value| value.as_bool()),
      concurrent_manual_count: properties.concurrent_manual_count,
    }
  }
}
