use crate::model::{
  BorderStyle, LineItem, LineItemKind, PageItem, PdfTextSegmentation, RectItem, RgbColor, TextItem,
  TextStyle, common_point, common_rect, common_rgb,
};
use crate::render::chart::{
  ChartLegendPosition, ChartSeriesGrouping, ChartSeriesKind, ChartTitleText, ClusteredColumnChart,
  PieChartModel, RadialChartKind, clustered_column_slot, linear_axis_scale,
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
  pub data_label: TextStyle,
  pub gridline_color: RgbColor,
  pub series_colors: Vec<RgbColor>,
  pub series_point_colors: Vec<Vec<Option<RgbColor>>>,
  pub chart_area_fill_color: Option<RgbColor>,
  pub plot_area_fill_color: Option<RgbColor>,
  pub chart_area_stroke_color: Option<RgbColor>,
  pub plot_area_stroke_color: Option<RgbColor>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ChartLayoutProfile {
  PowerPoint,
  Word,
  Excel,
}

#[derive(Clone, Debug)]
pub(crate) struct RadialChartStyle {
  pub layout_profile: ChartLayoutProfile,
  pub title: TextStyle,
  pub label: TextStyle,
  pub data_label: TextStyle,
  pub point_colors: Vec<RgbColor>,
  pub chart_area_fill_color: Option<RgbColor>,
  pub plot_area_fill_color: Option<RgbColor>,
  pub chart_area_stroke_color: Option<RgbColor>,
  pub plot_area_stroke_color: Option<RgbColor>,
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
  let scale_values = cartesian_scale_values(chart, category_count);
  let percent_stacked = chart
    .series
    .iter()
    .all(|series| series.grouping == ChartSeriesGrouping::PercentStacked);
  let scale = if percent_stacked {
    Some(crate::render::chart::LinearAxisScale {
      minimum: chart
        .value_axis
        .and_then(|axis| axis.scaling.min_axis_value.as_ref())
        .map_or(0.0, |value| value.val),
      maximum: chart
        .value_axis
        .and_then(|axis| axis.scaling.max_axis_value.as_ref())
        .map_or(1.0, |value| value.val),
      major_unit: chart
        .value_axis
        .and_then(|axis| axis.major_unit.as_ref())
        .map_or(0.1, |value| value.val),
      logarithmic_base: None,
      reversed: false,
    })
  } else {
    linear_axis_scale(scale_values, chart.value_axis, 10)
  };
  let Some(scale) = scale else {
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
  let data_label_line_height = line_height(&style.data_label);
  let radar_only = chart
    .series
    .iter()
    .all(|series| series.kind == ChartSeriesKind::Radar);
  let horizontal_bar_only = chart
    .series
    .iter()
    .all(|series| series.kind == ChartSeriesKind::Bar);
  let scatter_only = chart.series.iter().all(|series| {
    matches!(
      series.kind,
      ChartSeriesKind::Scatter | ChartSeriesKind::Bubble
    )
  });
  let value_axis_visible = chart.value_axis.is_none_or(value_axis_is_visible);
  let value_tick_labels_visible = value_axis_visible
    && !radar_only
    && !horizontal_bar_only
    && chart.value_axis.is_none_or(|axis| {
      axis
        .tick_label_position
        .as_ref()
        .is_none_or(|position| position.val != Some(c::TickLabelPositionValues::None))
    });
  let value_gridlines_visible = value_axis_visible
    && !radar_only
    && !horizontal_bar_only
    && chart
      .value_axis
      .is_none_or(|axis| axis.major_gridlines.is_some());
  let category_axis_visible = chart
    .category_axis
    .map(category_axis_is_visible)
    .or_else(|| chart.date_axis.map(date_axis_is_visible))
    .unwrap_or(true);
  let category_tick_labels_visible = chart.data_table.is_none()
    && !radar_only
    && !horizontal_bar_only
    && category_axis_visible
    && chart
      .category_axis
      .map(|axis| {
        axis
          .tick_label_position
          .as_ref()
          .is_none_or(|position| position.val != Some(c::TickLabelPositionValues::None))
          && category_axis_text_rotation_is_supported(
            axis.text_properties.as_deref(),
            category_count,
          )
      })
      .or_else(|| {
        chart.date_axis.map(|axis| {
          axis
            .tick_label_position
            .as_ref()
            .is_none_or(|position| position.val != Some(c::TickLabelPositionValues::None))
            && category_axis_text_rotation_is_supported(
              axis.text_properties.as_deref(),
              category_count,
            )
        })
      })
      .unwrap_or(true);
  let category_label_lines: Vec<Vec<String>> = if category_tick_labels_visible {
    let slot_width = frame.width_pt / category_count as f32 * 0.9;
    chart
      .categories
      .iter()
      .map(|category| wrap_chart_label(category, slot_width, &style.label, &mut metrics))
      .collect()
  } else {
    Vec::new()
  };
  let category_label_line_count =
    category_label_lines.iter().map(Vec::len).max().unwrap_or(1) as f32;
  let category_label_height = label_line_height * category_label_line_count;
  let legend_position = chart.legend_position;
  let has_bottom_legend =
    legend_position == Some(ChartLegendPosition::Bottom) && !chart.legend_overlay;
  let has_explicit_powerpoint_title =
    style.layout_profile == ChartLayoutProfile::PowerPoint && style.has_explicit_title;
  let has_top_legend = legend_position == Some(ChartLegendPosition::Top) && !chart.legend_overlay;
  let has_side_legend = matches!(
    legend_position,
    Some(ChartLegendPosition::Left | ChartLegendPosition::Right | ChartLegendPosition::TopRight)
  ) && !chart.legend_overlay;
  let data_table_height = chart.data_table.map_or(0.0, |_| {
    label_line_height * (chart.series.len() as f32 + 1.0) + label_line_height * 0.45
  });

  // Office's automatic chart layout reserves semantic bands around the plot:
  // title, value labels, category labels, and legend. The distances scale with
  // chart height, while actual label widths determine the left plot inset.
  let title_top = frame.y_pt
    + frame.height_pt
      * if style.layout_profile == ChartLayoutProfile::Word {
        0.0365
      } else {
        0.024
      };
  let legend_bottom_margin = style.label.font_size_pt * 0.81;
  let legend_top = frame.y_pt + frame.height_pt - legend_bottom_margin - label_line_height;
  let category_bottom_ratio = match style.layout_profile {
    ChartLayoutProfile::Excel => 0.05,
    ChartLayoutProfile::Word => 0.022_87,
    ChartLayoutProfile::PowerPoint => 0.018,
  };
  let category_top = if chart.data_table.is_some() {
    frame.y_pt + frame.height_pt
      - data_table_height
      - if has_bottom_legend {
        label_line_height + frame.height_pt * 0.045
      } else {
        frame.height_pt * category_bottom_ratio
      }
  } else if !category_tick_labels_visible {
    frame.y_pt + frame.height_pt * (1.0 - category_bottom_ratio)
  } else if has_bottom_legend {
    legend_top
      - category_label_height
      - frame.height_pt
        * if has_explicit_powerpoint_title {
          0.026_1
        } else {
          0.021
        }
  } else {
    frame.y_pt + frame.height_pt - category_label_height - frame.height_pt * category_bottom_ratio
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
    plot_top += label_line_height
      + frame.height_pt
        * if style.layout_profile == ChartLayoutProfile::Word {
          0.031
        } else {
          0.018
        };
  }
  let side_category_gap_ratio = match style.layout_profile {
    ChartLayoutProfile::Excel => 0.025,
    ChartLayoutProfile::Word => 0.027_75,
    ChartLayoutProfile::PowerPoint => 0.033_35,
  };
  let category_plot_gap_ratio =
    if style.layout_profile == ChartLayoutProfile::Word && category_label_line_count > 1.0 {
      0.039
    } else {
      0.018
    };
  let mut plot_bottom = category_top
    - frame.height_pt
      * if has_side_legend {
        side_category_gap_ratio
      } else if has_bottom_legend && has_explicit_powerpoint_title {
        0.0225
      } else {
        category_plot_gap_ratio
      };
  if plot_bottom <= plot_top {
    return Vec::new();
  }

  let value_number_format = chart
    .value_axis
    .and_then(|axis| axis.numbering_format.as_ref())
    .map(|format| format.format_code.as_str())
    .or(percent_stacked.then_some("0%"));
  let tick_labels = scale_tick_labels(
    scale.minimum,
    scale.maximum,
    scale.major_unit,
    value_number_format,
    scale.logarithmic_base,
  );
  let maximum_tick_width = if value_tick_labels_visible {
    tick_labels
      .iter()
      .map(|(_, label)| metrics.measure_text(label, &style.label))
      .fold(0.0_f32, f32::max)
  } else {
    0.0
  };
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
  let tick_left_ratio =
    if !value_tick_labels_visible && style.layout_profile == ChartLayoutProfile::Word {
      0.0455
    } else if has_side_legend {
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
  let tick_gap = if value_tick_labels_visible {
    frame.height_pt
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
      }
  } else {
    0.0
  };
  let mut plot_left = tick_left + maximum_tick_width + tick_gap;
  let mut plot_right = frame.x_pt + frame.width_pt
    - if matches!(
      legend_position,
      Some(ChartLegendPosition::Right | ChartLegendPosition::TopRight)
    ) {
      side_legend_width + side_plot_outer_margin + side_plot_gap
    } else {
      frame.height_pt
        * if has_bottom_legend && has_explicit_powerpoint_title {
          0.0301
        } else if !value_tick_labels_visible && style.layout_profile == ChartLayoutProfile::Word {
          0.041
        } else {
          0.026
        }
    };
  if horizontal_bar_only {
    let category_width = chart
      .categories
      .iter()
      .map(|category| metrics.measure_text(category, &style.label))
      .fold(0.0_f32, f32::max);
    plot_left += category_width + style.label.font_size_pt * 0.8;
    plot_bottom -= label_line_height * 1.35;
  }
  if plot_right <= plot_left {
    return Vec::new();
  }
  if let Some(layout) = chart.plot_layout {
    let automatic = PlotRect {
      left: plot_left,
      top: plot_top,
      width: plot_right - plot_left,
      height: plot_bottom - plot_top,
    };
    let manual = apply_manual_layout(frame, automatic, layout);
    plot_left = manual.left;
    plot_top = manual.top;
    plot_right = manual.left + manual.width;
    plot_bottom = manual.top + manual.height;
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
  if style.chart_area_fill_color.is_some() || style.chart_area_stroke_color.is_some() {
    items.push(PageItem::Rect(RectItem {
      x_pt: frame.x_pt,
      y_pt: frame.y_pt,
      width_pt: frame.width_pt,
      height_pt: frame.height_pt,
      fill_color: style.chart_area_fill_color,
      fill_opacity: 1.0,
      stroke: style.chart_area_stroke_color.map(|color| BorderStyle {
        width_pt: 0.75,
        color,
        ..BorderStyle::default()
      }),
      stroke_opacity: 1.0,
    }));
  }
  if style.plot_area_fill_color.is_some() || style.plot_area_stroke_color.is_some() {
    items.push(PageItem::Rect(RectItem {
      x_pt: plot_left,
      y_pt: plot_top,
      width_pt: plot_width,
      height_pt: plot_height,
      fill_color: style.plot_area_fill_color,
      fill_opacity: 1.0,
      stroke: style.plot_area_stroke_color.map(|color| BorderStyle {
        width_pt: 0.75,
        color,
        ..BorderStyle::default()
      }),
      stroke_opacity: 1.0,
    }));
  }
  if value_gridlines_visible {
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
  }
  if radar_only {
    lower_radar_axes(
      &mut items,
      chart,
      PlotRect {
        left: plot_left,
        top: plot_top,
        width: plot_width,
        height: plot_height,
      },
      scale,
      style,
      &mut metrics,
    );
  } else if horizontal_bar_only {
    lower_horizontal_bar_axes(
      &mut items,
      chart,
      PlotRect {
        left: plot_left,
        top: plot_top,
        width: plot_width,
        height: plot_height,
      },
      &tick_labels,
      scale,
      style,
      &mut metrics,
    );
  } else if scatter_only {
    lower_scatter_x_axis(
      &mut items,
      chart,
      PlotRect {
        left: plot_left,
        top: plot_top,
        width: plot_width,
        height: plot_height,
      },
      style,
      &mut metrics,
    );
  }

  lower_series_geometry(
    &mut items,
    chart,
    style,
    PlotRect {
      left: plot_left,
      top: plot_top,
      width: plot_width,
      height: plot_height,
    },
    scale,
    zero_y,
    category_count,
  );
  if chart.has_high_low_lines || chart.has_up_down_bars {
    lower_stock_overlays(
      &mut items,
      chart,
      PlotRect {
        left: plot_left,
        top: plot_top,
        width: plot_width,
        height: plot_height,
      },
      scale,
      style,
      category_count,
    );
  }

  if category_axis_visible && !radar_only && !horizontal_bar_only {
    items.push(PageItem::Line(LineItem {
      x1_pt: plot_left,
      y1_pt: zero_y,
      x2_pt: plot_right,
      y2_pt: zero_y,
      width_pt: 0.75,
      color: style.gridline_color,
      kind: LineItemKind::Stroke,
    }));
  }
  if style.layout_profile == ChartLayoutProfile::Word {
    let tick_length = frame.height_pt * 0.012_59;
    if value_axis_visible {
      items.push(PageItem::Line(LineItem {
        x1_pt: plot_left,
        y1_pt: plot_top,
        x2_pt: plot_left,
        y2_pt: zero_y,
        width_pt: 0.75,
        color: style.gridline_color,
        kind: LineItemKind::Stroke,
      }));
      if chart.value_axis.is_none_or(value_axis_has_major_ticks) {
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
      }
    }
    if category_axis_visible
      && chart
        .category_axis
        .map(category_axis_has_major_ticks)
        .or_else(|| chart.date_axis.map(date_axis_has_major_ticks))
        .unwrap_or(true)
    {
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
  }

  for (series_index, series) in chart.series.iter().enumerate() {
    for label in &series.data_labels {
      let Some(anchor) = data_label_anchor(
        chart,
        series_index,
        label.point_index,
        PlotRect {
          left: plot_left,
          top: plot_top,
          width: plot_width,
          height: plot_height,
        },
        scale,
        zero_y,
        category_count,
      ) else {
        continue;
      };
      let width = metrics.measure_text(&label.text, &style.data_label);
      let (x, y) = match label.position {
        c::DataLabelPositionValues::Center => (
          (anchor.x + anchor.base_x) * 0.5 - width * 0.5,
          (anchor.y + anchor.base_y) * 0.5 - data_label_line_height * 0.5,
        ),
        c::DataLabelPositionValues::InsideBase => (
          anchor.base_x - width * 0.5,
          anchor.base_y - data_label_line_height,
        ),
        c::DataLabelPositionValues::InsideEnd => (anchor.x - width * 0.5, anchor.y),
        c::DataLabelPositionValues::Bottom => (
          anchor.x - width * 0.5,
          anchor.y + data_label_line_height * 0.15,
        ),
        c::DataLabelPositionValues::Left => (
          anchor.x - width - style.data_label.font_size_pt * 0.2,
          anchor.y - data_label_line_height * 0.5,
        ),
        c::DataLabelPositionValues::Right => (
          anchor.x + style.data_label.font_size_pt * 0.2,
          anchor.y - data_label_line_height * 0.5,
        ),
        c::DataLabelPositionValues::OutsideEnd
        | c::DataLabelPositionValues::Top
        | c::DataLabelPositionValues::BestFit => {
          (anchor.x - width * 0.5, anchor.y - data_label_line_height)
        }
      };
      push_text(
        &mut items,
        x,
        y,
        label.text.clone(),
        style.data_label.clone(),
      );
    }
  }

  // Preserve Office's content-stream ordering: value ticks from minimum to
  // maximum, then categories, title, and legend. This is also the reading order
  // exposed by tagged fixed-format output.
  if value_tick_labels_visible {
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
  }
  if category_tick_labels_visible {
    for (category_index, lines) in category_label_lines.iter().enumerate() {
      let display_index = category_display_index(chart, category_index, category_count);
      let center = plot_left + (display_index as f32 + 0.5) / category_count as f32 * plot_width;
      for (line_index, line) in lines.iter().enumerate() {
        let width = metrics.measure_text(line, &style.label);
        push_text(
          &mut items,
          center - width / 2.0,
          category_top + line_index as f32 * label_line_height,
          line.clone(),
          style.label.clone(),
        );
      }
    }
  }
  if let Some(data_table) = chart.data_table {
    lower_data_table(
      &mut items,
      chart,
      data_table,
      PlotRect {
        left: plot_left,
        top: category_top,
        width: plot_width,
        height: data_table_height,
      },
      style,
      &mut metrics,
    );
  }
  lower_axis_titles(
    &mut items,
    frame,
    PlotRect {
      left: plot_left,
      top: plot_top,
      width: plot_width,
      height: plot_height,
    },
    category_top,
    chart,
    style,
    &mut metrics,
  );
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
  if let Some(layout) = chart.legend_layout {
    lower_manual_legend(&mut items, frame, layout, chart, style);
  } else {
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
        frame.x_pt + frame.height_pt * 0.004,
        if title_text.is_some() {
          title_top
            + title_line_height
            + frame.height_pt
              * if style.layout_profile == ChartLayoutProfile::Word {
                0.0375
              } else {
                0.009
              }
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
  }
  items
}

pub(crate) fn lower_radial_chart(
  frame: ChartFrame,
  chart: &PieChartModel<'_>,
  automatic_title: &str,
  style: &RadialChartStyle,
) -> Vec<PageItem> {
  if frame.width_pt <= 0.0
    || frame.height_pt <= 0.0
    || chart.values.is_empty()
    || style.point_colors.is_empty()
  {
    return Vec::new();
  }
  let total = chart
    .values
    .iter()
    .flatten()
    .filter(|value| value.is_finite() && **value > 0.0)
    .sum::<f64>();
  if total <= f64::EPSILON {
    return Vec::new();
  }

  let mut metrics = TextMetrics::new();
  let title = match chart.title.as_ref() {
    Some(ChartTitleText::Explicit(title)) => Some(title.as_str()),
    Some(ChartTitleText::Automatic) => Some(automatic_title),
    None => None,
  };
  let title_height = title.map_or(0.0, |_| line_height(&style.title) * 1.5);
  let legend = chart.legend_position;
  let side_legend = matches!(
    legend,
    Some(ChartLegendPosition::Left | ChartLegendPosition::Right | ChartLegendPosition::TopRight)
  ) && !chart.legend_overlay;
  let bottom_legend = legend == Some(ChartLegendPosition::Bottom) && !chart.legend_overlay;
  let top_legend = legend == Some(ChartLegendPosition::Top) && !chart.legend_overlay;
  let legend_width = if side_legend {
    chart
      .visible_legend_indices
      .iter()
      .filter_map(|index| chart.categories.get(*index))
      .map(|text| metrics.measure_text(text, &style.label))
      .fold(0.0_f32, f32::max)
      + style.label.font_size_pt * 2.2
  } else {
    0.0
  };
  let legend_height = if bottom_legend || top_legend {
    line_height(&style.label) * 1.8
  } else {
    0.0
  };
  let mut plot = PlotRect {
    left: frame.x_pt
      + if legend == Some(ChartLegendPosition::Left) {
        legend_width
      } else {
        0.0
      },
    top: frame.y_pt + title_height + if top_legend { legend_height } else { 0.0 },
    width: frame.width_pt - if side_legend { legend_width } else { 0.0 },
    height: frame.height_pt - title_height - legend_height,
  };
  if let Some(layout) = chart.plot_layout {
    plot = apply_manual_layout(frame, plot, layout);
  }
  if plot.width <= 0.0 || plot.height <= 0.0 {
    return Vec::new();
  }

  let depth = if chart.kind == RadialChartKind::Pie3D {
    plot.height * 0.09
  } else {
    0.0
  };
  let radius_scale = match style.layout_profile {
    ChartLayoutProfile::PowerPoint => 0.40,
    ChartLayoutProfile::Word => 0.42,
    ChartLayoutProfile::Excel => 0.41,
  };
  let radius_x = plot.width.min(plot.height * 1.35) * radius_scale;
  let radius_y = if chart.kind == RadialChartKind::Pie3D {
    radius_x * 0.62
  } else {
    radius_x
  };
  let center_x = plot.left + plot.width * 0.5;
  let center_y = plot.top + (plot.height - depth) * 0.5;
  let hole_ratio = (chart.hole_size_percent / 100.0).clamp(0.0, 0.9) as f32;
  let mut items = Vec::new();
  if style.chart_area_fill_color.is_some() || style.chart_area_stroke_color.is_some() {
    items.push(PageItem::Rect(RectItem {
      x_pt: frame.x_pt,
      y_pt: frame.y_pt,
      width_pt: frame.width_pt,
      height_pt: frame.height_pt,
      fill_color: style.chart_area_fill_color,
      fill_opacity: 1.0,
      stroke: style.chart_area_stroke_color.map(|color| BorderStyle {
        width_pt: 0.75,
        color,
        ..BorderStyle::default()
      }),
      stroke_opacity: 1.0,
    }));
  }
  if style.plot_area_fill_color.is_some() || style.plot_area_stroke_color.is_some() {
    items.push(PageItem::Rect(RectItem {
      x_pt: plot.left,
      y_pt: plot.top,
      width_pt: plot.width,
      height_pt: plot.height,
      fill_color: style.plot_area_fill_color,
      fill_opacity: 1.0,
      stroke: style.plot_area_stroke_color.map(|color| BorderStyle {
        width_pt: 0.75,
        color,
        ..BorderStyle::default()
      }),
      stroke_opacity: 1.0,
    }));
  }
  let mut start_angle = chart.first_slice_angle_deg.to_radians() as f32;

  if matches!(
    chart.kind,
    RadialChartKind::PieOfPie | RadialChartKind::BarOfPie
  ) && !chart.secondary_indices.is_empty()
  {
    lower_of_pie_geometry(&mut items, plot, chart, style);
  } else {
    for (index, value) in chart.values.iter().enumerate() {
      let Some(value) = value
        .as_ref()
        .copied()
        .filter(|value| value.is_finite() && *value > 0.0)
      else {
        continue;
      };
      let sweep = (value / total * std::f64::consts::TAU) as f32;
      let mid = start_angle + sweep * 0.5;
      let explosion = (chart
        .point_explosion_percent
        .get(index)
        .copied()
        .unwrap_or(chart.series_explosion_percent)
        / 100.0)
        .clamp(0.0, 1.0) as f32;
      let offset_x = mid.sin() * radius_x * explosion * 0.24;
      let offset_y = -mid.cos() * radius_y * explosion * 0.24;
      let color = style.point_colors[index % style.point_colors.len()];
      if depth > 0.0 {
        items.push(radial_segment_path(
          (center_x + offset_x, center_y + offset_y + depth),
          (radius_x, radius_y),
          hole_ratio,
          (start_angle, sweep),
          (color, 0.58),
        ));
      }
      items.push(radial_segment_path(
        (center_x + offset_x, center_y + offset_y),
        (radius_x, radius_y),
        hole_ratio,
        (start_angle, sweep),
        (color, 1.0),
      ));
      start_angle += sweep;
    }
  }

  for label in &chart.data_labels {
    let before = chart
      .values
      .iter()
      .take(label.point_index)
      .flatten()
      .filter(|value| value.is_finite() && **value > 0.0)
      .sum::<f64>();
    let Some(value) = chart
      .values
      .get(label.point_index)
      .copied()
      .flatten()
      .filter(|value| value.is_finite() && *value > 0.0)
    else {
      continue;
    };
    let angle = chart.first_slice_angle_deg.to_radians() as f32
      + ((before + value * 0.5) / total * std::f64::consts::TAU) as f32;
    let outside = matches!(
      label.position,
      c::DataLabelPositionValues::OutsideEnd
        | c::DataLabelPositionValues::Left
        | c::DataLabelPositionValues::Right
    );
    let ring = if outside {
      1.08
    } else {
      (1.0 + hole_ratio) * 0.5
    };
    let width = metrics.measure_text(&label.text, &style.data_label);
    let label_x = center_x + angle.sin() * radius_x * ring - width * 0.5;
    let label_y = center_y - angle.cos() * radius_y * ring - line_height(&style.data_label) * 0.5;
    if outside && chart.show_leader_lines {
      items.push(PageItem::Line(LineItem {
        x1_pt: center_x + angle.sin() * radius_x * 0.94,
        y1_pt: center_y - angle.cos() * radius_y * 0.94,
        x2_pt: label_x + width * 0.5,
        y2_pt: label_y + line_height(&style.data_label) * 0.5,
        width_pt: 0.75,
        color: style.data_label.color,
        kind: LineItemKind::Stroke,
      }));
    }
    push_text(
      &mut items,
      label_x,
      label_y,
      label.text.clone(),
      style.data_label.clone(),
    );
  }

  if let Some(title) = title {
    let width = metrics.measure_text(title, &style.title);
    push_text(
      &mut items,
      frame.x_pt + (frame.width_pt - width) * 0.5,
      frame.y_pt + frame.height_pt * 0.025,
      title.to_string(),
      style.title.clone(),
    );
  }
  lower_radial_legend(
    &mut items,
    frame,
    chart,
    style,
    &mut metrics,
    legend_width,
    legend_height,
  );
  items
}

fn lower_of_pie_geometry(
  items: &mut Vec<PageItem>,
  plot: PlotRect,
  chart: &PieChartModel<'_>,
  style: &RadialChartStyle,
) {
  let secondary = chart
    .secondary_indices
    .iter()
    .filter_map(|index| chart.values.get(*index).copied().flatten())
    .filter(|value| value.is_finite() && *value > 0.0)
    .sum::<f64>();
  let mut primary = chart
    .values
    .iter()
    .enumerate()
    .filter(|(index, _)| !chart.secondary_indices.contains(index))
    .filter_map(|(index, value)| {
      value
        .as_ref()
        .copied()
        .filter(|value| value.is_finite() && *value > 0.0)
        .map(|value| (index, value))
    })
    .collect::<Vec<_>>();
  if secondary > 0.0 {
    primary.push((chart.secondary_indices[0], secondary));
  }
  let primary_total = primary.iter().map(|(_, value)| *value).sum::<f64>();
  if primary_total <= f64::EPSILON {
    return;
  }
  let primary_center = (plot.left + plot.width * 0.32, plot.top + plot.height * 0.5);
  let primary_radius = plot.width.min(plot.height) * 0.29;
  let secondary_center = (plot.left + plot.width * 0.77, plot.top + plot.height * 0.5);
  let secondary_radius =
    primary_radius * (chart.secondary_size_percent / 100.0).clamp(0.05, 2.0) as f32 * 0.65;
  let mut angle = 0.0_f32;
  for (index, value) in primary {
    let sweep = (value / primary_total * std::f64::consts::TAU) as f32;
    items.push(radial_segment_path(
      primary_center,
      (primary_radius, primary_radius),
      0.0,
      (angle, sweep),
      (style.point_colors[index % style.point_colors.len()], 1.0),
    ));
    angle += sweep;
  }

  if chart.kind == RadialChartKind::PieOfPie {
    let mut angle = 0.0_f32;
    for index in chart.secondary_indices.iter().copied() {
      let Some(value) = chart
        .values
        .get(index)
        .copied()
        .flatten()
        .filter(|value| value.is_finite() && *value > 0.0)
      else {
        continue;
      };
      let sweep = (value / secondary * std::f64::consts::TAU) as f32;
      items.push(radial_segment_path(
        secondary_center,
        (secondary_radius, secondary_radius),
        0.0,
        (angle, sweep),
        (style.point_colors[index % style.point_colors.len()], 1.0),
      ));
      angle += sweep;
    }
  } else {
    let mut y = secondary_center.1 - secondary_radius;
    for index in chart.secondary_indices.iter().copied() {
      let Some(value) = chart
        .values
        .get(index)
        .copied()
        .flatten()
        .filter(|value| value.is_finite() && *value > 0.0)
      else {
        continue;
      };
      let height = (value / secondary) as f32 * secondary_radius * 2.0;
      items.push(PageItem::Rect(RectItem {
        x_pt: secondary_center.0 - secondary_radius * 0.45,
        y_pt: y,
        width_pt: secondary_radius * 0.9,
        height_pt: height,
        fill_color: Some(style.point_colors[index % style.point_colors.len()]),
        fill_opacity: 1.0,
        stroke: None,
        stroke_opacity: 1.0,
      }));
      y += height;
    }
  }
  for direction in [-1.0_f32, 1.0] {
    items.push(PageItem::Line(LineItem {
      x1_pt: primary_center.0 + primary_radius * 0.75,
      y1_pt: primary_center.1 + direction * primary_radius * 0.45,
      x2_pt: secondary_center.0 - secondary_radius,
      y2_pt: secondary_center.1 + direction * secondary_radius * 0.7,
      width_pt: 0.75,
      color: RgbColor {
        r: 128,
        g: 128,
        b: 128,
      },
      kind: LineItemKind::Stroke,
    }));
  }
}

fn radial_segment_path(
  center: (f32, f32),
  radii: (f32, f32),
  hole_ratio: f32,
  angles: (f32, f32),
  paint: (RgbColor, f32),
) -> PageItem {
  let (center_x, center_y) = center;
  let (radius_x, radius_y) = radii;
  let (start_angle, sweep) = angles;
  let (color, opacity) = paint;
  let segment_count = ((sweep.to_degrees().abs() / 2.0).ceil() as usize).max(2);
  let mut points = Vec::with_capacity(segment_count * 2 + 3);
  if hole_ratio <= f32::EPSILON {
    points.push(common_point(center_x, center_y));
  }
  for segment in 0..=segment_count {
    let angle = start_angle + sweep * segment as f32 / segment_count as f32;
    points.push(common_point(
      center_x + angle.sin() * radius_x,
      center_y - angle.cos() * radius_y,
    ));
  }
  if hole_ratio > f32::EPSILON {
    for segment in (0..=segment_count).rev() {
      let angle = start_angle + sweep * segment as f32 / segment_count as f32;
      points.push(common_point(
        center_x + angle.sin() * radius_x * hole_ratio,
        center_y - angle.cos() * radius_y * hole_ratio,
      ));
    }
  }
  PageItem::Path(crate::common::PathItem {
    bounds: common_rect(
      center_x - radius_x,
      center_y - radius_y,
      radius_x * 2.0,
      radius_y * 2.0,
    ),
    points,
    commands: Vec::new(),
    closed: true,
    fill: crate::common::Fill::Solid(common_rgb(color, opacity)),
    stroke: Some(crate::common::Stroke {
      width: crate::common::Pt(0.75),
      color: common_rgb(
        RgbColor {
          r: 255,
          g: 255,
          b: 255,
        },
        opacity,
      ),
      dash: None,
      source_style_id: None,
    }),
  })
}

fn lower_radial_legend(
  items: &mut Vec<PageItem>,
  frame: ChartFrame,
  chart: &PieChartModel<'_>,
  style: &RadialChartStyle,
  metrics: &mut TextMetrics,
  side_width: f32,
  horizontal_height: f32,
) {
  let Some(position) = chart.legend_position else {
    return;
  };
  let marker = style.label.font_size_pt * 0.55;
  let gap = style.label.font_size_pt * 0.3;
  if let Some(layout) = chart.legend_layout {
    let mut y = frame.y_pt + layout.y.unwrap_or(0.1) * frame.height_pt;
    let x = frame.x_pt + layout.x.unwrap_or(0.75) * frame.width_pt;
    for index in chart.visible_legend_indices.iter().copied() {
      let Some(text) = chart.categories.get(index) else {
        continue;
      };
      items.push(PageItem::Rect(RectItem {
        x_pt: x,
        y_pt: y,
        width_pt: marker,
        height_pt: marker,
        fill_color: Some(style.point_colors[index % style.point_colors.len()]),
        fill_opacity: 1.0,
        stroke: None,
        stroke_opacity: 1.0,
      }));
      push_text(
        items,
        x + marker + gap,
        y - (line_height(&style.label) - marker) * 0.5,
        text.clone(),
        style.label.clone(),
      );
      y += line_height(&style.label) * 1.25;
    }
    return;
  }
  if matches!(
    position,
    ChartLegendPosition::Bottom | ChartLegendPosition::Top
  ) {
    let entry_gap = style.label.font_size_pt;
    let widths = chart
      .visible_legend_indices
      .iter()
      .filter_map(|index| chart.categories.get(*index))
      .map(|text| marker + gap + metrics.measure_text(text, &style.label))
      .collect::<Vec<_>>();
    let total = widths.iter().sum::<f32>() + entry_gap * widths.len().saturating_sub(1) as f32;
    let mut x = frame.x_pt + (frame.width_pt - total) * 0.5;
    let y = if position == ChartLegendPosition::Top {
      frame.y_pt + horizontal_height * 0.2
    } else {
      frame.y_pt + frame.height_pt - horizontal_height * 0.8
    };
    for ((index, text), width) in chart
      .visible_legend_indices
      .iter()
      .filter_map(|index| chart.categories.get(*index).map(|text| (*index, text)))
      .zip(widths)
    {
      items.push(PageItem::Rect(RectItem {
        x_pt: x,
        y_pt: y + (line_height(&style.label) - marker) * 0.5,
        width_pt: marker,
        height_pt: marker,
        fill_color: Some(style.point_colors[index % style.point_colors.len()]),
        fill_opacity: 1.0,
        stroke: None,
        stroke_opacity: 1.0,
      }));
      push_text(
        items,
        x + marker + gap,
        y,
        text.clone(),
        style.label.clone(),
      );
      x += width + entry_gap;
    }
  } else {
    let x = if position == ChartLegendPosition::Left {
      frame.x_pt + style.label.font_size_pt * 0.4
    } else {
      frame.x_pt + frame.width_pt - side_width + style.label.font_size_pt * 0.4
    };
    let total_height = line_height(&style.label) * chart.visible_legend_indices.len() as f32;
    let mut y = frame.y_pt + (frame.height_pt - total_height) * 0.5;
    for index in chart.visible_legend_indices.iter().copied() {
      let Some(text) = chart.categories.get(index) else {
        continue;
      };
      items.push(PageItem::Rect(RectItem {
        x_pt: x,
        y_pt: y + (line_height(&style.label) - marker) * 0.5,
        width_pt: marker,
        height_pt: marker,
        fill_color: Some(style.point_colors[index % style.point_colors.len()]),
        fill_opacity: 1.0,
        stroke: None,
        stroke_opacity: 1.0,
      }));
      push_text(
        items,
        x + marker + gap,
        y,
        text.clone(),
        style.label.clone(),
      );
      y += line_height(&style.label);
    }
  }
}

#[derive(Clone, Copy)]
struct PlotRect {
  left: f32,
  top: f32,
  width: f32,
  height: f32,
}

struct SeriesGeometryContext<'chart, 'data> {
  chart: &'chart ClusteredColumnChart<'data>,
  plot: PlotRect,
  scale: crate::render::chart::LinearAxisScale,
  zero_y: f32,
  category_count: usize,
}

#[derive(Clone, Copy)]
struct ScatterGeometry {
  plot: PlotRect,
  scale: crate::render::chart::LinearAxisScale,
  x_extent: Option<(f64, f64)>,
  bubble_maximum: f64,
}

fn cartesian_scale_values(chart: &ClusteredColumnChart<'_>, category_count: usize) -> Vec<f64> {
  let mut values = chart
    .series
    .iter()
    .filter(|series| {
      !matches!(
        series.grouping,
        ChartSeriesGrouping::Stacked | ChartSeriesGrouping::PercentStacked
      )
    })
    .flat_map(|series| series.values.iter().flatten().copied())
    .collect::<Vec<_>>();
  if chart
    .series
    .iter()
    .any(|series| series.grouping == ChartSeriesGrouping::PercentStacked)
  {
    values.extend([0.0, 1.0]);
  }
  for kind in [
    ChartSeriesKind::Column,
    ChartSeriesKind::Bar,
    ChartSeriesKind::Line,
    ChartSeriesKind::Area,
  ] {
    for category_index in 0..category_count {
      let mut positive = 0.0;
      let mut negative = 0.0;
      let mut found = false;
      for series in chart
        .series
        .iter()
        .filter(|series| series.kind == kind && series.grouping == ChartSeriesGrouping::Stacked)
      {
        if let Some(value) = series.values.get(category_index).copied().flatten() {
          found = true;
          if value >= 0.0 {
            positive += value;
          } else {
            negative += value;
          }
        }
      }
      if found {
        values.push(positive);
        values.push(negative);
      }
    }
  }
  values
}

fn lower_series_geometry(
  items: &mut Vec<PageItem>,
  chart: &ClusteredColumnChart<'_>,
  style: &ClusteredColumnStyle,
  plot: PlotRect,
  scale: crate::render::chart::LinearAxisScale,
  zero_y: f32,
  category_count: usize,
) {
  let x_extent = chart
    .series
    .iter()
    .flat_map(|series| series.x_values.iter().flatten().copied())
    .fold(None, |extent, value| match extent {
      Some((minimum, maximum)) => Some((f64::min(minimum, value), f64::max(maximum, value))),
      None => Some((value, value)),
    });
  let bubble_maximum = chart
    .series
    .iter()
    .flat_map(|series| series.bubble_sizes.iter().flatten().copied())
    .fold(0.0_f64, f64::max);
  let context = SeriesGeometryContext {
    chart,
    plot,
    scale,
    zero_y,
    category_count,
  };
  let scatter_geometry = ScatterGeometry {
    plot,
    scale,
    x_extent,
    bubble_maximum,
  };

  for (series_index, series) in chart.series.iter().enumerate() {
    let Some(color) = style.series_colors.get(series_index).copied() else {
      continue;
    };
    match series.kind {
      ChartSeriesKind::Column => {
        lower_column_series(items, &context, style, series_index, color);
      }
      ChartSeriesKind::Bar => {
        lower_bar_series(items, &context, style, series_index, color);
      }
      ChartSeriesKind::Line | ChartSeriesKind::Stock => {
        lower_line_series(items, &context, series_index, color, false, style);
      }
      ChartSeriesKind::Area | ChartSeriesKind::Surface => {
        lower_line_series(items, &context, series_index, color, true, style);
      }
      ChartSeriesKind::Scatter => {
        lower_scatter_series(
          items,
          series,
          series_index,
          color,
          scatter_geometry,
          false,
          style,
        );
      }
      ChartSeriesKind::Bubble => {
        lower_scatter_series(
          items,
          series,
          series_index,
          color,
          scatter_geometry,
          true,
          style,
        );
      }
      ChartSeriesKind::Radar => {
        lower_radar_series(items, &context, series, series_index, color, style);
      }
    }
    if !series.trendlines.is_empty() {
      lower_trendlines(items, series, color, plot, scale, category_count, x_extent);
    }
  }
}

fn lower_trendlines(
  items: &mut Vec<PageItem>,
  series: &crate::render::chart::ClusteredColumnSeries<'_>,
  color: RgbColor,
  plot: PlotRect,
  scale: crate::render::chart::LinearAxisScale,
  category_count: usize,
  x_extent: Option<(f64, f64)>,
) {
  let source = series
    .values
    .iter()
    .enumerate()
    .filter_map(|(index, value)| {
      value.as_ref().copied().map(|value| {
        (
          series
            .x_values
            .get(index)
            .copied()
            .flatten()
            .unwrap_or(index as f64 + 1.0),
          value,
        )
      })
    })
    .filter(|(x, y)| x.is_finite() && y.is_finite())
    .collect::<Vec<_>>();
  if source.len() < 2 {
    return;
  }
  for trendline in series.trendlines {
    let kind = trendline
      .trendline_type
      .val
      .unwrap_or(c::TrendlineValues::Linear);
    let points = if kind == c::TrendlineValues::MovingAverage {
      let period = trendline
        .period
        .as_ref()
        .map_or(2, |period| period.val as usize)
        .clamp(2, source.len());
      source
        .windows(period)
        .map(|window| {
          (
            window[period - 1].0,
            window.iter().map(|(_, value)| *value).sum::<f64>() / period as f64,
          )
        })
        .collect::<Vec<_>>()
    } else {
      let transformed = source
        .iter()
        .filter_map(|(x, y)| match kind {
          c::TrendlineValues::Exponential if *y > 0.0 => Some((*x, y.ln())),
          c::TrendlineValues::Logarithmic if *x > 0.0 => Some((x.ln(), *y)),
          c::TrendlineValues::Power if *x > 0.0 && *y > 0.0 => Some((x.ln(), y.ln())),
          c::TrendlineValues::Linear
          | c::TrendlineValues::Polynomial
          | c::TrendlineValues::Exponential
          | c::TrendlineValues::Logarithmic
          | c::TrendlineValues::Power => Some((*x, *y)),
          c::TrendlineValues::MovingAverage => None,
        })
        .collect::<Vec<_>>();
      let Some((slope, intercept)) = linear_regression(&transformed) else {
        continue;
      };
      let (x_minimum, x_maximum) = source.iter().fold(
        (f64::INFINITY, f64::NEG_INFINITY),
        |(minimum, maximum), (x, _)| (minimum.min(*x), maximum.max(*x)),
      );
      (0..=24)
        .map(|step| x_minimum + (x_maximum - x_minimum) * step as f64 / 24.0)
        .filter_map(|x| {
          let y = match kind {
            c::TrendlineValues::Exponential => (slope * x + intercept).exp(),
            c::TrendlineValues::Logarithmic if x > 0.0 => slope * x.ln() + intercept,
            c::TrendlineValues::Power if x > 0.0 => (slope * x.ln() + intercept).exp(),
            _ => slope * x + intercept,
          };
          y.is_finite().then_some((x, y))
        })
        .collect()
    };
    let mut previous = None;
    for (x_value, y_value) in points {
      let x = if matches!(
        series.kind,
        ChartSeriesKind::Scatter | ChartSeriesKind::Bubble
      ) {
        let (minimum, maximum) = x_extent.unwrap_or((0.0, 1.0));
        if (maximum - minimum).abs() <= f64::EPSILON {
          plot.left + plot.width * 0.5
        } else {
          plot.left + ((x_value - minimum) / (maximum - minimum)) as f32 * plot.width
        }
      } else {
        plot.left + (x_value as f32 - 0.5) / category_count.max(1) as f32 * plot.width
      };
      let y = value_y(y_value, scale, plot.top, plot.height);
      if let Some((previous_x, previous_y)) = previous {
        items.push(PageItem::Line(LineItem {
          x1_pt: previous_x,
          y1_pt: previous_y,
          x2_pt: x,
          y2_pt: y,
          width_pt: 1.0,
          color,
          kind: LineItemKind::Stroke,
        }));
      }
      previous = Some((x, y));
    }
  }
}

fn linear_regression(values: &[(f64, f64)]) -> Option<(f64, f64)> {
  if values.len() < 2 {
    return None;
  }
  let count = values.len() as f64;
  let sum_x = values.iter().map(|(x, _)| *x).sum::<f64>();
  let sum_y = values.iter().map(|(_, y)| *y).sum::<f64>();
  let sum_xy = values.iter().map(|(x, y)| x * y).sum::<f64>();
  let sum_x2 = values.iter().map(|(x, _)| x * x).sum::<f64>();
  let denominator = count * sum_x2 - sum_x * sum_x;
  if denominator.abs() <= f64::EPSILON {
    return None;
  }
  let slope = (count * sum_xy - sum_x * sum_y) / denominator;
  Some((slope, (sum_y - slope * sum_x) / count))
}

fn lower_stock_overlays(
  items: &mut Vec<PageItem>,
  chart: &ClusteredColumnChart<'_>,
  plot: PlotRect,
  scale: crate::render::chart::LinearAxisScale,
  style: &ClusteredColumnStyle,
  category_count: usize,
) {
  for category_index in 0..category_count {
    let values = chart
      .series
      .iter()
      .filter(|series| matches!(series.kind, ChartSeriesKind::Line | ChartSeriesKind::Stock))
      .filter_map(|series| series.values.get(category_index).copied().flatten())
      .collect::<Vec<_>>();
    if values.len() < 2 {
      continue;
    }
    let display_index = category_display_index(chart, category_index, category_count);
    let x = plot.left + (display_index as f32 + 0.5) / category_count.max(1) as f32 * plot.width;
    if chart.has_high_low_lines {
      let minimum = values.iter().copied().fold(f64::INFINITY, f64::min);
      let maximum = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
      items.push(PageItem::Line(LineItem {
        x1_pt: x,
        y1_pt: value_y(minimum, scale, plot.top, plot.height),
        x2_pt: x,
        y2_pt: value_y(maximum, scale, plot.top, plot.height),
        width_pt: 0.75,
        color: style.gridline_color,
        kind: LineItemKind::Stroke,
      }));
    }
    if chart.has_up_down_bars {
      let first = values[0];
      let last = values[values.len() - 1];
      let top = value_y(first.max(last), scale, plot.top, plot.height);
      let bottom = value_y(first.min(last), scale, plot.top, plot.height);
      items.push(PageItem::Rect(RectItem {
        x_pt: x - plot.width / category_count.max(1) as f32 * 0.16,
        y_pt: top,
        width_pt: plot.width / category_count.max(1) as f32 * 0.32,
        height_pt: (bottom - top).abs(),
        fill_color: Some(if last >= first {
          RgbColor {
            r: 255,
            g: 255,
            b: 255,
          }
        } else {
          RgbColor { r: 0, g: 0, b: 0 }
        }),
        fill_opacity: 1.0,
        stroke: None,
        stroke_opacity: 1.0,
      }));
    }
  }
}

fn lower_column_series(
  items: &mut Vec<PageItem>,
  context: &SeriesGeometryContext<'_, '_>,
  style: &ClusteredColumnStyle,
  series_index: usize,
  color: RgbColor,
) {
  let chart = context.chart;
  let series = &chart.series[series_index];
  let peer_count = chart
    .series
    .iter()
    .filter(|peer| peer.kind == ChartSeriesKind::Column && peer.grouping == series.grouping)
    .count()
    .max(1);
  let peer_index = chart.series[..series_index]
    .iter()
    .filter(|peer| peer.kind == ChartSeriesKind::Column && peer.grouping == series.grouping)
    .count();
  for (category_index, value) in series.values.iter().enumerate() {
    let Some(value) = value else {
      continue;
    };
    let (start_value, end_value) =
      stacked_value_bounds(chart, series_index, category_index, *value);
    let slot_series_count = if series.grouping == ChartSeriesGrouping::Clustered {
      peer_count
    } else {
      1
    };
    let slot_series_index = if series.grouping == ChartSeriesGrouping::Clustered {
      peer_index
    } else {
      0
    };
    let Some(slot) = clustered_column_slot(
      category_display_index(chart, category_index, context.category_count),
      slot_series_index,
      context.category_count,
      slot_series_count,
      chart.gap_width_percent,
      chart.overlap_percent,
    ) else {
      continue;
    };
    let start_y = if series.grouping == ChartSeriesGrouping::Clustered {
      context.zero_y
    } else {
      value_y(
        start_value,
        context.scale,
        context.plot.top,
        context.plot.height,
      )
    };
    let end_y = value_y(
      end_value,
      context.scale,
      context.plot.top,
      context.plot.height,
    );
    let x = context.plot.left + (slot.center - slot.width / 2.0) as f32 * context.plot.width;
    let width = slot.width as f32 * context.plot.width;
    let point_color = chart_point_color(style, series_index, category_index).unwrap_or(color);
    if series.is_3d {
      let depth = (width * 0.16).clamp(1.0, 5.0);
      items.push(PageItem::Rect(RectItem {
        x_pt: x + depth,
        y_pt: end_y.min(start_y) - depth,
        width_pt: width,
        height_pt: (start_y - end_y).abs(),
        fill_color: Some(point_color),
        fill_opacity: 0.62,
        stroke: None,
        stroke_opacity: 1.0,
      }));
    }
    items.push(PageItem::Rect(RectItem {
      x_pt: x,
      y_pt: end_y.min(start_y),
      width_pt: width,
      height_pt: (start_y - end_y).abs(),
      fill_color: Some(point_color),
      fill_opacity: 1.0,
      stroke: None,
      stroke_opacity: 1.0,
    }));
  }
}

fn lower_bar_series(
  items: &mut Vec<PageItem>,
  context: &SeriesGeometryContext<'_, '_>,
  style: &ClusteredColumnStyle,
  series_index: usize,
  color: RgbColor,
) {
  let chart = context.chart;
  let series = &chart.series[series_index];
  let peer_count = chart
    .series
    .iter()
    .filter(|peer| peer.kind == ChartSeriesKind::Bar && peer.grouping == series.grouping)
    .count()
    .max(1);
  let peer_index = chart.series[..series_index]
    .iter()
    .filter(|peer| peer.kind == ChartSeriesKind::Bar && peer.grouping == series.grouping)
    .count();
  let zero_x = value_x(
    0.0_f64.clamp(context.scale.minimum, context.scale.maximum),
    context.scale,
    context.plot,
  );
  for (category_index, value) in series.values.iter().enumerate() {
    let Some(value) = value else {
      continue;
    };
    let (start_value, end_value) =
      stacked_value_bounds(chart, series_index, category_index, *value);
    let slot_series_count = if series.grouping == ChartSeriesGrouping::Clustered {
      peer_count
    } else {
      1
    };
    let slot_series_index = if series.grouping == ChartSeriesGrouping::Clustered {
      peer_index
    } else {
      0
    };
    let Some(slot) = clustered_column_slot(
      category_display_index(chart, category_index, context.category_count),
      slot_series_index,
      context.category_count,
      slot_series_count,
      chart.gap_width_percent,
      chart.overlap_percent,
    ) else {
      continue;
    };
    let start_x = if series.grouping == ChartSeriesGrouping::Clustered {
      zero_x
    } else {
      value_x(start_value, context.scale, context.plot)
    };
    let end_x = value_x(end_value, context.scale, context.plot);
    let point_color = chart_point_color(style, series_index, category_index).unwrap_or(color);
    items.push(PageItem::Rect(RectItem {
      x_pt: start_x.min(end_x),
      y_pt: context.plot.top + (slot.center - slot.width / 2.0) as f32 * context.plot.height,
      width_pt: (end_x - start_x).abs(),
      height_pt: slot.width as f32 * context.plot.height,
      fill_color: Some(point_color),
      fill_opacity: 1.0,
      stroke: None,
      stroke_opacity: 1.0,
    }));
  }
}

fn lower_line_series(
  items: &mut Vec<PageItem>,
  context: &SeriesGeometryContext<'_, '_>,
  series_index: usize,
  color: RgbColor,
  fill_to_axis: bool,
  style: &ClusteredColumnStyle,
) {
  let chart = context.chart;
  let plot = context.plot;
  let scale = context.scale;
  let category_count = context.category_count;
  let series = &chart.series[series_index];
  let mut previous = None;
  if fill_to_axis {
    let mut upper = Vec::new();
    let mut lower = Vec::new();
    for (index, value) in series.values.iter().enumerate() {
      let Some(value) = value else {
        continue;
      };
      let (stack_start, stack_end) = stacked_value_bounds(chart, series_index, index, *value);
      let stacked = matches!(
        series.grouping,
        ChartSeriesGrouping::Stacked | ChartSeriesGrouping::PercentStacked
      );
      let display_index = category_display_index(chart, index, category_count);
      let x = plot.left + (display_index as f32 + 0.5) / category_count.max(1) as f32 * plot.width;
      upper.push(common_point(
        x,
        value_y(
          if stacked { stack_end } else { *value },
          scale,
          plot.top,
          plot.height,
        ),
      ));
      lower.push(common_point(
        x,
        value_y(
          if stacked {
            stack_start
          } else {
            0.0_f64.clamp(scale.minimum, scale.maximum)
          },
          scale,
          plot.top,
          plot.height,
        ),
      ));
    }
    if upper.len() >= 2 {
      upper.extend(lower.into_iter().rev());
      items.push(PageItem::Path(crate::common::PathItem {
        bounds: common_rect(plot.left, plot.top, plot.width, plot.height),
        points: upper,
        commands: Vec::new(),
        closed: true,
        fill: crate::common::Fill::Solid(common_rgb(color, 0.52)),
        stroke: None,
      }));
    }
  }
  for (index, value) in series.values.iter().enumerate() {
    let Some(value) = value else {
      previous = None;
      continue;
    };
    let (_, stack_end) = stacked_value_bounds(chart, series_index, index, *value);
    let point_value = if matches!(
      series.grouping,
      ChartSeriesGrouping::Stacked | ChartSeriesGrouping::PercentStacked
    ) {
      stack_end
    } else {
      *value
    };
    let display_index = category_display_index(chart, index, category_count);
    let point = (
      plot.left + (display_index as f32 + 0.5) / category_count.max(1) as f32 * plot.width,
      value_y(point_value, scale, plot.top, plot.height),
    );
    if let Some((previous_x, previous_y)) = previous {
      lower_chart_line_segment(
        items,
        (previous_x, previous_y),
        point,
        color,
        1.5,
        series.smooth,
      );
    }
    if let Some(marker) = chart_marker_size(series, if series.is_3d { 4.0 } else { 3.0 }) {
      lower_chart_marker(
        items,
        point.0,
        point.1,
        marker,
        chart_point_color(style, series_index, index).unwrap_or(color),
        series.marker,
      );
    }
    previous = Some(point);
  }
}

fn lower_scatter_series(
  items: &mut Vec<PageItem>,
  series: &crate::render::chart::ClusteredColumnSeries<'_>,
  series_index: usize,
  color: RgbColor,
  geometry: ScatterGeometry,
  bubbles: bool,
  style: &ClusteredColumnStyle,
) {
  let (x_minimum, x_maximum) = geometry.x_extent.unwrap_or((0.0, 1.0));
  let mut previous = None;
  for (index, value) in series.values.iter().enumerate() {
    let Some(value) = value else {
      previous = None;
      continue;
    };
    let x_value = series
      .x_values
      .get(index)
      .copied()
      .flatten()
      .unwrap_or(index as f64 + 1.0);
    let x = if (x_maximum - x_minimum).abs() <= f64::EPSILON {
      geometry.plot.left + geometry.plot.width * 0.5
    } else {
      geometry.plot.left
        + ((x_value - x_minimum) / (x_maximum - x_minimum)) as f32 * geometry.plot.width
    };
    let y = value_y(
      *value,
      geometry.scale,
      geometry.plot.top,
      geometry.plot.height,
    );
    if !bubbles && let Some((previous_x, previous_y)) = previous {
      lower_chart_line_segment(
        items,
        (previous_x, previous_y),
        (x, y),
        color,
        1.25,
        series.smooth,
      );
    }
    let size = if bubbles {
      let bubble = series
        .bubble_sizes
        .get(index)
        .copied()
        .flatten()
        .unwrap_or(1.0)
        .abs();
      ((bubble / geometry.bubble_maximum.max(f64::EPSILON)).sqrt() as f32
        * geometry.plot.width.min(geometry.plot.height)
        * 0.16)
        .clamp(
          0.5,
          (geometry.plot.width.min(geometry.plot.height) * 0.24).max(0.5),
        )
    } else {
      let Some(size) = chart_marker_size(series, 4.0) else {
        previous = Some((x, y));
        continue;
      };
      size
    };
    lower_chart_marker(
      items,
      x,
      y,
      size,
      chart_point_color(style, series_index, index).unwrap_or(color),
      series.marker,
    );
    previous = Some((x, y));
  }
}

fn lower_chart_line_segment(
  items: &mut Vec<PageItem>,
  start: (f32, f32),
  end: (f32, f32),
  color: RgbColor,
  width_pt: f32,
  smooth: bool,
) {
  let steps = if smooth { 8 } else { 1 };
  let mut previous = start;
  for step in 1..=steps {
    let t = step as f32 / steps as f32;
    let eased = if smooth { t * t * (3.0 - 2.0 * t) } else { t };
    let point = (
      start.0 + (end.0 - start.0) * t,
      start.1 + (end.1 - start.1) * eased,
    );
    items.push(PageItem::Line(LineItem {
      x1_pt: previous.0,
      y1_pt: previous.1,
      x2_pt: point.0,
      y2_pt: point.1,
      width_pt,
      color,
      kind: LineItemKind::Stroke,
    }));
    previous = point;
  }
}

fn chart_marker_size(
  series: &crate::render::chart::ClusteredColumnSeries<'_>,
  default_size: f32,
) -> Option<f32> {
  let marker = series.marker?;
  if marker
    .symbol
    .as_ref()
    .is_some_and(|symbol| symbol.val == c::MarkerStyleValues::None)
  {
    return None;
  }
  Some(
    marker
      .size
      .as_ref()
      .and_then(|size| size.val)
      .map_or(default_size, f32::from),
  )
}

fn lower_chart_marker(
  items: &mut Vec<PageItem>,
  x: f32,
  y: f32,
  size: f32,
  color: RgbColor,
  marker: Option<&c::Marker>,
) {
  let symbol = marker
    .and_then(|marker| marker.symbol.as_ref())
    .map_or(c::MarkerStyleValues::Circle, |symbol| symbol.val);
  match symbol {
    c::MarkerStyleValues::None => {}
    c::MarkerStyleValues::Plus | c::MarkerStyleValues::X => {
      let diagonal = symbol == c::MarkerStyleValues::X;
      for direction in [-1.0_f32, 1.0] {
        let (x1, y1, x2, y2) = if diagonal {
          (
            x - size * 0.5,
            y + direction * size * 0.5,
            x + size * 0.5,
            y - direction * size * 0.5,
          )
        } else if direction < 0.0 {
          (x - size * 0.5, y, x + size * 0.5, y)
        } else {
          (x, y - size * 0.5, x, y + size * 0.5)
        };
        items.push(PageItem::Line(LineItem {
          x1_pt: x1,
          y1_pt: y1,
          x2_pt: x2,
          y2_pt: y2,
          width_pt: 1.0,
          color,
          kind: LineItemKind::Stroke,
        }));
      }
    }
    c::MarkerStyleValues::Dash => items.push(PageItem::Line(LineItem {
      x1_pt: x - size * 0.5,
      y1_pt: y,
      x2_pt: x + size * 0.5,
      y2_pt: y,
      width_pt: 1.5,
      color,
      kind: LineItemKind::Stroke,
    })),
    c::MarkerStyleValues::Square | c::MarkerStyleValues::Auto | c::MarkerStyleValues::Picture => {
      items.push(PageItem::Rect(RectItem {
        x_pt: x - size * 0.5,
        y_pt: y - size * 0.5,
        width_pt: size,
        height_pt: size,
        fill_color: Some(color),
        fill_opacity: 1.0,
        stroke: None,
        stroke_opacity: 1.0,
      }))
    }
    c::MarkerStyleValues::Diamond => {
      push_marker_polygon(
        items,
        vec![
          common_point(x, y - size * 0.55),
          common_point(x + size * 0.55, y),
          common_point(x, y + size * 0.55),
          common_point(x - size * 0.55, y),
        ],
        x,
        y,
        size,
        color,
      );
    }
    c::MarkerStyleValues::Triangle => {
      push_marker_polygon(
        items,
        vec![
          common_point(x, y - size * 0.6),
          common_point(x + size * 0.55, y + size * 0.45),
          common_point(x - size * 0.55, y + size * 0.45),
        ],
        x,
        y,
        size,
        color,
      );
    }
    c::MarkerStyleValues::Circle | c::MarkerStyleValues::Dot | c::MarkerStyleValues::Star => {
      let count = if symbol == c::MarkerStyleValues::Star {
        10
      } else {
        20
      };
      let points = (0..count)
        .map(|index| {
          let angle = std::f32::consts::TAU * index as f32 / count as f32;
          let radius = if symbol == c::MarkerStyleValues::Star && index % 2 == 1 {
            size * 0.23
          } else {
            size * 0.5
          };
          common_point(x + angle.sin() * radius, y - angle.cos() * radius)
        })
        .collect();
      push_marker_polygon(items, points, x, y, size, color);
    }
  }
}

fn push_marker_polygon(
  items: &mut Vec<PageItem>,
  points: Vec<crate::common::Point>,
  x: f32,
  y: f32,
  size: f32,
  color: RgbColor,
) {
  items.push(PageItem::Path(crate::common::PathItem {
    bounds: common_rect(x - size * 0.6, y - size * 0.6, size * 1.2, size * 1.2),
    points,
    commands: Vec::new(),
    closed: true,
    fill: crate::common::Fill::Solid(common_rgb(color, 1.0)),
    stroke: None,
  }));
}

fn chart_point_color(
  style: &ClusteredColumnStyle,
  series_index: usize,
  point_index: usize,
) -> Option<RgbColor> {
  style
    .series_point_colors
    .get(series_index)
    .and_then(|colors| colors.get(point_index))
    .copied()
    .flatten()
}

fn lower_radar_series(
  items: &mut Vec<PageItem>,
  context: &SeriesGeometryContext<'_, '_>,
  series: &crate::render::chart::ClusteredColumnSeries<'_>,
  series_index: usize,
  color: RgbColor,
  style: &ClusteredColumnStyle,
) {
  let plot = context.plot;
  let scale = context.scale;
  let category_reversed = context.chart.category_axis_reversed;
  let count = series.values.len();
  if count < 2 {
    return;
  }
  let center = (plot.left + plot.width * 0.5, plot.top + plot.height * 0.5);
  let radius = plot.width.min(plot.height) * 0.46;
  let geometry_start = items.len();
  let mut polygon_points = Vec::new();
  let mut first = None;
  let mut previous = None;
  for (index, value) in series.values.iter().enumerate() {
    let Some(value) = value else {
      previous = None;
      continue;
    };
    let ratio = ((*value - scale.minimum) / (scale.maximum - scale.minimum)).clamp(0.0, 1.0);
    let display_index = if category_reversed {
      count - 1 - index
    } else {
      index
    };
    let angle = std::f32::consts::TAU * display_index as f32 / count as f32;
    let point = (
      center.0 + angle.sin() * radius * ratio as f32,
      center.1 - angle.cos() * radius * ratio as f32,
    );
    polygon_points.push(common_point(point.0, point.1));
    if let Some((previous_x, previous_y)) = previous {
      items.push(PageItem::Line(LineItem {
        x1_pt: previous_x,
        y1_pt: previous_y,
        x2_pt: point.0,
        y2_pt: point.1,
        width_pt: 1.25,
        color,
        kind: LineItemKind::Stroke,
      }));
    } else {
      first = Some(point);
    }
    previous = Some(point);
    if let Some(size) = chart_marker_size(series, 3.5) {
      lower_chart_marker(
        items,
        point.0,
        point.1,
        size,
        chart_point_color(style, series_index, index).unwrap_or(color),
        series.marker,
      );
    }
  }
  if let (Some(first), Some(last)) = (first, previous) {
    items.push(PageItem::Line(LineItem {
      x1_pt: last.0,
      y1_pt: last.1,
      x2_pt: first.0,
      y2_pt: first.1,
      width_pt: 1.25,
      color,
      kind: LineItemKind::Stroke,
    }));
  }
  if series.filled_area && polygon_points.len() >= 3 {
    items.insert(
      geometry_start,
      PageItem::Path(crate::common::PathItem {
        bounds: common_rect(plot.left, plot.top, plot.width, plot.height),
        points: polygon_points,
        commands: Vec::new(),
        closed: true,
        fill: crate::common::Fill::Solid(common_rgb(color, 0.42)),
        stroke: None,
      }),
    );
  }
}

fn lower_radar_axes(
  items: &mut Vec<PageItem>,
  chart: &ClusteredColumnChart<'_>,
  plot: PlotRect,
  scale: crate::render::chart::LinearAxisScale,
  style: &ClusteredColumnStyle,
  metrics: &mut TextMetrics,
) {
  let count = chart.categories.len().max(
    chart
      .series
      .iter()
      .map(|series| series.values.len())
      .max()
      .unwrap_or(0),
  );
  if count < 2 {
    return;
  }
  let center = (plot.left + plot.width * 0.5, plot.top + plot.height * 0.5);
  let radius = plot.width.min(plot.height) * 0.46;
  let rings = ((scale.maximum - scale.minimum) / scale.major_unit)
    .round()
    .clamp(1.0, 10.0) as usize;
  for ring in 1..=rings {
    let ring_radius = radius * ring as f32 / rings as f32;
    let mut previous = None;
    let mut first = None;
    for index in 0..count {
      let angle = std::f32::consts::TAU * index as f32 / count as f32;
      let point = (
        center.0 + angle.sin() * ring_radius,
        center.1 - angle.cos() * ring_radius,
      );
      if let Some((x, y)) = previous {
        items.push(PageItem::Line(LineItem {
          x1_pt: x,
          y1_pt: y,
          x2_pt: point.0,
          y2_pt: point.1,
          width_pt: 0.75,
          color: style.gridline_color,
          kind: LineItemKind::Stroke,
        }));
      } else {
        first = Some(point);
      }
      previous = Some(point);
    }
    if let (Some(first), Some(last)) = (first, previous) {
      items.push(PageItem::Line(LineItem {
        x1_pt: last.0,
        y1_pt: last.1,
        x2_pt: first.0,
        y2_pt: first.1,
        width_pt: 0.75,
        color: style.gridline_color,
        kind: LineItemKind::Stroke,
      }));
    }
  }
  for index in 0..count {
    let display_index = category_display_index(chart, index, count);
    let angle = std::f32::consts::TAU * display_index as f32 / count as f32;
    let outer = (
      center.0 + angle.sin() * radius,
      center.1 - angle.cos() * radius,
    );
    items.push(PageItem::Line(LineItem {
      x1_pt: center.0,
      y1_pt: center.1,
      x2_pt: outer.0,
      y2_pt: outer.1,
      width_pt: 0.75,
      color: style.gridline_color,
      kind: LineItemKind::Stroke,
    }));
    if let Some(category) = chart.categories.get(index) {
      let width = metrics.measure_text(category, &style.label);
      let label_radius = radius + style.label.font_size_pt * 0.85;
      push_text(
        items,
        center.0 + angle.sin() * label_radius - width * 0.5,
        center.1 - angle.cos() * label_radius - line_height(&style.label) * 0.5,
        category.clone(),
        style.label.clone(),
      );
    }
  }
}

fn lower_horizontal_bar_axes(
  items: &mut Vec<PageItem>,
  chart: &ClusteredColumnChart<'_>,
  plot: PlotRect,
  tick_labels: &[(f64, String)],
  scale: crate::render::chart::LinearAxisScale,
  style: &ClusteredColumnStyle,
  metrics: &mut TextMetrics,
) {
  for (value, label) in tick_labels {
    let x = value_x(*value, scale, plot);
    items.push(PageItem::Line(LineItem {
      x1_pt: x,
      y1_pt: plot.top,
      x2_pt: x,
      y2_pt: plot.top + plot.height,
      width_pt: 0.75,
      color: style.gridline_color,
      kind: LineItemKind::Stroke,
    }));
    let width = metrics.measure_text(label, &style.label);
    push_text(
      items,
      x - width * 0.5,
      plot.top + plot.height + style.label.font_size_pt * 0.25,
      label.clone(),
      style.label.clone(),
    );
  }
  let count = chart.categories.len().max(1);
  for (index, category) in chart.categories.iter().enumerate() {
    let width = metrics.measure_text(category, &style.label);
    let display_index = category_display_index(chart, index, count);
    let y = plot.top + (display_index as f32 + 0.5) / count as f32 * plot.height
      - line_height(&style.label) * 0.5;
    push_text(
      items,
      plot.left - width - style.label.font_size_pt * 0.45,
      y,
      category.clone(),
      style.label.clone(),
    );
  }
}

fn lower_scatter_x_axis(
  items: &mut Vec<PageItem>,
  chart: &ClusteredColumnChart<'_>,
  plot: PlotRect,
  style: &ClusteredColumnStyle,
  metrics: &mut TextMetrics,
) {
  let values = chart
    .series
    .iter()
    .flat_map(|series| series.x_values.iter().flatten().copied())
    .collect::<Vec<_>>();
  let Some(scale) = linear_axis_scale(values, None, 10) else {
    return;
  };
  for (value, label) in scale_tick_labels(
    scale.minimum,
    scale.maximum,
    scale.major_unit,
    None,
    scale.logarithmic_base,
  ) {
    let x = value_x(value, scale, plot);
    items.push(PageItem::Line(LineItem {
      x1_pt: x,
      y1_pt: plot.top,
      x2_pt: x,
      y2_pt: plot.top + plot.height,
      width_pt: 0.75,
      color: style.gridline_color,
      kind: LineItemKind::Stroke,
    }));
    let width = metrics.measure_text(&label, &style.label);
    push_text(
      items,
      x - width * 0.5,
      plot.top + plot.height + style.label.font_size_pt * 0.25,
      label,
      style.label.clone(),
    );
  }
}

fn stacked_value_bounds(
  chart: &ClusteredColumnChart<'_>,
  series_index: usize,
  category_index: usize,
  value: f64,
) -> (f64, f64) {
  let series = &chart.series[series_index];
  if !matches!(
    series.grouping,
    ChartSeriesGrouping::Stacked | ChartSeriesGrouping::PercentStacked
  ) {
    return (0.0, value);
  }
  let total = if series.grouping == ChartSeriesGrouping::PercentStacked {
    chart
      .series
      .iter()
      .filter(|peer| peer.kind == series.kind && peer.grouping == series.grouping)
      .filter_map(|peer| peer.values.get(category_index).copied().flatten())
      .map(f64::abs)
      .sum::<f64>()
      .max(f64::EPSILON)
  } else {
    1.0
  };
  let normalized = value / total;
  let mut start = 0.0;
  for peer in chart.series[..series_index]
    .iter()
    .filter(|peer| peer.kind == series.kind && peer.grouping == series.grouping)
  {
    if let Some(previous) = peer.values.get(category_index).copied().flatten()
      && previous.signum() == value.signum()
    {
      start += previous / total;
    }
  }
  (start, start + normalized)
}

fn value_x(value: f64, scale: crate::render::chart::LinearAxisScale, plot: PlotRect) -> f32 {
  let ratio = axis_value_ratio(value, scale);
  plot.left
    + if scale.reversed {
      1.0 - ratio as f32
    } else {
      ratio as f32
    } * plot.width
}

fn axis_value_ratio(value: f64, scale: crate::render::chart::LinearAxisScale) -> f64 {
  if let Some(base) = scale
    .logarithmic_base
    .filter(|base| *base > 1.0 && value > 0.0 && scale.minimum > 0.0)
  {
    let minimum = scale.minimum.log(base);
    let maximum = scale.maximum.log(base);
    if (maximum - minimum).abs() <= f64::EPSILON {
      0.5
    } else {
      ((value.log(base) - minimum) / (maximum - minimum)).clamp(0.0, 1.0)
    }
  } else {
    ((value - scale.minimum) / (scale.maximum - scale.minimum)).clamp(0.0, 1.0)
  }
}

#[derive(Clone, Copy)]
struct ChartPointAnchor {
  x: f32,
  y: f32,
  base_x: f32,
  base_y: f32,
}

fn data_label_anchor(
  chart: &ClusteredColumnChart<'_>,
  series_index: usize,
  point_index: usize,
  plot: PlotRect,
  scale: crate::render::chart::LinearAxisScale,
  zero_y: f32,
  category_count: usize,
) -> Option<ChartPointAnchor> {
  let series = chart.series.get(series_index)?;
  let value = series.values.get(point_index).copied().flatten()?;
  match series.kind {
    ChartSeriesKind::Column => {
      let peers = chart
        .series
        .iter()
        .filter(|peer| peer.kind == series.kind && peer.grouping == series.grouping)
        .count()
        .max(1);
      let peer_index = chart.series[..series_index]
        .iter()
        .filter(|peer| peer.kind == series.kind && peer.grouping == series.grouping)
        .count();
      let clustered = series.grouping == ChartSeriesGrouping::Clustered;
      let slot = clustered_column_slot(
        category_display_index(chart, point_index, category_count),
        if clustered { peer_index } else { 0 },
        category_count,
        if clustered { peers } else { 1 },
        chart.gap_width_percent,
        chart.overlap_percent,
      )?;
      let (start, end) = stacked_value_bounds(chart, series_index, point_index, value);
      let x = plot.left + slot.center as f32 * plot.width;
      Some(ChartPointAnchor {
        x,
        y: value_y(end, scale, plot.top, plot.height),
        base_x: x,
        base_y: if clustered {
          zero_y
        } else {
          value_y(start, scale, plot.top, plot.height)
        },
      })
    }
    ChartSeriesKind::Bar => {
      let peers = chart
        .series
        .iter()
        .filter(|peer| peer.kind == series.kind && peer.grouping == series.grouping)
        .count()
        .max(1);
      let peer_index = chart.series[..series_index]
        .iter()
        .filter(|peer| peer.kind == series.kind && peer.grouping == series.grouping)
        .count();
      let clustered = series.grouping == ChartSeriesGrouping::Clustered;
      let slot = clustered_column_slot(
        category_display_index(chart, point_index, category_count),
        if clustered { peer_index } else { 0 },
        category_count,
        if clustered { peers } else { 1 },
        chart.gap_width_percent,
        chart.overlap_percent,
      )?;
      let (start, end) = stacked_value_bounds(chart, series_index, point_index, value);
      let y = plot.top + slot.center as f32 * plot.height;
      Some(ChartPointAnchor {
        x: value_x(end, scale, plot),
        y,
        base_x: if clustered {
          value_x(0.0_f64.clamp(scale.minimum, scale.maximum), scale, plot)
        } else {
          value_x(start, scale, plot)
        },
        base_y: y,
      })
    }
    ChartSeriesKind::Scatter | ChartSeriesKind::Bubble => {
      let extent = chart
        .series
        .iter()
        .flat_map(|series| series.x_values.iter().flatten().copied())
        .fold(None, |extent, value| match extent {
          Some((minimum, maximum)) => Some((f64::min(minimum, value), f64::max(maximum, value))),
          None => Some((value, value)),
        })
        .unwrap_or((0.0, 1.0));
      let x_value = series
        .x_values
        .get(point_index)
        .copied()
        .flatten()
        .unwrap_or(point_index as f64 + 1.0);
      let x = if (extent.1 - extent.0).abs() <= f64::EPSILON {
        plot.left + plot.width * 0.5
      } else {
        plot.left + ((x_value - extent.0) / (extent.1 - extent.0)) as f32 * plot.width
      };
      let y = value_y(value, scale, plot.top, plot.height);
      Some(ChartPointAnchor {
        x,
        y,
        base_x: x,
        base_y: y,
      })
    }
    ChartSeriesKind::Radar => {
      let center = (plot.left + plot.width * 0.5, plot.top + plot.height * 0.5);
      let ratio = ((value - scale.minimum) / (scale.maximum - scale.minimum)).clamp(0.0, 1.0);
      let display_index = category_display_index(chart, point_index, series.values.len());
      let angle = std::f32::consts::TAU * display_index as f32 / series.values.len().max(1) as f32;
      let radius = plot.width.min(plot.height) * 0.46 * ratio as f32;
      Some(ChartPointAnchor {
        x: center.0 + angle.sin() * radius,
        y: center.1 - angle.cos() * radius,
        base_x: center.0,
        base_y: center.1,
      })
    }
    ChartSeriesKind::Line
    | ChartSeriesKind::Area
    | ChartSeriesKind::Stock
    | ChartSeriesKind::Surface => {
      let (start, end) = stacked_value_bounds(chart, series_index, point_index, value);
      let display_index = category_display_index(chart, point_index, category_count);
      let x = plot.left + (display_index as f32 + 0.5) / category_count.max(1) as f32 * plot.width;
      Some(ChartPointAnchor {
        x,
        y: value_y(end, scale, plot.top, plot.height),
        base_x: x,
        base_y: if matches!(
          series.grouping,
          ChartSeriesGrouping::Stacked | ChartSeriesGrouping::PercentStacked
        ) {
          value_y(start, scale, plot.top, plot.height)
        } else {
          zero_y
        },
      })
    }
  }
}

fn category_display_index(
  chart: &ClusteredColumnChart<'_>,
  source_index: usize,
  category_count: usize,
) -> usize {
  if chart.category_axis_reversed && source_index < category_count {
    category_count - 1 - source_index
  } else {
    source_index
  }
}

fn value_axis_is_visible(axis: &c::ValueAxis) -> bool {
  axis
    .delete
    .as_ref()
    .is_none_or(|delete| delete.val.is_some_and(|value| !value.as_bool()))
}

fn category_axis_is_visible(axis: &c::CategoryAxis) -> bool {
  axis
    .delete
    .as_ref()
    .is_none_or(|delete| delete.val.is_some_and(|value| !value.as_bool()))
}

fn date_axis_is_visible(axis: &c::DateAxis) -> bool {
  axis
    .delete
    .as_ref()
    .is_none_or(|delete| delete.val.is_some_and(|value| !value.as_bool()))
}

fn value_axis_has_major_ticks(axis: &c::ValueAxis) -> bool {
  axis
    .major_tick_mark
    .as_ref()
    .is_none_or(|tick| tick.val != Some(c::TickMarkValues::None))
}

fn category_axis_has_major_ticks(axis: &c::CategoryAxis) -> bool {
  axis
    .major_tick_mark
    .as_ref()
    .is_none_or(|tick| tick.val != Some(c::TickMarkValues::None))
}

fn date_axis_has_major_ticks(axis: &c::DateAxis) -> bool {
  axis
    .major_tick_mark
    .as_ref()
    .is_none_or(|tick| tick.val != Some(c::TickMarkValues::None))
}

fn category_axis_text_rotation_is_supported(
  properties: Option<&c::TextProperties>,
  category_count: usize,
) -> bool {
  properties
    .and_then(|properties| properties.body_properties.rotation)
    // DrawingML chart tick-label rotation is limited to -90..90 degrees.
    // Office still lays out short axes carrying an out-of-range legacy value,
    // but suppresses dense label sets whose invalid geometry cannot fit.
    .is_none_or(|rotation| (-5_400_000..=5_400_000).contains(&rotation) || category_count <= 6)
}

fn wrap_chart_label(
  text: &str,
  maximum_width: f32,
  style: &TextStyle,
  metrics: &mut TextMetrics,
) -> Vec<String> {
  let words: Vec<&str> = text.split_whitespace().collect();
  if words.len() <= 1 || maximum_width <= 0.0 {
    return vec![text.to_string()];
  }
  let mut lines = Vec::new();
  let mut current = String::new();
  for word in words {
    let candidate = if current.is_empty() {
      word.to_string()
    } else {
      format!("{current} {word}")
    };
    if current.is_empty() || metrics.measure_text(&candidate, style) <= maximum_width {
      current = candidate;
    } else {
      lines.push(std::mem::take(&mut current));
      current.push_str(word);
    }
  }
  if !current.is_empty() {
    lines.push(current);
  }
  lines
}

fn apply_manual_layout(
  frame: ChartFrame,
  automatic: PlotRect,
  layout: crate::render::chart::ChartManualLayout,
) -> PlotRect {
  let left = layout
    .x
    .map_or(automatic.left, |value| frame.x_pt + value * frame.width_pt);
  let top = layout
    .y
    .map_or(automatic.top, |value| frame.y_pt + value * frame.height_pt);
  let width = layout
    .width
    .map_or(automatic.width, |value| value * frame.width_pt)
    .max(0.0);
  let height = layout
    .height
    .map_or(automatic.height, |value| value * frame.height_pt)
    .max(0.0);
  PlotRect {
    left,
    top,
    width: width.min(frame.x_pt + frame.width_pt - left),
    height: height.min(frame.y_pt + frame.height_pt - top),
  }
}

fn lower_axis_titles(
  items: &mut Vec<PageItem>,
  frame: ChartFrame,
  plot: PlotRect,
  category_band_top: f32,
  chart: &ClusteredColumnChart<'_>,
  style: &ClusteredColumnStyle,
  metrics: &mut TextMetrics,
) {
  if let Some(title) = chart.value_axis_title.as_deref() {
    let width = metrics.measure_text(title, &style.label);
    let x = frame.x_pt + style.label.font_size_pt * 0.2;
    let y = plot.top + (plot.height - width) * 0.5;
    items.push(PageItem::Text(TextItem {
      x_pt: x,
      y_pt: y,
      line_height_pt: line_height(&style.label),
      text: title.to_string(),
      style: style.label.clone(),
      rotation_center_pt: Some((x, y)),
      hyperlink_url: None,
      form_widget_id: None,
      paragraph_bidi: false,
      preserve_text_portion: true,
      pdf_text_segmentation: PdfTextSegmentation::Line,
      source_path: Vec::new(),
    }));
  }
  if let Some(title) = chart.category_axis_title.as_deref() {
    let width = metrics.measure_text(title, &style.label);
    push_text(
      items,
      plot.left + (plot.width - width) * 0.5,
      if chart.data_table.is_some() {
        category_band_top + line_height(&style.label) * 0.05
      } else {
        category_band_top + line_height(&style.label)
      },
      title.to_string(),
      style.label.clone(),
    );
  }
  for (index, title) in chart.additional_axis_titles.iter().enumerate() {
    let width = metrics.measure_text(title, &style.label);
    let x = frame.x_pt + frame.width_pt - style.label.font_size_pt * (0.4 + index as f32 * 1.25);
    let y = plot.top + (plot.height - width) * 0.5;
    items.push(PageItem::Text(TextItem {
      x_pt: x,
      y_pt: y,
      line_height_pt: line_height(&style.label),
      text: title.clone(),
      style: style.label.clone(),
      rotation_center_pt: Some((x, y)),
      hyperlink_url: None,
      form_widget_id: None,
      paragraph_bidi: false,
      preserve_text_portion: true,
      pdf_text_segmentation: PdfTextSegmentation::Line,
      source_path: Vec::new(),
    }));
  }
}

fn lower_data_table(
  items: &mut Vec<PageItem>,
  chart: &ClusteredColumnChart<'_>,
  table: &c::DataTable,
  bounds: PlotRect,
  style: &ClusteredColumnStyle,
  metrics: &mut TextMetrics,
) {
  if bounds.width <= 0.0 || bounds.height <= 0.0 || chart.categories.is_empty() {
    return;
  }
  let row_count = chart.series.len() + 1;
  let column_count = chart.categories.len();
  let row_height = bounds.height / row_count.max(1) as f32;
  let column_width = bounds.width / column_count.max(1) as f32;
  let show_horizontal = table
    .show_horizontal_border
    .as_ref()
    .is_none_or(|value| value.val.is_none_or(|value| value.as_bool()));
  let show_vertical = table
    .show_vertical_border
    .as_ref()
    .is_none_or(|value| value.val.is_none_or(|value| value.as_bool()));
  let show_outline = table
    .show_outline_border
    .as_ref()
    .is_none_or(|value| value.val.is_none_or(|value| value.as_bool()));
  let show_keys = table
    .show_keys
    .as_ref()
    .is_some_and(|value| value.val.is_none_or(|value| value.as_bool()));

  let line = |x1_pt, y1_pt, x2_pt, y2_pt| {
    PageItem::Line(LineItem {
      x1_pt,
      y1_pt,
      x2_pt,
      y2_pt,
      width_pt: 0.75,
      color: style.gridline_color,
      kind: LineItemKind::Stroke,
    })
  };
  if show_outline {
    items.push(line(
      bounds.left,
      bounds.top,
      bounds.left + bounds.width,
      bounds.top,
    ));
    items.push(line(
      bounds.left,
      bounds.top + bounds.height,
      bounds.left + bounds.width,
      bounds.top + bounds.height,
    ));
    items.push(line(
      bounds.left,
      bounds.top,
      bounds.left,
      bounds.top + bounds.height,
    ));
    items.push(line(
      bounds.left + bounds.width,
      bounds.top,
      bounds.left + bounds.width,
      bounds.top + bounds.height,
    ));
  }
  if show_horizontal {
    for row in 1..row_count {
      let y = bounds.top + row as f32 * row_height;
      items.push(line(bounds.left, y, bounds.left + bounds.width, y));
    }
  }
  if show_vertical {
    for column in 1..column_count {
      let x = bounds.left + column as f32 * column_width;
      items.push(line(x, bounds.top, x, bounds.top + bounds.height));
    }
  }

  for (column, category) in chart.categories.iter().enumerate() {
    let width = metrics.measure_text(category, &style.label);
    let display_column = category_display_index(chart, column, column_count);
    push_text(
      items,
      bounds.left + display_column as f32 * column_width + (column_width - width) * 0.5,
      bounds.top + (row_height - line_height(&style.label)) * 0.5,
      category.clone(),
      style.label.clone(),
    );
  }
  for (row, series) in chart.series.iter().enumerate() {
    let y = bounds.top + (row + 1) as f32 * row_height;
    if show_keys && let Some(color) = style.series_colors.get(row).copied() {
      let key_size = style.label.font_size_pt * 0.45;
      items.push(PageItem::Rect(RectItem {
        x_pt: bounds.left - key_size * 1.5,
        y_pt: y + (row_height - key_size) * 0.5,
        width_pt: key_size,
        height_pt: key_size,
        fill_color: Some(color),
        fill_opacity: 1.0,
        stroke: None,
        stroke_opacity: 1.0,
      }));
    }
    let name_width = metrics.measure_text(&series.name, &style.label);
    push_text(
      items,
      bounds.left - name_width - style.label.font_size_pt * 0.45,
      y + (row_height - line_height(&style.label)) * 0.5,
      series.name.clone(),
      style.label.clone(),
    );
    for (column, value) in series.values.iter().enumerate().take(column_count) {
      let Some(value) = value else {
        continue;
      };
      let text = crate::render::chart::format_chart_number(*value, series.number_format_code);
      let width = metrics.measure_text(&text, &style.label);
      let display_column = category_display_index(chart, column, column_count);
      push_text(
        items,
        bounds.left + display_column as f32 * column_width + (column_width - width) * 0.5,
        y + (row_height - line_height(&style.label)) * 0.5,
        text,
        style.label.clone(),
      );
    }
  }
}

fn lower_manual_legend(
  items: &mut Vec<PageItem>,
  frame: ChartFrame,
  layout: crate::render::chart::ChartManualLayout,
  chart: &ClusteredColumnChart<'_>,
  style: &ClusteredColumnStyle,
) {
  let x = frame.x_pt + layout.x.unwrap_or(0.8) * frame.width_pt;
  let mut y = frame.y_pt + layout.y.unwrap_or(0.1) * frame.height_pt;
  let marker_size = style.label.font_size_pt * 0.55;
  let marker_gap = style.label.font_size_pt * 0.26;
  for index in chart.visible_legend_indices.iter().copied() {
    let Some(series) = chart.series.get(index) else {
      continue;
    };
    if let Some(color) = style.series_colors.get(index).copied() {
      items.push(PageItem::Rect(RectItem {
        x_pt: x,
        y_pt: y + (line_height(&style.label) - marker_size) * 0.5,
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
    y += line_height(&style.label) * 1.25;
  }
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
    .visible_legend_indices
    .iter()
    .filter_map(|index| chart.series.get(*index))
    .map(|series| marker_size + marker_gap + metrics.measure_text(&series.name, &style.label))
    .collect();
  let total_width = widths.iter().sum::<f32>()
    + entry_gap * chart.visible_legend_indices.len().saturating_sub(1) as f32;
  // The value-axis label band has already consumed the leading side of the
  // diagram. Center a bottom legend in the remaining horizontal region, as
  // Office does, rather than recentering it over the full graphic frame.
  let available_right = frame.x_pt + frame.width_pt;
  let mut x = available_left + (available_right - available_left - total_width) / 2.0;
  for (index, width) in chart.visible_legend_indices.iter().copied().zip(widths) {
    let Some(series) = chart.series.get(index) else {
      continue;
    };
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
    .visible_legend_indices
    .iter()
    .filter_map(|index| chart.series.get(*index))
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
  let total_height = line_height * chart.visible_legend_indices.len() as f32
    + entry_gap * chart.visible_legend_indices.len().saturating_sub(1) as f32;
  let mut y = if align_top {
    frame.y_pt + frame.height_pt * 0.04
  } else {
    frame.y_pt + (frame.height_pt - total_height) / 2.0
  };
  if style.layout_profile == ChartLayoutProfile::Excel && !align_top {
    y -= frame.height_pt * 0.0148;
  }
  for index in chart.visible_legend_indices.iter().copied() {
    let Some(series) = chart.series.get(index) else {
      continue;
    };
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
  let ratio = axis_value_ratio(value, scale);
  plot_top
    + if scale.reversed {
      ratio as f32
    } else {
      1.0 - ratio as f32
    } * plot_height
}

fn scale_tick_labels(
  minimum: f64,
  maximum: f64,
  unit: f64,
  format_code: Option<&str>,
  logarithmic_base: Option<f64>,
) -> Vec<(f64, String)> {
  if let Some(base) = logarithmic_base.filter(|base| *base > 1.0 && minimum > 0.0 && maximum > 0.0)
  {
    let first = minimum.log(base).ceil() as i32;
    let last = maximum.log(base).floor() as i32;
    return (first..=last)
      .map(|exponent| {
        let value = base.powi(exponent);
        (
          value,
          format_code.map_or_else(
            || crate::render::chart::format_chart_number(value, None),
            |format| crate::render::chart::format_chart_number(value, Some(format)),
          ),
        )
      })
      .collect();
  }
  let count = ((maximum - minimum) / unit).floor().max(0.0) as usize;
  (0..=count)
    .map(|index| {
      let value = minimum + index as f64 * unit;
      (
        value,
        format_code.map_or_else(
          || format_axis_value(value, unit),
          |format| crate::render::chart::format_chart_number(value, Some(format)),
        ),
      )
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
    source_path: Vec::new(),
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
