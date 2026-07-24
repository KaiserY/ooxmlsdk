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
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use crate::render::chart_layout_profiles as profiles;

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
  pub modern_excel_profile: bool,
  pub stroke_scale: f32,
  pub has_explicit_title: bool,
  pub title: TextStyle,
  pub title_fill_color: Option<RgbColor>,
  pub label: TextStyle,
  pub category_label: TextStyle,
  pub value_label: TextStyle,
  pub data_label: TextStyle,
  pub gridline_color: RgbColor,
  pub series_colors: Vec<RgbColor>,
  pub series_point_colors: Vec<Vec<Option<RgbColor>>>,
  pub data_label_fill_colors: Vec<Vec<Option<RgbColor>>>,
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

fn radial_host_defaults(profile: ChartLayoutProfile) -> profiles::RadialHostDefaults {
  match profile {
    ChartLayoutProfile::PowerPoint => profiles::POWERPOINT_RADIAL_DEFAULTS,
    ChartLayoutProfile::Word => profiles::WORD_RADIAL_DEFAULTS,
    ChartLayoutProfile::Excel => profiles::EXCEL_RADIAL_DEFAULTS,
  }
}

#[derive(Clone, Debug)]
pub(crate) struct RadialChartStyle {
  pub layout_profile: ChartLayoutProfile,
  pub title: TextStyle,
  pub label: TextStyle,
  pub data_label: TextStyle,
  pub point_colors: Vec<RgbColor>,
  pub data_label_fill_colors: Vec<Option<RgbColor>>,
  pub chart_area_fill_color: Option<RgbColor>,
  pub plot_area_fill_color: Option<RgbColor>,
  pub chart_area_stroke_color: Option<RgbColor>,
  pub plot_area_stroke_color: Option<RgbColor>,
}

fn excel_derived_single_series_side_title_layout(
  chart: &ClusteredColumnChart<'_>,
  style: &ClusteredColumnStyle,
) -> bool {
  style.layout_profile == ChartLayoutProfile::Excel
    && !style.has_explicit_title
    && matches!(chart.title.as_ref(), Some(ChartTitleText::Explicit(_)))
    && chart.series.len() == 1
    && matches!(
      chart.legend_position,
      Some(ChartLegendPosition::Left | ChartLegendPosition::Right | ChartLegendPosition::TopRight)
    )
    && !chart.legend_overlay
    && chart.plot_layout.is_none()
}

fn excel_explicit_single_series_side_title_layout(
  chart: &ClusteredColumnChart<'_>,
  style: &ClusteredColumnStyle,
) -> bool {
  style.layout_profile == ChartLayoutProfile::Excel
    && style.has_explicit_title
    && matches!(chart.title.as_ref(), Some(ChartTitleText::Explicit(_)))
    && chart.series.len() == 1
    && matches!(
      chart.legend_position,
      Some(ChartLegendPosition::Left | ChartLegendPosition::Right | ChartLegendPosition::TopRight)
    )
    && !chart.legend_overlay
    && chart.plot_layout.is_none()
    && (chart.gap_width_percent - 219.0).abs() < f64::EPSILON
    && (chart.overlap_percent + 27.0).abs() < f64::EPSILON
}

fn excel_explicit_bottom_column_layout(
  chart: &ClusteredColumnChart<'_>,
  style: &ClusteredColumnStyle,
) -> bool {
  style.layout_profile == ChartLayoutProfile::Excel
    && style.has_explicit_title
    && !chart.title_overlay
    && matches!(chart.title.as_ref(), Some(ChartTitleText::Explicit(_)))
    && chart.legend_position == Some(ChartLegendPosition::Bottom)
    && !chart.legend_overlay
    && chart.series.len() == 2
    && chart
      .series
      .iter()
      .all(|series| series.kind == ChartSeriesKind::Column)
    && (chart.gap_width_percent - 219.0).abs() < f64::EPSILON
    && (chart.overlap_percent + 27.0).abs() < f64::EPSILON
    && chart.plot_layout.is_none()
}

fn excel_untitled_bottom_column_layout(
  chart: &ClusteredColumnChart<'_>,
  style: &ClusteredColumnStyle,
) -> bool {
  style.layout_profile == ChartLayoutProfile::Excel
    && chart.legend_position == Some(ChartLegendPosition::Bottom)
    && !chart.legend_overlay
    && chart.title.is_none()
    && !chart.has_automatic_title_marker
    && !chart.has_explicit_categories
    && chart.series.len() == 2
    && chart
      .series
      .iter()
      .all(|series| series.kind == ChartSeriesKind::Column)
}

fn excel_untitled_bottom_line_no_marker_layout(
  chart: &ClusteredColumnChart<'_>,
  style: &ClusteredColumnStyle,
) -> bool {
  style.layout_profile == ChartLayoutProfile::Excel
    && chart.legend_position == Some(ChartLegendPosition::Bottom)
    && !chart.legend_overlay
    && chart.title.is_none()
    && chart.series.len() == 2
    && chart.series.iter().all(|series| {
      series.kind == ChartSeriesKind::Line
        && series.marker.is_some_and(|marker| {
          marker
            .symbol
            .as_ref()
            .is_some_and(|symbol| symbol.val == c::MarkerStyleValues::None)
        })
    })
    && chart.plot_layout.is_none()
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
  let has_layout_title = title_text.is_some() && !chart.title_overlay;
  let has_layout_explicit_title = has_layout_title && style.has_explicit_title;
  let title_line_height = line_height(&style.title);
  let category_label_line_height = line_height(&style.category_label);
  let value_label_line_height = line_height(&style.value_label);
  let label_line_height = category_label_line_height
    .max(value_label_line_height)
    .max(line_height(&style.label));
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
  let has_multicomponent_data_labels = chart.series.iter().any(|series| {
    series
      .data_labels
      .iter()
      .any(|label| label.text_components.len() > 1)
  });
  let has_indexed_scatter_automatic_layout = style.layout_profile == ChartLayoutProfile::Excel
    && scatter_only
    && title_text.is_none()
    && scatter_uses_index_x_values(chart)
    && (has_multicomponent_data_labels || chart.has_automatic_title_marker);
  let has_legacy_indexed_scatter_layout = style.layout_profile == ChartLayoutProfile::Excel
    && scatter_only
    && title_text.is_none()
    && !chart.has_automatic_title_marker
    && scatter_uses_index_x_values(chart)
    && !has_multicomponent_data_labels
    && chart.plot_layout.is_none();
  let has_titled_indexed_scatter_layout = style.layout_profile == ChartLayoutProfile::Excel
    && scatter_only
    && !chart.title_overlay
    && matches!(chart.title.as_ref(), Some(ChartTitleText::Automatic))
    && scatter_uses_index_x_values(chart);
  let has_legacy_single_series_title_layout = style.layout_profile == ChartLayoutProfile::Excel
    && !style.has_explicit_title
    && matches!(chart.title.as_ref(), Some(ChartTitleText::Explicit(_)))
    && chart.series.len() == 1
    && chart.legend_position.is_none();
  let has_modern_single_series_title_layout =
    has_legacy_single_series_title_layout && style.modern_excel_profile;
  let has_derived_single_series_side_title_layout =
    excel_derived_single_series_side_title_layout(chart, style);
  let has_explicit_single_series_side_title_layout =
    excel_explicit_single_series_side_title_layout(chart, style);
  let has_automatic_untitled_layout = chart.has_automatic_title_marker
    || (chart.title_overlay && matches!(chart.title.as_ref(), Some(ChartTitleText::Automatic)))
    || has_indexed_scatter_automatic_layout;
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
      .map(|category| wrap_chart_label(category, slot_width, &style.category_label, &mut metrics))
      .collect()
  } else {
    Vec::new()
  };
  let category_label_line_count =
    category_label_lines.iter().map(Vec::len).max().unwrap_or(1) as f32;
  let category_label_height = category_label_line_height * category_label_line_count;
  let legend_position = chart.legend_position;
  let has_bottom_legend =
    legend_position == Some(ChartLegendPosition::Bottom) && !chart.legend_overlay;
  let has_untitled_bottom_column_layout = excel_untitled_bottom_column_layout(chart, style);
  let has_untitled_bottom_line_no_marker_layout =
    excel_untitled_bottom_line_no_marker_layout(chart, style);
  let has_explicit_bottom_column_layout = excel_explicit_bottom_column_layout(chart, style);
  let has_explicit_powerpoint_title =
    style.layout_profile == ChartLayoutProfile::PowerPoint && has_layout_explicit_title;
  let has_top_legend = legend_position == Some(ChartLegendPosition::Top) && !chart.legend_overlay;
  let has_side_legend = matches!(
    legend_position,
    Some(ChartLegendPosition::Left | ChartLegendPosition::Right | ChartLegendPosition::TopRight)
  ) && !chart.legend_overlay;
  let has_unshifted_side_line_layout = style.layout_profile == ChartLayoutProfile::Excel
    && has_side_legend
    && !has_layout_title
    && !chart.category_axis_shifted
    && chart.plot_layout.is_none()
    && chart
      .series
      .iter()
      .all(|series| matches!(series.kind, ChartSeriesKind::Line | ChartSeriesKind::Stock));
  let has_independent_axis_text_layout = style.layout_profile == ChartLayoutProfile::Excel
    && has_side_legend
    && !has_layout_title
    && chart.has_automatic_title_marker
    && chart.plot_layout.is_none()
    && chart
      .category_axis
      .is_some_and(|axis| axis.text_properties.is_some())
    && chart
      .value_axis
      .is_some_and(|axis| axis.text_properties.is_some());
  let host_defaults = match style.layout_profile {
    ChartLayoutProfile::PowerPoint => profiles::POWERPOINT_CARTESIAN_DEFAULTS,
    ChartLayoutProfile::Word => profiles::WORD_CARTESIAN_DEFAULTS,
    ChartLayoutProfile::Excel => profiles::EXCEL_CARTESIAN_DEFAULTS,
  };
  let host_side_legend_bands = match style.layout_profile {
    ChartLayoutProfile::PowerPoint => profiles::POWERPOINT_SIDE_LEGEND_BANDS,
    ChartLayoutProfile::Word if has_layout_explicit_title => {
      profiles::WORD_EXPLICIT_TITLE_SIDE_LEGEND_BANDS
    }
    ChartLayoutProfile::Word => profiles::WORD_SIDE_LEGEND_BANDS,
    ChartLayoutProfile::Excel => profiles::EXCEL_SIDE_LEGEND_BANDS,
  };
  // Word's automatic cartesian layout places a side-legend plot band lower
  // than the corresponding PowerPoint chart-space layout. The ratio is
  // stable across the three Office title-fill fixtures, which share no
  // c:manualLayout but differ in title-area fill.
  let word_side_adjustment = if style.layout_profile == ChartLayoutProfile::Word
    && has_side_legend
    && has_layout_explicit_title
    && chart.plot_layout.is_none()
  {
    profiles::WORD_EXPLICIT_TITLE_SIDE_LEGEND
  } else {
    profiles::CartesianLayoutAdjustment::default()
  };
  let excel_side_adjustment = if style.layout_profile == ChartLayoutProfile::Excel
    && has_side_legend
    && has_layout_explicit_title
    && chart.plot_layout.is_none()
  {
    profiles::EXCEL_EXPLICIT_TITLE_SIDE_LEGEND
  } else {
    profiles::CartesianLayoutAdjustment::default()
  };
  let excel_untitled_side_adjustment = if style.layout_profile == ChartLayoutProfile::Excel
    && has_side_legend
    && !has_layout_title
    && has_automatic_untitled_layout
    && chart.plot_layout.is_none()
  {
    if chart.title_overlay && matches!(chart.title.as_ref(), Some(ChartTitleText::Automatic)) {
      profiles::EXCEL_LEGACY_EMPTY_OVERLAY_SIDE_LEGEND
    } else if chart.title.is_none() && chart.has_explicit_categories {
      profiles::EXCEL_UNTITLED_EXPLICIT_CATEGORY_SIDE_LEGEND
    } else {
      profiles::EXCEL_AUTOMATIC_UNTITLED_SIDE_LEGEND
    }
  } else {
    profiles::CartesianLayoutAdjustment::default()
  };
  // Excel's automatic cartesian layout uses a separate title reservation when
  // no legend is authored. Office fixed-output evidence keeps both the plot
  // and category-label bands lower than the side-legend profile.
  let excel_title_only_adjustment = if style.layout_profile == ChartLayoutProfile::Excel
    && legend_position.is_none()
    && has_layout_explicit_title
    && chart.plot_layout.is_none()
  {
    profiles::EXCEL_TITLE_ONLY
  } else {
    profiles::CartesianLayoutAdjustment::default()
  };
  let data_table_height = chart.data_table.map_or(0.0, |_| {
    label_line_height * (chart.series.len() as f32 + 1.0) + label_line_height * 0.45
  });

  // Office's automatic chart layout reserves semantic bands around the plot:
  // title, value labels, category labels, and legend. The distances scale with
  // chart height, while actual label widths determine the left plot inset.
  let title_top = frame.y_pt
    + frame.height_pt
      * if style.layout_profile == ChartLayoutProfile::Excel
        && chart.title_overlay
        && matches!(chart.title.as_ref(), Some(ChartTitleText::Automatic))
      {
        profiles::EXCEL_LEGACY_EMPTY_OVERLAY_TITLE_TOP_RATIO
      } else if has_titled_indexed_scatter_layout {
        profiles::EXCEL_TITLED_INDEXED_SCATTER_TITLE_TOP_RATIO
      } else if has_legacy_single_series_title_layout {
        profiles::EXCEL_LEGACY_SINGLE_SERIES_TITLE_TOP_RATIO
      } else {
        host_defaults.title_top_ratio
      }
    + if has_derived_single_series_side_title_layout {
      frame.height_pt * profiles::EXCEL_DERIVED_SINGLE_SERIES_SIDE_TITLE.title_top_ratio
    } else if has_explicit_single_series_side_title_layout {
      frame.height_pt * profiles::EXCEL_EXPLICIT_SINGLE_SERIES_SIDE_TITLE.title_top_ratio
    } else if has_explicit_bottom_column_layout {
      frame.height_pt * profiles::EXCEL_EXPLICIT_BOTTOM_COLUMN.title_top_ratio
    } else {
      0.0
    };
  let legend_bottom_margin = style.label.font_size_pt * 0.81;
  let legend_top = frame.y_pt + frame.height_pt - legend_bottom_margin - label_line_height;
  let category_bottom_ratio = host_defaults.category_bottom_ratio;
  let mut category_top = if chart.data_table.is_some() {
    frame.y_pt + frame.height_pt
      - data_table_height
      - if has_bottom_legend {
        label_line_height + frame.height_pt * profiles::DATA_TABLE_BOTTOM_LEGEND_GAP_RATIO
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
          profiles::POWERPOINT_TITLED_BOTTOM_LEGEND_CATEGORY_GAP_RATIO
        } else {
          profiles::DEFAULT_BOTTOM_LEGEND_CATEGORY_GAP_RATIO
        }
  } else {
    frame.y_pt + frame.height_pt - category_label_height - frame.height_pt * category_bottom_ratio
  } + frame.height_pt * word_side_adjustment.category_top_ratio
    + frame.height_pt * excel_side_adjustment.category_top_ratio
    + frame.height_pt * excel_title_only_adjustment.category_top_ratio
    + frame.height_pt * excel_untitled_side_adjustment.category_top_ratio;
  if has_independent_axis_text_layout {
    // Excel's automatic plot reservation is slightly tighter when both axes
    // carry independent text bodies. Keep that authored-axis profile separate
    // from the ordinary untitled side-legend layout.
    category_top += frame.height_pt * profiles::EXCEL_INDEPENDENT_AXIS_TEXT.category_top_ratio;
  } else if has_titled_indexed_scatter_layout {
    // Excel 2013's automatic-title scatter layout reserves a shallower plot
    // above the bottom legend than the legacy untitled scatter profile.
    category_top += frame.height_pt * profiles::EXCEL_TITLED_INDEXED_SCATTER.category_top_ratio;
  } else if has_legacy_indexed_scatter_layout {
    // Pre-2013 string-valued scatter caches use indexed x positions, but keep
    // the ordinary single-component label and legend bands.
    category_top += frame.height_pt * profiles::EXCEL_LEGACY_INDEXED_SCATTER.category_top_ratio;
  }
  if has_modern_single_series_title_layout {
    category_top += frame.height_pt * profiles::EXCEL_MODERN_SINGLE_SERIES_TITLE.category_top_ratio;
  }
  if has_unshifted_side_line_layout {
    // `crossBetween="midCat"` puts the first and last line markers on the
    // plot edges. Excel's automatic side-legend layout reserves a slightly
    // lower category band for that on-marker axis profile.
    category_top += frame.height_pt * profiles::EXCEL_UNSHIFTED_SIDE_LINE.category_top_ratio;
  }
  if has_legacy_single_series_title_layout {
    category_top += frame.height_pt * profiles::EXCEL_LEGACY_SINGLE_SERIES_TITLE.category_top_ratio;
  } else if has_untitled_bottom_column_layout {
    // Modern Excel's missing autoTitleDeleted marker keeps the chart
    // untitled and reserves a compact bottom category/legend band.
    category_top += frame.height_pt * profiles::EXCEL_UNTITLED_BOTTOM_COLUMN.category_top_ratio;
  } else if has_derived_single_series_side_title_layout {
    category_top +=
      frame.height_pt * profiles::EXCEL_DERIVED_SINGLE_SERIES_SIDE_TITLE.category_top_ratio;
  } else if has_explicit_single_series_side_title_layout {
    category_top +=
      frame.height_pt * profiles::EXCEL_EXPLICIT_SINGLE_SERIES_SIDE_TITLE.category_top_ratio;
  } else if has_untitled_bottom_line_no_marker_layout {
    category_top +=
      frame.height_pt * profiles::EXCEL_UNTITLED_BOTTOM_LINE_NO_MARKER.category_top_ratio;
  } else if has_explicit_bottom_column_layout {
    category_top += frame.height_pt * profiles::EXCEL_EXPLICIT_BOTTOM_COLUMN.category_top_ratio;
  }
  let untitled_plot_top_ratio = if has_side_legend {
    host_defaults.untitled_side_plot_top_ratio
  } else {
    host_defaults.untitled_no_side_plot_top_ratio
  };
  let mut plot_top = if has_layout_title {
    title_top + title_line_height + label_line_height * 0.9
  } else {
    frame.y_pt + frame.height_pt * untitled_plot_top_ratio
  } + frame.height_pt * word_side_adjustment.plot_top_ratio
    + frame.height_pt * excel_side_adjustment.plot_top_ratio
    + frame.height_pt * excel_title_only_adjustment.plot_top_ratio
    + frame.height_pt * excel_untitled_side_adjustment.plot_top_ratio;
  if has_independent_axis_text_layout {
    plot_top += frame.height_pt * profiles::EXCEL_INDEPENDENT_AXIS_TEXT.plot_top_ratio;
  } else if has_legacy_indexed_scatter_layout {
    plot_top += frame.height_pt * profiles::EXCEL_LEGACY_INDEXED_SCATTER.plot_top_ratio;
  }
  if has_modern_single_series_title_layout {
    plot_top += frame.height_pt * profiles::EXCEL_MODERN_SINGLE_SERIES_TITLE.plot_top_ratio;
  }
  if has_titled_indexed_scatter_layout {
    plot_top += frame.height_pt * profiles::EXCEL_TITLED_INDEXED_SCATTER.plot_top_ratio;
  } else if has_legacy_single_series_title_layout {
    plot_top += frame.height_pt * profiles::EXCEL_LEGACY_SINGLE_SERIES_TITLE.plot_top_ratio;
  } else if has_untitled_bottom_column_layout {
    plot_top += frame.height_pt * profiles::EXCEL_UNTITLED_BOTTOM_COLUMN.plot_top_ratio;
  } else if has_derived_single_series_side_title_layout {
    // title_top already carries the derived-title displacement; only the
    // residual plot reservation is added here.
    plot_top += frame.height_pt * profiles::EXCEL_DERIVED_SINGLE_SERIES_SIDE_TITLE.plot_top_ratio;
  } else if has_explicit_single_series_side_title_layout {
    // The title displacement above is part of the automatic plot
    // reservation; retain only the remaining Office plot-band offset.
    plot_top += frame.height_pt * profiles::EXCEL_EXPLICIT_SINGLE_SERIES_SIDE_TITLE.plot_top_ratio;
  } else if has_untitled_bottom_line_no_marker_layout {
    plot_top += frame.height_pt * profiles::EXCEL_UNTITLED_BOTTOM_LINE_NO_MARKER.plot_top_ratio;
  } else if has_explicit_bottom_column_layout {
    // title_top carries most of the authored-title displacement.
    plot_top += frame.height_pt * profiles::EXCEL_EXPLICIT_BOTTOM_COLUMN.plot_top_ratio;
  }
  if has_top_legend {
    plot_top += label_line_height
      + frame.height_pt
        * if style.layout_profile == ChartLayoutProfile::Word {
          profiles::WORD_TOP_LEGEND_GAP_RATIO
        } else {
          profiles::DEFAULT_TOP_LEGEND_GAP_RATIO
        };
  }
  let side_category_gap_ratio = match style.layout_profile {
    ChartLayoutProfile::Excel
      if has_side_legend && !has_layout_title && has_automatic_untitled_layout =>
    {
      profiles::EXCEL_AUTOMATIC_UNTITLED_SIDE_CATEGORY_GAP_RATIO
    }
    ChartLayoutProfile::Excel if has_derived_single_series_side_title_layout => {
      profiles::EXCEL_DERIVED_TITLE_SIDE_CATEGORY_GAP_RATIO
    }
    ChartLayoutProfile::Excel if has_explicit_single_series_side_title_layout => {
      profiles::EXCEL_EXPLICIT_TITLE_SIDE_CATEGORY_GAP_RATIO
    }
    ChartLayoutProfile::PowerPoint | ChartLayoutProfile::Word | ChartLayoutProfile::Excel => {
      host_side_legend_bands.category_gap_ratio
    }
  };
  let category_plot_gap_ratio =
    if style.layout_profile == ChartLayoutProfile::Word && category_label_line_count > 1.0 {
      profiles::WORD_MULTILINE_CATEGORY_PLOT_GAP_RATIO
    } else if style.layout_profile == ChartLayoutProfile::Excel
      && legend_position.is_none()
      && has_layout_explicit_title
      && chart.plot_layout.is_none()
    {
      profiles::EXCEL_TITLE_ONLY_CATEGORY_PLOT_GAP_RATIO
    } else if (style.layout_profile == ChartLayoutProfile::Excel
      && legend_position.is_none()
      && has_legacy_single_series_title_layout
      && chart.plot_layout.is_none())
      || has_untitled_bottom_column_layout
    {
      profiles::EXCEL_LEGACY_TITLE_CATEGORY_PLOT_GAP_RATIO
    } else if has_untitled_bottom_line_no_marker_layout || has_explicit_bottom_column_layout {
      profiles::EXCEL_BOTTOM_CATEGORY_PLOT_GAP_RATIO
    } else {
      profiles::DEFAULT_CATEGORY_PLOT_GAP_RATIO
    };
  let mut plot_bottom = category_top
    - frame.height_pt
      * if has_side_legend {
        side_category_gap_ratio
      } else if has_bottom_legend && has_explicit_powerpoint_title {
        profiles::POWERPOINT_TITLED_BOTTOM_PLOT_GAP_RATIO
      } else {
        category_plot_gap_ratio
      };
  if has_titled_indexed_scatter_layout {
    plot_bottom += frame.height_pt * profiles::EXCEL_TITLED_INDEXED_SCATTER.plot_bottom_ratio;
  } else if has_indexed_scatter_automatic_layout {
    // Excel's untitled scatter profile reserves a wider four-sided axis
    // band than the generic cartesian profile. This is visible when string
    // x-values are imported as indexed positions: Office's fixed output for
    // ser_labels.xlsx places the plot at
    // (0.1078, 0.0579)-(0.9573, 0.8721) of the chart frame. Keep the
    // adjustment relative to the frame so worksheet print scaling and
    // horizontal page clipping continue to apply normally.
    category_top += frame.height_pt * profiles::EXCEL_AUTOMATIC_INDEXED_SCATTER.category_top_ratio;
    plot_top += frame.height_pt * profiles::EXCEL_AUTOMATIC_INDEXED_SCATTER.plot_top_ratio;
    plot_bottom += frame.height_pt * profiles::EXCEL_AUTOMATIC_INDEXED_SCATTER.plot_bottom_ratio;
  }
  if has_unshifted_side_line_layout {
    plot_top += frame.height_pt * profiles::EXCEL_UNSHIFTED_SIDE_LINE.plot_top_ratio;
    plot_bottom += frame.height_pt * profiles::EXCEL_UNSHIFTED_SIDE_LINE.plot_bottom_ratio;
  }
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
      .map(|(_, label)| metrics.measure_text(label, &style.value_label))
      .fold(0.0_f32, f32::max)
  } else {
    0.0
  };
  let side_legend_width = if has_side_legend {
    vertical_legend_width(chart, style, &mut metrics)
  } else {
    0.0
  };
  let side_plot_outer_margin = frame.height_pt * profiles::CARTESIAN_SIDE_PLOT_OUTER_MARGIN_RATIO;
  let side_legend_outer_margin = frame.height_pt
    * match style.layout_profile {
      ChartLayoutProfile::Excel
        if !has_layout_title
          && has_automatic_untitled_layout
          && (chart.has_explicit_categories
            || (chart.title_overlay
              && matches!(chart.title.as_ref(), Some(ChartTitleText::Automatic)))
            || has_indexed_scatter_automatic_layout) =>
      {
        profiles::EXCEL_AUTOMATIC_UNTITLED_SIDE_LEGEND_OUTER_MARGIN_RATIO
      }
      ChartLayoutProfile::Excel if has_derived_single_series_side_title_layout => {
        profiles::EXCEL_DERIVED_TITLE_SIDE_LEGEND_OUTER_MARGIN_RATIO
      }
      ChartLayoutProfile::PowerPoint | ChartLayoutProfile::Word | ChartLayoutProfile::Excel => {
        host_side_legend_bands.legend_outer_margin_ratio
      }
    };
  let side_plot_gap = frame.height_pt
    * match style.layout_profile {
      ChartLayoutProfile::Excel
        if has_side_legend
          && !has_layout_title
          && has_automatic_untitled_layout
          && (chart.cached_category_count == 2
            || chart.has_explicit_categories
            || (chart.title_overlay
              && matches!(chart.title.as_ref(), Some(ChartTitleText::Automatic)))) =>
      {
        profiles::EXCEL_AUTOMATIC_UNTITLED_COMPACT_SIDE_PLOT_GAP_RATIO
      }
      ChartLayoutProfile::Excel
        if has_side_legend && !has_layout_title && has_automatic_untitled_layout =>
      {
        profiles::EXCEL_AUTOMATIC_UNTITLED_WIDE_SIDE_PLOT_GAP_RATIO
      }
      ChartLayoutProfile::Excel if has_unshifted_side_line_layout => {
        profiles::EXCEL_UNSHIFTED_LINE_SIDE_PLOT_GAP_RATIO
      }
      ChartLayoutProfile::PowerPoint | ChartLayoutProfile::Word | ChartLayoutProfile::Excel => {
        host_side_legend_bands.plot_gap_ratio
      }
    };
  let tick_left_ratio = if has_titled_indexed_scatter_layout {
    profiles::EXCEL_TITLED_INDEXED_SCATTER_TICK_LEFT_RATIO
  } else if has_legacy_single_series_title_layout {
    profiles::EXCEL_LEGACY_SINGLE_SERIES_TICK_LEFT_RATIO
  } else if has_untitled_bottom_column_layout {
    profiles::EXCEL_UNTITLED_BOTTOM_COLUMN_TICK_LEFT_RATIO
  } else if !value_tick_labels_visible && style.layout_profile == ChartLayoutProfile::Word {
    profiles::WORD_HIDDEN_VALUE_TICK_LEFT_RATIO
  } else if has_side_legend {
    match style.layout_profile {
      ChartLayoutProfile::Excel if !has_layout_title && has_automatic_untitled_layout => {
        profiles::EXCEL_AUTOMATIC_UNTITLED_SIDE_TICK_LEFT_RATIO
      }
      ChartLayoutProfile::PowerPoint | ChartLayoutProfile::Word | ChartLayoutProfile::Excel => {
        host_side_legend_bands.tick_left_ratio
      }
    }
  } else {
    if style.layout_profile == ChartLayoutProfile::Excel
      && legend_position.is_none()
      && has_layout_explicit_title
      && chart.plot_layout.is_none()
    {
      profiles::EXCEL_TITLE_ONLY_TICK_LEFT_RATIO
    } else if has_bottom_legend && has_explicit_powerpoint_title {
      profiles::POWERPOINT_TITLED_BOTTOM_TICK_LEFT_RATIO
    } else {
      profiles::DEFAULT_TICK_LEFT_RATIO
    }
  };
  let tick_left = frame.x_pt
    + frame.height_pt * tick_left_ratio
    + if has_derived_single_series_side_title_layout {
      frame.height_pt * profiles::EXCEL_DERIVED_SINGLE_SERIES_SIDE_TITLE.tick_left_ratio
    } else if has_explicit_single_series_side_title_layout {
      frame.height_pt * profiles::EXCEL_EXPLICIT_SINGLE_SERIES_SIDE_TITLE.tick_left_ratio
    } else if has_untitled_bottom_line_no_marker_layout {
      frame.height_pt * profiles::EXCEL_UNTITLED_BOTTOM_LINE_NO_MARKER.tick_left_ratio
    } else if has_explicit_bottom_column_layout {
      frame.height_pt * profiles::EXCEL_EXPLICIT_BOTTOM_COLUMN.tick_left_ratio
    } else {
      0.0
    }
    + if legend_position == Some(ChartLegendPosition::Left) {
      side_legend_width + side_plot_outer_margin + side_plot_gap
    } else {
      0.0
    };
  let tick_left = tick_left
    + if has_indexed_scatter_automatic_layout {
      frame.height_pt * profiles::EXCEL_AUTOMATIC_INDEXED_SCATTER.tick_left_ratio
    } else if has_legacy_indexed_scatter_layout {
      frame.height_pt * profiles::EXCEL_LEGACY_INDEXED_SCATTER.tick_left_ratio
    } else {
      0.0
    };
  let tick_gap = if value_tick_labels_visible {
    frame.height_pt
      * if has_side_legend {
        if has_unshifted_side_line_layout {
          profiles::EXCEL_UNSHIFTED_LINE_SIDE_TICK_GAP_RATIO
        } else {
          host_side_legend_bands.tick_gap_ratio
        }
      } else {
        if has_bottom_legend && has_explicit_powerpoint_title {
          profiles::POWERPOINT_TITLED_BOTTOM_TICK_GAP_RATIO
        } else {
          profiles::DEFAULT_TICK_GAP_RATIO
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
          profiles::POWERPOINT_TITLED_BOTTOM_RIGHT_MARGIN_RATIO
        } else if !value_tick_labels_visible && style.layout_profile == ChartLayoutProfile::Word {
          profiles::WORD_HIDDEN_VALUE_RIGHT_MARGIN_RATIO
        } else {
          profiles::DEFAULT_RIGHT_MARGIN_RATIO
        }
    };
  if style.layout_profile == ChartLayoutProfile::Excel
    && has_side_legend
    && has_layout_explicit_title
    && chart.plot_layout.is_none()
  {
    plot_left += frame.height_pt * excel_side_adjustment.plot_left_ratio;
    plot_right += frame.height_pt * excel_side_adjustment.plot_right_ratio;
  } else if style.layout_profile == ChartLayoutProfile::Excel
    && has_side_legend
    && !has_layout_title
    && has_automatic_untitled_layout
    && chart.plot_layout.is_none()
  {
    plot_left += frame.height_pt * excel_untitled_side_adjustment.plot_left_ratio;
  } else if style.layout_profile == ChartLayoutProfile::Excel
    && legend_position.is_none()
    && has_layout_explicit_title
    && chart.plot_layout.is_none()
  {
    plot_left += frame.height_pt * excel_title_only_adjustment.plot_left_ratio;
    plot_right += frame.height_pt * excel_title_only_adjustment.plot_right_ratio;
  }
  if has_titled_indexed_scatter_layout {
    plot_left += frame.height_pt * profiles::EXCEL_TITLED_INDEXED_SCATTER.plot_left_ratio;
    plot_right += frame.height_pt * profiles::EXCEL_TITLED_INDEXED_SCATTER.plot_right_ratio;
  } else if has_indexed_scatter_automatic_layout {
    plot_left += frame.height_pt * profiles::EXCEL_AUTOMATIC_INDEXED_SCATTER.plot_left_ratio;
    plot_right += frame.height_pt * profiles::EXCEL_AUTOMATIC_INDEXED_SCATTER.plot_right_ratio;
  } else if has_legacy_indexed_scatter_layout {
    plot_right += frame.height_pt * profiles::EXCEL_LEGACY_INDEXED_SCATTER.plot_right_ratio;
  } else if has_legacy_single_series_title_layout {
    plot_left += frame.height_pt * profiles::EXCEL_LEGACY_SINGLE_SERIES_TITLE.plot_left_ratio;
    plot_right += frame.height_pt * profiles::EXCEL_LEGACY_SINGLE_SERIES_TITLE.plot_right_ratio;
  } else if has_untitled_bottom_column_layout {
    plot_left += frame.height_pt * profiles::EXCEL_UNTITLED_BOTTOM_COLUMN.plot_left_ratio;
    plot_right += frame.height_pt * profiles::EXCEL_UNTITLED_BOTTOM_COLUMN.plot_right_ratio;
  } else if has_derived_single_series_side_title_layout {
    plot_right +=
      frame.height_pt * profiles::EXCEL_DERIVED_SINGLE_SERIES_SIDE_TITLE.plot_right_ratio;
  } else if has_explicit_single_series_side_title_layout {
    plot_left +=
      frame.height_pt * profiles::EXCEL_EXPLICIT_SINGLE_SERIES_SIDE_TITLE.plot_left_ratio;
    plot_right +=
      frame.height_pt * profiles::EXCEL_EXPLICIT_SINGLE_SERIES_SIDE_TITLE.plot_right_ratio;
  } else if has_untitled_bottom_line_no_marker_layout {
    plot_left += frame.height_pt * profiles::EXCEL_UNTITLED_BOTTOM_LINE_NO_MARKER.plot_left_ratio;
    plot_right += frame.height_pt * profiles::EXCEL_UNTITLED_BOTTOM_LINE_NO_MARKER.plot_right_ratio;
  } else if has_explicit_bottom_column_layout {
    plot_left += frame.height_pt * profiles::EXCEL_EXPLICIT_BOTTOM_COLUMN.plot_left_ratio;
    plot_right += frame.height_pt * profiles::EXCEL_EXPLICIT_BOTTOM_COLUMN.plot_right_ratio;
  }
  if style.layout_profile == ChartLayoutProfile::Excel
    && has_side_legend
    && !has_layout_title
    && has_automatic_untitled_layout
    && chart.has_explicit_categories
    && chart.plot_layout.is_none()
  {
    // Excel's explicit-category automatic chart profile keeps a slightly
    // larger value-axis inset while extending the right edge of the plot.
    // The ratios come from the Office fixed-output geometry of
    // chart-area-style-border.xlsx and are independent of page coordinates.
    plot_left += frame.height_pt * profiles::EXCEL_EXPLICIT_CATEGORY_AUTOMATIC.plot_left_ratio;
    plot_right += frame.height_pt * profiles::EXCEL_EXPLICIT_CATEGORY_AUTOMATIC.plot_right_ratio;
    plot_top += frame.height_pt * profiles::EXCEL_EXPLICIT_CATEGORY_AUTOMATIC.plot_top_ratio;
    plot_bottom += frame.height_pt * profiles::EXCEL_EXPLICIT_CATEGORY_AUTOMATIC.plot_bottom_ratio;
  }
  if horizontal_bar_only {
    let category_width = chart
      .categories
      .iter()
      .map(|category| metrics.measure_text(category, &style.category_label))
      .fold(0.0_f32, f32::max);
    plot_left += category_width + style.category_label.font_size_pt * 0.8;
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
  let axis_line_width = match style.layout_profile {
    ChartLayoutProfile::Word | ChartLayoutProfile::Excel => 1.0,
    ChartLayoutProfile::PowerPoint => 0.75,
  } * style.stroke_scale;
  let zero_y = value_y(
    0.0_f64.clamp(scale.minimum, scale.maximum),
    scale,
    plot_top,
    plot_height,
  );

  let mut items = Vec::new();
  if let Some(fill_color) = style.chart_area_fill_color {
    items.push(PageItem::Rect(RectItem {
      x_pt: frame.x_pt,
      y_pt: frame.y_pt,
      width_pt: frame.width_pt,
      height_pt: frame.height_pt,
      fill_color: Some(fill_color),
      fill_opacity: 1.0,
      stroke: None,
      stroke_opacity: 1.0,
    }));
  }
  if let Some(color) = style.chart_area_stroke_color {
    let (outline_x, outline_y, outline_width, outline_height) = if style.layout_profile
      == ChartLayoutProfile::Excel
      && has_side_legend
      && !has_layout_title
      && has_automatic_untitled_layout
      && chart.has_explicit_categories
      && chart.plot_layout.is_none()
    {
      (
        frame.x_pt - frame.height_pt * 0.000_85,
        frame.y_pt + frame.height_pt * 0.001_46,
        frame.width_pt - frame.height_pt * 0.004,
        frame.height_pt + frame.height_pt * 0.004_31,
      )
    } else {
      (frame.x_pt, frame.y_pt, frame.width_pt, frame.height_pt)
    };
    push_chart_rect_outline(
      &mut items,
      outline_x,
      outline_y,
      outline_width,
      outline_height,
      color,
      0.75 * style.stroke_scale,
    );
  }
  if let Some(fill_color) = style.plot_area_fill_color {
    items.push(PageItem::Rect(RectItem {
      x_pt: plot_left,
      y_pt: plot_top,
      width_pt: plot_width,
      height_pt: plot_height,
      fill_color: Some(fill_color),
      fill_opacity: 1.0,
      stroke: None,
      stroke_opacity: 1.0,
    }));
  }
  if let Some(color) = style.plot_area_stroke_color {
    push_chart_rect_outline(
      &mut items,
      plot_left,
      plot_top,
      plot_width,
      plot_height,
      color,
      0.75 * style.stroke_scale,
    );
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
        width_pt: axis_line_width,
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
      width_pt: axis_line_width,
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
        width_pt: axis_line_width,
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
            width_pt: axis_line_width,
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
          width_pt: axis_line_width,
          color: style.gridline_color,
          kind: LineItemKind::Stroke,
        }));
      }
    }
  }

  for (series_index, series) in chart.series.iter().enumerate() {
    for (label_index, label) in series.data_labels.iter().enumerate() {
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
          anchor.x
            + if has_unshifted_side_line_layout {
              chart_marker_size(&chart.series[series_index], 3.0)
                .map(|size| size * style.stroke_scale)
                .unwrap_or(0.0)
            } else {
              0.0
            }
            + style.data_label.font_size_pt * 0.2,
          anchor.y - data_label_line_height * 0.5,
        ),
        c::DataLabelPositionValues::OutsideEnd | c::DataLabelPositionValues::BestFit => (
          anchor.x - width * 0.5,
          anchor.y
            - data_label_line_height
            - if has_derived_single_series_side_title_layout {
              frame.height_pt * 0.020_82
            } else {
              0.0
            },
        ),
        c::DataLabelPositionValues::Top => {
          let marker_clearance = if matches!(
            chart.series[series_index].kind,
            ChartSeriesKind::Line | ChartSeriesKind::Scatter | ChartSeriesKind::Stock
          ) {
            chart_marker_size(&chart.series[series_index], 3.0)
              .map(|size| size * style.stroke_scale * 0.5)
              .unwrap_or(0.0)
              + style.data_label.font_size_pt * 0.45
          } else {
            0.0
          };
          (
            anchor.x - width * 0.5,
            anchor.y - data_label_line_height - marker_clearance,
          )
        }
      };
      if let Some(fill_color) = style
        .data_label_fill_colors
        .get(series_index)
        .and_then(|colors| colors.get(label_index))
        .copied()
        .flatten()
      {
        let horizontal_padding = style.data_label.font_size_pt * 0.25;
        let vertical_padding = style.data_label.font_size_pt * 0.26;
        items.push(PageItem::Rect(RectItem {
          x_pt: x - horizontal_padding,
          y_pt: y - vertical_padding,
          width_pt: width + horizontal_padding * 2.0,
          height_pt: data_label_line_height + vertical_padding * 2.0,
          fill_color: Some(fill_color),
          fill_opacity: 1.0,
          stroke: None,
          stroke_opacity: 1.0,
        }));
      }
      push_data_label_text_components(&mut items, &mut metrics, x, y, label, &style.data_label);
    }
  }

  // Preserve Office's content-stream ordering: value ticks from minimum to
  // maximum, then categories, title, and legend. This is also the reading order
  // exposed by tagged fixed-format output.
  if value_tick_labels_visible {
    for (value, label) in &tick_labels {
      let indexed_scatter_text_offset =
        if has_indexed_scatter_automatic_layout && has_multicomponent_data_labels {
          let major_index = ((*value - scale.minimum) / scale.major_unit).round() as i32;
          let major_count = ((scale.maximum - scale.minimum) / scale.major_unit).round() as i32;
          if major_index == major_count - 1 {
            style.value_label.font_size_pt * 0.367
          } else if major_index == major_count / 2 {
            -style.value_label.font_size_pt * 0.256
          } else if major_index == major_count / 4 {
            -style.value_label.font_size_pt * 0.244
          } else {
            0.0
          }
        } else {
          0.0
        };
      let width = metrics.measure_text(label, &style.value_label);
      push_text(
        &mut items,
        tick_left + maximum_tick_width - width,
        value_y(*value, scale, plot_top, plot_height) - value_label_line_height / 2.0
          + indexed_scatter_text_offset,
        label.clone(),
        style.value_label.clone(),
      );
    }
  }
  if category_tick_labels_visible {
    for (category_index, lines) in category_label_lines.iter().enumerate() {
      let display_index = category_display_index(chart, category_index, category_count);
      let center = category_point_x(
        chart,
        display_index,
        category_count,
        PlotRect {
          left: plot_left,
          top: plot_top,
          width: plot_width,
          height: plot_height,
        },
      );
      for (line_index, line) in lines.iter().enumerate() {
        let width = metrics.measure_text(line, &style.category_label);
        push_text(
          &mut items,
          center - width / 2.0,
          category_top + line_index as f32 * category_label_line_height,
          line.clone(),
          style.category_label.clone(),
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
    let painted_title_top = title_top
      + if style.layout_profile == ChartLayoutProfile::Excel
        && chart.title_vertical_anchor == Some(a::TextAnchoringTypeValues::Bottom)
        && !chart.title_overlay
        && chart.plot_layout.is_none()
      {
        // A rich title anchored to the bottom of Excel's automatic title slot
        // moves within that reserved slot; it does not enlarge the plot's
        // title reservation. Title fill and text move together.
        frame.height_pt * 0.005
      } else {
        0.0
      };
    let width = metrics.measure_text(title, &style.title);
    let title_x = frame.x_pt + (frame.width_pt - width) / 2.0
      - if has_explicit_single_series_side_title_layout {
        frame.height_pt * 0.004_76
      } else if has_explicit_bottom_column_layout {
        frame.height_pt * 0.003_23
      } else {
        0.0
      };
    if let Some(color) = style.title_fill_color {
      let horizontal_padding = style.title.font_size_pt * 0.162;
      let vertical_padding = style.title.font_size_pt * 0.092;
      items.push(PageItem::Rect(RectItem {
        x_pt: title_x - horizontal_padding,
        y_pt: painted_title_top - vertical_padding,
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
      painted_title_top,
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
        frame.x_pt + frame.height_pt * profiles::TOP_LEGEND_LEFT_INSET_RATIO,
        if has_layout_title {
          title_top
            + title_line_height
            + frame.height_pt * host_defaults.titled_top_legend_gap_ratio
        } else {
          frame.y_pt + frame.height_pt * profiles::UNTITLED_TOP_LEGEND_TOP_RATIO
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
        frame.x_pt + frame.width_pt - side_legend_outer_margin - side_legend_width
          + if has_explicit_single_series_side_title_layout {
            -frame.height_pt * 0.009_84
          } else if style.layout_profile == ChartLayoutProfile::Excel
            && chart.title.is_none()
            && chart.has_automatic_title_marker
            && chart.has_explicit_categories
            && chart.plot_layout.is_none()
          {
            frame.height_pt * 0.001_42
          } else {
            0.0
          },
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

fn push_chart_rect_outline(
  items: &mut Vec<PageItem>,
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  color: RgbColor,
  line_width_pt: f32,
) {
  let right = x_pt + width_pt;
  let bottom = y_pt + height_pt;
  for (x1_pt, y1_pt, x2_pt, y2_pt) in [
    (x_pt, y_pt, right, y_pt),
    (right, y_pt, right, bottom),
    (right, bottom, x_pt, bottom),
    (x_pt, bottom, x_pt, y_pt),
  ] {
    items.push(PageItem::Line(LineItem {
      x1_pt,
      y1_pt,
      x2_pt,
      y2_pt,
      width_pt: line_width_pt,
      color,
      kind: LineItemKind::Stroke,
    }));
  }
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
  let host_defaults = radial_host_defaults(style.layout_profile);
  let title_height = title.map_or(0.0, |_| {
    line_height(&style.title) * host_defaults.title_height_scale
  });
  let legend = chart.legend_position;
  let side_legend = matches!(
    legend,
    Some(ChartLegendPosition::Left | ChartLegendPosition::Right | ChartLegendPosition::TopRight)
  ) && !chart.legend_overlay;
  let bottom_legend = legend == Some(ChartLegendPosition::Bottom) && !chart.legend_overlay;
  let top_legend = legend == Some(ChartLegendPosition::Top) && !chart.legend_overlay;
  let legend_width = if side_legend {
    let maximum_label_width = chart
      .visible_legend_indices
      .iter()
      .filter_map(|index| chart.categories.get(*index))
      .map(|text| metrics.measure_text(text, &style.label))
      .fold(0.0_f32, f32::max);
    maximum_label_width + style.label.font_size_pt * host_defaults.side_legend_width_em
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
  // Excel keeps a circular 2-D pie inside the plot height. PowerPoint and
  // Word use a 4:3-expanded height basis.
  let radius_basis = plot
    .width
    .min(plot.height * host_defaults.radius_height_basis_scale);
  let compact_plot =
    style.layout_profile == ChartLayoutProfile::PowerPoint && title.is_some() && side_legend
      || style.layout_profile == ChartLayoutProfile::Excel && title.is_some() && bottom_legend;
  let radius_scale = if compact_plot {
    host_defaults.compact_radius_scale
  } else {
    host_defaults.radius_scale
  };
  let radius_x = radius_basis * radius_scale;
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
      // Excel interprets c:explosion as approximately the percentage of the
      // pie radius. The host profile retains its smaller Word/PowerPoint
      // displacement policy.
      let explosion_scale = host_defaults.explosion_scale;
      let offset_x = mid.sin() * radius_x * explosion * explosion_scale;
      let offset_y = -mid.cos() * radius_y * explosion * explosion_scale;
      let color = style.point_colors[index % style.point_colors.len()];
      if depth > 0.0 {
        items.push(radial_segment_path(
          (center_x + offset_x, center_y + offset_y + depth),
          (radius_x, radius_y),
          hole_ratio,
          (start_angle, sweep),
          (color, 0.58),
          true,
        ));
      }
      items.push(radial_segment_path(
        (center_x + offset_x, center_y + offset_y),
        (radius_x, radius_y),
        hole_ratio,
        (start_angle, sweep),
        (color, 1.0),
        true,
      ));
      start_angle += sweep;
    }
  }

  for (label_index, label) in chart.data_labels.iter().enumerate() {
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
    let width = metrics.measure_text(&label.text, &style.data_label);
    let label_height = line_height(&style.data_label);
    let (label_x, label_y) = if style.layout_profile == ChartLayoutProfile::Excel
      && chart.kind == RadialChartKind::Pie
      && label.position == c::DataLabelPositionValues::BestFit
    {
      excel_best_fit_pie_label_position(
        (center_x, center_y),
        (radius_x, radius_y),
        (angle, value / total * std::f64::consts::TAU),
        (width, label_height),
      )
    } else {
      let ring = if outside {
        1.08
      } else {
        (1.0 + hole_ratio) * 0.5
      };
      (
        center_x + angle.sin() * radius_x * ring - width * 0.5,
        center_y - angle.cos() * radius_y * ring - label_height * 0.5,
      )
    };
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
    if let Some(fill_color) = style
      .data_label_fill_colors
      .get(label_index)
      .copied()
      .flatten()
    {
      let horizontal_padding = style.data_label.font_size_pt * 0.3;
      let vertical_padding = style.data_label.font_size_pt * 0.4;
      items.push(PageItem::Rect(RectItem {
        x_pt: label_x - horizontal_padding,
        y_pt: label_y - vertical_padding,
        width_pt: width + horizontal_padding * 2.0,
        height_pt: line_height(&style.data_label) + vertical_padding * 2.0,
        fill_color: Some(fill_color),
        fill_opacity: 1.0,
        stroke: None,
        stroke_opacity: 1.0,
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
      frame.y_pt
        + frame.height_pt * profiles::RADIAL_TITLE_TOP_RATIO
        + if style.layout_profile == ChartLayoutProfile::Excel && bottom_legend {
          style.title.font_size_pt * profiles::EXCEL_BOTTOM_LEGEND_TITLE_OFFSET_EM
        } else {
          0.0
        },
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

fn excel_best_fit_pie_label_position(
  center: (f32, f32),
  radii: (f32, f32),
  angles: (f32, f64),
  label_size: (f32, f32),
) -> (f32, f32) {
  let (angle, sweep) = angles;
  let sweep_degrees = sweep.to_degrees() as f32;
  // Excel's best-fit position is not the fixed half-radius "center"
  // position. It moves the complete label box toward the slice boundary and
  // may rotate the anchor away from the bisector so every corner remains in
  // the sector. These three continuous regions correspond to narrow,
  // ordinary, and reflex sectors in the Office fixed-output layout.
  let profile = if sweep_degrees > 180.0 {
    profiles::EXCEL_REFLEX_BEST_FIT_LABEL
  } else if sweep_degrees > 75.0 {
    profiles::EXCEL_ORDINARY_BEST_FIT_LABEL
  } else {
    profiles::EXCEL_NARROW_BEST_FIT_LABEL
  };
  let adjusted_angle = angle + profile.angle_adjustment_degrees.to_radians();
  let (width, height) = label_size;
  (
    center.0 + adjusted_angle.sin() * radii.0 * profile.radius_factor - width * 0.5,
    center.1 - adjusted_angle.cos() * radii.1 * profile.radius_factor - height * 0.5,
  )
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
    // The aggregate "Other" slice inherits the series fill. For varying
    // colors this is the first palette color, not the color of the first
    // point moved into the secondary chart.
    primary.push((0, secondary));
  }
  let primary_total = primary.iter().map(|(_, value)| *value).sum::<f64>();
  if primary_total <= f64::EPSILON {
    return;
  }
  let plot_profile = if style.layout_profile == ChartLayoutProfile::Excel {
    match chart.kind {
      RadialChartKind::PieOfPie => profiles::EXCEL_PIE_OF_PIE_PLOT,
      RadialChartKind::BarOfPie => profiles::EXCEL_BAR_OF_PIE_PLOT,
      _ => profiles::DEFAULT_OF_PIE_PLOT,
    }
  } else {
    profiles::DEFAULT_OF_PIE_PLOT
  };
  let primary_center = (
    plot.left + plot.width * plot_profile.primary_x_ratio,
    plot.top + plot.height * plot_profile.center_y_ratio,
  );
  let primary_radius = plot.width.min(plot.height) * plot_profile.primary_radius_scale;
  let secondary_center = (
    plot.left + plot.width * plot_profile.secondary_x_ratio,
    plot.top + plot.height * plot_profile.center_y_ratio,
  );
  let secondary_radius =
    primary_radius * (chart.secondary_size_percent / 100.0).clamp(0.05, 2.0) as f32;
  // Office rotates an of-pie chart so the aggregate slice is centered on
  // the secondary chart at the right. The aggregate is appended after the
  // original primary points, so the first slice starts half its sweep past
  // the right-facing radial.
  let aggregate_sweep = (secondary / primary_total * std::f64::consts::TAU) as f32;
  let initial_angle = std::f32::consts::FRAC_PI_2 + aggregate_sweep * 0.5;
  let mut angle = initial_angle;
  for (index, value) in primary {
    let sweep = (value / primary_total * std::f64::consts::TAU) as f32;
    items.push(radial_segment_path(
      primary_center,
      (primary_radius, primary_radius),
      0.0,
      (angle, sweep),
      (style.point_colors[index % style.point_colors.len()], 1.0),
      style.layout_profile != ChartLayoutProfile::Excel,
    ));
    angle += sweep;
  }

  if chart.kind == RadialChartKind::PieOfPie {
    // Excel gives the secondary pie the same starting angle as the primary
    // pie. This keeps the aggregate slice centered toward the secondary plot
    // and preserves the source point order in both pies.
    let mut angle = if style.layout_profile == ChartLayoutProfile::Excel {
      initial_angle
    } else {
      0.0
    };
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
        style.layout_profile != ChartLayoutProfile::Excel,
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
        width_pt: secondary_radius,
        height_pt: height,
        fill_color: Some(style.point_colors[index % style.point_colors.len()]),
        fill_opacity: 1.0,
        stroke: None,
        stroke_opacity: 1.0,
      }));
      y += height;
    }
  }
  // c:serLines is optional. Preserve the historical connector fallback for
  // Word and PowerPoint, but do not invent connector lines for Excel
  // of-pie charts when the source does not provide them.
  if style.layout_profile != ChartLayoutProfile::Excel {
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
}

fn radial_segment_path(
  center: (f32, f32),
  radii: (f32, f32),
  hole_ratio: f32,
  angles: (f32, f32),
  paint: (RgbColor, f32),
  stroke_outline: bool,
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
    stroke: stroke_outline.then(|| crate::common::Stroke {
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
  let host_defaults = radial_host_defaults(style.layout_profile);
  let marker = style.label.font_size_pt * host_defaults.legend_marker_em;
  let gap = style.label.font_size_pt * host_defaults.legend_marker_gap_em;
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
    // A pie legend represents data points rather than series. Excel's
    // automatic horizontal row uses a compact point-entry gap; keeping the
    // cartesian one-em gap makes the first/last entries fan out around the
    // correct center.
    let entry_gap = style.label.font_size_pt * host_defaults.horizontal_legend_entry_gap_em;
    let widths = chart
      .visible_legend_indices
      .iter()
      .filter_map(|index| chart.categories.get(*index))
      .map(|text| marker + gap + metrics.measure_text(text, &style.label))
      .collect::<Vec<_>>();
    let total = widths.iter().sum::<f32>() + entry_gap * widths.len().saturating_sub(1) as f32;
    let mut x = frame.x_pt + (frame.width_pt - total) * 0.5;
    x += style.label.font_size_pt * host_defaults.horizontal_legend_center_offset_em;
    let y = if position == ChartLegendPosition::Top {
      frame.y_pt + horizontal_height * 0.2
    } else if style.layout_profile == ChartLayoutProfile::Excel {
      frame.y_pt + frame.height_pt
        - horizontal_height
          * if chart.title.is_some() {
            profiles::EXCEL_TITLED_BOTTOM_LEGEND_HEIGHT_SCALE
          } else {
            profiles::EXCEL_UNTITLED_BOTTOM_LEGEND_HEIGHT_SCALE
          }
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
    let side_inset = style.label.font_size_pt * 0.4;
    let x = if position == ChartLegendPosition::Left {
      frame.x_pt + side_inset
    } else {
      frame.x_pt + frame.width_pt - side_width + side_inset
    };
    let entry_step = line_height(&style.label) * host_defaults.side_legend_entry_step;
    let entry_count = chart.visible_legend_indices.len();
    let total_height =
      line_height(&style.label) + entry_step * entry_count.saturating_sub(1) as f32;
    let center_y = frame.y_pt
      + frame.height_pt * 0.5
      + style.label.font_size_pt * host_defaults.side_legend_center_offset_em;
    let mut y = center_y - total_height * 0.5;
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
      y += entry_step;
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
  let x_extent = linear_axis_scale(scatter_x_axis_values(chart), None, 10)
    .map(|scale| (scale.minimum, scale.maximum));
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
      lower_trendlines(
        items,
        chart,
        series,
        color,
        TrendlineGeometry {
          plot,
          scale,
          category_count,
          x_extent,
        },
      );
    }
  }
}

#[derive(Clone, Copy)]
struct TrendlineGeometry {
  plot: PlotRect,
  scale: crate::render::chart::LinearAxisScale,
  category_count: usize,
  x_extent: Option<(f64, f64)>,
}

fn lower_trendlines(
  items: &mut Vec<PageItem>,
  chart: &ClusteredColumnChart<'_>,
  series: &crate::render::chart::ClusteredColumnSeries<'_>,
  color: RgbColor,
  geometry: TrendlineGeometry,
) {
  let TrendlineGeometry {
    plot,
    scale,
    category_count,
    x_extent,
  } = geometry;
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
        category_value_x(chart, x_value as f32, category_count, plot)
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
    let x = category_point_x(chart, display_index, category_count, plot);
    if chart.has_high_low_lines {
      let minimum = values.iter().copied().fold(f64::INFINITY, f64::min);
      let maximum = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
      items.push(PageItem::Line(LineItem {
        x1_pt: x,
        y1_pt: value_y(minimum, scale, plot.top, plot.height),
        x2_pt: x,
        y2_pt: value_y(maximum, scale, plot.top, plot.height),
        width_pt: style.stroke_scale,
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
  if series.smooth == Some(true) {
    let mut runs = Vec::new();
    let mut run = Vec::new();
    let mut marker_points = Vec::new();
    for (index, value) in series.values.iter().enumerate() {
      let Some(value) = value else {
        if run.len() >= 2 {
          runs.push(std::mem::take(&mut run));
        } else {
          run.clear();
        }
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
        category_point_x(chart, display_index, category_count, plot),
        value_y(point_value, scale, plot.top, plot.height),
        index,
      );
      marker_points.push(point);
      run.push(point);
    }
    if run.len() >= 2 {
      runs.push(run);
    }
    if !series.line_hidden {
      for run in &runs {
        lower_natural_cubic_chart_line(
          items,
          run,
          color,
          series.line_width_pt.unwrap_or(1.5) * style.stroke_scale,
        );
      }
    }
    if let Some(marker) = chart_marker_size(series, if series.is_3d { 4.0 } else { 3.0 }) {
      for (x, y, index) in marker_points {
        lower_chart_marker(
          items,
          x,
          y,
          marker * style.stroke_scale,
          chart_point_color(style, series_index, index).unwrap_or(color),
          series.marker,
        );
      }
    }
    return;
  }
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
      let x = category_point_x(chart, display_index, category_count, plot);
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
      category_point_x(chart, display_index, category_count, plot),
      value_y(point_value, scale, plot.top, plot.height),
    );
    if !series.line_hidden
      && let Some((previous_x, previous_y)) = previous
    {
      lower_chart_line_segment(
        items,
        (previous_x, previous_y),
        point,
        color,
        series.line_width_pt.unwrap_or(1.5) * style.stroke_scale,
        series.smooth.unwrap_or(false),
      );
    }
    if let Some(marker) = chart_marker_size(series, if series.is_3d { 4.0 } else { 3.0 }) {
      lower_chart_marker(
        items,
        point.0,
        point.1,
        marker * style.stroke_scale,
        chart_point_color(style, series_index, index).unwrap_or(color),
        series.marker,
      );
    }
    previous = Some(point);
  }
}

fn lower_natural_cubic_chart_line(
  items: &mut Vec<PageItem>,
  points: &[(f32, f32, usize)],
  color: RgbColor,
  width_pt: f32,
) {
  if points.len() < 2 {
    return;
  }

  // LibreOffice's CUBIC_SPLINES chart path uses a natural cubic spline:
  // endpoint second derivatives are zero and the interior derivatives are
  // solved as a tridiagonal system. Chart categories are monotonically
  // spaced, so the same formulation can be emitted directly as cubic Bézier
  // segments instead of sampling the curve into short straight lines.
  let count = points.len();
  let mut second = vec![0.0_f32; count];
  let mut upper = vec![0.0_f32; count];
  let mut rhs = vec![0.0_f32; count];
  for index in 1..count - 1 {
    let h0 = points[index].0 - points[index - 1].0;
    let h1 = points[index + 1].0 - points[index].0;
    if h0 <= f32::EPSILON || h1 <= f32::EPSILON {
      return;
    }
    let lower = h0;
    let diagonal = 2.0 * (h0 + h1);
    let prior = if index == 1 { 0.0 } else { upper[index - 1] };
    let divisor = diagonal - lower * prior;
    upper[index] = h1 / divisor;
    rhs[index] = (6.0
      * ((points[index + 1].1 - points[index].1) / h1
        - (points[index].1 - points[index - 1].1) / h0)
      - lower * rhs[index - 1])
      / divisor;
  }
  for index in (1..count - 1).rev() {
    second[index] = rhs[index] - upper[index] * second[index + 1];
  }

  let mut commands = Vec::with_capacity(count);
  commands.push(crate::common::PathCommand::MoveTo(common_point(
    points[0].0,
    points[0].1,
  )));
  let mut left = points[0].0;
  let mut top = points[0].1;
  let mut right = points[0].0;
  let mut bottom = points[0].1;
  for index in 0..count - 1 {
    let start = points[index];
    let end = points[index + 1];
    let h = end.0 - start.0;
    let start_derivative =
      (end.1 - start.1) / h - h * (2.0 * second[index] + second[index + 1]) / 6.0;
    let end_derivative =
      (end.1 - start.1) / h + h * (second[index] + 2.0 * second[index + 1]) / 6.0;
    let control1 = common_point(start.0 + h / 3.0, start.1 + start_derivative * h / 3.0);
    let control2 = common_point(end.0 - h / 3.0, end.1 - end_derivative * h / 3.0);
    let end = common_point(end.0, end.1);
    for point in [control1, control2, end] {
      left = left.min(point.x.0);
      top = top.min(point.y.0);
      right = right.max(point.x.0);
      bottom = bottom.max(point.y.0);
    }
    commands.push(crate::common::PathCommand::CubicTo {
      control1,
      control2,
      end,
    });
  }
  items.push(PageItem::Path(crate::common::PathItem {
    bounds: common_rect(left, top, right - left, bottom - top),
    points: Vec::new(),
    commands,
    closed: false,
    fill: crate::common::Fill::None,
    stroke: Some(crate::common::Stroke {
      width: crate::common::Pt(width_pt),
      color: common_rgb(color, 1.0),
      dash: None,
      source_style_id: None,
    }),
  }));
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
    if !bubbles
      && !series.line_hidden
      && let Some((previous_x, previous_y)) = previous
    {
      lower_chart_line_segment(
        items,
        (previous_x, previous_y),
        (x, y),
        color,
        series.line_width_pt.unwrap_or(1.25) * style.stroke_scale,
        series.smooth.unwrap_or(false),
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
      size * style.stroke_scale
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
        size * style.stroke_scale,
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
  let values = scatter_x_axis_values(chart);
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
    let width = metrics.measure_text(&label, &style.category_label);
    push_text(
      items,
      x - width * 0.5,
      plot.top
        + plot.height
        + style.category_label.font_size_pt
          * if style.layout_profile == ChartLayoutProfile::Excel
            && scatter_uses_index_x_values(chart)
          {
            0.70
          } else {
            0.25
          },
      label,
      style.category_label.clone(),
    );
  }
}

fn scatter_uses_index_x_values(chart: &ClusteredColumnChart<'_>) -> bool {
  let scatter_series = chart.series.iter().filter(|series| {
    matches!(
      series.kind,
      ChartSeriesKind::Scatter | ChartSeriesKind::Bubble
    )
  });
  let mut has_x_slots = false;
  for series in scatter_series {
    has_x_slots |= !series.x_values.is_empty();
    if series.x_values.iter().any(Option::is_some) {
      return false;
    }
  }
  has_x_slots
}

fn scatter_x_axis_values(chart: &ClusteredColumnChart<'_>) -> Vec<f64> {
  chart
    .series
    .iter()
    .filter(|series| {
      matches!(
        series.kind,
        ChartSeriesKind::Scatter | ChartSeriesKind::Bubble
      )
    })
    .flat_map(|series| {
      let point_count = series.x_values.len().max(series.values.len());
      (0..point_count).map(|index| {
        series
          .x_values
          .get(index)
          .copied()
          .flatten()
          .unwrap_or(index as f64 + 1.0)
      })
    })
    .filter(|value| value.is_finite())
    .collect()
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
      let extent = linear_axis_scale(scatter_x_axis_values(chart), None, 10)
        .map(|scale| (scale.minimum, scale.maximum))
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
      let x = category_point_x(chart, display_index, category_count, plot);
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

fn category_point_x(
  chart: &ClusteredColumnChart<'_>,
  display_index: usize,
  category_count: usize,
  plot: PlotRect,
) -> f32 {
  category_value_x(chart, display_index as f32 + 1.0, category_count, plot)
}

fn category_value_x(
  chart: &ClusteredColumnChart<'_>,
  one_based_value: f32,
  category_count: usize,
  plot: PlotRect,
) -> f32 {
  let ratio = if chart.category_axis_shifted || category_count <= 1 {
    (one_based_value - 0.5) / category_count.max(1) as f32
  } else {
    (one_based_value - 1.0) / (category_count - 1) as f32
  };
  plot.left + ratio * plot.width
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
    let legend_style = style.label.clone();
    let name_width = metrics.measure_text(&series.name, &legend_style);
    push_text(
      items,
      bounds.left - name_width - style.label.font_size_pt * 0.45,
      y + (row_height - line_height(&style.label)) * 0.5,
      series.name.clone(),
      legend_style,
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
  let marker_gap = style.label.font_size_pt * profiles::CARTESIAN_LEGEND_MARKER_GAP_EM;
  let titled_indexed_scatter = style.layout_profile == ChartLayoutProfile::Excel
    && !chart.title_overlay
    && matches!(chart.title.as_ref(), Some(ChartTitleText::Automatic))
    && scatter_uses_index_x_values(chart);
  let explicit_bottom_column = excel_explicit_bottom_column_layout(chart, style);
  let untitled_bottom_column = excel_untitled_bottom_column_layout(chart, style);
  let untitled_bottom_line_no_marker = excel_untitled_bottom_line_no_marker_layout(chart, style);
  let legend_profile = if titled_indexed_scatter {
    profiles::EXCEL_TITLED_INDEXED_SCATTER_LEGEND
  } else if explicit_bottom_column {
    profiles::EXCEL_EXPLICIT_BOTTOM_COLUMN_LEGEND
  } else if untitled_bottom_column {
    profiles::EXCEL_UNTITLED_BOTTOM_COLUMN_LEGEND
  } else if untitled_bottom_line_no_marker {
    profiles::EXCEL_UNTITLED_BOTTOM_LINE_LEGEND
  } else {
    profiles::DEFAULT_HORIZONTAL_CARTESIAN_LEGEND
  };
  let base_entry_gap =
    style.label.font_size_pt * profiles::DEFAULT_HORIZONTAL_CARTESIAN_LEGEND.entry_gap_em;
  let entry_gap = style.label.font_size_pt * legend_profile.entry_gap_em;
  let widths: Vec<f32> = chart
    .visible_legend_indices
    .iter()
    .filter_map(|index| chart.series.get(*index))
    .map(|series| {
      let legend_style = style.label.clone();
      horizontal_legend_key_width(series, style)
        + marker_gap
        + metrics.measure_text(&series.name, &legend_style)
    })
    .collect();
  let total_width = widths.iter().sum::<f32>()
    + entry_gap * chart.visible_legend_indices.len().saturating_sub(1) as f32;
  // The value-axis label band has already consumed the leading side of the
  // diagram. Center a bottom legend in the remaining horizontal region, as
  // Office does, rather than recentering it over the full graphic frame.
  let available_right = frame.x_pt + frame.width_pt;
  let mut x = available_left + (available_right - available_left - total_width) / 2.0;
  if titled_indexed_scatter {
    x += (entry_gap - base_entry_gap) * chart.visible_legend_indices.len().saturating_sub(1) as f32
      / 2.0;
  }
  x += frame.height_pt * legend_profile.x_offset_height_ratio;
  let y = y + frame.height_pt * legend_profile.y_offset_height_ratio;
  for (index, width) in chart.visible_legend_indices.iter().copied().zip(widths) {
    let Some(series) = chart.series.get(index) else {
      continue;
    };
    let key_width = horizontal_legend_key_width(series, style);
    if let Some(color) = style.series_colors.get(index).copied() {
      if series.kind == ChartSeriesKind::Line {
        items.push(PageItem::Line(LineItem {
          x1_pt: x,
          y1_pt: y + line_height(&style.label) * 0.5,
          x2_pt: x + key_width,
          y2_pt: y + line_height(&style.label) * 0.5,
          width_pt: 1.5,
          color,
          kind: LineItemKind::Stroke,
        }));
      } else {
        items.push(PageItem::Rect(RectItem {
          x_pt: x,
          y_pt: y + (line_height(&style.label) - key_width) / 2.0,
          width_pt: key_width,
          height_pt: key_width,
          fill_color: Some(color),
          fill_opacity: 1.0,
          stroke: None,
          stroke_opacity: 1.0,
        }));
      }
    }
    let legend_style = style.label.clone();
    push_text(
      items,
      x + key_width + marker_gap,
      y,
      series.name.clone(),
      legend_style,
    );
    x += width + entry_gap;
  }
}

fn horizontal_legend_key_width(
  series: &crate::render::chart::ClusteredColumnSeries<'_>,
  style: &ClusteredColumnStyle,
) -> f32 {
  style.label.font_size_pt
    * if series.kind == ChartSeriesKind::Line {
      profiles::CARTESIAN_LINE_LEGEND_KEY_WIDTH_EM
    } else {
      0.55
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
    .map(|series| {
      let legend_style = style.label.clone();
      marker_size + marker_gap + metrics.measure_text(&series.name, &legend_style)
    })
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
      profiles::OFFICE_VERTICAL_LEGEND_ENTRY_GAP_EM
    } else {
      profiles::POWERPOINT_VERTICAL_LEGEND_ENTRY_GAP_EM
    };
  let line_height = line_height(&style.label);
  let total_height = line_height * chart.visible_legend_indices.len() as f32
    + entry_gap * chart.visible_legend_indices.len().saturating_sub(1) as f32;
  let mut y = if align_top {
    frame.y_pt + frame.height_pt * 0.04
  } else {
    frame.y_pt + (frame.height_pt - total_height) / 2.0
  };
  if style.layout_profile == ChartLayoutProfile::Excel && style.has_explicit_title && !align_top {
    y += frame.height_pt
      * if excel_explicit_single_series_side_title_layout(chart, style) {
        profiles::EXCEL_EXPLICIT_SINGLE_SERIES_LEGEND_Y_RATIO
      } else {
        profiles::EXCEL_EXPLICIT_TITLE_LEGEND_Y_RATIO
      };
  } else if excel_derived_single_series_side_title_layout(chart, style) && !align_top {
    y += frame.height_pt * profiles::EXCEL_DERIVED_TITLE_LEGEND_Y_RATIO;
  } else if style.layout_profile == ChartLayoutProfile::Excel
    && (chart.title.is_none()
      || (chart.title_overlay && matches!(chart.title.as_ref(), Some(ChartTitleText::Automatic))))
    && (chart.has_automatic_title_marker
      || (chart.title_overlay && matches!(chart.title.as_ref(), Some(ChartTitleText::Automatic)))
      || (chart.title.is_none() && scatter_uses_index_x_values(chart)))
    && !align_top
  {
    y += frame.height_pt
      * if chart.title.is_none() && scatter_uses_index_x_values(chart) {
        if chart.series.iter().any(|series| {
          series
            .data_labels
            .iter()
            .any(|label| label.text_components.len() > 1)
        }) {
          profiles::EXCEL_INDEXED_SCATTER_MULTICOMPONENT_LEGEND_Y_RATIO
        } else {
          0.0
        }
      } else {
        if chart.title.is_none() && chart.has_explicit_categories {
          profiles::EXCEL_EXPLICIT_CATEGORY_LEGEND_Y_RATIO
        } else {
          profiles::EXCEL_AUTOMATIC_UNTITLED_LEGEND_Y_RATIO
        }
      };
  } else if style.layout_profile == ChartLayoutProfile::Excel && !align_top {
    y += frame.height_pt * profiles::EXCEL_GENERIC_LEGEND_Y_RATIO;
  } else if style.layout_profile == ChartLayoutProfile::Word
    && style.has_explicit_title
    && !align_top
  {
    // Word centers a right/left automatic legend in its lower chart band,
    // rather than around the raw drawing frame used by PowerPoint.
    y += frame.height_pt * profiles::WORD_EXPLICIT_TITLE_LEGEND_Y_RATIO;
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
    let legend_style = style.label.clone();
    push_text(
      items,
      x + marker_size + marker_gap,
      y,
      series.name.clone(),
      legend_style,
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

fn push_data_label_text_components(
  items: &mut Vec<PageItem>,
  metrics: &mut TextMetrics,
  x: f32,
  y: f32,
  label: &crate::render::chart::ClusteredColumnDataLabel<'_>,
  style: &TextStyle,
) {
  if label.text_components.len() <= 1
    || label.separator.contains('\r')
    || label.separator.contains('\n')
  {
    push_text(items, x, y, label.text.clone(), style.clone());
    return;
  }

  let mut component_x = x;
  let painted_separator = label.separator.trim_end();
  for (index, component) in label.text_components.iter().enumerate() {
    let is_last = index + 1 == label.text_components.len();
    let painted_text = if is_last {
      component.clone()
    } else {
      format!("{component}{painted_separator}")
    };
    push_text(items, component_x, y, painted_text, style.clone());
    if !is_last {
      component_x += metrics.measure_text(&format!("{component}{}", label.separator), style);
    }
  }
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
