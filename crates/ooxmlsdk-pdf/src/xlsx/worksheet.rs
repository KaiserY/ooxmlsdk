use std::collections::HashMap;

use ooxmlsdk::parts::chartsheet_part::ChartsheetPart;
use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::parts::worksheet_part::WorksheetPart;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;
use quick_xml::escape::unescape;
use quick_xml::events::Event;

use super::comments::CommentsCatalog;
use super::drawing::DrawingResourceCatalog;
use super::object_resources::WorksheetObjectResourceCatalog;
use super::page_settings::CalcPageSettings;
use super::pivot::PivotTableCatalog;
use super::query::QueryTableCatalog;
use super::sheet_conditions::SheetConditionCatalog;
use super::sheet_objects::SheetObjectCatalog;
use super::sheet_relationships::SheetRelationshipCatalog;
use super::sheet_settings::SheetSettingsCatalog;
use super::sheet_view::SheetViewCatalog;
use super::styles::StylesCatalog;
use super::table::TableResourceCatalog;
use super::text::decode_excel_escaped_text;
use super::workbook::{SharedStringModel, SharedStringRun};
use crate::error::Result;
use crate::units;

const CALC_DIGIT_WIDTH_MM: f32 = 2.0;
const CALC_BASE_COLUMN_PADDING_PX: f32 = 5.0;

#[derive(Clone, Debug)]
pub(crate) struct CalcSheet {
  pub(crate) workbook_index: usize,
  pub(crate) name: String,
  pub(crate) relationship_id: String,
  pub(crate) sheet_id: u32,
  pub(crate) sheet_type: SheetType,
  pub(crate) state: Option<x::SheetStateValues>,
  pub(crate) active: bool,
  pub(crate) page_settings: CalcPageSettings,
  pub(crate) metrics: SheetMetrics,
  pub(crate) chartsheet_metrics: Option<ChartsheetMetrics>,
  pub(crate) resources: SheetResourceCatalog,
  pub(crate) rows: Vec<CalcRow>,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub(crate) struct CellAddress {
  pub(crate) col: u32,
  pub(crate) row: u32,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct CellRange {
  pub(crate) start: CellAddress,
  pub(crate) end: CellAddress,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct CellRect {
  pub(crate) x_pt: f32,
  pub(crate) y_pt: f32,
  pub(crate) width_pt: f32,
  pub(crate) height_pt: f32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SheetType {
  Worksheet,
  Chartsheet,
  Unresolved,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SheetResourceCatalog {
  pub(crate) drawings: Vec<DrawingResourceCatalog>,
  pub(crate) object_resources: WorksheetObjectResourceCatalog,
  pub(crate) comments: CommentsCatalog,
  pub(crate) tables: Vec<TableResourceCatalog>,
  pub(crate) pivot_tables: PivotTableCatalog,
  pub(crate) query_tables: QueryTableCatalog,
  pub(crate) relationships: SheetRelationshipCatalog,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SheetMetrics {
  pub(crate) dimension: Option<String>,
  pub(crate) settings: SheetSettingsCatalog,
  pub(crate) views: SheetViewCatalog,
  pub(crate) format: SheetFormatModel,
  pub(crate) digit_width_pt: f32,
  pub(crate) columns: Vec<ColumnModel>,
  pub(crate) merged_ranges: Vec<String>,
  pub(crate) hyperlinks: Vec<HyperlinkModel>,
  pub(crate) row_breaks: Vec<PageBreakModel>,
  pub(crate) column_breaks: Vec<PageBreakModel>,
  pub(crate) conditions: SheetConditionCatalog,
  pub(crate) objects: SheetObjectCatalog,
  pub(crate) protected_ranges: usize,
  pub(crate) scenarios: usize,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct ChartsheetMetrics {
  pub(crate) published: bool,
  pub(crate) code_name: Option<String>,
  pub(crate) has_tab_color: bool,
  pub(crate) views: usize,
  pub(crate) selected_views: usize,
  pub(crate) zoom_to_fit_views: usize,
  pub(crate) view_extensions: usize,
  pub(crate) custom_views: usize,
  pub(crate) custom_view_flags: usize,
  pub(crate) protection_flags: usize,
  pub(crate) web_publish_items: usize,
  pub(crate) has_extensions: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SheetFormatModel {
  pub(crate) base_column_width: Option<u32>,
  pub(crate) default_column_width: Option<f64>,
  pub(crate) default_row_height: f64,
  pub(crate) custom_height: bool,
  pub(crate) zero_height: bool,
  pub(crate) thick_top: bool,
  pub(crate) thick_bottom: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ColumnModel {
  pub(crate) first: u32,
  pub(crate) last: u32,
  pub(crate) width: Option<f64>,
  pub(crate) style_index: Option<u32>,
  pub(crate) hidden: bool,
  pub(crate) best_fit: bool,
  pub(crate) custom_width: bool,
  pub(crate) phonetic: bool,
  pub(crate) outline_level: u8,
  pub(crate) collapsed: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct HyperlinkModel {
  pub(crate) reference: String,
  pub(crate) relationship_id: Option<String>,
  pub(crate) location: Option<String>,
  pub(crate) display: Option<String>,
  pub(crate) tooltip: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PageBreakModel {
  pub(crate) id: u32,
  pub(crate) min: u32,
  pub(crate) max: u32,
  pub(crate) manual: bool,
  pub(crate) pivot: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct CalcRow {
  pub(crate) row_index: Option<u32>,
  pub(crate) height: Option<f64>,
  pub(crate) custom_height: bool,
  pub(crate) style_index: Option<u32>,
  pub(crate) hidden: bool,
  pub(crate) cells: Vec<CalcCell>,
}

#[derive(Clone, Debug)]
pub(crate) struct CalcCell {
  pub(crate) reference: Option<String>,
  pub(crate) style_index: Option<u32>,
  pub(crate) data_type: Option<x::CellValues>,
  pub(crate) cell_meta_index: Option<u32>,
  pub(crate) value_meta_index: Option<u32>,
  pub(crate) show_phonetic: bool,
  pub(crate) formula: Option<FormulaModel>,
  pub(crate) cached_value: Option<String>,
  pub(crate) display_text: String,
  pub(crate) rich_text_runs: Vec<SharedStringRun>,
  pub(crate) has_extensions: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FormulaModel {
  pub(crate) formula_type: x::CellFormulaValues,
  pub(crate) reference: Option<String>,
  pub(crate) shared_index: Option<u32>,
  pub(crate) text: String,
  pub(crate) always_calculate_array: bool,
  pub(crate) calculate_cell: bool,
  pub(crate) data_table_2d: bool,
  pub(crate) data_table_row: bool,
  pub(crate) input1_deleted: bool,
  pub(crate) input2_deleted: bool,
  pub(crate) input1_reference: Option<String>,
  pub(crate) input2_reference: Option<String>,
  pub(crate) assigns_value_to_name: bool,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct RawWorksheetData {
  pub(crate) cell_values: HashMap<String, String>,
  pub(crate) odd_header: Option<String>,
  pub(crate) odd_footer: Option<String>,
  pub(crate) even_header: Option<String>,
  pub(crate) even_footer: Option<String>,
  pub(crate) first_header: Option<String>,
  pub(crate) first_footer: Option<String>,
}

impl CalcSheet {
  pub(crate) fn from_worksheet(
    workbook_index: usize,
    name: String,
    sheet_id: u32,
    relationship_id: String,
    state: Option<x::SheetStateValues>,
    active: bool,
    worksheet: x::Worksheet,
    resources: SheetResourceCatalog,
    shared_strings: &[SharedStringModel],
    _styles: &StylesCatalog,
    raw_values: &HashMap<String, String>,
    mso_document: bool,
  ) -> Self {
    let page_settings = CalcPageSettings::from_worksheet(&worksheet);
    let metrics = SheetMetrics::from_worksheet(&worksheet, mso_document);
    Self {
      workbook_index,
      name,
      relationship_id,
      sheet_id,
      sheet_type: SheetType::Worksheet,
      state,
      active,
      page_settings,
      metrics,
      chartsheet_metrics: None,
      resources,
      rows: worksheet_rows(&worksheet, shared_strings, raw_values, mso_document),
    }
  }

  pub(crate) fn from_chartsheet(
    workbook_index: usize,
    name: String,
    sheet_id: u32,
    relationship_id: String,
    state: Option<x::SheetStateValues>,
    active: bool,
    chartsheet: x::Chartsheet,
    resources: SheetResourceCatalog,
  ) -> Self {
    let page_settings = CalcPageSettings::from_chartsheet(&chartsheet);
    Self {
      workbook_index,
      name,
      relationship_id,
      sheet_id,
      sheet_type: SheetType::Chartsheet,
      state,
      active,
      page_settings,
      metrics: SheetMetrics::default(),
      chartsheet_metrics: Some(ChartsheetMetrics::from_chartsheet(&chartsheet)),
      resources,
      rows: Vec::new(),
    }
  }

  pub(crate) fn unresolved(
    workbook_index: usize,
    name: String,
    sheet_id: u32,
    relationship_id: String,
    state: Option<x::SheetStateValues>,
    active: bool,
  ) -> Self {
    Self {
      workbook_index,
      name,
      relationship_id,
      sheet_id,
      sheet_type: SheetType::Unresolved,
      state,
      active,
      page_settings: CalcPageSettings::default(),
      metrics: SheetMetrics::default(),
      chartsheet_metrics: None,
      resources: SheetResourceCatalog::default(),
      rows: Vec::new(),
    }
  }

  pub(crate) fn visible(&self) -> bool {
    self.active
      || !matches!(
        self.state,
        Some(x::SheetStateValues::Hidden | x::SheetStateValues::VeryHidden)
      )
  }

  pub(crate) fn used_range(&self) -> Option<CellRange> {
    let mut used: Option<CellRange> = None;
    for (row_position, row) in self.rows.iter().enumerate() {
      let row_index = row.row_index.unwrap_or(row_position as u32 + 1);
      for (cell_position, cell) in row.cells.iter().enumerate() {
        if !cell.has_print_data() {
          continue;
        }
        let address = cell.address().unwrap_or(CellAddress {
          col: cell_position as u32 + 1,
          row: row_index,
        });
        let Some(print_address) = super::pivot::pivot_print_address(self, address) else {
          continue;
        };
        used = Some(match used {
          Some(range) => range.union_address(print_address),
          None => CellRange::single(print_address),
        });
      }
    }
    for pivot in &self.resources.pivot_tables.tables {
      used = Some(match used {
        Some(used) => used.union(pivot.output_geometry.whole_range),
        None => pivot.output_geometry.whole_range,
      });
    }
    for anchor in self
      .resources
      .drawings
      .iter()
      .flat_map(|drawing| drawing.anchors.iter())
    {
      if anchor.object.hidden || !anchor.print_with_sheet {
        continue;
      }
      let Some(range) = self.drawing_anchor_range(anchor) else {
        continue;
      };
      used = Some(match used {
        Some(used) => used.union(range),
        None => range,
      });
    }
    for shape in self
      .resources
      .object_resources
      .vml_drawings
      .iter()
      .flat_map(|drawing| drawing.shapes.iter())
    {
      if shape.hidden || !shape.print_object {
        continue;
      }
      let Some(anchor) = shape.anchor else {
        continue;
      };
      used = Some(match used {
        Some(used) => used.union(anchor.cell_range()),
        None => anchor.cell_range(),
      });
    }
    used
  }

  fn drawing_anchor_range(&self, anchor: &super::drawing::DrawingAnchorModel) -> Option<CellRange> {
    match anchor.kind {
      super::drawing::DrawingAnchorKind::TwoCell => {
        let start = marker_address(anchor.from.as_ref()?);
        let end = marker_address(anchor.to.as_ref()?);
        Some(CellRange::new(start, end))
      }
      super::drawing::DrawingAnchorKind::OneCell => {
        let start = marker_address(anchor.from.as_ref()?);
        Some(CellRange::single(start))
      }
      super::drawing::DrawingAnchorKind::Absolute => {
        Some(CellRange::single(CellAddress { col: 1, row: 1 }))
      }
    }
  }

  pub(crate) fn cell_rect(&self, address: CellAddress) -> CellRect {
    self.cell_rect_with_merge(address, true)
  }

  pub(crate) fn cell_rect_with_merge(
    &self,
    address: CellAddress,
    include_merged_cell: bool,
  ) -> CellRect {
    let x_pt = (1..address.col)
      .map(|column| self.column_width_pt(column))
      .sum();
    let y_pt = (1..address.row).map(|row| self.row_height_pt(row)).sum();
    let end = if include_merged_cell {
      self
        .merged_range_for_cell(address)
        .filter(|range| range.start == address)
        .map_or(address, |range| range.end)
    } else {
      address
    };
    CellRect {
      x_pt,
      y_pt,
      width_pt: (address.col..=end.col)
        .map(|column| self.column_width_pt(column))
        .sum(),
      height_pt: (address.row..=end.row)
        .map(|row| self.row_height_pt(row))
        .sum(),
    }
  }

  pub(crate) fn merged_range_for_cell(&self, address: CellAddress) -> Option<CellRange> {
    self
      .metrics
      .merged_ranges
      .iter()
      .filter_map(|reference| CellRange::parse_a1_range(reference))
      .find(|range| range.contains(address))
  }

  pub(crate) fn is_covered_merged_cell(&self, address: CellAddress) -> bool {
    self
      .merged_range_for_cell(address)
      .is_some_and(|range| range.start != address)
  }

  pub(crate) fn range_rect(&self, range: CellRange) -> CellRect {
    let start = self.cell_rect_with_merge(range.start, false);
    let width_pt = (range.start.col..=range.end.col)
      .map(|column| self.column_width_pt(column))
      .sum();
    let height_pt = (range.start.row..=range.end.row)
      .map(|row| self.row_height_pt(row))
      .sum();
    CellRect {
      width_pt,
      height_pt,
      ..start
    }
  }

  pub(crate) fn marker_position_pt(
    &self,
    marker: &super::drawing::DrawingMarkerModel,
  ) -> (f32, f32) {
    let column = u32::try_from(marker.column).unwrap_or(0).saturating_add(1);
    let row = u32::try_from(marker.row).unwrap_or(0).saturating_add(1);
    let base = self.cell_rect(CellAddress { col: column, row });
    (
      base.x_pt + units::emu_to_points(marker.column_offset_emu),
      base.y_pt + units::emu_to_points(marker.row_offset_emu),
    )
  }

  pub(crate) fn column_width_pt(&self, column: u32) -> f32 {
    if let Some(model) = self
      .metrics
      .columns
      .iter()
      .find(|model| column >= model.first && column <= model.last)
    {
      if model.hidden {
        return 0.0;
      }
      if let Some(width) = model.width {
        return digit_width_to_lo_points(width as f32, self.metrics.digit_width_pt);
      }
    }
    if self
      .metrics
      .columns
      .iter()
      .any(|model| model.hidden && column >= model.first && column <= model.last)
    {
      return 0.0;
    }
    self
      .metrics
      .format
      .default_column_width_points(self.metrics.digit_width_pt)
  }

  pub(crate) fn row_height_pt(&self, row_index: u32) -> f32 {
    if self.metrics.format.zero_height {
      return 0.0;
    }
    if let Some(row) = self
      .rows
      .iter()
      .find(|row| row.row_index.unwrap_or(0) == row_index)
    {
      if row.hidden {
        return 0.0;
      }
      if let Some(height) = row.height {
        return height as f32;
      }
    }
    self.metrics.format.default_row_height as f32
  }

  pub(crate) fn column_style_index(&self, column: u32) -> Option<u32> {
    self
      .metrics
      .columns
      .iter()
      .find(|model| column >= model.first && column <= model.last)
      .and_then(|model| model.style_index)
  }

  pub(crate) fn effective_cell_style_index(
    &self,
    row: &CalcRow,
    cell: &CalcCell,
    address: CellAddress,
  ) -> Option<u32> {
    cell
      .style_index
      .or(row.style_index)
      .or_else(|| self.column_style_index(address.col))
  }
}

fn marker_address(marker: &super::drawing::DrawingMarkerModel) -> CellAddress {
  CellAddress {
    col: u32::try_from(marker.column).unwrap_or(0).saturating_add(1),
    row: u32::try_from(marker.row).unwrap_or(0).saturating_add(1),
  }
}

impl CalcCell {
  fn has_print_data(&self) -> bool {
    self.formula.is_some()
      || self.cached_value.is_some()
      || !self.display_text.is_empty()
      || self.data_type.is_some()
  }
}

impl CellAddress {
  pub(crate) fn parse_a1(reference: &str) -> Option<Self> {
    let reference = reference
      .rsplit(['!', ':'])
      .next()
      .unwrap_or(reference)
      .trim_matches('\'')
      .trim_start_matches('$');
    let mut col = 0u32;
    let mut row = 0u32;
    let mut seen_digit = false;
    for ch in reference.chars().filter(|ch| *ch != '$') {
      if ch.is_ascii_alphabetic() && !seen_digit {
        col = col
          .saturating_mul(26)
          .saturating_add(ch.to_ascii_uppercase() as u32 - 'A' as u32 + 1);
      } else if ch.is_ascii_digit() {
        seen_digit = true;
        row = row
          .saturating_mul(10)
          .saturating_add(ch as u32 - '0' as u32);
      } else {
        return None;
      }
    }
    (col > 0 && row > 0).then_some(Self { col, row })
  }
}

impl CellRange {
  pub(crate) fn single(address: CellAddress) -> Self {
    Self {
      start: address,
      end: address,
    }
  }

  pub(crate) fn parse_a1_range(reference: &str) -> Option<Self> {
    let reference = reference.trim();
    let reference = reference
      .rsplit_once('!')
      .map_or(reference, |(_, range)| range)
      .trim_matches('\'');
    let (start, end) = reference.split_once(':').unwrap_or((reference, reference));
    let start = CellAddress::parse_a1(start)?;
    let end = CellAddress::parse_a1(end)?;
    Some(Self::new(start, end))
  }

  pub(crate) fn new(start: CellAddress, end: CellAddress) -> Self {
    Self {
      start: CellAddress {
        col: start.col.min(end.col),
        row: start.row.min(end.row),
      },
      end: CellAddress {
        col: start.col.max(end.col),
        row: start.row.max(end.row),
      },
    }
  }

  pub(crate) fn contains(&self, address: CellAddress) -> bool {
    address.col >= self.start.col
      && address.col <= self.end.col
      && address.row >= self.start.row
      && address.row <= self.end.row
  }

  pub(crate) fn intersects(&self, other: Self) -> bool {
    self.start.col <= other.end.col
      && self.end.col >= other.start.col
      && self.start.row <= other.end.row
      && self.end.row >= other.start.row
  }

  pub(crate) fn union_address(self, address: CellAddress) -> Self {
    Self {
      start: CellAddress {
        col: self.start.col.min(address.col),
        row: self.start.row.min(address.row),
      },
      end: CellAddress {
        col: self.end.col.max(address.col),
        row: self.end.row.max(address.row),
      },
    }
  }

  pub(crate) fn union(self, other: Self) -> Self {
    Self {
      start: CellAddress {
        col: self.start.col.min(other.start.col),
        row: self.start.row.min(other.start.row),
      },
      end: CellAddress {
        col: self.end.col.max(other.end.col),
        row: self.end.row.max(other.end.row),
      },
    }
  }

  pub(crate) fn cell_count_hint(&self) -> u64 {
    u64::from(self.end.col - self.start.col + 1) * u64::from(self.end.row - self.start.row + 1)
  }
}

impl ChartsheetMetrics {
  fn from_chartsheet(chartsheet: &x::Chartsheet) -> Self {
    // Source: LibreOffice sc/source/filter/oox/chartsheetfragment.cxx imports
    // chartsheet properties/protection/views through WorksheetSettings and
    // SheetViewSettings before final worksheet import.
    let properties = chartsheet.chart_sheet_properties.as_deref();
    let protection = chartsheet.chart_sheet_protection.as_ref();
    Self {
      published: properties
        .and_then(|properties| properties.published)
        .is_some_and(|value| value.as_bool()),
      code_name: properties.and_then(|properties| properties.code_name.clone()),
      has_tab_color: properties.is_some_and(|properties| properties.tab_color.is_some()),
      views: chartsheet.chart_sheet_views.chart_sheet_view.len(),
      selected_views: chartsheet
        .chart_sheet_views
        .chart_sheet_view
        .iter()
        .filter(|view| view.tab_selected.is_some_and(|value| value.as_bool()))
        .count(),
      zoom_to_fit_views: chartsheet
        .chart_sheet_views
        .chart_sheet_view
        .iter()
        .filter(|view| view.zoom_to_fit.is_some_and(|value| value.as_bool()))
        .count(),
      view_extensions: usize::from(chartsheet.chart_sheet_views.extension_list.is_some())
        + chartsheet
          .chart_sheet_views
          .chart_sheet_view
          .iter()
          .filter(|view| view.extension_list.is_some())
          .count(),
      custom_views: chartsheet
        .custom_chartsheet_views
        .as_ref()
        .map_or(0, |views| views.custom_chartsheet_view.len()),
      custom_view_flags: chartsheet
        .custom_chartsheet_views
        .as_ref()
        .map_or(0, |views| {
          views
            .custom_chartsheet_view
            .iter()
            .map(|view| {
              usize::from(view.scale.is_some())
                + usize::from(view.state.is_some())
                + usize::from(view.zoom_to_fit.is_some_and(|value| value.as_bool()))
                + usize::from(view.page_margins.is_some())
                + usize::from(view.chart_sheet_page_setup.is_some())
                + usize::from(view.header_footer.is_some())
                + view.guid.len()
            })
            .sum()
        }),
      protection_flags: protection.map_or(0, |protection| {
        usize::from(protection.password.is_some())
          + usize::from(protection.algorithm_name.is_some())
          + usize::from(protection.hash_value.is_some())
          + usize::from(protection.salt_value.is_some())
          + usize::from(protection.spin_count.is_some())
          + usize::from(protection.content.is_some_and(|value| value.as_bool()))
          + usize::from(protection.objects.is_some_and(|value| value.as_bool()))
      }),
      web_publish_items: chartsheet
        .web_publish_items
        .as_ref()
        .map_or(0, |items| items.web_publish_item.len()),
      has_extensions: chartsheet.extension_list.is_some(),
    }
  }
}

impl SheetMetrics {
  fn from_worksheet(worksheet: &x::Worksheet, mso_document: bool) -> Self {
    // Source: LibreOffice sc/source/filter/oox/worksheetfragment.cxx
    // WorksheetFragment imports dimensions, sheetFormatPr, cols,
    // mergeCells, hyperlinks, rowBreaks, and colBreaks before page layout.
    Self {
      dimension: worksheet
        .sheet_dimension
        .as_ref()
        .map(|dimension| dimension.reference.clone()),
      settings: SheetSettingsCatalog::from_worksheet(worksheet),
      views: SheetViewCatalog::from_worksheet(worksheet),
      format: worksheet
        .sheet_format_properties
        .as_ref()
        .map(|format| SheetFormatModel::from_sheet_format_properties(format, mso_document))
        .unwrap_or_default(),
      digit_width_pt: default_digit_width_pt(),
      columns: worksheet
        .columns
        .iter()
        .flat_map(|columns| columns.column.iter().map(ColumnModel::from_column))
        .collect(),
      merged_ranges: worksheet
        .merge_cells
        .as_ref()
        .map(|merge_cells| {
          merge_cells
            .merge_cell
            .iter()
            .map(|merge_cell| merge_cell.reference.clone())
            .collect()
        })
        .unwrap_or_default(),
      hyperlinks: worksheet
        .hyperlinks
        .as_ref()
        .map(|hyperlinks| {
          hyperlinks
            .hyperlink
            .iter()
            .map(HyperlinkModel::from_hyperlink)
            .collect()
        })
        .unwrap_or_default(),
      row_breaks: worksheet
        .row_breaks
        .as_ref()
        .map(|breaks| {
          breaks
            .r#break
            .iter()
            .map(PageBreakModel::from_break)
            .collect()
        })
        .unwrap_or_default(),
      column_breaks: worksheet
        .column_breaks
        .as_ref()
        .map(|breaks| {
          breaks
            .r#break
            .iter()
            .map(PageBreakModel::from_break)
            .collect()
        })
        .unwrap_or_default(),
      conditions: SheetConditionCatalog::from_worksheet(worksheet),
      objects: SheetObjectCatalog::from_worksheet(worksheet),
      protected_ranges: worksheet
        .protected_ranges
        .as_ref()
        .map_or(0, |ranges| ranges.protected_range.len()),
      scenarios: worksheet
        .scenarios
        .as_ref()
        .map_or(0, |scenarios| scenarios.scenario.len()),
    }
  }
}

impl SheetFormatModel {
  fn from_sheet_format_properties(format: &x::SheetFormatProperties, mso_document: bool) -> Self {
    Self {
      base_column_width: format.base_column_width,
      default_column_width: format.default_column_width,
      default_row_height: if mso_document {
        mso_row_height_pt(format.default_row_height)
      } else {
        format.default_row_height
      },
      custom_height: format.custom_height.is_some_and(|value| value.as_bool()),
      zero_height: format.zero_height.is_some_and(|value| value.as_bool()),
      thick_top: format.thick_top.is_some_and(|value| value.as_bool()),
      thick_bottom: format.thick_bottom.is_some_and(|value| value.as_bool()),
    }
  }

  fn default_column_width_points(&self, digit_width_pt: f32) -> f32 {
    if let Some(width) = self.default_column_width {
      return digit_width_to_points(width as f32, digit_width_pt);
    }
    // Source: LibreOffice sc/source/filter/oox/worksheethelper.cxx
    // setBaseColumnWidth() uses baseColWidth plus 5 screen pixels converted
    // through UnitConverter after UnitConverter::finalizeImport() has replaced
    // Unit::Digit with the default font's maximum digit width.
    let base = self.base_column_width.unwrap_or(8) as f32;
    digit_width_to_lo_points(
      base + CALC_BASE_COLUMN_PADDING_PX * screen_pixel_width_pt() / digit_width_pt,
      digit_width_pt,
    )
  }
}

impl ColumnModel {
  fn from_column(column: &x::Column) -> Self {
    Self {
      first: column.min,
      last: column.max,
      width: column.width,
      style_index: column.style,
      hidden: column.hidden.is_some_and(|value| value.as_bool()),
      best_fit: column.best_fit.is_some_and(|value| value.as_bool()),
      custom_width: column.custom_width.is_some_and(|value| value.as_bool()),
      phonetic: column.phonetic.is_some_and(|value| value.as_bool()),
      outline_level: column.outline_level.unwrap_or(0),
      collapsed: column.collapsed.is_some_and(|value| value.as_bool()),
    }
  }
}

impl HyperlinkModel {
  fn from_hyperlink(hyperlink: &x::Hyperlink) -> Self {
    Self {
      reference: hyperlink.reference.clone(),
      relationship_id: hyperlink.id.clone(),
      location: hyperlink.location.clone(),
      display: hyperlink.display.clone(),
      tooltip: hyperlink.tooltip.clone(),
    }
  }
}

impl PageBreakModel {
  fn from_break(page_break: &x::Break) -> Self {
    let id = page_break.id.unwrap_or(0);
    Self {
      id,
      min: page_break.min.unwrap_or(id),
      max: page_break.max.unwrap_or(id),
      manual: page_break
        .manual_page_break
        .is_some_and(|value| value.as_bool()),
      pivot: page_break
        .pivot_table_page_break
        .is_some_and(|value| value.as_bool()),
    }
  }
}

impl SheetResourceCatalog {
  pub(crate) fn from_worksheet_part(
    package: &mut SpreadsheetDocument,
    part: &WorksheetPart,
    sheet_name: &str,
    worksheet: &x::Worksheet,
    raw_values: &HashMap<String, String>,
    shared_strings: &[SharedStringModel],
    styles: &StylesCatalog,
    date_1904: bool,
  ) -> Result<Self> {
    let drawings = part
      .drawings_part(package)
      .map(|drawing| DrawingResourceCatalog::from_part(package, &drawing))
      .transpose()?
      .map(|drawing| vec![drawing])
      .unwrap_or_default();
    let table_parts = part.table_definition_parts(package).collect::<Vec<_>>();
    let tables = table_parts
      .iter()
      .map(|table| TableResourceCatalog::from_part(package, table))
      .collect::<Result<Vec<_>>>()?;
    let comments_part = part.worksheet_comments_part(package);
    let threaded_comment_parts = part
      .worksheet_threaded_comments_parts(package)
      .collect::<Vec<_>>();
    let comments =
      CommentsCatalog::from_worksheet_part(package, comments_part, threaded_comment_parts)?;
    let pivot_table_parts = part.pivot_table_parts(package).collect::<Vec<_>>();
    let pivot_tables = PivotTableCatalog::from_parts(
      package,
      &pivot_table_parts,
      worksheet,
      raw_values,
      sheet_name,
      shared_strings,
      styles,
      date_1904,
    )?;
    let query_table_parts = part.query_table_parts(package).collect::<Vec<_>>();
    let query_tables = QueryTableCatalog::from_parts(package, &query_table_parts)?;
    let relationships = SheetRelationshipCatalog::from_worksheet_part(package, part)?;
    let object_resources = WorksheetObjectResourceCatalog::from_worksheet_part(package, part)?;
    Ok(Self {
      drawings,
      object_resources,
      comments,
      tables,
      pivot_tables,
      query_tables,
      relationships,
    })
  }

  pub(crate) fn from_chartsheet_part(
    package: &mut SpreadsheetDocument,
    part: &ChartsheetPart,
  ) -> Result<Self> {
    let drawings = part
      .drawings_part(package)
      .map(|drawing| DrawingResourceCatalog::from_part(package, &drawing))
      .transpose()?
      .map(|drawing| vec![drawing])
      .unwrap_or_default();
    Ok(Self {
      drawings,
      object_resources: WorksheetObjectResourceCatalog::from_chartsheet_part(package, part),
      ..Self::default()
    })
  }
}

pub(crate) fn worksheet_raw_data(xml: &str) -> RawWorksheetData {
  let mut reader = quick_xml::Reader::from_str(xml);
  reader.config_mut().trim_text(false);
  let mut data = RawWorksheetData::default();
  let mut current_cell: Option<String> = None;
  let mut in_value = false;
  let mut header_footer_tag: Option<&'static str> = None;
  loop {
    match reader.read_event() {
      Ok(Event::Start(event)) => {
        let name = event.name();
        if name.as_ref().ends_with(b"c") {
          current_cell = event
            .attributes()
            .flatten()
            .find(|attr| attr.key.as_ref().ends_with(b"r"))
            .map(|attr| String::from_utf8_lossy(attr.value.as_ref()).into_owned());
        } else if name.as_ref().ends_with(b"v") {
          in_value = current_cell.is_some();
        } else {
          header_footer_tag = raw_header_footer_tag(name.as_ref());
        }
      }
      Ok(Event::Text(event)) if in_value => {
        if let Some(cell) = &current_cell {
          data.cell_values.insert(
            cell.clone(),
            super::text::decode_excel_escaped_text(&String::from_utf8_lossy(event.as_ref())),
          );
        }
      }
      Ok(Event::Text(event)) => {
        if let Some(tag) = header_footer_tag {
          let raw = String::from_utf8_lossy(event.as_ref());
          let text = unescape(&raw)
            .map(|value| value.into_owned())
            .unwrap_or_else(|_| raw.into_owned());
          match tag {
            "oddHeader" => data.odd_header = Some(text),
            "oddFooter" => data.odd_footer = Some(text),
            "evenHeader" => data.even_header = Some(text),
            "evenFooter" => data.even_footer = Some(text),
            "firstHeader" => data.first_header = Some(text),
            "firstFooter" => data.first_footer = Some(text),
            _ => {}
          }
        }
      }
      Ok(Event::End(event)) => {
        let name = event.name();
        if name.as_ref().ends_with(b"v") {
          in_value = false;
        } else if name.as_ref().ends_with(b"c") {
          current_cell = None;
        } else if raw_header_footer_tag(name.as_ref()).is_some() {
          header_footer_tag = None;
        }
      }
      Ok(Event::Eof) => break,
      Err(_) => break,
      _ => {}
    }
  }
  data
}

fn raw_header_footer_tag(name: &[u8]) -> Option<&'static str> {
  if name.ends_with(b"oddHeader") {
    Some("oddHeader")
  } else if name.ends_with(b"oddFooter") {
    Some("oddFooter")
  } else if name.ends_with(b"evenHeader") {
    Some("evenHeader")
  } else if name.ends_with(b"evenFooter") {
    Some("evenFooter")
  } else if name.ends_with(b"firstHeader") {
    Some("firstHeader")
  } else if name.ends_with(b"firstFooter") {
    Some("firstFooter")
  } else {
    None
  }
}

fn worksheet_rows(
  worksheet: &x::Worksheet,
  shared_strings: &[SharedStringModel],
  raw_values: &HashMap<String, String>,
  mso_document: bool,
) -> Vec<CalcRow> {
  worksheet
    .sheet_data
    .row
    .iter()
    .enumerate()
    .map(|(row_position, row)| {
      let row_index = row.row_index.unwrap_or(row_position as u32 + 1);
      let mut current_col = 0u32;
      let cells = row
        .cell
        .iter()
        .map(|cell| {
          // Source: LibreOffice sc/source/filter/oox/sheetdatacontext.cxx
          // SheetDataContext::importCell falls back to the next column in the
          // current row when XML_r is missing or cannot be converted to an A1
          // address. Malformed producer output still imports as ordered cells.
          let address = cell
            .cell_reference
            .as_deref()
            .and_then(CellAddress::parse_a1)
            .inspect(|address| current_col = address.col)
            .unwrap_or_else(|| {
              current_col = current_col.saturating_add(1);
              CellAddress {
                col: current_col,
                row: row_index,
              }
            });
          CalcCell::from_cell(cell, shared_strings, raw_values, Some(address))
        })
        .collect();
      CalcRow {
        row_index: Some(row_index),
        height: row.height.map(|height| {
          if mso_document {
            mso_row_height_pt(height)
          } else {
            height
          }
        }),
        custom_height: row.custom_height.is_some_and(|value| value.as_bool()),
        style_index: row.style_index,
        hidden: row.hidden.is_some_and(|value| value.as_bool()),
        cells,
      }
    })
    .collect()
}

fn mso_row_height_pt(height: f64) -> f64 {
  if height > 0.0 {
    // Source: LibreOffice sc/source/filter/oox/sheetdatacontext.cxx and
    // worksheetfragment.cxx round MSO OOXML row heights down to 0.75pt.
    height - height % 0.75
  } else {
    height
  }
}

fn digit_width_to_points(value: f32, digit_width_pt: f32) -> f32 {
  // Source: LibreOffice sc/source/filter/oox/unitconverter.cxx:
  // UnitConverter::scaleValue(value, Unit::Digit, Unit::Twip).
  value * digit_width_pt
}

fn digit_width_to_lo_points(value: f32, digit_width_pt: f32) -> f32 {
  (digit_width_to_points(value, digit_width_pt) * units::TWIPS_PER_POINT).round()
    / units::TWIPS_PER_POINT
}

fn default_digit_width_pt() -> f32 {
  // Source: LibreOffice sc/source/filter/oox/unitconverter.cxx initializes
  // Unit::Digit to 2mm and only replaces it from the document ReferenceDevice
  // XFont. ooxmlsdk-pdf has no LO reference device, so keep the LO fallback
  // instead of measuring through an unrelated local text backend.
  units::millimeters_to_points(CALC_DIGIT_WIDTH_MM)
}

fn screen_pixel_width_pt() -> f32 {
  // Source: LibreOffice sc/source/filter/oox/unitconverter.cxx initializes
  // Unit::ScreenX from GraphicHelper device pixels. The headless Calc export
  // path used by the upstream fixtures has a 96dpi reference device, i.e. one
  // screen pixel is 0.75pt.
  units::POINTS_PER_INCH / 96.0
}

impl CalcCell {
  pub(crate) fn address(&self) -> Option<CellAddress> {
    self.reference.as_deref().and_then(CellAddress::parse_a1)
  }

  fn from_cell(
    cell: &x::Cell,
    shared_strings: &[SharedStringModel],
    raw_values: &HashMap<String, String>,
    resolved_address: Option<CellAddress>,
  ) -> Self {
    let raw_value = cell
      .cell_reference
      .as_ref()
      .and_then(|reference| raw_values.get(reference.as_str()));
    let cached_value = cell
      .cell_value
      .as_ref()
      .and_then(|value| value.xml_content.as_deref())
      .map(|value| raw_value.cloned().unwrap_or_else(|| value.to_string()));
    Self {
      reference: resolved_address
        .map(format_cell_address_a1)
        .or_else(|| cell.cell_reference.as_ref().map(ToString::to_string)),
      style_index: cell.style_index,
      data_type: cell.data_type,
      cell_meta_index: cell.cell_meta_index,
      value_meta_index: cell.value_meta_index,
      show_phonetic: cell.show_phonetic.is_some_and(|value| value.as_bool()),
      formula: cell.cell_formula.as_ref().map(FormulaModel::from_formula),
      cached_value,
      display_text: cell_text(cell, shared_strings, raw_value.map(String::as_str)),
      rich_text_runs: cell_rich_text_runs(cell, shared_strings),
      has_extensions: cell.extension_list.is_some(),
    }
  }
}

fn format_cell_address_a1(address: CellAddress) -> String {
  let mut col = address.col;
  let mut letters = Vec::new();
  while col > 0 {
    col -= 1;
    letters.push(char::from(b'A' + u8::try_from(col % 26).unwrap_or(0)));
    col /= 26;
  }
  letters.iter().rev().collect::<String>() + &address.row.to_string()
}

impl FormulaModel {
  fn from_formula(formula: &x::CellFormula) -> Self {
    // Source: LibreOffice sc/source/filter/oox/sheetdatacontext.cxx and
    // sheetdatabuffer.cxx. These fields decide whether the cell is imported
    // as a normal formula, shared formula, array formula, or data-table
    // operation. Token conversion is a later FormulaBuffer responsibility.
    Self {
      formula_type: formula.formula_type.unwrap_or_default(),
      reference: formula.reference.clone(),
      shared_index: formula.shared_index,
      text: formula.xml_content.clone().unwrap_or_default(),
      always_calculate_array: formula
        .always_calculate_array
        .is_some_and(|value| value.as_bool()),
      calculate_cell: formula.calculate_cell.is_some_and(|value| value.as_bool()),
      data_table_2d: formula.data_table2_d.is_some_and(|value| value.as_bool()),
      data_table_row: formula.data_table_row.is_some_and(|value| value.as_bool()),
      input1_deleted: formula.input1_deleted.is_some_and(|value| value.as_bool()),
      input2_deleted: formula.input2_deleted.is_some_and(|value| value.as_bool()),
      input1_reference: formula.r1.clone(),
      input2_reference: formula.r2.clone(),
      assigns_value_to_name: formula.bx.is_some_and(|value| value.as_bool()),
    }
  }
}

fn inline_string_text(value: &x::InlineString) -> String {
  if let Some(text) = &value.text
    && let Some(content) = &text.xml_content
  {
    return decode_excel_escaped_text(content);
  }

  decode_excel_escaped_text(
    &value
      .run
      .iter()
      .filter_map(|run| run.text.xml_content.as_deref())
      .collect::<String>(),
  )
}

fn cell_text(
  cell: &x::Cell,
  shared_strings: &[SharedStringModel],
  raw_value: Option<&str>,
) -> String {
  match cell.data_type {
    Some(x::CellValues::SharedString) => cell
      .cell_value
      .as_ref()
      .and_then(|value| value.xml_content.as_deref())
      .and_then(|index| index.parse::<usize>().ok())
      .and_then(|index| shared_strings.get(index))
      .map(|shared| shared.text.clone())
      .unwrap_or_default(),
    Some(x::CellValues::InlineString) => cell
      .inline_string
      .as_deref()
      .map(inline_string_text)
      .unwrap_or_default(),
    Some(x::CellValues::Boolean) => match cell
      .cell_value
      .as_ref()
      .and_then(|value| value.xml_content.as_deref())
    {
      Some(value) if boolean_cell_value(value) => "TRUE".to_string(),
      Some(_) => "FALSE".to_string(),
      None => String::new(),
    },
    _ => raw_value
      .map(ToString::to_string)
      .or_else(|| {
        cell
          .cell_value
          .as_ref()
          .and_then(|value| value.xml_content.as_deref())
          .map(ToString::to_string)
      })
      .unwrap_or_default(),
  }
}

fn cell_rich_text_runs(
  cell: &x::Cell,
  shared_strings: &[SharedStringModel],
) -> Vec<SharedStringRun> {
  match cell.data_type {
    Some(x::CellValues::SharedString) => cell
      .cell_value
      .as_ref()
      .and_then(|value| value.xml_content.as_deref())
      .and_then(|index| index.parse::<usize>().ok())
      .and_then(|index| shared_strings.get(index))
      .map(|shared| shared.runs.clone())
      .unwrap_or_default(),
    Some(x::CellValues::InlineString) => cell
      .inline_string
      .as_ref()
      .map(|inline| {
        inline
          .run
          .iter()
          .map(|run| SharedStringRun {
            text: run
              .text
              .xml_content
              .as_deref()
              .map(decode_excel_escaped_text)
              .unwrap_or_default(),
            ..SharedStringRun::default()
          })
          .collect()
      })
      .unwrap_or_default(),
    _ => Vec::new(),
  }
}

fn boolean_cell_value(value: &str) -> bool {
  match value.trim().to_ascii_lowercase().as_str() {
    "true" => true,
    "false" | "" => false,
    value => value.parse::<f64>().is_ok_and(|number| number != 0.0),
  }
}
