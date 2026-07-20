use std::collections::HashMap;

use ooxmlsdk::parts::chartsheet_part::ChartsheetPart;
use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::parts::worksheet_part::WorksheetPart;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;

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
use crate::text_metrics::TextMetrics;
use crate::units;

const CALC_DIGIT_WIDTH_MM: f32 = 2.0;
const CALC_BASE_COLUMN_PADDING_PX: f32 = 5.0;
const XLSX_MAX_COLUMN: u32 = 16_384;

#[derive(Clone, Debug)]
pub(crate) struct CalcSheet {
  pub(crate) workbook_index: usize,
  pub(crate) name: String,
  pub(crate) sheet_type: SheetType,
  pub(crate) state: Option<x::SheetStateValues>,
  pub(crate) active: bool,
  pub(crate) page_settings: CalcPageSettings,
  pub(crate) metrics: SheetMetrics,
  pub(crate) resources: SheetResourceCatalog,
  pub(crate) rows: Vec<CalcRow>,
  geometry: SheetGeometry,
  cell_positions: HashMap<CellAddress, (usize, usize)>,
  row_positions: Box<[(u32, usize)]>,
}

#[derive(Clone, Debug)]
struct SheetGeometry {
  column_offsets_pt: Box<[f32]>,
  row_overrides: Box<[RowGeometry]>,
  merged_ranges: Box<[CellRange]>,
  default_row_height_pt: f32,
}

#[derive(Clone, Copy, Debug)]
struct RowGeometry {
  index: u32,
  height_pt: f32,
  cumulative_delta_pt: f32,
}

#[derive(Clone, Debug)]
pub(crate) struct SheetIdentity {
  pub(crate) workbook_index: usize,
  pub(crate) name: String,
  pub(crate) state: Option<x::SheetStateValues>,
  pub(crate) active: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct CellAddress {
  pub(crate) col: u32,
  pub(crate) row: u32,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
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
  pub(crate) default_digit_width_pt: f32,
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

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SheetFormatModel {
  pub(crate) base_column_width: Option<u32>,
  pub(crate) default_column_width: Option<f64>,
  pub(crate) default_row_height: f64,
  pub(crate) custom_height: bool,
  pub(crate) zero_height: bool,
  pub(crate) thick_top: bool,
  pub(crate) thick_bottom: bool,
}

impl Default for SheetFormatModel {
  fn default() -> Self {
    Self {
      base_column_width: None,
      default_column_width: None,
      // ScGlobal::nStdRowHeight = 256 twips.
      default_row_height: 256.0 / units::TWIPS_PER_POINT as f64,
      custom_height: false,
      zero_height: false,
      thick_top: false,
      thick_bottom: false,
    }
  }
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
  address: Option<CellAddress>,
  pub(crate) style_index: Option<u32>,
  pub(crate) data_type: Option<x::CellValues>,
  pub(crate) formula: Option<FormulaModel>,
  pub(crate) cached_value: Option<String>,
  pub(crate) display_text: String,
  pub(crate) rich_text_runs: Vec<SharedStringRun>,
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

impl CalcSheet {
  pub(crate) fn from_worksheet(
    identity: SheetIdentity,
    worksheet: x::Worksheet,
    resources: SheetResourceCatalog,
    shared_strings: &[SharedStringModel],
    styles: &StylesCatalog,
    mso_document: bool,
  ) -> Self {
    let page_settings = CalcPageSettings::from_worksheet(&worksheet);
    let metrics = SheetMetrics::from_worksheet(&worksheet, styles, mso_document);
    let rows = worksheet_rows(&worksheet, shared_strings, mso_document);
    let geometry = SheetGeometry::new(&metrics, &rows);
    let cell_positions = cell_positions(&rows);
    let row_positions = row_positions(&rows);
    Self {
      workbook_index: identity.workbook_index,
      name: identity.name,
      sheet_type: SheetType::Worksheet,
      state: identity.state,
      active: identity.active,
      page_settings,
      metrics,
      resources,
      rows,
      geometry,
      cell_positions,
      row_positions,
    }
  }

  pub(crate) fn from_chartsheet(
    identity: SheetIdentity,
    chartsheet: x::Chartsheet,
    resources: SheetResourceCatalog,
  ) -> Self {
    let page_settings = CalcPageSettings::from_chartsheet(&chartsheet);
    let metrics = SheetMetrics::default();
    let rows = Vec::new();
    let geometry = SheetGeometry::new(&metrics, &rows);
    Self {
      workbook_index: identity.workbook_index,
      name: identity.name,
      sheet_type: SheetType::Chartsheet,
      state: identity.state,
      active: identity.active,
      page_settings,
      metrics,
      resources,
      rows,
      geometry,
      cell_positions: HashMap::new(),
      row_positions: Box::default(),
    }
  }

  pub(crate) fn unresolved(identity: SheetIdentity) -> Self {
    let metrics = SheetMetrics::default();
    let rows = Vec::new();
    let geometry = SheetGeometry::new(&metrics, &rows);
    Self {
      workbook_index: identity.workbook_index,
      name: identity.name,
      sheet_type: SheetType::Unresolved,
      state: identity.state,
      active: identity.active,
      page_settings: CalcPageSettings::default(),
      metrics,
      resources: SheetResourceCatalog::default(),
      rows,
      geometry,
      cell_positions: HashMap::new(),
      row_positions: Box::default(),
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
    for table in &self.resources.tables {
      let Some(range) = table.range else {
        continue;
      };
      used = Some(match used {
        Some(used) => used.union(range),
        None => range,
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
    let x_pt = self.geometry.column_offset_pt(address.col);
    let y_pt = self.geometry.row_offset_pt(address.row);
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
      width_pt: self.geometry.column_range_width_pt(address.col, end.col),
      height_pt: self.geometry.row_range_height_pt(address.row, end.row),
    }
  }

  pub(crate) fn merged_range_for_cell(&self, address: CellAddress) -> Option<CellRange> {
    self
      .geometry
      .merged_ranges
      .iter()
      .copied()
      .find(|range| range.contains(address))
  }

  pub(crate) fn is_covered_merged_cell(&self, address: CellAddress) -> bool {
    self
      .merged_range_for_cell(address)
      .is_some_and(|range| range.start != address)
  }

  pub(crate) fn range_rect(&self, range: CellRange) -> CellRect {
    let start = self.cell_rect_with_merge(range.start, false);
    let width_pt = self
      .geometry
      .column_range_width_pt(range.start.col, range.end.col);
    let height_pt = self
      .geometry
      .row_range_height_pt(range.start.row, range.end.row);
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
    self.geometry.column_width_pt(column)
  }

  pub(crate) fn row_height_pt(&self, row_index: u32) -> f32 {
    self.geometry.row_height_pt(row_index)
  }

  pub(crate) fn cell_at(&self, address: CellAddress) -> Option<&CalcCell> {
    let &(row, cell) = self.cell_positions.get(&address)?;
    self.rows.get(row)?.cells.get(cell)
  }

  pub(crate) fn cell_at_mut(&mut self, address: CellAddress) -> Option<&mut CalcCell> {
    let &(row, cell) = self.cell_positions.get(&address)?;
    self.rows.get_mut(row)?.cells.get_mut(cell)
  }

  pub(crate) fn rows_intersecting_print_area(
    &self,
    area: CellRange,
  ) -> impl Iterator<Item = (u32, &CalcRow)> {
    let first_row = self
      .geometry
      .merged_ranges
      .iter()
      .filter(|merged| merged.intersects(area))
      .map(|merged| merged.start.row)
      .fold(area.start.row, u32::min);
    let start = self
      .row_positions
      .partition_point(|(row, _)| *row < first_row);
    let end = self
      .row_positions
      .partition_point(|(row, _)| *row <= area.end.row);
    self.row_positions[start..end]
      .iter()
      .filter_map(|(row_index, position)| self.rows.get(*position).map(|row| (*row_index, row)))
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

impl SheetGeometry {
  fn new(metrics: &SheetMetrics, rows: &[CalcRow]) -> Self {
    let default_column_width_pt = metrics
      .format
      .default_column_width_points(metrics.default_digit_width_pt);
    let mut column_offsets_pt = Vec::with_capacity(XLSX_MAX_COLUMN as usize + 1);
    column_offsets_pt.push(0.0);
    for column in 1..=XLSX_MAX_COLUMN {
      let width = column_width_from_metrics(metrics, column, default_column_width_pt);
      column_offsets_pt.push(column_offsets_pt.last().copied().unwrap_or(0.0) + width);
    }

    let default_row_height_pt = if metrics.format.zero_height {
      0.0
    } else {
      metrics.format.default_row_height as f32
    };
    let mut row_overrides = rows
      .iter()
      .filter_map(|row| {
        let index = row.row_index?;
        let height_pt = if metrics.format.zero_height || row.hidden {
          0.0
        } else {
          row
            .height
            .map_or(default_row_height_pt, |height| height as f32)
        };
        Some((index, height_pt))
      })
      .collect::<Vec<_>>();
    row_overrides.sort_by_key(|(index, _)| *index);
    row_overrides.dedup_by_key(|(index, _)| *index);
    let mut cumulative_delta_pt = 0.0;
    let row_overrides = row_overrides
      .into_iter()
      .map(|(index, height_pt)| {
        cumulative_delta_pt += height_pt - default_row_height_pt;
        RowGeometry {
          index,
          height_pt,
          cumulative_delta_pt,
        }
      })
      .collect();

    Self {
      column_offsets_pt: column_offsets_pt.into_boxed_slice(),
      row_overrides,
      merged_ranges: metrics
        .merged_ranges
        .iter()
        .filter_map(|reference| CellRange::parse_a1_range(reference))
        .collect(),
      default_row_height_pt,
    }
  }

  fn column_offset_pt(&self, column: u32) -> f32 {
    let preceding_columns = column.saturating_sub(1) as usize;
    self
      .column_offsets_pt
      .get(preceding_columns)
      .copied()
      .unwrap_or_else(|| *self.column_offsets_pt.last().unwrap_or(&0.0))
  }

  fn column_width_pt(&self, column: u32) -> f32 {
    self.column_range_width_pt(column, column)
  }

  fn column_range_width_pt(&self, start: u32, end: u32) -> f32 {
    if start == 0 || end < start {
      return 0.0;
    }
    let start = start.saturating_sub(1) as usize;
    let end = end as usize;
    match (
      self.column_offsets_pt.get(start),
      self.column_offsets_pt.get(end),
    ) {
      (Some(start), Some(end)) => end - start,
      _ => 0.0,
    }
  }

  fn row_height_pt(&self, row: u32) -> f32 {
    self
      .row_overrides
      .binary_search_by_key(&row, |geometry| geometry.index)
      .ok()
      .map_or(self.default_row_height_pt, |index| {
        self.row_overrides[index].height_pt
      })
  }

  fn row_offset_pt(&self, row: u32) -> f32 {
    let preceding_rows = row.saturating_sub(1);
    let override_count = self
      .row_overrides
      .partition_point(|geometry| geometry.index <= preceding_rows);
    let override_delta = override_count
      .checked_sub(1)
      .map_or(0.0, |index| self.row_overrides[index].cumulative_delta_pt);
    preceding_rows as f32 * self.default_row_height_pt + override_delta
  }

  fn row_range_height_pt(&self, start: u32, end: u32) -> f32 {
    if start == 0 || end < start {
      return 0.0;
    }
    self.row_offset_pt(end.saturating_add(1)) - self.row_offset_pt(start)
  }
}

fn column_width_from_metrics(metrics: &SheetMetrics, column: u32, default_width_pt: f32) -> f32 {
  if let Some(model) = metrics
    .columns
    .iter()
    .find(|model| column >= model.first && column <= model.last)
  {
    if model.hidden {
      return 0.0;
    }
    if let Some(width) = model.width {
      return digit_width_to_lo_points(width as f32, metrics.digit_width_pt);
    }
  }
  if metrics
    .columns
    .iter()
    .any(|model| model.hidden && column >= model.first && column <= model.last)
  {
    return 0.0;
  }
  default_width_pt
}

fn cell_positions(rows: &[CalcRow]) -> HashMap<CellAddress, (usize, usize)> {
  let mut positions = HashMap::with_capacity(rows.iter().map(|row| row.cells.len()).sum());
  for (row_index, row) in rows.iter().enumerate() {
    for (cell_index, cell) in row.cells.iter().enumerate() {
      if let Some(address) = cell.address {
        positions.entry(address).or_insert((row_index, cell_index));
      }
    }
  }
  positions
}

fn row_positions(rows: &[CalcRow]) -> Box<[(u32, usize)]> {
  let mut positions = rows
    .iter()
    .enumerate()
    .map(|(position, row)| (row.row_index.unwrap_or(position as u32 + 1), position))
    .collect::<Vec<_>>();
  positions.sort_unstable();
  positions.into_boxed_slice()
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
}

impl SheetMetrics {
  fn from_worksheet(worksheet: &x::Worksheet, styles: &StylesCatalog, mso_document: bool) -> Self {
    // WorksheetFragment imports dimensions, sheetFormatPr, cols,
    // mergeCells, hyperlinks, rowBreaks, and colBreaks before page layout.
    let digit_width_pt = styles
      .document_font_text_style_for_column_width()
      .as_ref()
      .map(measured_digit_width_pt)
      // UnitConverter starts with 1 digit = 2mm. finalizeImport() only
      // replaces it when StylesBuffer::getDefaultFont() finds an XF/font.
      .unwrap_or_else(|| units::millimeters_to_points(CALC_DIGIT_WIDTH_MM));
    let default_digit_width_pt = if styles.uses_application_default_minor_theme() {
      quantize_digit_width_to_screen_pixel(measured_digit_width_pt(
        &styles.default_font_text_style(),
      ))
    } else {
      digit_width_pt
    };
    let mut format = worksheet
      .sheet_format_properties
      .as_ref()
      .map(|format| SheetFormatModel::from_sheet_format_properties(format, mso_document))
      .unwrap_or_default();
    if !format.custom_height && styles.default_font_uses_theme() {
      format.default_row_height = automatic_default_row_height_pt(styles) as f64;
    }
    Self {
      dimension: worksheet
        .sheet_dimension
        .as_ref()
        .map(|dimension| dimension.reference.clone()),
      settings: SheetSettingsCatalog::from_worksheet(worksheet),
      views: SheetViewCatalog::from_worksheet(worksheet),
      format,
      digit_width_pt,
      default_digit_width_pt,
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
      return digit_width_to_lo_points(width as f32, digit_width_pt);
    }
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
    context: WorksheetResourceImportContext<'_>,
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
      super::pivot::PivotTableImportContext {
        current_worksheet: context.worksheet,
        current_sheet_name: context.sheet_name,
        shared_strings: context.shared_strings,
        styles: context.styles,
        date_1904: context.date_1904,
      },
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

pub(crate) struct WorksheetResourceImportContext<'a> {
  pub(crate) sheet_name: &'a str,
  pub(crate) worksheet: &'a x::Worksheet,
  pub(crate) shared_strings: &'a [SharedStringModel],
  pub(crate) styles: &'a StylesCatalog,
  pub(crate) date_1904: bool,
}

fn worksheet_rows(
  worksheet: &x::Worksheet,
  shared_strings: &[SharedStringModel],
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
          CalcCell::from_cell(cell, shared_strings, Some(address))
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
    // worksheetfragment.cxx round MSO OOXML row heights down to 0.75pt.
    height - height % 0.75
  } else {
    height
  }
}

fn digit_width_to_points(value: f32, digit_width_pt: f32) -> f32 {
  // UnitConverter::scaleValue(value, Unit::Digit, Unit::Twip).
  value * digit_width_pt
}

fn digit_width_to_lo_points(value: f32, digit_width_pt: f32) -> f32 {
  (digit_width_to_points(value, digit_width_pt) * units::TWIPS_PER_POINT).round()
    / units::TWIPS_PER_POINT
}

fn measured_digit_width_pt(style: &crate::model::TextStyle) -> f32 {
  // Unit::Digit to 2mm, then UnitConverter::finalizeImport() replaces it with
  // the default font XFont maximum width across '0'..'9'. Explicit column
  // widths retain this document-font measure. The caller separately derives
  // the localized application default width when theme1.xml is absent.
  let mut text_metrics = TextMetrics::new();
  let digit_width = ('0'..='9')
    .map(|ch| {
      let mut encoded = [0; 4];
      text_metrics.measure_text(ch.encode_utf8(&mut encoded), style)
    })
    .fold(0.0_f32, f32::max);
  if digit_width > 0.0 {
    digit_width
  } else {
    units::millimeters_to_points(CALC_DIGIT_WIDTH_MM)
  }
}

fn quantize_digit_width_to_screen_pixel(width_pt: f32) -> f32 {
  (width_pt / screen_pixel_width_pt()).round().max(1.0) * screen_pixel_width_pt()
}

fn automatic_default_row_height_pt(styles: &StylesCatalog) -> f32 {
  // A false sheetFormatPr@customHeight marks the default row height as
  // automatic. LibreOffice's tdf#124741/tdf#168892 calibration keeps that
  // distinction because Excel otherwise recalculates the stored height.
  // Derive the printable height from the resolved Normal font rather than
  // treating the serialized defaultRowHeight as a manual measurement. Excel
  // leaves one screen-pixel worth of leading above and below the font box and
  // stores row heights on the 96 dpi (0.75 pt) grid.
  let style = styles.default_font_text_style();
  let mut text_metrics = TextMetrics::new();
  let natural_height = text_metrics.vertical_metrics(&style).line_height_pt();
  let padded_height = natural_height + 2.0 * screen_pixel_width_pt();
  // MSO row measurements are truncated to the device grid; LO mirrors this
  // for imported OOXML heights in WorksheetFragment::importSheetFormatPr.
  (padded_height / screen_pixel_width_pt()).floor() * screen_pixel_width_pt()
}

fn screen_pixel_width_pt() -> f32 {
  // Unit::ScreenX from GraphicHelper device pixels. The headless Calc export
  // path used by the upstream fixtures has a 96dpi reference device, i.e. one
  // screen pixel is 0.75pt.
  units::POINTS_PER_INCH / 96.0
}

impl CalcCell {
  pub(crate) fn address(&self) -> Option<CellAddress> {
    self.address
  }

  fn from_cell(
    cell: &x::Cell,
    shared_strings: &[SharedStringModel],
    resolved_address: Option<CellAddress>,
  ) -> Self {
    let cell_value = decoded_cell_value(cell);
    let cached_value = cell
      .cell_value
      .as_ref()
      .and_then(|value| value.xml_content.as_deref())
      .map(|_| cell_value.clone());
    Self {
      address: resolved_address.or_else(|| {
        cell
          .cell_reference
          .as_deref()
          .and_then(CellAddress::parse_a1)
      }),
      style_index: cell.style_index,
      data_type: cell.data_type,
      formula: cell.cell_formula.as_ref().map(FormulaModel::from_formula),
      cached_value,
      display_text: cell_text(cell, shared_strings),
      rich_text_runs: cell_rich_text_runs(cell, shared_strings),
    }
  }
}

fn decoded_cell_value(cell: &x::Cell) -> String {
  cell
    .cell_value
    .as_ref()
    .and_then(|value| value.xml_content.as_deref())
    .map(decode_excel_escaped_text)
    .unwrap_or_default()
}

impl FormulaModel {
  fn from_formula(formula: &x::CellFormula) -> Self {
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

fn cell_text(cell: &x::Cell, shared_strings: &[SharedStringModel]) -> String {
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
    _ => decoded_cell_value(cell),
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

#[cfg(test)]
mod tests {
  use super::*;

  fn empty_row(row_index: Option<u32>) -> CalcRow {
    CalcRow {
      row_index,
      height: None,
      custom_height: false,
      style_index: None,
      hidden: false,
      cells: Vec::new(),
    }
  }

  #[test]
  fn print_row_index_is_sorted_once_with_implicit_row_fallback() {
    let rows = vec![empty_row(Some(9)), empty_row(None), empty_row(Some(3))];

    assert_eq!(row_positions(&rows).as_ref(), &[(2, 1), (3, 2), (9, 0)]);
  }

  #[test]
  fn application_default_maximum_digit_width_is_quantized_to_a_96_dpi_pixel() {
    assert_eq!(quantize_digit_width_to_screen_pixel(5.79), 6.0);
    assert_eq!(
      SheetFormatModel::default().default_column_width_points(6.0),
      51.75
    );
  }
}
