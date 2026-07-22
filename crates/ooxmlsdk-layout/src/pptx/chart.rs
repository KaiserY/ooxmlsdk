use crate::model::{
  LineItem, LineItemKind, PageItem, PdfTextSegmentation, RectItem, RgbColor, TextItem, TextStyle,
};
use crate::render::chart::{
  ChartLegendPosition, ChartTitleText, ClusteredColumnChart, clustered_column_slot,
  linear_axis_scale,
};
use crate::text_metrics::TextMetrics;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_chart as c;

const TEXT_LINE_HEIGHT_SCALE: f32 = 1.2;

#[derive(Clone, Copy, Debug)]
pub(crate) struct ChartFrame {
  pub x_pt: f32,
  pub y_pt: f32,
  pub width_pt: f32,
  pub height_pt: f32,
}

#[derive(Clone, Debug)]
pub(crate) struct ClusteredColumnStyle {
  pub layout_profile: ChartLayoutProfile,
  pub has_explicit_title: bool,
  pub title: TextStyle,
  pub title_fill_color: Option<RgbColor>,
  pub label: TextStyle,
  pub gridline_color: RgbColor,
  pub series_colors: Vec<RgbColor>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ChartLayoutProfile {
  PowerPoint,
  Word,
  Excel,
}

pub(crate) fn lower_clustered_column_chart(
  frame: ChartFrame,
  chart: &ClusteredColumnChart<'_>,
  automatic_title: &str,
  style: &ClusteredColumnStyle,
) -> Vec<PageItem> {
  if frame.width_pt <= 0.0 || frame.height_pt <= 0.0 || chart.series.is_empty() {
    return Vec::new();
  }

  let category_count = chart
    .series
    .iter()
    .map(|series| series.values.len())
    .chain(std::iter::once(chart.categories.len()))
    .max()
    .unwrap_or(0);
  if category_count == 0 {
    return Vec::new();
  }
  let Some(scale) = linear_axis_scale(
    chart
      .series
      .iter()
      .flat_map(|series| series.values.iter().flatten().copied()),
    chart.value_axis,
    10,
  ) else {
    return Vec::new();
  };
  if scale.maximum <= scale.minimum {
    return Vec::new();
  }

  let mut metrics = TextMetrics::new();
  let title_text = match chart.title.as_ref() {
    Some(ChartTitleText::Explicit(title)) => Some(title.as_str()),
    Some(ChartTitleText::Automatic) => Some(automatic_title),
    None => None,
  };
  let title_line_height = line_height(&style.title);
  let label_line_height = line_height(&style.label);
  let legend_position = chart.legend_position;
  let has_bottom_legend = legend_position == Some(ChartLegendPosition::Bottom);
  let has_explicit_powerpoint_title =
    style.layout_profile == ChartLayoutProfile::PowerPoint && style.has_explicit_title;
  let has_top_legend = legend_position == Some(ChartLegendPosition::Top);
  let has_side_legend = matches!(
    legend_position,
    Some(ChartLegendPosition::Left | ChartLegendPosition::Right | ChartLegendPosition::TopRight)
  );

  // Office's automatic chart layout reserves semantic bands around the plot:
  // title, value labels, category labels, and legend. The distances scale with
  // chart height, while actual label widths determine the left plot inset.
  let title_top = frame.y_pt + frame.height_pt * 0.024;
  let legend_bottom_margin = style.label.font_size_pt * 0.81;
  let legend_top = frame.y_pt + frame.height_pt - legend_bottom_margin - label_line_height;
  let category_bottom_ratio = match style.layout_profile {
    ChartLayoutProfile::Excel => 0.05,
    ChartLayoutProfile::Word => 0.022_87,
    ChartLayoutProfile::PowerPoint => 0.018,
  };
  let category_top = if has_bottom_legend {
    legend_top
      - label_line_height
      - frame.height_pt
        * if has_explicit_powerpoint_title {
          0.026_1
        } else {
          0.021
        }
  } else {
    frame.y_pt + frame.height_pt - label_line_height - frame.height_pt * category_bottom_ratio
  };
  let untitled_plot_top_ratio = if style.layout_profile == ChartLayoutProfile::Excel {
    if has_side_legend { 0.032 } else { 0.025 }
  } else if has_side_legend {
    0.0449
  } else {
    0.035
  };
  let mut plot_top = if title_text.is_some() {
    title_top + title_line_height + label_line_height * 0.9
  } else {
    frame.y_pt + frame.height_pt * untitled_plot_top_ratio
  };
  if has_top_legend {
    plot_top += label_line_height + frame.height_pt * 0.018;
  }
  let side_category_gap_ratio = match style.layout_profile {
    ChartLayoutProfile::Excel => 0.025,
    ChartLayoutProfile::Word => 0.027_75,
    ChartLayoutProfile::PowerPoint => 0.033_35,
  };
  let plot_bottom = category_top
    - frame.height_pt
      * if has_side_legend {
        side_category_gap_ratio
      } else if has_bottom_legend && has_explicit_powerpoint_title {
        0.0225
      } else {
        0.018
      };
  if plot_bottom <= plot_top {
    return Vec::new();
  }

  let tick_labels = scale_tick_labels(scale.minimum, scale.maximum, scale.major_unit);
  let maximum_tick_width = tick_labels
    .iter()
    .map(|(_, label)| metrics.measure_text(label, &style.label))
    .fold(0.0_f32, f32::max);
  let side_legend_width = if has_side_legend {
    vertical_legend_width(chart, style, &mut metrics)
  } else {
    0.0
  };
  let side_plot_outer_margin = frame.height_pt * 0.0318;
  let side_legend_outer_margin = frame.height_pt
    * match style.layout_profile {
      ChartLayoutProfile::Excel => 0.0445,
      ChartLayoutProfile::Word => 0.039_77,
      ChartLayoutProfile::PowerPoint => 0.028_25,
    };
  let side_plot_gap = frame.height_pt
    * match style.layout_profile {
      ChartLayoutProfile::Excel => 0.0766,
      ChartLayoutProfile::Word => 0.070_91,
      ChartLayoutProfile::PowerPoint => 0.048,
    };
  let tick_left_ratio = if has_side_legend {
    match style.layout_profile {
      ChartLayoutProfile::Word => 0.025_81,
      ChartLayoutProfile::PowerPoint | ChartLayoutProfile::Excel => 0.018_25,
    }
  } else {
    if has_bottom_legend && has_explicit_powerpoint_title {
      0.019
    } else {
      0.015
    }
  };
  let tick_left = frame.x_pt
    + frame.height_pt * tick_left_ratio
    + if legend_position == Some(ChartLegendPosition::Left) {
      side_legend_width + side_plot_outer_margin + side_plot_gap
    } else {
      0.0
    };
  let tick_gap = frame.height_pt
    * if has_side_legend {
      if style.layout_profile == ChartLayoutProfile::Word {
        0.0367
      } else {
        0.046_85
      }
    } else {
      if has_bottom_legend && has_explicit_powerpoint_title {
        0.0323
      } else {
        0.026
      }
    };
  let plot_left = tick_left + maximum_tick_width + tick_gap;
  let plot_right = frame.x_pt + frame.width_pt
    - if matches!(
      legend_position,
      Some(ChartLegendPosition::Right | ChartLegendPosition::TopRight)
    ) {
      side_legend_width + side_plot_outer_margin + side_plot_gap
    } else {
      frame.height_pt
        * if has_bottom_legend && has_explicit_powerpoint_title {
          0.0301
        } else {
          0.026
        }
    };
  if plot_right <= plot_left {
    return Vec::new();
  }
  let plot_width = plot_right - plot_left;
  let plot_height = plot_bottom - plot_top;
  let zero_y = value_y(
    0.0_f64.clamp(scale.minimum, scale.maximum),
    scale,
    plot_top,
    plot_height,
  );

  let mut items = Vec::new();
  for (value, _) in &tick_labels {
    if style.layout_profile == ChartLayoutProfile::Word && value.abs() < f64::EPSILON {
      continue;
    }
    let y = value_y(*value, scale, plot_top, plot_height);
    items.push(PageItem::Line(LineItem {
      x1_pt: plot_left,
      y1_pt: y,
      x2_pt: plot_right,
      y2_pt: y,
      width_pt: 0.75,
      color: style.gridline_color,
      kind: LineItemKind::Stroke,
    }));
  }

  for (series_index, series) in chart.series.iter().enumerate() {
    let Some(color) = style.series_colors.get(series_index).copied() else {
      continue;
    };
    for (category_index, value) in series.values.iter().enumerate() {
      let Some(value) = value else {
        continue;
      };
      let Some(slot) = clustered_column_slot(
        category_index,
        series_index,
        category_count,
        chart.series.len(),
        chart.gap_width_percent,
        chart.overlap_percent,
      ) else {
        continue;
      };
      let value_y = value_y(*value, scale, plot_top, plot_height);
      let top = value_y.min(zero_y);
      items.push(PageItem::Rect(RectItem {
        x_pt: plot_left + (slot.center - slot.width / 2.0) as f32 * plot_width,
        y_pt: top,
        width_pt: slot.width as f32 * plot_width,
        height_pt: (zero_y - value_y).abs(),
        fill_color: Some(color),
        fill_opacity: 1.0,
        stroke: None,
        stroke_opacity: 1.0,
      }));
    }
  }

  items.push(PageItem::Line(LineItem {
    x1_pt: plot_left,
    y1_pt: zero_y,
    x2_pt: plot_right,
    y2_pt: zero_y,
    width_pt: 0.75,
    color: style.gridline_color,
    kind: LineItemKind::Stroke,
  }));
  if style.layout_profile == ChartLayoutProfile::Word {
    let tick_length = frame.height_pt * 0.012_59;
    items.push(PageItem::Line(LineItem {
      x1_pt: plot_left,
      y1_pt: plot_top,
      x2_pt: plot_left,
      y2_pt: zero_y,
      width_pt: 0.75,
      color: style.gridline_color,
      kind: LineItemKind::Stroke,
    }));
    for (value, _) in &tick_labels {
      let y = value_y(*value, scale, plot_top, plot_height);
      items.push(PageItem::Line(LineItem {
        x1_pt: plot_left - tick_length,
        y1_pt: y,
        x2_pt: plot_left,
        y2_pt: y,
        width_pt: 0.75,
        color: style.gridline_color,
        kind: LineItemKind::Stroke,
      }));
    }
    for boundary in 0..=category_count {
      let x = plot_left + boundary as f32 / category_count as f32 * plot_width;
      items.push(PageItem::Line(LineItem {
        x1_pt: x,
        y1_pt: zero_y,
        x2_pt: x,
        y2_pt: zero_y + tick_length,
        width_pt: 0.75,
        color: style.gridline_color,
        kind: LineItemKind::Stroke,
      }));
    }
  }

  for (series_index, series) in chart.series.iter().enumerate() {
    for label in &series.data_labels {
      let Some(value) = series.values.get(label.point_index).copied().flatten() else {
        continue;
      };
      let Some(slot) = clustered_column_slot(
        label.point_index,
        series_index,
        category_count,
        chart.series.len(),
        chart.gap_width_percent,
        chart.overlap_percent,
      ) else {
        continue;
      };
      let center_x = plot_left + slot.center as f32 * plot_width;
      let bar_end_y = value_y(value, scale, plot_top, plot_height);
      let width = metrics.measure_text(&label.text, &style.label);
      let y = match label.position {
        c::DataLabelPositionValues::Center => (bar_end_y + zero_y) / 2.0 - label_line_height / 2.0,
        c::DataLabelPositionValues::InsideBase => zero_y - label_line_height,
        c::DataLabelPositionValues::InsideEnd => bar_end_y,
        c::DataLabelPositionValues::Bottom => zero_y,
        c::DataLabelPositionValues::OutsideEnd
        | c::DataLabelPositionValues::Top
        | c::DataLabelPositionValues::BestFit => bar_end_y - label_line_height,
        c::DataLabelPositionValues::Left | c::DataLabelPositionValues::Right => {
          (bar_end_y + zero_y) / 2.0 - label_line_height / 2.0
        }
      };
      push_text(
        &mut items,
        center_x - width / 2.0,
        y,
        label.text.clone(),
        style.label.clone(),
      );
    }
  }

  // Preserve Office's content-stream ordering: value ticks from minimum to
  // maximum, then categories, title, and legend. This is also the reading order
  // exposed by tagged fixed-format output.
  for (value, label) in &tick_labels {
    let width = metrics.measure_text(label, &style.label);
    push_text(
      &mut items,
      tick_left + maximum_tick_width - width,
      value_y(*value, scale, plot_top, plot_height) - label_line_height / 2.0,
      label.clone(),
      style.label.clone(),
    );
  }
  for (category_index, category) in chart.categories.iter().enumerate() {
    let width = metrics.measure_text(category, &style.label);
    let center = plot_left + (category_index as f32 + 0.5) / category_count as f32 * plot_width;
    push_text(
      &mut items,
      center - width / 2.0,
      category_top,
      category.clone(),
      style.label.clone(),
    );
  }
  if let Some(title) = title_text {
    let width = metrics.measure_text(title, &style.title);
    let title_x = frame.x_pt + (frame.width_pt - width) / 2.0;
    if let Some(color) = style.title_fill_color {
      let horizontal_padding = style.title.font_size_pt * 0.162;
      let vertical_padding = style.title.font_size_pt * 0.092;
      items.push(PageItem::Rect(RectItem {
        x_pt: title_x - horizontal_padding,
        y_pt: title_top - vertical_padding,
        width_pt: width + horizontal_padding * 2.0,
        height_pt: title_line_height + vertical_padding * 2.0,
        fill_color: Some(color),
        fill_opacity: 1.0,
        stroke: None,
        stroke_opacity: 1.0,
      }));
    }
    push_text(
      &mut items,
      title_x,
      title_top,
      title.to_string(),
      style.title.clone(),
    );
  }
  match legend_position {
    Some(ChartLegendPosition::Bottom) => lower_horizontal_legend(
      &mut items,
      frame,
      tick_left,
      legend_top,
      chart,
      style,
      &mut metrics,
    ),
    Some(ChartLegendPosition::Top) => lower_horizontal_legend(
      &mut items,
      frame,
      tick_left,
      if title_text.is_some() {
        title_top + title_line_height + frame.height_pt * 0.009
      } else {
        frame.y_pt + frame.height_pt * 0.018
      },
      chart,
      style,
      &mut metrics,
    ),
    Some(ChartLegendPosition::Left) => lower_vertical_legend(
      &mut items,
      frame.x_pt + side_legend_outer_margin,
      frame,
      false,
      chart,
      style,
    ),
    Some(ChartLegendPosition::Right | ChartLegendPosition::TopRight) => lower_vertical_legend(
      &mut items,
      frame.x_pt + frame.width_pt - side_legend_outer_margin - side_legend_width,
      frame,
      legend_position == Some(ChartLegendPosition::TopRight),
      chart,
      style,
    ),
    None => {}
  }
  items
}

fn lower_horizontal_legend(
  items: &mut Vec<PageItem>,
  frame: ChartFrame,
  available_left: f32,
  y: f32,
  chart: &ClusteredColumnChart<'_>,
  style: &ClusteredColumnStyle,
  metrics: &mut TextMetrics,
) {
  let marker_size = style.label.font_size_pt * 0.55;
  let marker_gap = style.label.font_size_pt * 0.247;
  let entry_gap = style.label.font_size_pt * 0.94;
  let widths: Vec<f32> = chart
    .series
    .iter()
    .map(|series| marker_size + marker_gap + metrics.measure_text(&series.name, &style.label))
    .collect();
  let total_width =
    widths.iter().sum::<f32>() + entry_gap * chart.series.len().saturating_sub(1) as f32;
  // The value-axis label band has already consumed the leading side of the
  // diagram. Center a bottom legend in the remaining horizontal region, as
  // Office does, rather than recentering it over the full graphic frame.
  let available_right = frame.x_pt + frame.width_pt;
  let mut x = available_left + (available_right - available_left - total_width) / 2.0;
  for (index, (series, width)) in chart.series.iter().zip(widths).enumerate() {
    if let Some(color) = style.series_colors.get(index).copied() {
      items.push(PageItem::Rect(RectItem {
        x_pt: x,
        y_pt: y + (line_height(&style.label) - marker_size) / 2.0,
        width_pt: marker_size,
        height_pt: marker_size,
        fill_color: Some(color),
        fill_opacity: 1.0,
        stroke: None,
        stroke_opacity: 1.0,
      }));
    }
    push_text(
      items,
      x + marker_size + marker_gap,
      y,
      series.name.clone(),
      style.label.clone(),
    );
    x += width + entry_gap;
  }
}

fn vertical_legend_width(
  chart: &ClusteredColumnChart<'_>,
  style: &ClusteredColumnStyle,
  metrics: &mut TextMetrics,
) -> f32 {
  let marker_size = style.label.font_size_pt * 0.55;
  let marker_gap = style.label.font_size_pt * 0.26;
  chart
    .series
    .iter()
    .map(|series| marker_size + marker_gap + metrics.measure_text(&series.name, &style.label))
    .fold(0.0_f32, f32::max)
}

fn lower_vertical_legend(
  items: &mut Vec<PageItem>,
  x: f32,
  frame: ChartFrame,
  align_top: bool,
  chart: &ClusteredColumnChart<'_>,
  style: &ClusteredColumnStyle,
) {
  let marker_size = style.label.font_size_pt * 0.55;
  let marker_gap = style.label.font_size_pt * 0.26;
  let entry_gap = style.label.font_size_pt
    * if matches!(
      style.layout_profile,
      ChartLayoutProfile::Excel | ChartLayoutProfile::Word
    ) {
      0.61
    } else {
      0.344
    };
  let line_height = line_height(&style.label);
  let total_height = line_height * chart.series.len() as f32
    + entry_gap * chart.series.len().saturating_sub(1) as f32;
  let mut y = if align_top {
    frame.y_pt + frame.height_pt * 0.04
  } else {
    frame.y_pt + (frame.height_pt - total_height) / 2.0
  };
  if style.layout_profile == ChartLayoutProfile::Excel && !align_top {
    y -= frame.height_pt * 0.0148;
  }
  for (index, series) in chart.series.iter().enumerate() {
    if let Some(color) = style.series_colors.get(index).copied() {
      items.push(PageItem::Rect(RectItem {
        x_pt: x,
        y_pt: y + (line_height - marker_size) / 2.0,
        width_pt: marker_size,
        height_pt: marker_size,
        fill_color: Some(color),
        fill_opacity: 1.0,
        stroke: None,
        stroke_opacity: 1.0,
      }));
    }
    push_text(
      items,
      x + marker_size + marker_gap,
      y,
      series.name.clone(),
      style.label.clone(),
    );
    y += line_height + entry_gap;
  }
}

fn value_y(
  value: f64,
  scale: crate::render::chart::LinearAxisScale,
  plot_top: f32,
  plot_height: f32,
) -> f32 {
  plot_top + ((scale.maximum - value) / (scale.maximum - scale.minimum)) as f32 * plot_height
}

fn scale_tick_labels(minimum: f64, maximum: f64, unit: f64) -> Vec<(f64, String)> {
  let count = ((maximum - minimum) / unit).floor().max(0.0) as usize;
  (0..=count)
    .map(|index| {
      let value = minimum + index as f64 * unit;
      (value, format_axis_value(value, unit))
    })
    .collect()
}

fn format_axis_value(value: f64, unit: f64) -> String {
  if unit.fract().abs() < 1.0e-10 {
    format!("{value:.0}")
  } else {
    let decimals = (-unit.abs().log10().floor()).max(0.0) as usize;
    let mut result = format!("{value:.decimals$}");
    while result.contains('.') && result.ends_with('0') {
      result.pop();
    }
    if result.ends_with('.') {
      result.pop();
    }
    result
  }
}

fn line_height(style: &TextStyle) -> f32 {
  style.font_size_pt * TEXT_LINE_HEIGHT_SCALE
}

fn push_text(items: &mut Vec<PageItem>, x: f32, y: f32, text: String, style: TextStyle) {
  items.push(PageItem::Text(TextItem {
    x_pt: x,
    y_pt: y,
    line_height_pt: line_height(&style),
    text,
    style,
    rotation_center_pt: None,
    hyperlink_url: None,
    form_widget_id: None,
    paragraph_bidi: false,
    preserve_text_portion: true,
    pdf_text_segmentation: PdfTextSegmentation::Line,
  }));
}

#[cfg(test)]
mod tests {
  use super::format_axis_value;

  #[test]
  fn axis_values_do_not_expose_binary_float_artifacts() {
    let value_with_binary_artifact = f64::from_bits(4.4_f64.to_bits() + 1);
    assert_eq!(format_axis_value(value_with_binary_artifact, 0.2), "4.4");
    assert_eq!(format_axis_value(6.0, 1.0), "6");
  }
}
