use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SheetViewCatalog {
  pub(crate) views: Vec<SheetViewModel>,
  pub(crate) has_extensions: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SheetViewModel {
  pub(crate) workbook_view_id: u32,
  pub(crate) view_type: Option<x::SheetViewValues>,
  pub(crate) top_left_cell: Option<String>,
  pub(crate) color_id: Option<u32>,
  pub(crate) zoom_scale: Option<u32>,
  pub(crate) zoom_scale_normal: Option<u32>,
  pub(crate) zoom_scale_sheet_layout_view: Option<u32>,
  pub(crate) zoom_scale_page_layout_view: Option<u32>,
  pub(crate) window_protection: Option<bool>,
  pub(crate) show_formulas: Option<bool>,
  pub(crate) show_grid_lines: Option<bool>,
  pub(crate) show_row_col_headers: Option<bool>,
  pub(crate) show_zeros: Option<bool>,
  pub(crate) right_to_left: Option<bool>,
  pub(crate) tab_selected: Option<bool>,
  pub(crate) show_outline_symbols: Option<bool>,
  pub(crate) default_grid_color: Option<bool>,
  pub(crate) pane: Option<PaneModel>,
  pub(crate) selections: Vec<SelectionModel>,
  pub(crate) pivot_selections: usize,
  pub(crate) has_extensions: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PaneModel {
  pub(crate) horizontal_split: Option<f64>,
  pub(crate) vertical_split: Option<f64>,
  pub(crate) top_left_cell: Option<String>,
  pub(crate) active_pane: Option<x::PaneValues>,
  pub(crate) state: Option<x::PaneStateValues>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SelectionModel {
  pub(crate) pane: Option<x::PaneValues>,
  pub(crate) active_cell: Option<String>,
  pub(crate) active_cell_id: Option<u32>,
  pub(crate) sequence_of_references: Vec<String>,
}

impl SheetViewCatalog {
  pub(crate) fn from_worksheet(worksheet: &x::Worksheet) -> Self {
    // SheetViewSettings imports sheetView, pane, and selection before final
    // workbook view settings. Keep OOXML values typed; LO defaulting and pane
    // split conversion belong to the view finalizer.
    let Some(sheet_views) = &worksheet.sheet_views else {
      return Self::default();
    };

    Self {
      views: sheet_views
        .sheet_view
        .iter()
        .map(SheetViewModel::from_sheet_view)
        .collect(),
      has_extensions: sheet_views.extension_list.is_some(),
    }
  }
}

impl SheetViewModel {
  fn from_sheet_view(view: &x::SheetView) -> Self {
    Self {
      workbook_view_id: view.workbook_view_id,
      view_type: view.view,
      top_left_cell: view.top_left_cell.clone(),
      color_id: view.color_id,
      zoom_scale: view.zoom_scale,
      zoom_scale_normal: view.zoom_scale_normal,
      zoom_scale_sheet_layout_view: view.zoom_scale_sheet_layout_view,
      zoom_scale_page_layout_view: view.zoom_scale_page_layout_view,
      window_protection: view.window_protection.map(|value| value.as_bool()),
      show_formulas: view.show_formulas.map(|value| value.as_bool()),
      show_grid_lines: view.show_grid_lines.map(|value| value.as_bool()),
      show_row_col_headers: view.show_row_col_headers.map(|value| value.as_bool()),
      show_zeros: view.show_zeros.map(|value| value.as_bool()),
      right_to_left: view.right_to_left.map(|value| value.as_bool()),
      tab_selected: view.tab_selected.map(|value| value.as_bool()),
      show_outline_symbols: view.show_outline_symbols.map(|value| value.as_bool()),
      default_grid_color: view.default_grid_color.map(|value| value.as_bool()),
      pane: view.pane.as_ref().map(PaneModel::from_pane),
      selections: view
        .selection
        .iter()
        .map(SelectionModel::from_selection)
        .collect(),
      pivot_selections: view.pivot_selection.len(),
      has_extensions: view.extension_list.is_some(),
    }
  }
}

impl PaneModel {
  fn from_pane(pane: &x::Pane) -> Self {
    Self {
      horizontal_split: pane.horizontal_split,
      vertical_split: pane.vertical_split,
      top_left_cell: pane.top_left_cell.clone(),
      active_pane: pane.active_pane,
      state: pane.state,
    }
  }
}

impl SelectionModel {
  fn from_selection(selection: &x::Selection) -> Self {
    Self {
      pane: selection.pane,
      active_cell: selection.active_cell.clone(),
      active_cell_id: selection.active_cell_id,
      sequence_of_references: selection.sequence_of_references.clone().unwrap_or_default(),
    }
  }
}
