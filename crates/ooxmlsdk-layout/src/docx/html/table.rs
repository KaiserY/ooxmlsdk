use super::super::model::{
  BorderRelief, BorderStyle, CellBordersModel, CellMargins, ShadingPaint, Table, TableAlignment,
  TableBordersModel, TableCell, TableCellSpacing, TableCellVerticalAlignment, TableLayoutMode,
  TableRow,
};
use super::*;

#[derive(Clone, Debug, Default)]
pub(super) struct BoxStyle {
  width_pt: Option<f32>,
  width_pct: Option<f32>,
  height_pt: Option<f32>,
  padding: Option<CellMargins>,
  border: Option<BorderStyle>,
  background: Option<super::super::model::RgbColor>,
  vertical_alignment: Option<TableCellVerticalAlignment>,
  alignment: Option<TableAlignment>,
  fixed_layout: bool,
  collapse_borders: bool,
  no_wrap: bool,
}

impl BoxStyle {
  pub(super) fn from_tag(tag: &Tag, declarations: &[css::Declaration], font_size: f32) -> Self {
    let mut style = Self::default();
    if let Some(width) = attribute_value(&tag.attrs, "width") {
      style.set_width(width, true, font_size);
    }
    style.height_pt = attribute_value(&tag.attrs, "height").and_then(html_pixels);
    style.background = attribute_value(&tag.attrs, "bgcolor").and_then(css_color);
    style.no_wrap = attribute_present(&tag.attrs, "nowrap");
    if let Some(value) = attribute_value(&tag.attrs, "valign") {
      style.set_vertical_alignment(value);
    }
    style.alignment = attribute_value(&tag.attrs, "align").and_then(|value| match value {
      "left" => Some(TableAlignment::Left),
      "right" => Some(TableAlignment::Right),
      "center" => Some(TableAlignment::Center),
      _ => None,
    });
    for declaration in declarations {
      let name = declaration.name.as_str();
      let value = declaration.value.as_str();
      match name.trim().to_ascii_lowercase().as_str() {
        "width" => style.set_width(value, false, font_size),
        "height" | "min-height" => style.height_pt = css_length_pt(value, font_size),
        "background" | "background-color" => style.background = css_color(value),
        "vertical-align" => style.set_vertical_alignment(value),
        "white-space" => style.no_wrap = value.eq_ignore_ascii_case("nowrap"),
        "table-layout" => style.fixed_layout = value.eq_ignore_ascii_case("fixed"),
        "border-collapse" => style.collapse_borders = value.eq_ignore_ascii_case("collapse"),
        "padding" => {
          if let Some([top_pt, right_pt, bottom_pt, left_pt]) = box_lengths(value, font_size) {
            style.padding = Some(CellMargins {
              top_pt,
              right_pt,
              bottom_pt,
              left_pt,
            });
          }
        }
        "padding-top" | "padding-right" | "padding-bottom" | "padding-left" => {
          if let Some(points) = css_length_pt(value, font_size) {
            let padding = style.padding.get_or_insert(CellMargins::zero());
            match name.trim() {
              "padding-top" => padding.top_pt = points,
              "padding-right" => padding.right_pt = points,
              "padding-bottom" => padding.bottom_pt = points,
              _ => padding.left_pt = points,
            }
          }
        }
        "border" => {
          if value == "none" || value == "0" {
            style.border = Some(BorderStyle {
              width_pt: 0.0,
              ..Default::default()
            });
          } else {
            let mut border = BorderStyle::default();
            for part in value.split_whitespace() {
              if let Some(width) = css_length_pt(part, font_size) {
                border.width_pt = width;
              } else if let Some(color) = parse_vml_color(part) {
                border.color = color;
              }
            }
            style.border = Some(border);
          }
        }
        _ => {}
      }
    }
    style
  }

  fn set_width(&mut self, value: &str, pixels: bool, font_size: f32) {
    if let Some(percent) = value
      .trim()
      .strip_suffix('%')
      .and_then(|s| s.parse::<f32>().ok())
    {
      self.width_pct = Some(percent / 100.0);
      self.width_pt = None;
    } else if let Some(points) = if pixels {
      html_pixels(value)
    } else {
      css_length_pt(value, font_size)
    } {
      self.width_pt = Some(points.max(0.0));
      self.width_pct = None;
    }
  }

  fn set_vertical_alignment(&mut self, value: &str) {
    self.vertical_alignment = match value.to_ascii_lowercase().as_str() {
      "top" | "baseline" => Some(TableCellVerticalAlignment::Top),
      "middle" | "center" => Some(TableCellVerticalAlignment::Center),
      "bottom" => Some(TableCellVerticalAlignment::Bottom),
      _ => None,
    };
  }
}

fn html_pixels(value: &str) -> Option<f32> {
  value
    .trim()
    .parse::<f32>()
    .ok()
    .filter(|v| v.is_finite())
    .map(|v| v.max(0.0) * 0.75)
}

fn box_lengths(value: &str, font_size: f32) -> Option<[f32; 4]> {
  let values = value
    .split_whitespace()
    .map(|v| css_length_pt(v, font_size))
    .collect::<Option<Vec<_>>>()?;
  Some(match values.as_slice() {
    [all] => [*all; 4],
    [v, h] => [*v, *h, *v, *h],
    [t, h, b] => [*t, *h, *b, *h],
    [t, r, b, l] => [*t, *r, *b, *l],
    _ => return None,
  })
}

struct CellBuilder {
  block_start: usize,
  context: ElementContext,
  cell: TableCell,
  row_span: usize,
}

struct RawRow {
  model: TableRow,
  spans: Vec<usize>,
}

pub(super) struct TableBuilder {
  table: Table,
  rows: Vec<RawRow>,
  row: Option<RawRow>,
  cell: Option<CellBuilder>,
  margins: CellMargins,
  border: Option<BorderStyle>,
  shading: Option<ShadingPaint>,
  row_style: BoxStyle,
}

impl TableBuilder {
  pub(super) fn new(tag: &Tag, context: &ElementContext) -> Self {
    let style = &context.box_style;
    let padding = attribute_value(&tag.attrs, "cellpadding")
      .and_then(html_pixels)
      .unwrap_or(0.75);
    let spacing = attribute_value(&tag.attrs, "cellspacing")
      .and_then(html_pixels)
      .unwrap_or(0.75);
    let border = (|| {
      let width_pt = attribute_value(&tag.attrs, "border").and_then(html_pixels)?;
      (width_pt > 0.0).then_some(BorderStyle {
        width_pt,
        relief: Some(BorderRelief {
          inset: false,
          automatic_color: true,
        }),
        ..Default::default()
      })
    })();
    Self {
      table: Table {
        recovered_absolute_grid: false,
        column_widths_pt: Vec::new(),
        preferred_width_pt: style.width_pt,
        preferred_width_pct: style.width_pct,
        layout: if style.fixed_layout {
          TableLayoutMode::Fixed
        } else {
          TableLayoutMode::AutoFit
        },
        containing_table_layout: None,
        indent_left_pt: 0.0,
        alignment: style.alignment.unwrap_or_default(),
        right_to_left: context.paragraph_format.bidi,
        align_leading_cell_content: true,
        in_header_footer: false,
        placement: None,
        allow_overlap: true,
        split_allowed: false,
        following_text_flow: false,
        explicit_no_repeat_header: false,
        page_break_before: false,
        starts_after_last_rendered_page_break: false,
        borders: style.border.or(border).map(|border| TableBordersModel {
          top: Some(border),
          left: Some(border),
          bottom: Some(border),
          right: Some(border),
          inside_horizontal: None,
          inside_vertical: None,
        }),
        cell_spacing: if style.collapse_borders {
          TableCellSpacing::Collapsed
        } else {
          TableCellSpacing::Separated(spacing)
        },
        rows: Vec::new(),
      },
      rows: Vec::new(),
      row: None,
      cell: None,
      margins: CellMargins {
        top_pt: padding,
        right_pt: padding,
        bottom_pt: padding,
        left_pt: padding,
      },
      border,
      shading: style.background.map(ShadingPaint::Solid),
      row_style: BoxStyle::default(),
    }
  }

  pub(super) fn start_row(&mut self, context: &ElementContext, header: bool) {
    self.finish_row();
    self.row_style = context.box_style.clone();
    self.row = Some(RawRow {
      model: TableRow {
        cells: Vec::new(),
        height_pt: context.box_style.height_pt,
        exact_height: false,
        repeat_header: header,
        keep_with_next: false,
        cant_split: false,
        cell_spacing: None,
        grid_before: 0,
        grid_after: 0,
        width_before_pt: None,
        width_after_pt: None,
        layout: None,
        borders: None,
        spacing_shading: None,
        redline_color: None,
      },
      spans: Vec::new(),
    });
  }

  pub(super) fn start_cell(&mut self, tag: &Tag, context: &ElementContext, block_start: usize) {
    if self.row.is_none() {
      self.start_row(context, false);
    }
    let style = &context.box_style;
    if let Some(height) = style.height_pt {
      let row = self.row.as_mut().unwrap();
      row.model.height_pt = Some(row.model.height_pt.unwrap_or(0.0).max(height));
    }
    let border = style.border.or(self.border);
    let cell = TableCell {
      blocks: Vec::new(),
      shading: style
        .background
        .or(self.row_style.background)
        .map(ShadingPaint::Solid)
        .or(self.shading),
      borders: CellBordersModel {
        top: border,
        right: border,
        bottom: border,
        left: border,
      },
      border_suppressions: Default::default(),
      margins: style.padding.unwrap_or(self.margins),
      preferred_width_pt: style.width_pt,
      preferred_width_pct: style.width_pct,
      grid_span: attribute_value(&tag.attrs, "colspan")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(1)
        .clamp(1, 1000),
      vertical_merge_continue: false,
      no_wrap: style.no_wrap,
      fit_text: false,
      hide_end_mark: true,
      vertical_alignment: style
        .vertical_alignment
        .or(self.row_style.vertical_alignment)
        .unwrap_or(TableCellVerticalAlignment::Center),
      text_rotation_deg: None,
    };
    self.cell = Some(CellBuilder {
      block_start,
      context: context.clone(),
      cell,
      row_span: attribute_value(&tag.attrs, "rowspan")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(1)
        .min(65534),
    });
  }

  pub(super) fn finish_cell(&mut self, blocks: &mut Vec<Block>) {
    if let Some(mut cell) = self.cell.take() {
      cell.cell.blocks = blocks.split_off(cell.block_start);
      if cell.cell.blocks.is_empty() {
        let mut paragraph = ParagraphBuilder::new(&cell.context);
        paragraph.explicit = true;
        cell.cell.blocks.extend(paragraph.into_block());
      }
      let row = self.row.as_mut().unwrap();
      row.model.cells.push(cell.cell);
      row.spans.push(cell.row_span);
    }
  }

  pub(super) fn finish_row(&mut self) {
    if let Some(row) = self.row.take() {
      self.rows.push(row);
    }
  }

  pub(super) fn finish(mut self) -> Block {
    self.finish_row();
    // Convert HTML row spans to the existing WML vertical-merge model, leaving
    // content on the origin cell and inserting empty continuation cells.
    let row_count = self.rows.len();
    let mut active = std::collections::BTreeMap::<usize, (usize, TableCell)>::new();
    for (row_index, mut row) in self.rows.into_iter().enumerate() {
      let cells = std::mem::take(&mut row.model.cells);
      let mut column = 0;
      for (cell, span) in cells.into_iter().zip(row.spans) {
        while let Some((_, continuation)) = active.get(&column) {
          row.model.cells.push(continuation.clone());
          column += continuation.grid_span;
        }
        let span = if span == 0 {
          row_count - row_index
        } else {
          span
        };
        if span > 1 {
          let mut continuation = cell.clone();
          continuation.blocks.clear();
          continuation.vertical_merge_continue = true;
          active.insert(column, (span, continuation));
        }
        column += cell.grid_span;
        row.model.cells.push(cell);
      }
      while let Some((_, continuation)) = active.get(&column) {
        row.model.cells.push(continuation.clone());
        column += continuation.grid_span;
      }
      active.retain(|_, (remaining, _)| {
        *remaining -= 1;
        *remaining > 0
      });
      self
        .table
        .column_widths_pt
        .resize(self.table.column_widths_pt.len().max(column), 0.0);
      self.table.rows.push(row.model);
    }
    // Retain authored column constraints; the shared table formatter resolves
    // content minima, percentages and AutoFit against the available text area.
    for row in &self.table.rows {
      let mut column = 0;
      for cell in &row.cells {
        if let Some(width) = cell.preferred_width_pt {
          let per_column = width / cell.grid_span as f32;
          for value in &mut self.table.column_widths_pt[column..column + cell.grid_span] {
            if *value == 0.0 {
              *value = per_column;
            }
          }
        }
        column += cell.grid_span;
      }
    }
    Block::Table(self.table)
  }
}
