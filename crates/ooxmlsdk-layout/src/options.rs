#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LayoutOptions {
  pub source_file_name: Option<String>,
  pub action: LayoutActionOptions,
  pub diagnostics: LayoutDiagnosticsOptions,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LayoutActionOptions {
  pub paint: bool,
  pub complete: bool,
  pub calc_layout: bool,
  pub check_pages: bool,
}

impl Default for LayoutActionOptions {
  fn default() -> Self {
    Self {
      paint: true,
      complete: true,
      calc_layout: true,
      check_pages: true,
    }
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LayoutDiagnosticsOptions {
  pub collect_debug_records: bool,
  pub collect_reflow_records: bool,
  pub preserve_source_links: bool,
}
