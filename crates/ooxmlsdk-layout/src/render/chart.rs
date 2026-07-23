use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_chart as c;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use crate::render::math::text_math_text;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChartKind {
  Pie,
  Bar,
  Area,
  Line,
  Scatter,
  Bubble,
  Radar,
  Stock,
  Surface,
  Other,
}

#[derive(Clone, Copy)]
pub struct ChartSeriesRef<'a> {
  pub series_text: Option<&'a c::SeriesText>,
  pub category_axis_data: Option<&'a c::CategoryAxisData>,
  pub values: Option<&'a c::Values>,
  pub y_values: Option<&'a c::YValues>,
  pub x_values: Option<&'a c::XValues>,
  pub bubble_size: Option<&'a c::BubbleSize>,
  pub data_labels: Option<&'a c::DataLabels>,
  pub chart_shape_properties: Option<&'a c::ChartShapeProperties>,
  pub data_points: &'a [c::DataPoint],
  pub marker: Option<&'a c::Marker>,
  pub smooth: Option<&'a c::Smooth>,
  pub trendlines: &'a [c::Trendline],
}

#[derive(Clone, Copy, Debug)]
pub struct ChartDataPointFill<'a> {
  pub index: u32,
  pub fill: &'a a::SolidFill,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ChartTitleText {
  Explicit(String),
  Automatic,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChartLegendPosition {
  Bottom,
  Top,
  Left,
  Right,
  TopRight,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChartSeriesKind {
  Column,
  Bar,
  Line,
  Area,
  Scatter,
  Bubble,
  Radar,
  Stock,
  Surface,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChartSeriesGrouping {
  Clustered,
  Standard,
  Stacked,
  PercentStacked,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ChartManualLayout {
  pub x: Option<f32>,
  pub y: Option<f32>,
  pub width: Option<f32>,
  pub height: Option<f32>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RadialChartKind {
  Pie,
  Pie3D,
  Doughnut,
  PieOfPie,
  BarOfPie,
}

#[derive(Clone, Debug)]
pub struct ClusteredColumnSeries<'a> {
  pub name: String,
  pub has_explicit_name: bool,
  pub values: Vec<Option<f64>>,
  pub number_format_code: Option<&'a str>,
  pub x_values: Vec<Option<f64>>,
  pub bubble_sizes: Vec<Option<f64>>,
  pub solid_fill: Option<&'a a::SolidFill>,
  pub data_point_fills: Vec<ChartDataPointFill<'a>>,
  pub data_labels: Vec<ClusteredColumnDataLabel>,
  pub kind: ChartSeriesKind,
  pub grouping: ChartSeriesGrouping,
  pub is_3d: bool,
  pub smooth: bool,
  pub marker: Option<&'a c::Marker>,
  pub filled_area: bool,
  pub trendlines: &'a [c::Trendline],
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClusteredColumnDataLabel {
  pub point_index: usize,
  pub text: String,
  pub position: c::DataLabelPositionValues,
}

#[derive(Clone, Debug)]
pub struct ClusteredColumnChart<'a> {
  pub title: Option<ChartTitleText>,
  pub category_axis_title: Option<String>,
  pub value_axis_title: Option<String>,
  pub additional_axis_titles: Vec<String>,
  pub categories: Vec<String>,
  pub series: Vec<ClusteredColumnSeries<'a>>,
  pub gap_width_percent: f64,
  pub overlap_percent: f64,
  pub category_axis: Option<&'a c::CategoryAxis>,
  pub date_axis: Option<&'a c::DateAxis>,
  pub category_axis_reversed: bool,
  pub value_axis: Option<&'a c::ValueAxis>,
  pub legend_position: Option<ChartLegendPosition>,
  pub legend_overlay: bool,
  pub visible_legend_indices: Vec<usize>,
  pub legend_layout: Option<ChartManualLayout>,
  pub plot_layout: Option<ChartManualLayout>,
  pub data_table: Option<&'a c::DataTable>,
  pub data_label_text_properties: Option<&'a c::TextProperties>,
  pub has_high_low_lines: bool,
  pub has_up_down_bars: bool,
}

#[derive(Clone, Debug)]
pub struct PieChartModel<'a> {
  pub kind: RadialChartKind,
  pub title: Option<ChartTitleText>,
  pub series_name: String,
  pub categories: Vec<String>,
  pub values: Vec<Option<f64>>,
  pub series_solid_fill: Option<&'a a::SolidFill>,
  pub data_point_fills: Vec<ChartDataPointFill<'a>>,
  pub first_slice_angle_deg: f64,
  pub hole_size_percent: f64,
  pub series_explosion_percent: f64,
  pub point_explosion_percent: Vec<f64>,
  pub secondary_indices: Vec<usize>,
  pub secondary_size_percent: f64,
  pub vary_colors: bool,
  pub legend_position: Option<ChartLegendPosition>,
  pub legend_overlay: bool,
  pub visible_legend_indices: Vec<usize>,
  pub legend_layout: Option<ChartManualLayout>,
  pub plot_layout: Option<ChartManualLayout>,
  pub data_labels: Vec<ClusteredColumnDataLabel>,
  pub data_label_text_properties: Option<&'a c::TextProperties>,
  pub show_leader_lines: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LinearAxisScale {
  pub minimum: f64,
  pub maximum: f64,
  pub major_unit: f64,
  pub logarithmic_base: Option<f64>,
  pub reversed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClusteredColumnSlot {
  pub center: f64,
  pub width: f64,
}

pub fn automatic_chart_title(ui_language: Option<&str>) -> &'static str {
  let language = ui_language.unwrap_or("en").to_ascii_lowercase();
  if language == "zh-tw" || language == "zh-hk" || language == "zh-mo" || language == "zh-hant" {
    "圖表標題"
  } else if language == "zh" || language == "zh-cn" || language == "zh-sg" || language == "zh-hans"
  {
    "图表标题"
  } else {
    "Chart Title"
  }
}

pub fn automatic_series_title(ui_language: Option<&str>, series_index: usize) -> String {
  let language = ui_language.unwrap_or("en").to_ascii_lowercase();
  if language == "zh-tw" || language == "zh-hk" || language == "zh-mo" || language == "zh-hant" {
    format!("數列 {series_index}")
  } else if language == "zh" || language == "zh-cn" || language == "zh-sg" || language == "zh-hans"
  {
    format!("系列 {series_index}")
  } else {
    format!("Series {series_index}")
  }
}

/// Extracts the first ordinary two-dimensional clustered column chart.
///
/// Cached category/value sequences are data sources, not inherently visible
/// text.  Keeping them in a typed chart model lets each renderer decide which
/// labels are visible from the chart/axis/data-label settings.
pub fn clustered_column_chart(chart_space: &c::ChartSpace) -> Option<ClusteredColumnChart<'_>> {
  clustered_column_chart_for_ui_language(chart_space, None)
}

pub fn clustered_column_chart_for_ui_language<'a>(
  chart_space: &'a c::ChartSpace,
  ui_language: Option<&str>,
) -> Option<ClusteredColumnChart<'a>> {
  let bar_chart = chart_space
    .chart
    .plot_area
    .plot_area_choice1
    .iter()
    .find_map(|choice| match choice {
      c::PlotAreaChoice::BarChart(chart)
        if chart.bar_direction.val == c::BarDirectionValues::Column
          && chart
            .bar_grouping
            .as_ref()
            .and_then(|grouping| grouping.val)
            .unwrap_or(c::BarGroupingValues::Clustered)
            == c::BarGroupingValues::Clustered =>
      {
        Some(chart.as_ref())
      }
      _ => None,
    })?;

  let mut series = Vec::with_capacity(bar_chart.bar_chart_series.len());
  let mut categories = Vec::new();
  for (series_index, source) in bar_chart.bar_chart_series.iter().enumerate() {
    let series_ref = bar_series_ref(source);
    let name = series_ref
      .series_text
      .map(series_text_value)
      .filter(|value| !value.is_empty())
      .unwrap_or_else(|| default_series_label(series_ref, series_index + 1, ui_language));
    let source_categories = source
      .category_axis_data
      .as_deref()
      .map(indexed_category_axis_text_values)
      .unwrap_or_default();
    if categories.is_empty() && !source_categories.is_empty() {
      categories.clone_from(&source_categories);
    }
    let values = source
      .values
      .as_deref()
      .map(indexed_values)
      .unwrap_or_default();
    let solid_fill = source
      .chart_shape_properties
      .as_deref()
      .and_then(|properties| {
        chart_shape_solid_fill(properties).or_else(|| chart_shape_outline_solid_fill(properties))
      });
    let mut data_point_fills = Vec::new();
    collect_data_point_solid_fills(&source.data_point, &mut data_point_fills);
    data_point_fills.sort_by_key(|fill| fill.index);
    let label_categories = if source_categories.is_empty() {
      (1..=values.len()).map(|index| index.to_string()).collect()
    } else {
      source_categories
    };
    let data_labels = resolved_data_labels(
      source.data_labels.as_deref(),
      bar_chart.data_labels.as_deref(),
      &name,
      &label_categories,
      &values,
      None,
      DataLabelDefaults {
        value_format_code: series_number_format_code(series_ref),
        position: c::DataLabelPositionValues::OutsideEnd,
        supports_percent: false,
        separator: "; ",
      },
    );
    series.push(ClusteredColumnSeries {
      name,
      has_explicit_name: series_ref.series_text.is_some(),
      values,
      number_format_code: series_number_format_code(series_ref),
      x_values: Vec::new(),
      bubble_sizes: Vec::new(),
      solid_fill,
      data_point_fills,
      data_labels,
      kind: ChartSeriesKind::Column,
      grouping: ChartSeriesGrouping::Clustered,
      is_3d: false,
      smooth: false,
      marker: None,
      filled_area: false,
      trendlines: &[],
    });
  }

  if categories.is_empty()
    && series.iter().any(|series| {
      !matches!(
        series.kind,
        ChartSeriesKind::Scatter | ChartSeriesKind::Bubble
      )
    })
  {
    let category_count = series
      .iter()
      .map(|series| series.values.len())
      .max()
      .unwrap_or(0);
    // LibreOffice VCartesianAxis::getTextLabelString treats a category axis
    // without an explicit category sequence as a numeric axis. Its first
    // category is tick value 1.0, so the visible labels are 1, 2, ... rather
    // than cached series values or an empty label band.
    categories = (1..=category_count)
      .map(|index| index.to_string())
      .collect();
  }

  let title = chart_title_text(&chart_space.chart);
  let value_axes = chart_space
    .chart
    .plot_area
    .plot_area_choice2
    .iter()
    .filter_map(|choice| match choice {
      c::PlotAreaChoice2::ValueAxis(axis) => Some(axis.as_ref()),
      _ => None,
    })
    .collect::<Vec<_>>();
  let value_axis = value_axes.first().copied();
  let category_axis = chart_space
    .chart
    .plot_area
    .plot_area_choice2
    .iter()
    .find_map(|choice| match choice {
      c::PlotAreaChoice2::CategoryAxis(axis) => Some(axis.as_ref()),
      _ => None,
    });
  let date_axis = chart_space
    .chart
    .plot_area
    .plot_area_choice2
    .iter()
    .find_map(|choice| match choice {
      c::PlotAreaChoice2::DateAxis(axis) => Some(axis.as_ref()),
      _ => None,
    });
  let legend_position = chart_space.chart.legend.as_deref().map(|legend| {
    match legend
      .legend_position
      .as_ref()
      .and_then(|position| position.val)
      .unwrap_or(c::LegendPositionValues::Right)
    {
      c::LegendPositionValues::Bottom => ChartLegendPosition::Bottom,
      c::LegendPositionValues::Top => ChartLegendPosition::Top,
      c::LegendPositionValues::Left => ChartLegendPosition::Left,
      c::LegendPositionValues::Right => ChartLegendPosition::Right,
      c::LegendPositionValues::TopRight => ChartLegendPosition::TopRight,
    }
  });

  Some(ClusteredColumnChart {
    title,
    category_axis_title: category_axis
      .and_then(|axis| axis.title.as_deref())
      .and_then(|title| title_text_or_automatic(title, ui_language)),
    value_axis_title: value_axes
      .first()
      .copied()
      .and_then(|axis| axis.title.as_deref())
      .and_then(|title| title_text_or_automatic(title, ui_language)),
    additional_axis_titles: Vec::new(),
    categories,
    series,
    gap_width_percent: f64::from(
      bar_chart
        .gap_width
        .as_ref()
        .and_then(|gap| gap.val)
        .unwrap_or(150),
    ),
    overlap_percent: f64::from(
      bar_chart
        .overlap
        .as_ref()
        .and_then(|overlap| overlap.val)
        .unwrap_or(0),
    ),
    category_axis,
    date_axis,
    category_axis_reversed: category_axis
      .and_then(|axis| axis.scaling.orientation.as_ref())
      .or_else(|| date_axis.and_then(|axis| axis.scaling.orientation.as_ref()))
      .and_then(|orientation| orientation.val)
      == Some(c::OrientationValues::MaxMin),
    value_axis,
    legend_position,
    legend_overlay: chart_space
      .chart
      .legend
      .as_deref()
      .and_then(|legend| legend.overlay.as_ref())
      .is_some_and(|overlay| overlay.val.is_none_or(|value| value.as_bool())),
    visible_legend_indices: visible_series_legend_indices(
      chart_space.chart.legend.as_deref(),
      bar_chart.bar_chart_series.len(),
    ),
    legend_layout: chart_space
      .chart
      .legend
      .as_deref()
      .and_then(|legend| chart_layout(legend.layout.as_deref())),
    plot_layout: chart_layout(chart_space.chart.plot_area.layout.as_deref()),
    data_table: chart_space.chart.plot_area.data_table.as_deref(),
    data_label_text_properties: chart_data_label_text_properties(chart_space),
    has_high_low_lines: false,
    has_up_down_bars: false,
  })
}

/// Extracts every cartesian/polar series group that can share the common
/// Office chart frame. Pie-family groups use [`pie_chart_model`] because their
/// category legends and radial geometry have different semantics.
pub fn cartesian_chart_for_ui_language<'a>(
  chart_space: &'a c::ChartSpace,
  ui_language: Option<&str>,
) -> Option<ClusteredColumnChart<'a>> {
  let mut series = Vec::new();
  let mut categories = Vec::new();
  let mut gap_width_percent = 150.0;
  let mut overlap_percent = 0.0;
  let mut has_high_low_lines = false;
  let mut has_up_down_bars = false;

  for choice in &chart_space.chart.plot_area.plot_area_choice1 {
    match choice {
      c::PlotAreaChoice::AreaChart(chart) => append_cartesian_series(
        &mut series,
        &mut categories,
        chart.area_chart_series.iter().map(area_series_ref),
        chart.data_labels.as_deref(),
        (
          ChartSeriesKind::Area,
          grouping(chart.grouping.as_ref()),
          false,
        ),
        ui_language,
      ),
      c::PlotAreaChoice::Area3DChart(chart) => append_cartesian_series(
        &mut series,
        &mut categories,
        chart.area_chart_series.iter().map(area_series_ref),
        chart.data_labels.as_deref(),
        (
          ChartSeriesKind::Area,
          grouping(chart.grouping.as_ref()),
          true,
        ),
        ui_language,
      ),
      c::PlotAreaChoice::LineChart(chart) => {
        has_high_low_lines |= chart.high_low_lines.is_some();
        has_up_down_bars |= chart.up_down_bars.is_some();
        append_cartesian_series(
          &mut series,
          &mut categories,
          chart.line_chart_series.iter().map(line_series_ref),
          chart.data_labels.as_deref(),
          (
            ChartSeriesKind::Line,
            grouping(chart.grouping.as_ref()),
            false,
          ),
          ui_language,
        );
      }
      c::PlotAreaChoice::Line3DChart(chart) => append_cartesian_series(
        &mut series,
        &mut categories,
        chart.line_chart_series.iter().map(line_series_ref),
        chart.data_labels.as_deref(),
        (ChartSeriesKind::Line, grouping(Some(&chart.grouping)), true),
        ui_language,
      ),
      c::PlotAreaChoice::StockChart(chart) => {
        has_high_low_lines |= chart.high_low_lines.is_some();
        has_up_down_bars |= chart.up_down_bars.is_some();
        append_cartesian_series(
          &mut series,
          &mut categories,
          chart.line_chart_series.iter().map(line_series_ref),
          chart.data_labels.as_deref(),
          (ChartSeriesKind::Stock, ChartSeriesGrouping::Standard, false),
          ui_language,
        );
      }
      c::PlotAreaChoice::RadarChart(chart) => {
        let first = series.len();
        append_cartesian_series(
          &mut series,
          &mut categories,
          chart.radar_chart_series.iter().map(radar_series_ref),
          chart.data_labels.as_deref(),
          (ChartSeriesKind::Radar, ChartSeriesGrouping::Standard, false),
          ui_language,
        );
        if chart.radar_style.val == c::RadarStyleValues::Filled {
          for series in &mut series[first..] {
            series.filled_area = true;
          }
        }
      }
      c::PlotAreaChoice::ScatterChart(chart) => append_cartesian_series(
        &mut series,
        &mut categories,
        chart.scatter_chart_series.iter().map(scatter_series_ref),
        chart.data_labels.as_deref(),
        (
          ChartSeriesKind::Scatter,
          ChartSeriesGrouping::Standard,
          false,
        ),
        ui_language,
      ),
      c::PlotAreaChoice::BarChart(chart) => {
        gap_width_percent = f64::from(
          chart
            .gap_width
            .as_ref()
            .and_then(|gap| gap.val)
            .unwrap_or(150),
        );
        overlap_percent = f64::from(
          chart
            .overlap
            .as_ref()
            .and_then(|overlap| overlap.val)
            .unwrap_or(0),
        );
        append_cartesian_series(
          &mut series,
          &mut categories,
          chart.bar_chart_series.iter().map(bar_series_ref),
          chart.data_labels.as_deref(),
          (
            if chart.bar_direction.val == c::BarDirectionValues::Bar {
              ChartSeriesKind::Bar
            } else {
              ChartSeriesKind::Column
            },
            bar_grouping(chart.bar_grouping.as_ref()),
            false,
          ),
          ui_language,
        );
      }
      c::PlotAreaChoice::Bar3DChart(chart) => {
        gap_width_percent = f64::from(
          chart
            .gap_width
            .as_ref()
            .and_then(|gap| gap.val)
            .unwrap_or(150),
        );
        append_cartesian_series(
          &mut series,
          &mut categories,
          chart.bar_chart_series.iter().map(bar_series_ref),
          chart.data_labels.as_deref(),
          (
            if chart.bar_direction.val == c::BarDirectionValues::Bar {
              ChartSeriesKind::Bar
            } else {
              ChartSeriesKind::Column
            },
            bar_grouping(chart.bar_grouping.as_ref()),
            true,
          ),
          ui_language,
        );
      }
      c::PlotAreaChoice::SurfaceChart(chart) => append_cartesian_series(
        &mut series,
        &mut categories,
        chart.surface_chart_series.iter().map(surface_series_ref),
        None,
        (
          ChartSeriesKind::Surface,
          ChartSeriesGrouping::Standard,
          false,
        ),
        ui_language,
      ),
      c::PlotAreaChoice::Surface3DChart(chart) => append_cartesian_series(
        &mut series,
        &mut categories,
        chart.surface_chart_series.iter().map(surface_series_ref),
        None,
        (
          ChartSeriesKind::Surface,
          ChartSeriesGrouping::Standard,
          true,
        ),
        ui_language,
      ),
      c::PlotAreaChoice::BubbleChart(chart) => append_cartesian_series(
        &mut series,
        &mut categories,
        chart.bubble_chart_series.iter().map(bubble_series_ref),
        chart.data_labels.as_deref(),
        (
          ChartSeriesKind::Bubble,
          ChartSeriesGrouping::Standard,
          false,
        ),
        ui_language,
      ),
      c::PlotAreaChoice::PieChart(_)
      | c::PlotAreaChoice::Pie3DChart(_)
      | c::PlotAreaChoice::DoughnutChart(_)
      | c::PlotAreaChoice::OfPieChart(_) => {}
    }
  }

  if series.is_empty() {
    return None;
  }
  if categories.is_empty()
    && series.iter().any(|series| {
      !matches!(
        series.kind,
        ChartSeriesKind::Scatter | ChartSeriesKind::Bubble
      )
    })
  {
    let category_count = series
      .iter()
      .map(|series| series.values.len())
      .max()
      .unwrap_or(0);
    categories = (1..=category_count)
      .map(|index| index.to_string())
      .collect();
  }

  let value_axes = chart_space
    .chart
    .plot_area
    .plot_area_choice2
    .iter()
    .filter_map(|choice| match choice {
      c::PlotAreaChoice2::ValueAxis(axis) => Some(axis.as_ref()),
      _ => None,
    })
    .collect::<Vec<_>>();
  let value_axis = value_axes.first().copied();
  let category_axis = chart_space
    .chart
    .plot_area
    .plot_area_choice2
    .iter()
    .find_map(|choice| match choice {
      c::PlotAreaChoice2::CategoryAxis(axis) => Some(axis.as_ref()),
      _ => None,
    });
  let date_axis = chart_space
    .chart
    .plot_area
    .plot_area_choice2
    .iter()
    .find_map(|choice| match choice {
      c::PlotAreaChoice2::DateAxis(axis) => Some(axis.as_ref()),
      _ => None,
    });
  let mut additional_axis_titles = Vec::new();
  let mut seen_category_title = false;
  let mut seen_value_title = false;
  for choice in &chart_space.chart.plot_area.plot_area_choice2 {
    let (is_value, title) = match choice {
      c::PlotAreaChoice2::ValueAxis(axis) => (true, axis.title.as_deref()),
      c::PlotAreaChoice2::CategoryAxis(axis) => (false, axis.title.as_deref()),
      c::PlotAreaChoice2::DateAxis(axis) => (false, axis.title.as_deref()),
      c::PlotAreaChoice2::SeriesAxis(axis) => (false, axis.title.as_deref()),
    };
    let Some(title) = title.and_then(|title| title_text_or_automatic(title, ui_language)) else {
      continue;
    };
    if is_value && !seen_value_title {
      seen_value_title = true;
    } else if !is_value && !seen_category_title {
      seen_category_title = true;
    } else {
      additional_axis_titles.push(title);
    }
  }
  let series_count = series.len();

  Some(ClusteredColumnChart {
    title: chart_title_text(&chart_space.chart),
    category_axis_title: category_axis
      .and_then(|axis| axis.title.as_deref())
      .and_then(|title| title_text_or_automatic(title, ui_language))
      .or_else(|| {
        chart_space
          .chart
          .plot_area
          .plot_area_choice2
          .iter()
          .find_map(|choice| match choice {
            c::PlotAreaChoice2::DateAxis(axis) => axis
              .title
              .as_deref()
              .and_then(|title| title_text_or_automatic(title, ui_language)),
            _ => None,
          })
      }),
    value_axis_title: value_axes
      .first()
      .copied()
      .and_then(|axis| axis.title.as_deref())
      .and_then(|title| title_text_or_automatic(title, ui_language)),
    additional_axis_titles,
    categories,
    series,
    gap_width_percent,
    overlap_percent,
    category_axis,
    date_axis,
    category_axis_reversed: category_axis
      .and_then(|axis| axis.scaling.orientation.as_ref())
      .or_else(|| date_axis.and_then(|axis| axis.scaling.orientation.as_ref()))
      .and_then(|orientation| orientation.val)
      == Some(c::OrientationValues::MaxMin),
    value_axis,
    legend_position: chart_space
      .chart
      .legend
      .as_deref()
      .map(chart_legend_position),
    legend_overlay: chart_space
      .chart
      .legend
      .as_deref()
      .and_then(|legend| legend.overlay.as_ref())
      .is_some_and(|overlay| overlay.val.is_none_or(|value| value.as_bool())),
    visible_legend_indices: visible_series_legend_indices(
      chart_space.chart.legend.as_deref(),
      series_count,
    ),
    legend_layout: chart_space
      .chart
      .legend
      .as_deref()
      .and_then(|legend| chart_layout(legend.layout.as_deref())),
    plot_layout: chart_layout(chart_space.chart.plot_area.layout.as_deref()),
    data_table: chart_space.chart.plot_area.data_table.as_deref(),
    data_label_text_properties: chart_data_label_text_properties(chart_space),
    has_high_low_lines,
    has_up_down_bars,
  })
}

fn append_cartesian_series<'a>(
  target: &mut Vec<ClusteredColumnSeries<'a>>,
  categories: &mut Vec<String>,
  sources: impl Iterator<Item = ChartSeriesRef<'a>>,
  chart_group_labels: Option<&'a c::DataLabels>,
  series_spec: (ChartSeriesKind, ChartSeriesGrouping, bool),
  ui_language: Option<&str>,
) {
  let (kind, grouping, is_3d) = series_spec;
  for source in sources {
    let series_index = target.len() + 1;
    let name = source
      .series_text
      .map(series_text_value)
      .filter(|value| !value.is_empty())
      .unwrap_or_else(|| default_series_label(source, series_index, ui_language));
    let source_categories = source
      .category_axis_data
      .map(indexed_category_axis_text_values)
      .unwrap_or_default();
    if categories.is_empty() && !source_categories.is_empty() {
      categories.clone_from(&source_categories);
    }
    let values = chart_series_numeric_values(source);
    let label_categories = if source_categories.is_empty() {
      (1..=values.len()).map(|index| index.to_string()).collect()
    } else {
      source_categories
    };
    let mut data_point_fills = Vec::new();
    collect_data_point_solid_fills(source.data_points, &mut data_point_fills);
    data_point_fills.sort_by_key(|fill| fill.index);
    let bubble_sizes = source
      .bubble_size
      .map(indexed_bubble_size_values)
      .unwrap_or_default();
    target.push(ClusteredColumnSeries {
      name: name.clone(),
      has_explicit_name: source.series_text.is_some(),
      x_values: chart_series_x_numeric_values(source),
      bubble_sizes: bubble_sizes.clone(),
      solid_fill: source.chart_shape_properties.and_then(|properties| {
        chart_shape_solid_fill(properties).or_else(|| chart_shape_outline_solid_fill(properties))
      }),
      data_point_fills,
      data_labels: resolved_data_labels(
        source.data_labels,
        chart_group_labels,
        &name,
        &label_categories,
        &values,
        (!bubble_sizes.is_empty()).then_some(bubble_sizes.as_slice()),
        DataLabelDefaults {
          value_format_code: series_number_format_code(source),
          position: default_data_label_position(kind),
          supports_percent: false,
          separator: "; ",
        },
      ),
      values,
      number_format_code: series_number_format_code(source),
      kind,
      grouping,
      is_3d,
      smooth: source
        .smooth
        .is_some_and(|smooth| smooth.val.is_none_or(|value| value.as_bool())),
      marker: source.marker,
      filled_area: false,
      trendlines: source.trendlines,
    });
  }
}

fn grouping(grouping: Option<&c::Grouping>) -> ChartSeriesGrouping {
  match grouping.and_then(|grouping| grouping.val) {
    Some(c::GroupingValues::Stacked) => ChartSeriesGrouping::Stacked,
    Some(c::GroupingValues::PercentStacked) => ChartSeriesGrouping::PercentStacked,
    Some(c::GroupingValues::Standard) | None => ChartSeriesGrouping::Standard,
  }
}

fn bar_grouping(grouping: Option<&c::BarGrouping>) -> ChartSeriesGrouping {
  match grouping.and_then(|grouping| grouping.val) {
    Some(c::BarGroupingValues::Stacked) => ChartSeriesGrouping::Stacked,
    Some(c::BarGroupingValues::PercentStacked) => ChartSeriesGrouping::PercentStacked,
    Some(c::BarGroupingValues::Standard) => ChartSeriesGrouping::Standard,
    Some(c::BarGroupingValues::Clustered) | None => ChartSeriesGrouping::Clustered,
  }
}

fn chart_legend_position(legend: &c::Legend) -> ChartLegendPosition {
  match legend
    .legend_position
    .as_ref()
    .and_then(|position| position.val)
    .unwrap_or(c::LegendPositionValues::Right)
  {
    c::LegendPositionValues::Bottom => ChartLegendPosition::Bottom,
    c::LegendPositionValues::Top => ChartLegendPosition::Top,
    c::LegendPositionValues::Left => ChartLegendPosition::Left,
    c::LegendPositionValues::Right => ChartLegendPosition::Right,
    c::LegendPositionValues::TopRight => ChartLegendPosition::TopRight,
  }
}

fn default_data_label_position(kind: ChartSeriesKind) -> c::DataLabelPositionValues {
  match kind {
    ChartSeriesKind::Column | ChartSeriesKind::Bar => c::DataLabelPositionValues::OutsideEnd,
    ChartSeriesKind::Area | ChartSeriesKind::Surface => c::DataLabelPositionValues::Center,
    ChartSeriesKind::Line
    | ChartSeriesKind::Scatter
    | ChartSeriesKind::Bubble
    | ChartSeriesKind::Radar
    | ChartSeriesKind::Stock => c::DataLabelPositionValues::Right,
  }
}

fn visible_series_legend_indices(legend: Option<&c::Legend>, count: usize) -> Vec<usize> {
  let mut indices = (0..count).collect::<Vec<_>>();
  if let Some(legend) = legend {
    indices.retain(|index| {
      !legend.legend_entry.iter().any(|entry| {
        entry.index.val as usize == *index
          && matches!(
            entry.legend_entry_choice.as_ref(),
            Some(c::LegendEntryChoice::Delete(delete))
              if delete.val.is_none_or(|value| value.as_bool())
          )
      })
    });
  }
  indices
}

fn chart_layout(layout: Option<&c::Layout>) -> Option<ChartManualLayout> {
  let manual = layout?.manual_layout.as_deref()?;
  Some(ChartManualLayout {
    x: manual.left.as_ref().map(|value| value.val as f32),
    y: manual.top.as_ref().map(|value| value.val as f32),
    width: manual.width.as_ref().map(|value| value.val as f32),
    height: manual.height.as_ref().map(|value| value.val as f32),
  })
}

/// Returns the clustered-column subset whose complete plot and chart-area
/// semantics are handled by the shared lowerer.
///
/// Combination charts, data tables, and chart-area fills require additional
/// visible objects; selecting only their bar series would silently discard
/// source content.
pub fn ordinary_clustered_column_chart(
  chart_space: &c::ChartSpace,
) -> Option<ClusteredColumnChart<'_>> {
  if chart_space
    .shape_properties
    .as_deref()
    .is_some_and(shape_properties_are_visible)
    || chart_space
      .chart
      .plot_area
      .shape_properties
      .as_deref()
      .is_some_and(shape_properties_are_visible)
    || chart_space.chart.plot_area.data_table.is_some()
    || chart_space.chart.plot_area.plot_area_choice1.len() != 1
  {
    return None;
  }
  clustered_column_chart(chart_space)
}

/// Extracts the radial pie-family semantics displayed by Office.
///
/// ECMA-376 permits more than one `c:ser` in `c:pieChart`, but MS-OI29500
/// §21.2.2.141 specifies that Office displays only the first series. The same
/// implementation note specifies that `c:varyColors` is ignored when multiple
/// series are present. Keeping those Office rules here prevents fixed-output
/// renderers from merging cached series that are not visible.
pub fn pie_chart_model(chart_space: &c::ChartSpace) -> Option<PieChartModel<'_>> {
  let (
    radial_kind,
    pie_series,
    chart_group_labels,
    first_slice_angle_deg,
    hole_size_percent,
    vary_colors,
    of_pie,
  ) = chart_space
    .chart
    .plot_area
    .plot_area_choice1
    .iter()
    .find_map(|choice| match choice {
      c::PlotAreaChoice::PieChart(chart) => Some((
        RadialChartKind::Pie,
        chart.pie_chart_series.as_slice(),
        chart.data_labels.as_deref(),
        chart
          .first_slice_angle
          .as_ref()
          .and_then(|angle| angle.val)
          .map_or(0.0, f64::from),
        0.0,
        chart.vary_colors.as_ref(),
        None,
      )),
      c::PlotAreaChoice::Pie3DChart(chart) => Some((
        RadialChartKind::Pie3D,
        chart.pie_chart_series.as_slice(),
        chart.data_labels.as_deref(),
        0.0,
        0.0,
        chart.vary_colors.as_ref(),
        None,
      )),
      c::PlotAreaChoice::DoughnutChart(chart) => Some((
        RadialChartKind::Doughnut,
        chart.pie_chart_series.as_slice(),
        chart.data_labels.as_deref(),
        chart
          .first_slice_angle
          .as_ref()
          .and_then(|angle| angle.val)
          .map_or(0.0, f64::from),
        f64::from(chart.hole_size.val),
        chart.vary_colors.as_ref(),
        None,
      )),
      c::PlotAreaChoice::OfPieChart(chart) => Some((
        if chart.of_pie_type.val == c::OfPieValues::Bar {
          RadialChartKind::BarOfPie
        } else {
          RadialChartKind::PieOfPie
        },
        chart.pie_chart_series.as_slice(),
        chart.data_labels.as_deref(),
        0.0,
        0.0,
        chart.vary_colors.as_ref(),
        Some(chart.as_ref()),
      )),
      _ => None,
    })?;
  let series = pie_series.first()?;
  let values = series
    .values
    .as_deref()
    .map(indexed_values)
    .unwrap_or_default();
  let mut categories = series
    .category_axis_data
    .as_deref()
    .map(indexed_category_axis_text_values)
    .unwrap_or_default();
  let cached_category_count = categories.len();
  if categories.len() < values.len() {
    categories.extend((categories.len() + 1..=values.len()).map(|index| index.to_string()));
  }
  categories.truncate(values.len());
  let series_ref = pie_series_ref(series);
  let series_name = series_ref
    .series_text
    .map(series_text_value)
    .filter(|value| !value.is_empty())
    .unwrap_or_else(|| default_series_label(series_ref, 1, None));
  let title = match chart_title_text(&chart_space.chart) {
    None
      if chart_space
        .chart
        .title
        .as_deref()
        .is_some_and(|title| explicit_title_text(title).is_none())
        && chart_automatic_title_is_visible(&chart_space.chart)
        && pie_series.len() == 1
        && !series_name.is_empty() =>
    {
      // LibreOffice ChartSpaceConverter::convertFromModel derives the
      // automatic chart title from the series title when the plot contains
      // exactly one series. Office fixed output follows the same rule for an
      // empty c:title with c:autoTitleDeleted=false.
      Some(ChartTitleText::Explicit(series_name.clone()))
    }
    title => title,
  };
  let series_labels_deleted = series.data_labels.as_deref().is_some_and(|labels| {
    matches!(
      labels.data_labels_choice.as_ref(),
      Some(c::DataLabelsChoice::Delete(delete))
        if delete.val.is_none_or(|value| value.as_bool())
    )
  });
  let mut data_labels = resolved_data_labels(
    if series_labels_deleted {
      None
    } else {
      series.data_labels.as_deref()
    },
    chart_group_labels,
    &series_name,
    &categories,
    &values,
    None,
    DataLabelDefaults {
      value_format_code: series_number_format_code(series_ref),
      position: if radial_kind == RadialChartKind::Doughnut {
        c::DataLabelPositionValues::Center
      } else {
        c::DataLabelPositionValues::BestFit
      },
      supports_percent: true,
      separator: " ",
    },
  );
  // LibreOffice writes a series-level delete marker for doughnut remainder
  // points while retaining chart-group percentage labels. Office applies the
  // group label only to the cached category points; generated remainder
  // points have no visible label.
  if radial_kind == RadialChartKind::Doughnut && series_labels_deleted {
    data_labels.retain(|label| label.point_index < cached_category_count);
  }
  let data_label_text_properties = series
    .data_labels
    .as_deref()
    .and_then(data_labels_text_properties)
    .or_else(|| chart_group_labels.and_then(data_labels_text_properties));

  let mut data_point_fills = Vec::new();
  collect_data_point_solid_fills(&series.data_point, &mut data_point_fills);
  data_point_fills.sort_by_key(|fill| fill.index);

  let legend = chart_space.chart.legend.as_deref();
  let legend_position = legend.map(|legend| {
    match legend
      .legend_position
      .as_ref()
      .and_then(|position| position.val)
      // ECMA-376 Part 1, CT_LegendPos defaults an omitted value to right.
      .unwrap_or(c::LegendPositionValues::Right)
    {
      c::LegendPositionValues::Bottom => ChartLegendPosition::Bottom,
      c::LegendPositionValues::Top => ChartLegendPosition::Top,
      c::LegendPositionValues::Left => ChartLegendPosition::Left,
      c::LegendPositionValues::Right => ChartLegendPosition::Right,
      c::LegendPositionValues::TopRight => ChartLegendPosition::TopRight,
    }
  });
  let mut visible_legend_indices = (0..categories.len()).collect::<Vec<_>>();
  if let Some(legend) = legend {
    visible_legend_indices.retain(|index| {
      !legend.legend_entry.iter().any(|entry| {
        entry.index.val as usize == *index
          && matches!(
            entry.legend_entry_choice.as_ref(),
            Some(c::LegendEntryChoice::Delete(delete))
              if delete.val.is_none_or(|value| value.as_bool())
          )
      })
    });
  }

  let mut point_explosion_percent = vec![0.0; values.len()];
  for point in &series.data_point {
    if let (Ok(index), Some(explosion)) =
      (usize::try_from(point.index.val), point.explosion.as_ref())
      && let Some(target) = point_explosion_percent.get_mut(index)
    {
      *target = f64::from(explosion.val);
    }
  }
  let secondary_indices = of_pie
    .map(|chart| of_pie_secondary_indices(chart, &values))
    .unwrap_or_default();

  Some(PieChartModel {
    kind: radial_kind,
    title,
    series_name,
    categories,
    values,
    series_solid_fill: series
      .chart_shape_properties
      .as_deref()
      .and_then(chart_shape_solid_fill),
    data_point_fills,
    first_slice_angle_deg,
    hole_size_percent,
    series_explosion_percent: series
      .explosion
      .as_ref()
      .map_or(0.0, |explosion| f64::from(explosion.val)),
    point_explosion_percent,
    secondary_indices,
    secondary_size_percent: of_pie
      .and_then(|chart| chart.second_pie_size.as_ref())
      .and_then(|size| size.val)
      .map_or(75.0, f64::from),
    vary_colors: pie_series.len() == 1
      && vary_colors.is_some_and(|vary| vary.val.is_none_or(|value| value.as_bool())),
    legend_position,
    legend_overlay: legend
      .and_then(|legend| legend.overlay.as_ref())
      .is_some_and(|overlay| overlay.val.is_none_or(|value| value.as_bool())),
    visible_legend_indices,
    legend_layout: legend.and_then(|legend| chart_layout(legend.layout.as_deref())),
    plot_layout: chart_layout(chart_space.chart.plot_area.layout.as_deref()),
    data_labels,
    data_label_text_properties,
    show_leader_lines: series
      .data_labels
      .as_deref()
      .and_then(data_labels_show_leader_lines)
      .or_else(|| chart_group_labels.and_then(data_labels_show_leader_lines))
      .unwrap_or(false),
  })
}

fn data_labels_show_leader_lines(labels: &c::DataLabels) -> Option<bool> {
  let c::DataLabelsChoice::Sequence(sequence) = labels.data_labels_choice.as_ref()? else {
    return None;
  };
  sequence
    .show_leader_lines
    .as_ref()
    .map(|show| show.val.is_none_or(|value| value.as_bool()))
}

fn of_pie_secondary_indices(chart: &c::OfPieChart, values: &[Option<f64>]) -> Vec<usize> {
  if values.len() <= 1 {
    return Vec::new();
  }
  let split_type = chart
    .split_type
    .as_ref()
    .map_or(c::SplitValues::Position, |split| split.val);
  let split_position = chart
    .split_position
    .as_ref()
    .map_or(2.0, |position| position.val);
  let mut indices: Vec<usize> = match split_type {
    c::SplitValues::Custom => chart
      .custom_split
      .as_ref()
      .map(|split| {
        split
          .second_pie_point
          .iter()
          .filter_map(|point| usize::try_from(point.val).ok())
          .filter(|index| *index < values.len())
          .collect()
      })
      .unwrap_or_default(),
    c::SplitValues::Position => {
      let count = (split_position.max(1.0) as usize).min(values.len() - 1);
      ((values.len() - count)..values.len()).collect()
    }
    c::SplitValues::Value => values
      .iter()
      .enumerate()
      .filter_map(|(index, value)| {
        value
          .is_some_and(|value| value <= split_position)
          .then_some(index)
      })
      .collect(),
    c::SplitValues::Percent => {
      let total = values.iter().flatten().sum::<f64>();
      values
        .iter()
        .enumerate()
        .filter_map(|(index, value)| {
          value
            .is_some_and(|value| total > 0.0 && value / total * 100.0 <= split_position)
            .then_some(index)
        })
        .collect()
    }
  };
  indices.sort_unstable();
  indices.dedup();
  if indices.len() >= values.len() {
    indices.truncate(values.len() - 1);
  }
  indices
}

/// Returns whether `c:spPr` contributes visible chart- or plot-area paint.
///
/// ISO/IEC 29500-1:2016 §21.2.2.197 delegates chart shape properties to
/// DrawingML.  An explicit `a:noFill` together with `a:ln/a:noFill` is the
/// same non-painting state as omitted properties.  LibreOffice writes that
/// pair on many otherwise ordinary charts, so treating mere element presence
/// as unsupported incorrectly bypasses the complete chart lowerer.
fn shape_properties_are_visible(properties: &c::ShapeProperties) -> bool {
  let fill_is_inert = matches!(
    properties.shape_properties_choice2.as_ref(),
    None | Some(c::ShapePropertiesChoice2::NoFill(_))
  );
  let outline_is_inert = properties.outline.as_deref().is_none_or(|outline| {
    matches!(
      outline.outline_choice1.as_ref(),
      None | Some(a::OutlineChoice::NoFill(_))
    )
  });

  !fill_is_inert
    || !outline_is_inert
    || properties.black_white_mode.is_some()
    || properties.transform2_d.is_some()
    || properties.shape_properties_choice1.is_some()
    || properties.shape_properties_choice3.is_some()
    || properties.scene3_d_type.is_some()
    || properties.shape3_d_type.is_some()
    || properties.shape_properties_extension_list.is_some()
}

struct DataLabelDefaults<'a> {
  value_format_code: Option<&'a str>,
  position: c::DataLabelPositionValues,
  supports_percent: bool,
  separator: &'a str,
}

fn resolved_data_labels<'a>(
  series_labels: Option<&'a c::DataLabels>,
  chart_group_labels: Option<&'a c::DataLabels>,
  series_name: &str,
  categories: &[String],
  values: &[Option<f64>],
  bubble_sizes: Option<&[Option<f64>]>,
  defaults: DataLabelDefaults<'a>,
) -> Vec<ClusteredColumnDataLabel> {
  // ECMA-376 Part 1 §21.2.2.49 defines c:dLbls as the settings for an
  // entire series or chart. MS-OI29500 §21.2.2.49 adds the Office override
  // hierarchy: chart-group dLbls < series dLbls < individual dLbl. Expand the
  // resolved series settings across every point, then apply point overrides.
  let mut settings = ClusteredColumnDataLabelSettings {
    separator: defaults.separator,
    position: defaults.position,
    value_format_code: defaults.value_format_code,
    ..ClusteredColumnDataLabelSettings::default()
  };
  apply_data_labels_settings(&mut settings, chart_group_labels);
  apply_data_labels_settings(&mut settings, series_labels);
  if !defaults.supports_percent {
    settings.show_percent = false;
  }
  if bubble_sizes.is_none() {
    settings.show_bubble_size = false;
  }
  let percentage_total = values.iter().flatten().sum::<f64>();
  let whole_percentages = (series_labels
    .or(chart_group_labels)
    .and_then(data_labels_format_code)
    .is_none()
    && percentage_total > f64::EPSILON)
    .then(|| largest_remainder_percentages(values, percentage_total));

  let mut point_labels = vec![None; values.len()];
  if let Some(labels) = series_labels {
    for label in &labels.data_label {
      let Ok(point_index) = usize::try_from(label.index.val) else {
        continue;
      };
      if point_index < point_labels.len() {
        point_labels[point_index] = Some(label);
      }
    }
  }

  values
    .iter()
    .enumerate()
    .filter_map(|(point_index, value)| {
      let value = value.as_ref().copied()?;
      let mut point_settings = settings;
      let percentage_text =
        (percentage_total.abs() > f64::EPSILON).then(|| {
          whole_percentages.as_ref().map_or_else(
            || {
              format_chart_number(
                value / percentage_total,
                series_labels
                  .or(chart_group_labels)
                  .and_then(data_labels_format_code)
                  .or(Some("0%")),
              )
            },
            |percentages| format!("{}%", percentages[point_index] as i32),
          )
        });
      let mut custom_text = None;
      if let Some(label) = point_labels[point_index] {
        if label.data_label_choice.iter().any(|choice| {
          matches!(choice, c::DataLabelChoice::Delete(delete) if delete.val.is_none_or(|value| value.as_bool()))
        }) {
          return None;
        }
        if let Some(sequence) = label
          .data_label_choice
          .iter()
          .find_map(|choice| match choice {
            c::DataLabelChoice::Sequence(sequence) => Some(sequence.as_ref()),
            _ => None,
          })
        {
          if let Some(chart_text) = sequence.chart_text.as_deref() {
            // MS-OI29500 §21.2.2.47: when c:tx is present Office ignores
            // the component-selection fields on the same individual label.
            custom_text = Some(data_label_chart_text(
              chart_text,
              series_name,
              categories.get(point_index).map(String::as_str),
              value,
              point_settings.value_format_code,
              percentage_text.as_deref(),
              bubble_sizes
                .and_then(|sizes| sizes.get(point_index))
                .copied()
                .flatten(),
            ));
          } else {
            apply_data_label_sequence_settings(&mut point_settings, sequence);
          }
        }
      }

      if point_settings.deleted {
        return None;
      }
      let text = match custom_text {
        Some(text) if !text.is_empty() => text,
        Some(_) => return None,
        None => compose_clustered_column_data_label(
          point_settings,
          series_name,
          categories.get(point_index).map(String::as_str),
          value,
          point_settings.value_format_code,
          bubble_sizes
            .and_then(|sizes| sizes.get(point_index))
            .copied()
            .flatten(),
          point_settings
            .show_percent
            .then_some(percentage_text.clone())
            .flatten(),
        )?,
      };
      Some(ClusteredColumnDataLabel {
        point_index,
        text,
        position: point_settings.position,
      })
    })
    .collect()
}

#[derive(Clone, Copy, Debug)]
struct ClusteredColumnDataLabelSettings<'a> {
  deleted: bool,
  show_value: bool,
  show_category_name: bool,
  show_series_name: bool,
  show_percent: bool,
  show_bubble_size: bool,
  separator: &'a str,
  value_format_code: Option<&'a str>,
  position: c::DataLabelPositionValues,
}

impl Default for ClusteredColumnDataLabelSettings<'_> {
  fn default() -> Self {
    Self {
      deleted: false,
      show_value: false,
      show_category_name: false,
      show_series_name: false,
      show_percent: false,
      show_bubble_size: false,
      separator: "; ",
      value_format_code: None,
      // MS-OI29500 §21.2.2.48 specifies OutsideEnd as the Office default
      // for a clustered bar/column chart when c:dLblPos is omitted.
      position: c::DataLabelPositionValues::OutsideEnd,
    }
  }
}

fn apply_data_labels_settings<'a>(
  settings: &mut ClusteredColumnDataLabelSettings<'a>,
  labels: Option<&'a c::DataLabels>,
) {
  let Some(labels) = labels else {
    return;
  };
  match labels.data_labels_choice.as_ref() {
    Some(c::DataLabelsChoice::Delete(delete)) => {
      settings.deleted = delete.val.is_none_or(|value| value.as_bool());
    }
    Some(c::DataLabelsChoice::Sequence(sequence)) => {
      settings.deleted = false;
      apply_data_labels_sequence_settings(settings, sequence);
    }
    None => {}
  }
}

fn apply_data_labels_sequence_settings<'a>(
  settings: &mut ClusteredColumnDataLabelSettings<'a>,
  sequence: &'a c::DataLabelsChoiceSequence,
) {
  if let Some(show) = sequence.show_value.as_ref() {
    settings.show_value = show.val.is_none_or(|value| value.as_bool());
  }
  if let Some(show) = sequence.show_category_name.as_ref() {
    settings.show_category_name = show.val.is_none_or(|value| value.as_bool());
  }
  if let Some(show) = sequence.show_series_name.as_ref() {
    settings.show_series_name = show.val.is_none_or(|value| value.as_bool());
  }
  if let Some(show) = sequence.show_percent.as_ref() {
    settings.show_percent = show.val.is_none_or(|value| value.as_bool());
  }
  if let Some(show) = sequence.show_bubble_size.as_ref() {
    settings.show_bubble_size = show.val.is_none_or(|value| value.as_bool());
  }
  if let Some(separator) = sequence.separator.as_deref() {
    settings.separator = separator;
  }
  if let Some(format) = sequence.numbering_format.as_ref() {
    settings.value_format_code = Some(format.format_code.as_str());
  }
  if let Some(position) = sequence.data_label_position.as_ref() {
    settings.position = position.val;
  }
}

fn data_labels_text_properties(labels: &c::DataLabels) -> Option<&c::TextProperties> {
  match labels.data_labels_choice.as_ref() {
    Some(c::DataLabelsChoice::Sequence(sequence)) => sequence.text_properties.as_deref(),
    _ => None,
  }
}

fn chart_data_label_text_properties(chart_space: &c::ChartSpace) -> Option<&c::TextProperties> {
  series(chart_space)
    .into_iter()
    .filter_map(|series| series.data_labels)
    .find_map(data_labels_text_properties)
    .or_else(|| data_labels(chart_space).find_map(data_labels_text_properties))
}

fn apply_data_label_sequence_settings<'a>(
  settings: &mut ClusteredColumnDataLabelSettings<'a>,
  sequence: &'a c::DataLabelChoiceSequence,
) {
  if let Some(show) = sequence.show_value.as_ref() {
    settings.show_value = show.val.is_none_or(|value| value.as_bool());
  }
  if let Some(show) = sequence.show_category_name.as_ref() {
    settings.show_category_name = show.val.is_none_or(|value| value.as_bool());
  }
  if let Some(show) = sequence.show_series_name.as_ref() {
    settings.show_series_name = show.val.is_none_or(|value| value.as_bool());
  }
  if let Some(show) = sequence.show_percent.as_ref() {
    settings.show_percent = show.val.is_none_or(|value| value.as_bool());
  }
  if let Some(show) = sequence.show_bubble_size.as_ref() {
    settings.show_bubble_size = show.val.is_none_or(|value| value.as_bool());
  }
  if let Some(separator) = sequence.separator.as_deref() {
    settings.separator = separator;
  }
  if let Some(format) = sequence.numbering_format.as_ref() {
    settings.value_format_code = Some(format.format_code.as_str());
  }
  if let Some(position) = sequence.data_label_position.as_ref() {
    settings.position = position.val;
  }
}

fn compose_clustered_column_data_label(
  settings: ClusteredColumnDataLabelSettings<'_>,
  series_name: &str,
  category: Option<&str>,
  value: f64,
  value_format_code: Option<&str>,
  bubble_size: Option<f64>,
  percentage: Option<String>,
) -> Option<String> {
  let mut components = Vec::with_capacity(4);
  if settings.show_series_name && !series_name.is_empty() {
    components.push(series_name.to_string());
  }
  if settings.show_category_name
    && let Some(category) = category.filter(|category| !category.is_empty())
  {
    components.push(category.to_string());
  }
  if settings.show_value {
    components.push(format_chart_number(value, value_format_code));
  }
  if settings.show_bubble_size
    && let Some(bubble_size) = bubble_size
  {
    components.push(general_chart_number(bubble_size));
  }
  if let Some(percentage) = percentage {
    components.push(percentage);
  }
  (!components.is_empty()).then(|| components.join(settings.separator))
}

fn largest_remainder_percentages(values: &[Option<f64>], total: f64) -> Vec<f64> {
  let mut percentages = values
    .iter()
    .map(|value| value.map(|value| value / total * 100.0).unwrap_or(0.0))
    .collect::<Vec<_>>();
  let mut remaining = 100_i32
    - percentages
      .iter()
      .map(|value| value.floor() as i32)
      .sum::<i32>();
  let mut order = (0..percentages.len()).collect::<Vec<_>>();
  order.sort_by(|left, right| {
    percentages[*right]
      .fract()
      .total_cmp(&percentages[*left].fract())
      .then_with(|| left.cmp(right))
  });
  for index in order {
    let floor = percentages[index].floor();
    percentages[index] = floor + f64::from(remaining > 0);
    remaining -= i32::from(remaining > 0);
  }
  percentages
}

fn data_label_chart_text(
  chart_text: &c::ChartText,
  series_name: &str,
  category_name: Option<&str>,
  value: f64,
  value_format_code: Option<&str>,
  percentage: Option<&str>,
  bubble_size: Option<f64>,
) -> String {
  let Some(c::ChartTextChoice::RichText(rich)) = chart_text.chart_text_choice.as_ref() else {
    let mut values = Vec::new();
    push_chart_text(&mut values, chart_text);
    return values.join(" ");
  };
  let mut result = String::new();
  for paragraph in &rich.paragraph {
    for choice in &paragraph.paragraph_choice {
      match choice {
        a::ParagraphChoice::Run(run) => result.push_str(&run.text),
        a::ParagraphChoice::Field(field) => {
          match field
            .r#type
            .as_deref()
            .map(str::to_ascii_uppercase)
            .as_deref()
          {
            Some("VALUE") => result.push_str(&format_chart_number(value, value_format_code)),
            Some("SERIESNAME") => result.push_str(series_name),
            Some("CATEGORYNAME") => result.push_str(category_name.unwrap_or_default()),
            Some("PERCENTAGE") => result.push_str(percentage.unwrap_or_default()),
            Some("BUBBLESIZE") => {
              if let Some(value) = bubble_size {
                result.push_str(&general_chart_number(value));
              }
            }
            _ => {
              if let Some(text) = field.text.as_deref() {
                result.push_str(text);
              }
            }
          }
        }
        a::ParagraphChoice::Break(_) => result.push('\n'),
        a::ParagraphChoice::TextMath(math) => result.push_str(&text_math_text(math)),
        a::ParagraphChoice::AlternateContent(_) => {}
      }
    }
  }
  result.trim().to_string()
}

fn general_chart_number(value: f64) -> String {
  let rounded = value.round();
  if (value - rounded).abs() < 1.0e-12 {
    format!("{rounded:.0}")
  } else {
    // Chart caches routinely contain IEEE-754 tails such as
    // -68.70000000000002. Office's General formatter keeps 15 significant
    // decimal digits and does not expose those transport artifacts.
    let magnitude = value.abs().log10().floor() as i32;
    let decimals = (14 - magnitude).clamp(0, 15) as usize;
    let formatted = format!("{value:.decimals$}");
    formatted
      .trim_end_matches('0')
      .trim_end_matches('.')
      .to_string()
  }
}

/// LibreOffice's `CategoryPositionHelper` slot calculation translated to a
/// normalized plot-area coordinate. OOXML gap width becomes its outer distance
/// and overlap becomes the negated inner distance.
pub fn clustered_column_slot(
  category_index: usize,
  series_index: usize,
  category_count: usize,
  series_count: usize,
  gap_width_percent: f64,
  overlap_percent: f64,
) -> Option<ClusteredColumnSlot> {
  if category_index >= category_count || series_index >= series_count || series_count == 0 {
    return None;
  }
  let category_width = 1.0 / category_count as f64;
  let outer_distance = (gap_width_percent / 100.0).clamp(0.0, 6.0);
  let inner_distance = (-overlap_percent / 100.0).clamp(-1.0, 1.0);
  let slot_width = category_width
    / (series_count as f64
      + outer_distance
      + inner_distance * (series_count.saturating_sub(1)) as f64);
  let category_center = (category_index as f64 + 0.5) * category_width;
  let center = category_center - category_width / 2.0
    + (outer_distance / 2.0 + series_index as f64 * (1.0 + inner_distance)) * slot_width
    + slot_width / 2.0;
  Some(ClusteredColumnSlot {
    center,
    width: slot_width,
  })
}

/// Calculates a linear numeric axis using the same broad rules as
/// LibreOffice `ScaleAutomatism::calculateExplicitIncrementAndScaleForLinear`:
/// wide all-positive ranges expand to zero, automatic intervals normalize to
/// 1/2/5 x 10^n, limits align to the interval rhythm, and a value sitting on
/// the upper border receives one interval of breathing room.
pub fn linear_axis_scale(
  values: impl IntoIterator<Item = f64>,
  axis: Option<&c::ValueAxis>,
  maximum_auto_increment_count: usize,
) -> Option<LinearAxisScale> {
  let mut source_minimum = f64::INFINITY;
  let mut source_maximum = f64::NEG_INFINITY;
  for value in values.into_iter().filter(|value| value.is_finite()) {
    source_minimum = source_minimum.min(value);
    source_maximum = source_maximum.max(value);
  }
  if !source_minimum.is_finite() || !source_maximum.is_finite() {
    return None;
  }

  let explicit_minimum = axis.and_then(|axis| axis.scaling.min_axis_value.as_ref().map(|v| v.val));
  let explicit_maximum = axis.and_then(|axis| axis.scaling.max_axis_value.as_ref().map(|v| v.val));
  let explicit_unit = axis.map(|axis| axis.major_unit.as_ref().map(|unit| unit.val));
  let logarithmic_base = axis
    .and_then(|axis| axis.scaling.log_base.as_ref())
    .map(|base| base.val)
    .filter(|base| *base > 1.0);
  let mut temporary_minimum = explicit_minimum.unwrap_or(source_minimum);
  let mut temporary_maximum = explicit_maximum.unwrap_or(source_maximum);
  if temporary_minimum > temporary_maximum {
    std::mem::swap(&mut temporary_minimum, &mut temporary_maximum);
  }
  if logarithmic_base.is_none() && explicit_minimum.is_none() && temporary_minimum > 0.0 {
    if temporary_minimum == temporary_maximum || temporary_minimum / temporary_maximum < 5.0 / 6.0 {
      temporary_minimum = 0.0;
    } else {
      // LibreOffice VSeriesPlotter enables
      // `isExpandNarrowValuesTowardZero()` for a Y value axis. Without this
      // half-range expansion, nearly equal values collapse against the lower
      // border and the subsequent increment alignment starts one tick too
      // high (tdf#130969).
      temporary_minimum -= (temporary_maximum - temporary_minimum) / 2.0;
    }
  }
  if temporary_minimum == temporary_maximum {
    if temporary_maximum == 0.0 {
      temporary_maximum = 1.0;
    } else {
      temporary_maximum *= 2.0;
    }
  }
  if let Some(base) = logarithmic_base
    && temporary_minimum > 0.0
    && temporary_maximum > 0.0
  {
    return Some(LinearAxisScale {
      minimum: explicit_minimum.unwrap_or_else(|| base.powf(temporary_minimum.log(base).floor())),
      maximum: explicit_maximum.unwrap_or_else(|| base.powf(temporary_maximum.log(base).ceil())),
      major_unit: explicit_unit.flatten().unwrap_or(1.0),
      logarithmic_base: Some(base),
      reversed: axis
        .and_then(|axis| axis.scaling.orientation.as_ref())
        .and_then(|orientation| orientation.val)
        == Some(c::OrientationValues::MaxMin),
    });
  }

  let max_increments = maximum_auto_increment_count.clamp(2, 10);
  let mut major_unit = explicit_unit
    .flatten()
    .filter(|unit| unit.is_finite() && *unit > 0.0)
    .unwrap_or_else(|| {
      nice_increment((temporary_maximum - temporary_minimum) / max_increments as f64)
    });
  let automatic_unit = explicit_unit.flatten().is_none();
  loop {
    let minimum =
      explicit_minimum.unwrap_or_else(|| increment_floor(temporary_minimum, major_unit));
    let mut maximum =
      explicit_maximum.unwrap_or_else(|| increment_ceil(temporary_maximum, major_unit));
    if explicit_maximum.is_none()
      && maximum != 0.0
      && (source_maximum - minimum) / (maximum - minimum) > 20.0 / 21.0
    {
      maximum += major_unit;
    }
    let increment_count = ((maximum - minimum) / major_unit).floor() as usize;
    if increment_count <= max_increments || !automatic_unit {
      return Some(LinearAxisScale {
        minimum,
        maximum,
        major_unit,
        logarithmic_base,
        reversed: axis
          .and_then(|axis| axis.scaling.orientation.as_ref())
          .and_then(|orientation| orientation.val)
          == Some(c::OrientationValues::MaxMin),
      });
    }
    major_unit = next_nice_increment(major_unit);
  }
}

fn nice_increment(raw: f64) -> f64 {
  if !raw.is_finite() || raw <= 1.0e-307 {
    return 1.0e-307;
  }
  let magnitude = 10.0_f64.powf(raw.log10().floor());
  let normalized = raw / magnitude;
  let normalized = if normalized <= 1.0 {
    1.0
  } else if normalized <= 2.0 {
    2.0
  } else if normalized <= 5.0 {
    5.0
  } else {
    10.0
  };
  normalized * magnitude
}

fn next_nice_increment(current: f64) -> f64 {
  let magnitude = 10.0_f64.powf(current.log10().floor());
  let normalized = current / magnitude;
  if normalized < 1.5 {
    2.0 * magnitude
  } else if normalized < 3.5 {
    5.0 * magnitude
  } else {
    10.0 * magnitude
  }
}

fn increment_floor(value: f64, increment: f64) -> f64 {
  (value / increment).floor() * increment
}

fn increment_ceil(value: f64, increment: f64) -> f64 {
  (value / increment).ceil() * increment
}

pub fn kind(chart_space: &c::ChartSpace) -> ChartKind {
  chart_space
    .chart
    .plot_area
    .plot_area_choice1
    .iter()
    .map(|choice| match choice {
      c::PlotAreaChoice::PieChart(_)
      | c::PlotAreaChoice::Pie3DChart(_)
      | c::PlotAreaChoice::DoughnutChart(_)
      | c::PlotAreaChoice::OfPieChart(_) => ChartKind::Pie,
      c::PlotAreaChoice::BarChart(_) | c::PlotAreaChoice::Bar3DChart(_) => ChartKind::Bar,
      c::PlotAreaChoice::AreaChart(_) | c::PlotAreaChoice::Area3DChart(_) => ChartKind::Area,
      c::PlotAreaChoice::LineChart(_) | c::PlotAreaChoice::Line3DChart(_) => ChartKind::Line,
      c::PlotAreaChoice::ScatterChart(_) => ChartKind::Scatter,
      c::PlotAreaChoice::BubbleChart(_) => ChartKind::Bubble,
      c::PlotAreaChoice::RadarChart(_) => ChartKind::Radar,
      c::PlotAreaChoice::StockChart(_) => ChartKind::Stock,
      c::PlotAreaChoice::SurfaceChart(_) | c::PlotAreaChoice::Surface3DChart(_) => {
        ChartKind::Surface
      }
    })
    .next()
    .unwrap_or(ChartKind::Other)
}

pub fn has_values(chart_space: &c::ChartSpace, expected: &[&str]) -> bool {
  let values = visible_texts(chart_space);
  expected
    .iter()
    .all(|expected| values.iter().any(|value| value == expected))
}

pub fn has_vertical_multilevel_category_axis(chart_space: &c::ChartSpace) -> bool {
  chart_space
    .chart
    .plot_area
    .plot_area_choice2
    .iter()
    .filter_map(|choice| match choice {
      c::PlotAreaChoice2::CategoryAxis(axis) => Some(axis.as_ref()),
      _ => None,
    })
    .any(|axis| {
      axis
        .text_properties
        .as_deref()
        .and_then(|properties| properties.body_properties.rotation)
        .is_some_and(|rotation| rotation.abs() >= 54_000_000)
        && axis
          .no_multi_level_labels
          .as_ref()
          .and_then(|labels| labels.val)
          .is_none_or(|value| !value.as_bool())
    })
}

pub fn visible_texts(chart_space: &c::ChartSpace) -> Vec<String> {
  visible_texts_for_ui_language(chart_space, None)
}

pub fn visible_texts_for_ui_language(
  chart_space: &c::ChartSpace,
  ui_language: Option<&str>,
) -> Vec<String> {
  visible_texts_with_default_series_label(chart_space, ui_language, |series, series_index| {
    default_series_label(series, series_index, ui_language)
  })
}

pub fn visible_texts_with_uncached_series_labels(chart_space: &c::ChartSpace) -> Vec<String> {
  visible_texts_with_default_series_label(chart_space, None, uncached_series_label)
}

/// Returns only series identity text, excluding category and numeric caches.
/// LibreOffice assigns `SeriesN` to imported OOXML series without a cached
/// label sequence.
pub fn series_identity_texts_with_uncached_labels(chart_space: &c::ChartSpace) -> Vec<String> {
  let mut texts = Vec::new();
  for (index, series) in series(chart_space).into_iter().enumerate() {
    if let Some(series_text) = series.series_text {
      push_series_text(&mut texts, series_text);
    } else {
      push_unique_text(&mut texts, &uncached_series_label(series, index + 1));
    }
  }
  texts
}

/// Returns explicit axis-title text without pulling in chart data caches.
pub fn explicit_axis_title_texts(chart_space: &c::ChartSpace) -> Vec<String> {
  let mut texts = Vec::new();
  for title in axis_titles(chart_space) {
    push_title_texts(&mut texts, title);
  }
  texts
}

/// Returns the first explicit Latin typeface applied to fixed-output chart
/// text. Chart-local text properties take precedence over the host theme.
pub fn fixed_output_latin_font_family(chart_space: &c::ChartSpace) -> Option<&str> {
  chart_space
    .text_properties
    .as_deref()
    .and_then(text_properties_latin_font_family)
    .or_else(|| {
      chart_space
        .chart
        .title
        .as_deref()
        .and_then(|title| title.text_properties.as_deref())
        .and_then(text_properties_latin_font_family)
    })
    .or_else(|| {
      chart_space
        .chart
        .title
        .as_deref()
        .and_then(|title| title.chart_text.as_deref())
        .and_then(|text| match text.chart_text_choice.as_ref() {
          Some(c::ChartTextChoice::RichText(rich)) => paragraphs_latin_font_family(&rich.paragraph),
          _ => None,
        })
    })
    .or_else(|| {
      chart_space
        .chart
        .plot_area
        .plot_area_choice2
        .iter()
        .find_map(|choice| match choice {
          c::PlotAreaChoice2::CategoryAxis(axis) => axis
            .text_properties
            .as_deref()
            .and_then(text_properties_latin_font_family),
          c::PlotAreaChoice2::DateAxis(axis) => axis
            .text_properties
            .as_deref()
            .and_then(text_properties_latin_font_family),
          c::PlotAreaChoice2::SeriesAxis(axis) => axis
            .text_properties
            .as_deref()
            .and_then(text_properties_latin_font_family),
          c::PlotAreaChoice2::ValueAxis(axis) => axis
            .text_properties
            .as_deref()
            .and_then(text_properties_latin_font_family),
        })
    })
    .or_else(|| {
      chart_space
        .chart
        .legend
        .as_deref()
        .and_then(|legend| legend.text_properties.as_deref())
        .and_then(text_properties_latin_font_family)
    })
}

fn text_properties_latin_font_family(properties: &c::TextProperties) -> Option<&str> {
  paragraphs_latin_font_family(&properties.paragraph)
}

fn paragraphs_latin_font_family(paragraphs: &[a::Paragraph]) -> Option<&str> {
  paragraphs.iter().find_map(|paragraph| {
    paragraph
      .paragraph_properties
      .as_deref()
      .and_then(|properties| properties.default_run_properties.as_deref())
      .and_then(|properties| properties.latin_font.as_ref())
      .and_then(|font| font.typeface.as_deref())
      .or_else(|| {
        paragraph
          .paragraph_choice
          .iter()
          .find_map(|choice| match choice {
            a::ParagraphChoice::Run(run) => run
              .run_properties
              .as_deref()
              .and_then(|properties| properties.latin_font.as_ref())
              .and_then(|font| font.typeface.as_deref()),
            a::ParagraphChoice::Field(field) => field
              .run_properties
              .as_deref()
              .and_then(|properties| properties.latin_font.as_ref())
              .and_then(|font| font.typeface.as_deref()),
            _ => None,
          })
      })
      .or_else(|| {
        paragraph
          .end_paragraph_run_properties
          .as_deref()
          .and_then(|properties| properties.latin_font.as_ref())
          .and_then(|font| font.typeface.as_deref())
      })
  })
}

/// Returns chart text that is present in fixed output, in the order used by
/// Office's chart object stream.
///
/// Cached source values are deliberately excluded unless they become data
/// labels or data-table cells. Axis ticks are derived from the resolved scale,
/// categories come from the first visible category sequence, and legend text
/// comes from the series names. This keeps chart data attached to its drawing
/// anchor instead of leaking the entire cache into the document body.
pub fn fixed_output_texts_for_ui_language(
  chart_space: &c::ChartSpace,
  ui_language: Option<&str>,
) -> Vec<String> {
  let mut texts = Vec::new();
  let chart_series = series(chart_space);

  if kind(chart_space) == ChartKind::Pie {
    // MS-OI29500 §21.2.2.141: Office displays only the first pie series even
    // though the ECMA schema permits multiple c:ser children.
    let displayed_series = chart_series
      .first()
      .copied()
      .into_iter()
      .collect::<Vec<_>>();
    let pie_model = pie_chart_model(chart_space);
    if chart_space.chart.legend.is_some() {
      if chart_space
        .chart
        .title
        .as_deref()
        .is_some_and(|title| title.chart_text.is_none())
      {
        for (series_index, series) in displayed_series.iter().copied().enumerate() {
          push_fixed_series_name(&mut texts, series, series_index + 1, ui_language);
        }
      }
      if let Some(model) = pie_model.as_ref() {
        texts.extend(
          model
            .visible_legend_indices
            .iter()
            .filter_map(|index| model.categories.get(*index).cloned()),
        );
      } else if let Some(categories) = displayed_series
        .iter()
        .find_map(|series| series.category_axis_data)
      {
        push_fixed_category_texts(&mut texts, categories, chart_space);
      }
    }
    if let Some(model) = pie_model {
      texts.extend(model.data_labels.into_iter().map(|label| label.text));
    } else {
      push_fixed_data_labels(&mut texts, &displayed_series, 1.0);
    }
    push_fixed_chart_title(&mut texts, chart_space, ui_language);
    return texts;
  }

  let value_axes = chart_space
    .chart
    .plot_area
    .plot_area_choice2
    .iter()
    .filter_map(|choice| match choice {
      c::PlotAreaChoice2::ValueAxis(axis) if axis_is_visible(axis) => Some(axis.as_ref()),
      _ => None,
    })
    .collect::<Vec<_>>();
  for axis in &value_axes {
    if axis
      .tick_label_position
      .as_ref()
      .is_some_and(|position| position.val == Some(c::TickLabelPositionValues::None))
    {
      continue;
    }
    let axis_series = series_for_value_axis(chart_space, axis.axis_id.val);
    let axis_series = if axis_series.is_empty() {
      chart_series.clone()
    } else {
      axis_series
    };
    let mode = value_mode_for_axis(chart_space, axis.axis_id.val);
    let scale_values = scale_values(&axis_series, mode);
    let scale = if mode == ChartValueMode::PercentStacked {
      Some(LinearAxisScale {
        minimum: axis
          .scaling
          .min_axis_value
          .as_ref()
          .map_or(0.0, |value| value.val),
        maximum: axis
          .scaling
          .max_axis_value
          .as_ref()
          .map_or(1.0, |value| value.val),
        major_unit: axis.major_unit.as_ref().map_or(0.1, |unit| unit.val),
        logarithmic_base: None,
        reversed: false,
      })
    } else {
      linear_axis_scale(scale_values, Some(axis), 10)
    };
    let Some(scale) = scale else {
      continue;
    };
    let display_unit = value_axis_display_unit(axis);
    let format_code = axis
      .numbering_format
      .as_ref()
      .map(|format| format.format_code.as_str());
    for value in axis_tick_values(scale) {
      push_unique_text(
        &mut texts,
        &format_chart_number(value / display_unit, format_code),
      );
    }
  }

  if let Some(categories) = chart_series
    .iter()
    .find_map(|series| series.category_axis_data)
  {
    push_fixed_category_texts(&mut texts, categories, chart_space);
  }

  let display_unit = value_axes
    .first()
    .copied()
    .map(value_axis_display_unit)
    .unwrap_or(1.0);
  push_fixed_data_labels(&mut texts, &chart_series, display_unit);

  push_fixed_axis_titles(&mut texts, chart_space, ui_language);

  if chart_space.chart.plot_area.data_table.is_some() {
    if let Some(categories) = chart_series
      .iter()
      .find_map(|series| series.category_axis_data)
    {
      push_fixed_category_texts(&mut texts, categories, chart_space);
    }
    for (series_index, series) in chart_series.iter().enumerate().rev() {
      push_fixed_series_name(&mut texts, *series, series_index + 1, ui_language);
      for value in chart_series_numeric_values(*series).into_iter().flatten() {
        push_unique_text(&mut texts, &format_chart_number(value / display_unit, None));
      }
    }
  }

  for axis in &value_axes {
    if axis.display_units.is_some() {
      push_fixed_display_unit_label(&mut texts, axis, ui_language);
    }
  }
  push_fixed_chart_title(&mut texts, chart_space, ui_language);

  if chart_space.chart.legend.is_some() {
    let reverse = chart_space.chart.view3_d.is_some();
    if reverse {
      for (series_index, series) in chart_series.iter().enumerate().rev() {
        push_fixed_series_name(&mut texts, *series, series_index + 1, ui_language);
      }
    } else {
      for (series_index, series) in chart_series.iter().enumerate() {
        push_fixed_series_name(&mut texts, *series, series_index + 1, ui_language);
      }
    }
  }
  texts
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ChartValueMode {
  Standard,
  Stacked,
  PercentStacked,
}

fn push_fixed_chart_title(
  texts: &mut Vec<String>,
  chart_space: &c::ChartSpace,
  ui_language: Option<&str>,
) {
  if let Some(title) = fixed_chart_title_for_ui_language(chart_space, ui_language) {
    push_unique_text(texts, &title);
  }
}

pub fn fixed_chart_title_for_ui_language(
  chart_space: &c::ChartSpace,
  ui_language: Option<&str>,
) -> Option<String> {
  match chart_title_text(&chart_space.chart) {
    Some(ChartTitleText::Explicit(title)) => Some(normalize_fixed_chart_title(&title)),
    Some(ChartTitleText::Automatic) => Some(automatic_chart_title(ui_language).to_string()),
    None => None,
  }
}

fn normalize_fixed_chart_title(title: &str) -> String {
  let chars = title.chars().collect::<Vec<_>>();
  let mut normalized = String::with_capacity(title.len() + 4);
  for (index, ch) in chars.iter().copied().enumerate() {
    if ch == '-'
      && index > 0
      && index + 1 < chars.len()
      && chars[index - 1].is_ascii_digit()
      && chars[index + 1].is_ascii_digit()
    {
      normalized.push_str(" - ");
    } else {
      normalized.push(ch);
    }
  }
  normalized
}

fn push_fixed_axis_titles(
  texts: &mut Vec<String>,
  chart_space: &c::ChartSpace,
  ui_language: Option<&str>,
) {
  for choice in &chart_space.chart.plot_area.plot_area_choice2 {
    let title = match choice {
      c::PlotAreaChoice2::ValueAxis(axis) => axis.title.as_deref(),
      _ => None,
    };
    push_fixed_axis_title(texts, title, chart_space, ui_language);
  }
  for choice in &chart_space.chart.plot_area.plot_area_choice2 {
    let title = match choice {
      c::PlotAreaChoice2::CategoryAxis(axis) => axis.title.as_deref(),
      c::PlotAreaChoice2::DateAxis(axis) => axis.title.as_deref(),
      c::PlotAreaChoice2::SeriesAxis(axis) => axis.title.as_deref(),
      c::PlotAreaChoice2::ValueAxis(_) => None,
    };
    push_fixed_axis_title(texts, title, chart_space, ui_language);
  }
}

fn push_fixed_axis_title(
  texts: &mut Vec<String>,
  title: Option<&c::Title>,
  _chart_space: &c::ChartSpace,
  ui_language: Option<&str>,
) {
  let Some(title) = title else {
    return;
  };
  let before = texts.len();
  push_title_texts(texts, title);
  if texts.len() == before {
    push_unique_text(texts, automatic_axis_title(ui_language));
  }
}

fn automatic_axis_title(ui_language: Option<&str>) -> &'static str {
  if ui_language.is_some_and(is_chinese_ui_language) {
    "坐标轴标题"
  } else {
    "Axis Title"
  }
}

fn push_fixed_category_texts(
  texts: &mut Vec<String>,
  data: &c::CategoryAxisData,
  chart_space: &c::ChartSpace,
) {
  match data.category_axis_data_choice.as_ref() {
    Some(c::CategoryAxisDataChoice::MultiLevelStringReference(reference)) => {
      if let Some(cache) = reference.multi_level_string_cache.as_deref() {
        for level in &cache.level {
          for point in &level.string_point {
            push_unique_text(texts, &point.numeric_value);
          }
        }
      }
    }
    Some(c::CategoryAxisDataChoice::NumberReference(reference)) => {
      if let Some(cache) = reference.numbering_cache.as_deref() {
        let format_code = cache
          .format_code
          .as_ref()
          .and_then(|format| format.xml_content.as_deref());
        for point in &cache.numeric_point {
          let text = point
            .numeric_value
            .trim()
            .parse::<f64>()
            .ok()
            .map(|value| {
              format_chart_category_number(
                value,
                format_code,
                chart_space
                  .date1904
                  .as_ref()
                  .and_then(|date| date.val)
                  .is_some_and(|value| value.as_bool()),
              )
            })
            .unwrap_or_else(|| point.numeric_value.trim().to_string());
          push_unique_text(texts, &text);
        }
      }
    }
    Some(c::CategoryAxisDataChoice::NumberLiteral(literal)) => {
      let format_code = literal
        .format_code
        .as_ref()
        .and_then(|format| format.xml_content.as_deref());
      for point in &literal.numeric_point {
        let text = point
          .numeric_value
          .trim()
          .parse::<f64>()
          .ok()
          .map(|value| format_chart_category_number(value, format_code, false))
          .unwrap_or_else(|| point.numeric_value.trim().to_string());
        push_unique_text(texts, &text);
      }
    }
    Some(c::CategoryAxisDataChoice::StringReference(reference)) => {
      if let Some(cache) = reference.string_cache.as_deref() {
        for point in &cache.string_point {
          push_unique_text(texts, &point.numeric_value);
        }
      }
    }
    Some(c::CategoryAxisDataChoice::StringLiteral(literal)) => {
      for point in &literal.string_point {
        push_unique_text(texts, &point.numeric_value);
      }
    }
    None => {}
  }
}

fn push_fixed_series_name(
  texts: &mut Vec<String>,
  series: ChartSeriesRef<'_>,
  series_index: usize,
  ui_language: Option<&str>,
) {
  if let Some(series_text) = series.series_text {
    push_series_text(texts, series_text);
  } else {
    push_unique_text(
      texts,
      &default_series_label(series, series_index, ui_language),
    );
  }
}

fn push_fixed_data_labels(
  texts: &mut Vec<String>,
  chart_series: &[ChartSeriesRef<'_>],
  display_unit: f64,
) {
  for (series_index, series) in chart_series.iter().copied().enumerate() {
    let Some(labels) = series.data_labels else {
      continue;
    };
    let Some(c::DataLabelsChoice::Sequence(settings)) = labels.data_labels_choice.as_ref() else {
      push_data_label_texts(texts, labels);
      continue;
    };
    let show_value = settings
      .show_value
      .as_ref()
      .and_then(|show| show.val)
      .is_some_and(|value| value.as_bool());
    let show_category = settings
      .show_category_name
      .as_ref()
      .and_then(|show| show.val)
      .is_some_and(|value| value.as_bool());
    let show_percent = settings
      .show_percent
      .as_ref()
      .and_then(|show| show.val)
      .is_some_and(|value| value.as_bool());
    let categories = series
      .category_axis_data
      .map(indexed_category_axis_text_values)
      .unwrap_or_default();
    let values = chart_series_numeric_values(series);
    let value_format =
      data_labels_format_code(labels).or_else(|| series_number_format_code(series));
    let percentage_total = values.iter().flatten().sum::<f64>();
    let whole_percentages = if show_percent
      && data_labels_format_code(labels).is_none()
      && percentage_total > f64::EPSILON
    {
      let mut percentages = values
        .iter()
        .map(|value| {
          value
            .map(|value| value / percentage_total * 100.0)
            .unwrap_or(0.0)
        })
        .collect::<Vec<_>>();
      let mut remaining = 100_i32
        - percentages
          .iter()
          .map(|value| value.floor() as i32)
          .sum::<i32>();
      let mut order = (0..percentages.len()).collect::<Vec<_>>();
      order.sort_by(|left, right| {
        percentages[*right]
          .fract()
          .total_cmp(&percentages[*left].fract())
          .then_with(|| left.cmp(right))
      });
      for index in order {
        let floor = percentages[index].floor();
        percentages[index] = floor + f64::from(remaining > 0);
        remaining -= i32::from(remaining > 0);
      }
      Some(percentages)
    } else {
      None
    };
    let point_count = values.len().max(categories.len());
    let separator = settings.separator.as_deref().unwrap_or(" ");
    for point_index in 0..point_count {
      let mut components = Vec::new();
      if show_category && let Some(category) = categories.get(point_index) {
        components.push(category.clone());
      }
      if show_value && let Some(value) = values.get(point_index).copied().flatten() {
        components.push(format_chart_number(value / display_unit, value_format));
      }
      if show_percent
        && percentage_total.abs() > f64::EPSILON
        && let Some(value) = values.get(point_index).copied().flatten()
      {
        components.push(if let Some(percentages) = &whole_percentages {
          format!("{}%", percentages[point_index] as i32)
        } else {
          format_chart_number(
            value / percentage_total,
            data_labels_format_code(labels).or(Some("0%")),
          )
        });
      }
      if !components.is_empty() {
        push_unique_text(texts, &components.join(separator));
      }
    }
    if settings
      .show_series_name
      .as_ref()
      .and_then(|show| show.val)
      .is_some_and(|value| value.as_bool())
    {
      push_fixed_series_name(texts, series, series_index + 1, None);
    }
    push_data_label_texts(texts, labels);
  }
}

fn series_number_format_code(series: ChartSeriesRef<'_>) -> Option<&str> {
  if let Some(values) = series.values {
    return match values.values_choice.as_ref() {
      Some(c::ValuesChoice::NumberReference(reference)) => reference
        .numbering_cache
        .as_deref()
        .and_then(|cache| cache.format_code.as_ref())
        .and_then(|format| format.xml_content.as_deref()),
      Some(c::ValuesChoice::NumberLiteral(literal)) => literal
        .format_code
        .as_ref()
        .and_then(|format| format.xml_content.as_deref()),
      None => None,
    };
  }
  if let Some(values) = series.y_values {
    return match values.y_values_choice.as_ref() {
      Some(c::YValuesChoice::NumberReference(reference)) => reference
        .numbering_cache
        .as_deref()
        .and_then(|cache| cache.format_code.as_ref())
        .and_then(|format| format.xml_content.as_deref()),
      Some(c::YValuesChoice::NumberLiteral(literal)) => literal
        .format_code
        .as_ref()
        .and_then(|format| format.xml_content.as_deref()),
      _ => None,
    };
  }
  None
}

fn data_labels_format_code(labels: &c::DataLabels) -> Option<&str> {
  match labels.data_labels_choice.as_ref() {
    Some(c::DataLabelsChoice::Sequence(sequence)) => sequence
      .numbering_format
      .as_ref()
      .map(|format| format.format_code.as_str()),
    _ => None,
  }
}

fn axis_is_visible(axis: &c::ValueAxis) -> bool {
  axis
    .delete
    .as_ref()
    .is_none_or(|delete| delete.val.is_some_and(|value| !value.as_bool()))
}

fn axis_tick_values(scale: LinearAxisScale) -> Vec<f64> {
  if !scale.minimum.is_finite()
    || !scale.maximum.is_finite()
    || !scale.major_unit.is_finite()
    || scale.major_unit <= 0.0
  {
    return Vec::new();
  }
  let count = ((scale.maximum - scale.minimum) / scale.major_unit)
    .floor()
    .clamp(0.0, 1_000.0) as usize;
  (0..=count)
    .map(|index| scale.minimum + scale.major_unit * index as f64)
    .collect()
}

fn value_axis_display_unit(axis: &c::ValueAxis) -> f64 {
  let Some(units) = axis.display_units.as_deref() else {
    return 1.0;
  };
  match units.display_units_choice.as_ref() {
    Some(c::DisplayUnitsChoice::CustomDisplayUnit(unit))
      if unit.val.is_finite() && unit.val > 0.0 =>
    {
      unit.val
    }
    Some(c::DisplayUnitsChoice::BuiltInUnit(unit)) => match unit.val.unwrap_or_default() {
      c::BuiltInUnitValues::Hundreds => 1.0e2,
      c::BuiltInUnitValues::Thousands => 1.0e3,
      c::BuiltInUnitValues::TenThousands => 1.0e4,
      c::BuiltInUnitValues::HundredThousands => 1.0e5,
      c::BuiltInUnitValues::Millions => 1.0e6,
      c::BuiltInUnitValues::TenMillions => 1.0e7,
      c::BuiltInUnitValues::HundredMillions => 1.0e8,
      c::BuiltInUnitValues::Billions => 1.0e9,
      c::BuiltInUnitValues::Trillions => 1.0e12,
    },
    _ => 1.0,
  }
}

fn push_fixed_display_unit_label(
  texts: &mut Vec<String>,
  axis: &c::ValueAxis,
  ui_language: Option<&str>,
) {
  let Some(units) = axis.display_units.as_deref() else {
    return;
  };
  if let Some(label) = units.display_units_label.as_deref()
    && let Some(text) = label.chart_text.as_deref()
  {
    push_chart_text(texts, text);
    return;
  }
  let Some(c::DisplayUnitsChoice::BuiltInUnit(unit)) = units.display_units_choice.as_ref() else {
    return;
  };
  let chinese = ui_language.is_some_and(is_chinese_ui_language);
  let label = match (unit.val.unwrap_or_default(), chinese) {
    (c::BuiltInUnitValues::Hundreds, true) => "百",
    (c::BuiltInUnitValues::Thousands, true) => "千",
    (c::BuiltInUnitValues::TenThousands, true) => "万",
    (c::BuiltInUnitValues::HundredThousands, true) => "十万",
    (c::BuiltInUnitValues::Millions, true) => "百万",
    (c::BuiltInUnitValues::TenMillions, true) => "千万",
    (c::BuiltInUnitValues::HundredMillions, true) => "亿",
    (c::BuiltInUnitValues::Billions, true) => "十亿",
    (c::BuiltInUnitValues::Trillions, true) => "万亿",
    (c::BuiltInUnitValues::Hundreds, false) => "Hundreds",
    (c::BuiltInUnitValues::Thousands, false) => "Thousands",
    (c::BuiltInUnitValues::TenThousands, false) => "Ten Thousands",
    (c::BuiltInUnitValues::HundredThousands, false) => "Hundred Thousands",
    (c::BuiltInUnitValues::Millions, false) => "Millions",
    (c::BuiltInUnitValues::TenMillions, false) => "Ten Millions",
    (c::BuiltInUnitValues::HundredMillions, false) => "Hundred Millions",
    (c::BuiltInUnitValues::Billions, false) => "Billions",
    (c::BuiltInUnitValues::Trillions, false) => "Trillions",
  };
  push_unique_text(texts, label);
}

fn series_for_value_axis(chart_space: &c::ChartSpace, axis_id: i32) -> Vec<ChartSeriesRef<'_>> {
  let mut result = Vec::new();
  for choice in &chart_space.chart.plot_area.plot_area_choice1 {
    match choice {
      c::PlotAreaChoice::AreaChart(chart) if has_axis(&chart.axis_id, axis_id) => {
        result.extend(chart.area_chart_series.iter().map(area_series_ref));
      }
      c::PlotAreaChoice::Area3DChart(chart) if has_axis(&chart.axis_id, axis_id) => {
        result.extend(chart.area_chart_series.iter().map(area_series_ref));
      }
      c::PlotAreaChoice::LineChart(chart) if has_axis(&chart.axis_id, axis_id) => {
        result.extend(chart.line_chart_series.iter().map(line_series_ref));
      }
      c::PlotAreaChoice::Line3DChart(chart) if has_axis(&chart.axis_id, axis_id) => {
        result.extend(chart.line_chart_series.iter().map(line_series_ref));
      }
      c::PlotAreaChoice::StockChart(chart) if has_axis(&chart.axis_id, axis_id) => {
        result.extend(chart.line_chart_series.iter().map(line_series_ref));
      }
      c::PlotAreaChoice::RadarChart(chart) if has_axis(&chart.axis_id, axis_id) => {
        result.extend(chart.radar_chart_series.iter().map(radar_series_ref));
      }
      c::PlotAreaChoice::ScatterChart(chart) if has_axis(&chart.axis_id, axis_id) => {
        result.extend(chart.scatter_chart_series.iter().map(scatter_series_ref));
      }
      c::PlotAreaChoice::BarChart(chart) if has_axis(&chart.axis_id, axis_id) => {
        result.extend(chart.bar_chart_series.iter().map(bar_series_ref));
      }
      c::PlotAreaChoice::Bar3DChart(chart) if has_axis(&chart.axis_id, axis_id) => {
        result.extend(chart.bar_chart_series.iter().map(bar_series_ref));
      }
      c::PlotAreaChoice::SurfaceChart(chart) if has_axis(&chart.axis_id, axis_id) => {
        result.extend(chart.surface_chart_series.iter().map(surface_series_ref));
      }
      c::PlotAreaChoice::Surface3DChart(chart) if has_axis(&chart.axis_id, axis_id) => {
        result.extend(chart.surface_chart_series.iter().map(surface_series_ref));
      }
      c::PlotAreaChoice::BubbleChart(chart) if has_axis(&chart.axis_id, axis_id) => {
        result.extend(chart.bubble_chart_series.iter().map(bubble_series_ref));
      }
      _ => {}
    }
  }
  result
}

fn has_axis(axis_ids: &[c::AxisId], axis_id: i32) -> bool {
  axis_ids.iter().any(|candidate| candidate.val == axis_id)
}

fn value_mode_for_axis(chart_space: &c::ChartSpace, axis_id: i32) -> ChartValueMode {
  let mut mode = ChartValueMode::Standard;
  for choice in &chart_space.chart.plot_area.plot_area_choice1 {
    let candidate = match choice {
      c::PlotAreaChoice::AreaChart(chart) if has_axis(&chart.axis_id, axis_id) => chart
        .grouping
        .as_ref()
        .and_then(|grouping| grouping.val)
        .map(grouping_value_mode),
      c::PlotAreaChoice::Area3DChart(chart) if has_axis(&chart.axis_id, axis_id) => chart
        .grouping
        .as_ref()
        .and_then(|grouping| grouping.val)
        .map(grouping_value_mode),
      c::PlotAreaChoice::LineChart(chart) if has_axis(&chart.axis_id, axis_id) => chart
        .grouping
        .as_ref()
        .and_then(|grouping| grouping.val)
        .map(grouping_value_mode),
      c::PlotAreaChoice::Line3DChart(chart) if has_axis(&chart.axis_id, axis_id) => {
        chart.grouping.val.map(grouping_value_mode)
      }
      c::PlotAreaChoice::BarChart(chart) if has_axis(&chart.axis_id, axis_id) => chart
        .bar_grouping
        .as_ref()
        .and_then(|grouping| grouping.val)
        .map(bar_grouping_value_mode),
      c::PlotAreaChoice::Bar3DChart(chart) if has_axis(&chart.axis_id, axis_id) => chart
        .bar_grouping
        .as_ref()
        .and_then(|grouping| grouping.val)
        .map(bar_grouping_value_mode),
      _ => None,
    };
    if candidate == Some(ChartValueMode::PercentStacked) {
      return ChartValueMode::PercentStacked;
    }
    if candidate == Some(ChartValueMode::Stacked) {
      mode = ChartValueMode::Stacked;
    }
  }
  mode
}

fn grouping_value_mode(grouping: c::GroupingValues) -> ChartValueMode {
  match grouping {
    c::GroupingValues::PercentStacked => ChartValueMode::PercentStacked,
    c::GroupingValues::Stacked => ChartValueMode::Stacked,
    c::GroupingValues::Standard => ChartValueMode::Standard,
  }
}

fn bar_grouping_value_mode(grouping: c::BarGroupingValues) -> ChartValueMode {
  match grouping {
    c::BarGroupingValues::PercentStacked => ChartValueMode::PercentStacked,
    c::BarGroupingValues::Stacked => ChartValueMode::Stacked,
    c::BarGroupingValues::Clustered | c::BarGroupingValues::Standard => ChartValueMode::Standard,
  }
}

fn scale_values(series: &[ChartSeriesRef<'_>], mode: ChartValueMode) -> Vec<f64> {
  if mode == ChartValueMode::PercentStacked {
    return vec![0.0, 1.0];
  }
  let values = series
    .iter()
    .copied()
    .map(chart_series_numeric_values)
    .collect::<Vec<_>>();
  if mode == ChartValueMode::Standard {
    return values.into_iter().flatten().flatten().collect();
  }
  let point_count = values.iter().map(Vec::len).max().unwrap_or(0);
  let mut result = Vec::with_capacity(point_count * 2);
  for point_index in 0..point_count {
    let mut positive = 0.0;
    let mut negative = 0.0;
    for value in values
      .iter()
      .filter_map(|values| values.get(point_index).copied().flatten())
    {
      if value >= 0.0 {
        positive += value;
      } else {
        negative += value;
      }
    }
    result.push(positive);
    if negative < 0.0 {
      result.push(negative);
    }
  }
  result
}

fn chart_series_numeric_values(series: ChartSeriesRef<'_>) -> Vec<Option<f64>> {
  if let Some(values) = series.values {
    return indexed_values(values);
  }
  if let Some(values) = series.y_values {
    let points = match values.y_values_choice.as_ref() {
      Some(c::YValuesChoice::NumberReference(reference)) => reference
        .numbering_cache
        .as_deref()
        .map(|cache| cache.numeric_point.as_slice()),
      Some(c::YValuesChoice::NumberLiteral(literal)) => Some(literal.numeric_point.as_slice()),
      _ => None,
    };
    return points.map(indexed_numeric_values).unwrap_or_default();
  }
  Vec::new()
}

fn chart_series_x_numeric_values(series: ChartSeriesRef<'_>) -> Vec<Option<f64>> {
  let Some(values) = series.x_values else {
    return Vec::new();
  };
  match values.x_values_choice.as_ref() {
    Some(c::XValuesChoice::NumberReference(reference)) => reference
      .numbering_cache
      .as_deref()
      .map(|cache| indexed_numeric_values(&cache.numeric_point))
      .unwrap_or_default(),
    Some(c::XValuesChoice::NumberLiteral(literal)) => {
      indexed_numeric_values(&literal.numeric_point)
    }
    Some(c::XValuesChoice::StringReference(reference)) => reference
      .string_cache
      .as_deref()
      .map(|cache| indexed_string_numeric_values(&cache.string_point))
      .unwrap_or_default(),
    Some(c::XValuesChoice::StringLiteral(literal)) => {
      indexed_string_numeric_values(&literal.string_point)
    }
    Some(c::XValuesChoice::MultiLevelStringReference(reference)) => reference
      .multi_level_string_cache
      .as_deref()
      .and_then(|cache| cache.level.first())
      .map(|level| indexed_string_numeric_values(&level.string_point))
      .unwrap_or_default(),
    None => Vec::new(),
  }
}

fn indexed_bubble_size_values(values: &c::BubbleSize) -> Vec<Option<f64>> {
  match values.bubble_size_choice.as_ref() {
    Some(c::BubbleSizeChoice::NumberReference(reference)) => reference
      .numbering_cache
      .as_deref()
      .map(|cache| indexed_numeric_values(&cache.numeric_point))
      .unwrap_or_default(),
    Some(c::BubbleSizeChoice::NumberLiteral(literal)) => {
      indexed_numeric_values(&literal.numeric_point)
    }
    None => Vec::new(),
  }
}

fn indexed_string_numeric_values(points: &[c::StringPoint]) -> Vec<Option<f64>> {
  let length = points
    .iter()
    .filter_map(|point| usize::try_from(point.index).ok())
    .max()
    .map_or(0, |index| index + 1);
  let mut result = vec![None; length];
  for point in points {
    let Ok(index) = usize::try_from(point.index) else {
      continue;
    };
    result[index] = point.numeric_value.trim().parse::<f64>().ok();
  }
  result
}

fn indexed_numeric_values(points: &[c::NumericPoint]) -> Vec<Option<f64>> {
  let length = points
    .iter()
    .filter_map(|point| usize::try_from(point.index).ok())
    .max()
    .map_or(0, |index| index + 1);
  let mut result = vec![None; length];
  for point in points {
    let Ok(index) = usize::try_from(point.index) else {
      continue;
    };
    result[index] = point.numeric_value.trim().parse::<f64>().ok();
  }
  result
}

pub(crate) fn format_chart_number(value: f64, format_code: Option<&str>) -> String {
  let value = if value.abs() < 1.0e-15 { 0.0 } else { value };
  let code = format_code.unwrap_or("General");
  if code.contains('%') {
    let decimals = format_decimal_places(code.split('%').next().unwrap_or(code));
    return format!("{:.*}%", decimals, value * 100.0);
  }
  let uppercase_code = code.to_ascii_uppercase();
  if uppercase_code.contains("E+")
    || uppercase_code.contains("E-")
    || (value != 0.0 && value.abs() < 1.0e-4)
  {
    return format_chart_scientific(value, format_decimal_places(code));
  }
  if code != "General" && code.contains('.') {
    return format!("{:.*}", format_decimal_places(code), value);
  }
  general_chart_number(value)
}

fn format_decimal_places(code: &str) -> usize {
  code
    .split_once('.')
    .map(|(_, fraction)| {
      fraction
        .chars()
        .take_while(|ch| matches!(ch, '0' | '#'))
        .count()
    })
    .unwrap_or(0)
}

fn format_chart_scientific(value: f64, requested_decimals: usize) -> String {
  if value == 0.0 {
    return "0".to_string();
  }
  let exponent = value.abs().log10().floor() as i32;
  let mantissa = value / 10.0_f64.powi(exponent);
  let mantissa = if requested_decimals == 0 {
    general_chart_number(mantissa)
  } else {
    format!("{mantissa:.requested_decimals$}")
  };
  format!(
    "{mantissa}E{}{absolute:02}",
    if exponent < 0 { '-' } else { '+' },
    absolute = exponent.unsigned_abs()
  )
}

fn format_chart_category_number(value: f64, format_code: Option<&str>, date_1904: bool) -> String {
  let is_date = format_code.is_some_and(|code| {
    let code = code.to_ascii_lowercase();
    code.contains('y') && code.contains('m') && code.contains('d')
  });
  if !is_date {
    return format_chart_number(value, format_code);
  }
  let days_since_unix = value.floor() as i64 - if date_1904 { 24_107 } else { 25_569 };
  let (year, month, day) = civil_from_days(days_since_unix);
  format!("{year}/{month}/{day}")
}

fn civil_from_days(days_since_unix_epoch: i64) -> (i64, u32, u32) {
  let z = days_since_unix_epoch + 719_468;
  let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
  let day_of_era = z - era * 146_097;
  let year_of_era =
    (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
  let mut year = year_of_era + era * 400;
  let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
  let month_prime = (5 * day_of_year + 2) / 153;
  let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
  let month = month_prime + if month_prime < 10 { 3 } else { -9 };
  year += i64::from(month <= 2);
  (year, month as u32, day as u32)
}

fn visible_texts_with_default_series_label(
  chart_space: &c::ChartSpace,
  ui_language: Option<&str>,
  default_label: impl Fn(ChartSeriesRef<'_>, usize) -> String,
) -> Vec<String> {
  let mut texts = Vec::new();
  let mut series_index = 0usize;

  if let Some(title) = chart_space.chart.title.as_deref() {
    push_title_texts(&mut texts, title);
  } else if chart_automatic_title_is_visible(&chart_space.chart) {
    push_unique_text(&mut texts, automatic_chart_title(ui_language));
  }
  for title in axis_titles(chart_space) {
    push_title_texts(&mut texts, title);
  }

  for series in series(chart_space) {
    series_index += 1;
    if let Some(series_text) = series.series_text {
      push_series_text(&mut texts, series_text);
    } else {
      push_unique_text(&mut texts, &default_label(series, series_index));
    }
    if let Some(category_axis_data) = series.category_axis_data {
      push_category_axis_data_texts(&mut texts, category_axis_data);
    }
    if let Some(values) = series.values {
      push_values_texts(&mut texts, values);
    }
    if let Some(y_values) = series.y_values {
      push_y_values_texts(&mut texts, y_values);
    }
    if let Some(x_values) = series.x_values {
      push_x_values_texts(&mut texts, x_values);
    }
    if let Some(bubble_size) = series.bubble_size {
      push_bubble_size_texts(&mut texts, bubble_size);
    }
    if let Some(data_labels) = series.data_labels {
      push_data_label_texts(&mut texts, data_labels);
      push_series_data_label_value_texts(&mut texts, series, data_labels);
    }
  }

  for data_labels in data_labels(chart_space) {
    push_data_label_texts(&mut texts, data_labels);
  }
  texts
}

fn uncached_series_label(_series: ChartSeriesRef<'_>, series_index: usize) -> String {
  // uses STR_DATA_UNNAMED_SERIES_WITH_INDEX ("Series%NUMBER") for imported
  // OOXML series without a cached label sequence.
  format!("Series{series_index}")
}

fn default_series_label(
  series: ChartSeriesRef<'_>,
  series_index: usize,
  ui_language: Option<&str>,
) -> String {
  // uses the localized STR_ROW_LABEL/STR_COLUMN_LABEL defaults when imported
  // chart data has no explicit series label. OOXML bar charts with a horizontal
  // value range map each series to a data row.
  if ui_language.is_some_and(is_chinese_ui_language) {
    return automatic_series_title(ui_language, series_index);
  }
  if let Some(formula) = series_value_formula(series)
    && let Some(range) = parse_chart_a1_range(formula)
    && range.start_col == range.end_col
    && range.start_row != range.end_row
  {
    return format!("Column {series_index}");
  }
  format!("Row {series_index}")
}

fn is_chinese_ui_language(language: &str) -> bool {
  let language = language.to_ascii_lowercase();
  language == "zh"
    || language == "zh-cn"
    || language == "zh-sg"
    || language == "zh-hans"
    || language == "zh-tw"
    || language == "zh-hk"
    || language == "zh-mo"
    || language == "zh-hant"
}

fn series_value_formula(series: ChartSeriesRef<'_>) -> Option<&str> {
  series
    .values
    .and_then(|values| match values.values_choice.as_ref() {
      Some(c::ValuesChoice::NumberReference(reference)) => reference.formula.xml_content.as_deref(),
      _ => None,
    })
    .or_else(|| {
      series
        .y_values
        .and_then(|values| match values.y_values_choice.as_ref() {
          Some(c::YValuesChoice::NumberReference(reference)) => {
            reference.formula.xml_content.as_deref()
          }
          _ => None,
        })
    })
}

#[derive(Clone, Copy)]
struct ChartCellRange {
  start_col: u32,
  start_row: u32,
  end_col: u32,
  end_row: u32,
}

fn parse_chart_a1_range(formula: &str) -> Option<ChartCellRange> {
  let reference = formula.rsplit('!').next().unwrap_or(formula);
  let (start, end) = reference.split_once(':').unwrap_or((reference, reference));
  let (start_col, start_row) = parse_chart_a1_cell(start)?;
  let (end_col, end_row) = parse_chart_a1_cell(end)?;
  Some(ChartCellRange {
    start_col: start_col.min(end_col),
    start_row: start_row.min(end_row),
    end_col: start_col.max(end_col),
    end_row: start_row.max(end_row),
  })
}

fn parse_chart_a1_cell(reference: &str) -> Option<(u32, u32)> {
  let reference = reference.trim().trim_matches('\'').trim_start_matches('$');
  let mut col = 0u32;
  let mut row = 0u32;
  let mut seen_digit = false;
  for ch in reference.chars() {
    if ch == '$' {
      continue;
    }
    if ch.is_ascii_alphabetic() && !seen_digit {
      col = col
        .saturating_mul(26)
        .saturating_add(u32::from(ch.to_ascii_uppercase() as u8 - b'A' + 1));
    } else if ch.is_ascii_digit() {
      seen_digit = true;
      row = row
        .saturating_mul(10)
        .saturating_add(ch.to_digit(10).unwrap_or(0));
    } else {
      return None;
    }
  }
  (col > 0 && row > 0).then_some((col, row))
}

pub fn axis_titles(chart_space: &c::ChartSpace) -> impl Iterator<Item = &c::Title> {
  chart_space
    .chart
    .plot_area
    .plot_area_choice2
    .iter()
    .filter_map(|choice| match choice {
      c::PlotAreaChoice2::ValueAxis(axis) => axis.title.as_deref(),
      c::PlotAreaChoice2::CategoryAxis(axis) => axis.title.as_deref(),
      c::PlotAreaChoice2::DateAxis(axis) => axis.title.as_deref(),
      c::PlotAreaChoice2::SeriesAxis(axis) => axis.title.as_deref(),
    })
}

pub fn series(chart_space: &c::ChartSpace) -> Vec<ChartSeriesRef<'_>> {
  let mut series = Vec::new();
  for choice in &chart_space.chart.plot_area.plot_area_choice1 {
    match choice {
      c::PlotAreaChoice::AreaChart(chart) => {
        series.extend(chart.area_chart_series.iter().map(area_series_ref));
      }
      c::PlotAreaChoice::Area3DChart(chart) => {
        series.extend(chart.area_chart_series.iter().map(area_series_ref));
      }
      c::PlotAreaChoice::LineChart(chart) => {
        series.extend(chart.line_chart_series.iter().map(line_series_ref));
      }
      c::PlotAreaChoice::Line3DChart(chart) => {
        series.extend(chart.line_chart_series.iter().map(line_series_ref));
      }
      c::PlotAreaChoice::StockChart(chart) => {
        series.extend(chart.line_chart_series.iter().map(line_series_ref));
      }
      c::PlotAreaChoice::RadarChart(chart) => {
        series.extend(chart.radar_chart_series.iter().map(radar_series_ref));
      }
      c::PlotAreaChoice::ScatterChart(chart) => {
        series.extend(chart.scatter_chart_series.iter().map(scatter_series_ref));
      }
      c::PlotAreaChoice::PieChart(chart) => {
        series.extend(chart.pie_chart_series.iter().map(pie_series_ref));
      }
      c::PlotAreaChoice::Pie3DChart(chart) => {
        series.extend(chart.pie_chart_series.iter().map(pie_series_ref));
      }
      c::PlotAreaChoice::DoughnutChart(chart) => {
        series.extend(chart.pie_chart_series.iter().map(pie_series_ref));
      }
      c::PlotAreaChoice::BarChart(chart) => {
        series.extend(chart.bar_chart_series.iter().map(bar_series_ref));
      }
      c::PlotAreaChoice::Bar3DChart(chart) => {
        series.extend(chart.bar_chart_series.iter().map(bar_series_ref));
      }
      c::PlotAreaChoice::OfPieChart(chart) => {
        series.extend(chart.pie_chart_series.iter().map(pie_series_ref));
      }
      c::PlotAreaChoice::SurfaceChart(chart) => {
        series.extend(chart.surface_chart_series.iter().map(surface_series_ref));
      }
      c::PlotAreaChoice::Surface3DChart(chart) => {
        series.extend(chart.surface_chart_series.iter().map(surface_series_ref));
      }
      c::PlotAreaChoice::BubbleChart(chart) => {
        series.extend(chart.bubble_chart_series.iter().map(bubble_series_ref));
      }
    }
  }
  series
}

pub fn data_labels(chart_space: &c::ChartSpace) -> impl Iterator<Item = &c::DataLabels> {
  chart_space
    .chart
    .plot_area
    .plot_area_choice1
    .iter()
    .filter_map(|choice| match choice {
      c::PlotAreaChoice::AreaChart(chart) => chart.data_labels.as_deref(),
      c::PlotAreaChoice::Area3DChart(chart) => chart.data_labels.as_deref(),
      c::PlotAreaChoice::LineChart(chart) => chart.data_labels.as_deref(),
      c::PlotAreaChoice::Line3DChart(chart) => chart.data_labels.as_deref(),
      c::PlotAreaChoice::StockChart(chart) => chart.data_labels.as_deref(),
      c::PlotAreaChoice::RadarChart(chart) => chart.data_labels.as_deref(),
      c::PlotAreaChoice::ScatterChart(chart) => chart.data_labels.as_deref(),
      c::PlotAreaChoice::PieChart(chart) => chart.data_labels.as_deref(),
      c::PlotAreaChoice::Pie3DChart(chart) => chart.data_labels.as_deref(),
      c::PlotAreaChoice::DoughnutChart(chart) => chart.data_labels.as_deref(),
      c::PlotAreaChoice::BarChart(chart) => chart.data_labels.as_deref(),
      c::PlotAreaChoice::Bar3DChart(chart) => chart.data_labels.as_deref(),
      c::PlotAreaChoice::OfPieChart(chart) => chart.data_labels.as_deref(),
      c::PlotAreaChoice::SurfaceChart(_) => None,
      c::PlotAreaChoice::Surface3DChart(_) => None,
      c::PlotAreaChoice::BubbleChart(chart) => chart.data_labels.as_deref(),
    })
}

pub fn data_point_solid_fills(chart_space: &c::ChartSpace) -> Vec<ChartDataPointFill<'_>> {
  let mut fills = Vec::new();
  for choice in &chart_space.chart.plot_area.plot_area_choice1 {
    match choice {
      c::PlotAreaChoice::PieChart(chart) => {
        for series in &chart.pie_chart_series {
          collect_data_point_solid_fills(&series.data_point, &mut fills);
        }
      }
      c::PlotAreaChoice::Pie3DChart(chart) => {
        for series in &chart.pie_chart_series {
          collect_data_point_solid_fills(&series.data_point, &mut fills);
        }
      }
      c::PlotAreaChoice::DoughnutChart(chart) => {
        for series in &chart.pie_chart_series {
          collect_data_point_solid_fills(&series.data_point, &mut fills);
        }
      }
      c::PlotAreaChoice::OfPieChart(chart) => {
        for series in &chart.pie_chart_series {
          collect_data_point_solid_fills(&series.data_point, &mut fills);
        }
      }
      c::PlotAreaChoice::BarChart(chart) => {
        for series in &chart.bar_chart_series {
          collect_data_point_solid_fills(&series.data_point, &mut fills);
        }
      }
      c::PlotAreaChoice::Bar3DChart(chart) => {
        for series in &chart.bar_chart_series {
          collect_data_point_solid_fills(&series.data_point, &mut fills);
        }
      }
      _ => {}
    }
  }
  fills.sort_by_key(|fill| fill.index);
  fills
}

fn collect_data_point_solid_fills<'a>(
  data_points: &'a [c::DataPoint],
  fills: &mut Vec<ChartDataPointFill<'a>>,
) {
  for data_point in data_points {
    let Some(fill) = data_point
      .chart_shape_properties
      .as_deref()
      .and_then(|properties| {
        chart_shape_solid_fill(properties).or_else(|| chart_shape_outline_solid_fill(properties))
      })
    else {
      continue;
    };
    fills.push(ChartDataPointFill {
      index: data_point.index.val,
      fill,
    });
  }
}

pub fn chart_shape_solid_fill(properties: &c::ChartShapeProperties) -> Option<&a::SolidFill> {
  match properties.chart_shape_properties_choice2.as_ref()? {
    c::ChartShapePropertiesChoice2::SolidFill(fill) => Some(fill.as_ref()),
    _ => None,
  }
}

pub fn shape_properties_solid_fill(properties: &c::ShapeProperties) -> Option<&a::SolidFill> {
  match properties.shape_properties_choice2.as_ref()? {
    c::ShapePropertiesChoice2::SolidFill(fill) => Some(fill.as_ref()),
    _ => None,
  }
}

pub fn shape_properties_outline_solid_fill(
  properties: &c::ShapeProperties,
) -> Option<&a::SolidFill> {
  match properties.outline.as_deref()?.outline_choice1.as_ref()? {
    a::OutlineChoice::SolidFill(fill) => Some(fill.as_ref()),
    _ => None,
  }
}

/// Returns the solid fill carried by the outline of `c:spPr`.
///
/// Chart gridlines are line objects, so ISO/IEC 29500-1:2016 §21.2.2.90
/// applies their color through `c:spPr/a:ln/a:solidFill`, not through the
/// shape fill directly.
pub fn chart_shape_outline_solid_fill(
  properties: &c::ChartShapeProperties,
) -> Option<&a::SolidFill> {
  match properties.outline.as_deref()?.outline_choice1.as_ref()? {
    a::OutlineChoice::SolidFill(fill) => Some(fill.as_ref()),
    _ => None,
  }
}

fn chart_title_text(chart: &c::Chart) -> Option<ChartTitleText> {
  if let Some(title) = chart.title.as_deref() {
    return explicit_title_text(title).map(ChartTitleText::Explicit);
  }

  chart_automatic_title_is_visible(chart).then_some(ChartTitleText::Automatic)
}

fn explicit_title_text(title: &c::Title) -> Option<String> {
  let mut values = Vec::new();
  push_chart_text(&mut values, title.chart_text.as_deref()?);
  let value = values.join(" ");
  (!value.is_empty()).then_some(value)
}

fn title_text_or_automatic(title: &c::Title, ui_language: Option<&str>) -> Option<String> {
  explicit_title_text(title).or(Some(automatic_axis_title(ui_language).to_string()))
}

pub fn has_powerpoint_automatic_title_placeholder(chart: &c::Chart) -> bool {
  // PowerPoint distinguishes a bare empty title from its generated insertion
  // placeholder. The latter carries both its layout slot and text properties
  // even though c:tx has not been populated yet. Word and Excel do not paint
  // that editing placeholder in fixed output, so this remains host-specific.
  chart.title.as_deref().is_some_and(|title| {
    title.chart_text.is_none() && title.layout.is_some() && title.text_properties.is_some()
  }) && chart_automatic_title_is_visible(chart)
}

pub fn has_word_automatic_title_placeholder(chart: &c::Chart) -> bool {
  // Word fixed output materializes its automatic chart title for an authored
  // empty c:title with autoTitleDeleted=false. Keep this host policy outside
  // the shared title parser because Excel preserves the same markup as empty.
  chart
    .title
    .as_deref()
    .is_some_and(|title| explicit_title_text(title).is_none())
    && chart_automatic_title_is_visible(chart)
}

pub fn has_excel_automatic_title_placeholder(chart: &c::Chart) -> bool {
  // MS-OI29500 §2.1.1431: Office applies autoTitleDeleted only when c:title is
  // absent. An empty c:title is an authored, empty title in Excel fixed
  // output, even when it retains editing text properties.
  chart.title.is_none() && chart_automatic_title_is_visible(chart)
}

fn chart_automatic_title_is_visible(chart: &c::Chart) -> bool {
  // ECMA-376 Part 1 §21.2.2.7: autoTitleDeleted suppresses the automatic
  // title, and an omitted val on a present element defaults to true. Office
  // serializes val="0" when an automatic title is intentionally visible; an
  // absent element does not create a placeholder title.
  chart
    .auto_title_deleted
    .as_ref()
    .and_then(|value| value.val)
    .is_some_and(|value| !value.as_bool())
}

fn series_text_value(series_text: &c::SeriesText) -> String {
  let mut values = Vec::new();
  push_series_text(&mut values, series_text);
  values.join(" ")
}

fn indexed_category_axis_text_values(data: &c::CategoryAxisData) -> Vec<String> {
  match data.category_axis_data_choice.as_ref() {
    Some(c::CategoryAxisDataChoice::StringReference(reference)) => reference
      .string_cache
      .as_deref()
      .map(|cache| indexed_string_points(&cache.string_point))
      .unwrap_or_default(),
    Some(c::CategoryAxisDataChoice::StringLiteral(literal)) => {
      indexed_string_points(&literal.string_point)
    }
    Some(c::CategoryAxisDataChoice::MultiLevelStringReference(reference)) => reference
      .multi_level_string_cache
      .as_deref()
      .map(|cache| {
        let levels = cache
          .level
          .iter()
          .map(|level| indexed_string_points(&level.string_point))
          .collect::<Vec<_>>();
        let count = levels.iter().map(Vec::len).max().unwrap_or(0);
        let mut previous = vec![String::new(); levels.len()];
        (0..count)
          .map(|index| {
            levels
              .iter()
              .enumerate()
              .filter_map(|(level_index, level)| {
                let value = level.get(index)?.trim();
                if value.is_empty() || (level_index > 0 && previous[level_index].as_str() == value)
                {
                  return None;
                }
                previous[level_index] = value.to_string();
                Some(value.to_string())
              })
              .collect::<Vec<_>>()
              .join(" ")
          })
          .collect()
      })
      .unwrap_or_default(),
    Some(c::CategoryAxisDataChoice::NumberReference(reference)) => reference
      .numbering_cache
      .as_deref()
      .map(|cache| {
        indexed_formatted_numeric_point_texts(
          &cache.numeric_point,
          cache
            .format_code
            .as_ref()
            .and_then(|code| code.xml_content.as_deref()),
        )
      })
      .unwrap_or_default(),
    Some(c::CategoryAxisDataChoice::NumberLiteral(literal)) => {
      indexed_formatted_numeric_point_texts(
        &literal.numeric_point,
        literal
          .format_code
          .as_ref()
          .and_then(|code| code.xml_content.as_deref()),
      )
    }
    _ => Vec::new(),
  }
}

fn indexed_values(values: &c::Values) -> Vec<Option<f64>> {
  let points = match values.values_choice.as_ref() {
    Some(c::ValuesChoice::NumberReference(reference)) => reference
      .numbering_cache
      .as_deref()
      .map(|cache| cache.numeric_point.as_slice()),
    Some(c::ValuesChoice::NumberLiteral(literal)) => Some(literal.numeric_point.as_slice()),
    None => None,
  };
  let Some(points) = points else {
    return Vec::new();
  };
  let length = points
    .iter()
    .filter_map(|point| usize::try_from(point.index).ok())
    .max()
    .map_or(0, |index| index + 1);
  let mut result = vec![None; length];
  for point in points {
    let Ok(index) = usize::try_from(point.index) else {
      continue;
    };
    result[index] = point.numeric_value.trim().parse::<f64>().ok();
  }
  result
}

fn indexed_string_points(points: &[c::StringPoint]) -> Vec<String> {
  let length = points
    .iter()
    .filter_map(|point| usize::try_from(point.index).ok())
    .max()
    .map_or(0, |index| index + 1);
  let mut result = vec![String::new(); length];
  for point in points {
    let Ok(index) = usize::try_from(point.index) else {
      continue;
    };
    result[index] = point.numeric_value.trim().to_string();
  }
  result
}

fn indexed_numeric_point_texts(points: &[c::NumericPoint]) -> Vec<String> {
  let length = points
    .iter()
    .filter_map(|point| usize::try_from(point.index).ok())
    .max()
    .map_or(0, |index| index + 1);
  let mut result = vec![String::new(); length];
  for point in points {
    let Ok(index) = usize::try_from(point.index) else {
      continue;
    };
    result[index] = point.numeric_value.trim().to_string();
  }
  result
}

fn indexed_formatted_numeric_point_texts(
  points: &[c::NumericPoint],
  format_code: Option<&str>,
) -> Vec<String> {
  let mut result = indexed_numeric_point_texts(points);
  for value in &mut result {
    let Ok(number) = value.parse::<f64>() else {
      continue;
    };
    *value = format_chart_category_number(number, format_code, false);
  }
  result
}

pub fn scheme_color_token(
  color_map: Option<&c::ColorMapOverride>,
  token: a::SchemeColorValues,
) -> Option<a::ColorSchemeIndexValues> {
  if let Some(color_map) = color_map {
    return match token {
      a::SchemeColorValues::Background1 => Some(color_map.background1),
      a::SchemeColorValues::Text1 => Some(color_map.text1),
      a::SchemeColorValues::Background2 => Some(color_map.background2),
      a::SchemeColorValues::Text2 => Some(color_map.text2),
      a::SchemeColorValues::Accent1 => Some(color_map.accent1),
      a::SchemeColorValues::Accent2 => Some(color_map.accent2),
      a::SchemeColorValues::Accent3 => Some(color_map.accent3),
      a::SchemeColorValues::Accent4 => Some(color_map.accent4),
      a::SchemeColorValues::Accent5 => Some(color_map.accent5),
      a::SchemeColorValues::Accent6 => Some(color_map.accent6),
      a::SchemeColorValues::Hyperlink => Some(color_map.hyperlink),
      a::SchemeColorValues::FollowedHyperlink => Some(color_map.followed_hyperlink),
      a::SchemeColorValues::Dark1 => Some(a::ColorSchemeIndexValues::Dark1),
      a::SchemeColorValues::Light1 => Some(a::ColorSchemeIndexValues::Light1),
      a::SchemeColorValues::Dark2 => Some(a::ColorSchemeIndexValues::Dark2),
      a::SchemeColorValues::Light2 => Some(a::ColorSchemeIndexValues::Light2),
      a::SchemeColorValues::PhColor => None,
    };
  }
  match token {
    a::SchemeColorValues::Background1 | a::SchemeColorValues::Light1 => {
      Some(a::ColorSchemeIndexValues::Light1)
    }
    a::SchemeColorValues::Text1 | a::SchemeColorValues::Dark1 => {
      Some(a::ColorSchemeIndexValues::Dark1)
    }
    a::SchemeColorValues::Background2 | a::SchemeColorValues::Light2 => {
      Some(a::ColorSchemeIndexValues::Light2)
    }
    a::SchemeColorValues::Text2 | a::SchemeColorValues::Dark2 => {
      Some(a::ColorSchemeIndexValues::Dark2)
    }
    a::SchemeColorValues::Accent1 => Some(a::ColorSchemeIndexValues::Accent1),
    a::SchemeColorValues::Accent2 => Some(a::ColorSchemeIndexValues::Accent2),
    a::SchemeColorValues::Accent3 => Some(a::ColorSchemeIndexValues::Accent3),
    a::SchemeColorValues::Accent4 => Some(a::ColorSchemeIndexValues::Accent4),
    a::SchemeColorValues::Accent5 => Some(a::ColorSchemeIndexValues::Accent5),
    a::SchemeColorValues::Accent6 => Some(a::ColorSchemeIndexValues::Accent6),
    a::SchemeColorValues::Hyperlink => Some(a::ColorSchemeIndexValues::Hyperlink),
    a::SchemeColorValues::FollowedHyperlink => Some(a::ColorSchemeIndexValues::FollowedHyperlink),
    a::SchemeColorValues::PhColor => None,
  }
}

fn area_series_ref(ser: &c::AreaChartSeries) -> ChartSeriesRef<'_> {
  ChartSeriesRef {
    series_text: ser.series_text.as_deref(),
    category_axis_data: ser.category_axis_data.as_deref(),
    values: ser.values.as_deref(),
    y_values: None,
    x_values: None,
    bubble_size: None,
    data_labels: ser.data_labels.as_deref(),
    chart_shape_properties: ser.chart_shape_properties.as_deref(),
    data_points: &ser.data_point,
    marker: None,
    smooth: None,
    trendlines: &ser.trendline,
  }
}

fn line_series_ref(ser: &c::LineChartSeries) -> ChartSeriesRef<'_> {
  ChartSeriesRef {
    series_text: ser.series_text.as_deref(),
    category_axis_data: ser.category_axis_data.as_deref(),
    values: ser.values.as_deref(),
    y_values: None,
    x_values: None,
    bubble_size: None,
    data_labels: ser.data_labels.as_deref(),
    chart_shape_properties: ser.chart_shape_properties.as_deref(),
    data_points: &ser.data_point,
    marker: ser.marker.as_deref(),
    smooth: ser.smooth.as_ref(),
    trendlines: &ser.trendline,
  }
}

fn radar_series_ref(ser: &c::RadarChartSeries) -> ChartSeriesRef<'_> {
  ChartSeriesRef {
    series_text: ser.series_text.as_deref(),
    category_axis_data: ser.category_axis_data.as_deref(),
    values: ser.values.as_deref(),
    y_values: None,
    x_values: None,
    bubble_size: None,
    data_labels: ser.data_labels.as_deref(),
    chart_shape_properties: ser.chart_shape_properties.as_deref(),
    data_points: &ser.data_point,
    marker: ser.marker.as_deref(),
    smooth: None,
    trendlines: &[],
  }
}

fn scatter_series_ref(ser: &c::ScatterChartSeries) -> ChartSeriesRef<'_> {
  ChartSeriesRef {
    series_text: ser.series_text.as_deref(),
    category_axis_data: None,
    values: None,
    y_values: ser.y_values.as_deref(),
    x_values: ser.x_values.as_deref(),
    bubble_size: None,
    data_labels: ser.data_labels.as_deref(),
    chart_shape_properties: ser.chart_shape_properties.as_deref(),
    data_points: &ser.data_point,
    marker: ser.marker.as_deref(),
    smooth: ser.smooth.as_ref(),
    trendlines: &ser.trendline,
  }
}

fn pie_series_ref(ser: &c::PieChartSeries) -> ChartSeriesRef<'_> {
  ChartSeriesRef {
    series_text: ser.series_text.as_deref(),
    category_axis_data: ser.category_axis_data.as_deref(),
    values: ser.values.as_deref(),
    y_values: None,
    x_values: None,
    bubble_size: None,
    data_labels: ser.data_labels.as_deref(),
    chart_shape_properties: ser.chart_shape_properties.as_deref(),
    data_points: &ser.data_point,
    marker: None,
    smooth: None,
    trendlines: &[],
  }
}

fn bar_series_ref(ser: &c::BarChartSeries) -> ChartSeriesRef<'_> {
  ChartSeriesRef {
    series_text: ser.series_text.as_deref(),
    category_axis_data: ser.category_axis_data.as_deref(),
    values: ser.values.as_deref(),
    y_values: None,
    x_values: None,
    bubble_size: None,
    data_labels: ser.data_labels.as_deref(),
    chart_shape_properties: ser.chart_shape_properties.as_deref(),
    data_points: &ser.data_point,
    marker: None,
    smooth: None,
    trendlines: &ser.trendline,
  }
}

fn surface_series_ref(ser: &c::SurfaceChartSeries) -> ChartSeriesRef<'_> {
  ChartSeriesRef {
    series_text: ser.series_text.as_deref(),
    category_axis_data: ser.category_axis_data.as_deref(),
    values: ser.values.as_deref(),
    y_values: None,
    x_values: None,
    bubble_size: None,
    data_labels: None,
    chart_shape_properties: ser.chart_shape_properties.as_deref(),
    data_points: &[],
    marker: None,
    smooth: None,
    trendlines: &[],
  }
}

fn bubble_series_ref(ser: &c::BubbleChartSeries) -> ChartSeriesRef<'_> {
  ChartSeriesRef {
    series_text: ser.series_text.as_deref(),
    category_axis_data: None,
    values: None,
    y_values: ser.y_values.as_deref(),
    x_values: ser.x_values.as_deref(),
    bubble_size: ser.bubble_size.as_deref(),
    data_labels: ser.data_labels.as_deref(),
    chart_shape_properties: ser.chart_shape_properties.as_deref(),
    data_points: &ser.data_point,
    marker: None,
    smooth: None,
    trendlines: &ser.trendline,
  }
}

fn push_title_texts(texts: &mut Vec<String>, title: &c::Title) {
  if let Some(chart_text) = title.chart_text.as_deref() {
    push_chart_text(texts, chart_text);
  }
}

fn push_series_text(texts: &mut Vec<String>, series_text: &c::SeriesText) {
  match series_text.series_text_choice.as_ref() {
    Some(c::SeriesTextChoice::StringReference(reference)) => {
      push_string_reference_texts(texts, reference);
    }
    Some(c::SeriesTextChoice::NumericValue(value)) => push_unique_text(texts, value),
    None => {}
  }
}

fn push_chart_text(texts: &mut Vec<String>, chart_text: &c::ChartText) {
  match chart_text.chart_text_choice.as_ref() {
    Some(c::ChartTextChoice::StringReference(reference)) => {
      push_string_reference_texts(texts, reference)
    }
    Some(c::ChartTextChoice::StringLiteral(literal)) => push_string_literal_texts(texts, literal),
    Some(c::ChartTextChoice::RichText(rich)) => push_rich_texts(texts, &rich.paragraph),
    None => {}
  }
}

fn push_rich_texts(texts: &mut Vec<String>, paragraphs: &[a::Paragraph]) {
  for paragraph in paragraphs {
    let mut text = String::new();
    for choice in &paragraph.paragraph_choice {
      match choice {
        a::ParagraphChoice::Run(run) => text.push_str(&run.text),
        a::ParagraphChoice::Field(field) => {
          if let Some(value) = field.text.as_deref() {
            text.push_str(value);
          }
        }
        a::ParagraphChoice::Break(_) => {}
        a::ParagraphChoice::TextMath(math) => text.push_str(&text_math_text(math)),
        a::ParagraphChoice::AlternateContent(_) => {}
      }
    }
    push_unique_text(texts, &text);
  }
}

fn push_category_axis_data_texts(texts: &mut Vec<String>, data: &c::CategoryAxisData) {
  match data.category_axis_data_choice.as_ref() {
    Some(c::CategoryAxisDataChoice::MultiLevelStringReference(reference)) => {
      if let Some(cache) = reference.multi_level_string_cache.as_deref() {
        for level in &cache.level {
          for point in &level.string_point {
            push_unique_text(texts, &point.numeric_value);
          }
        }
      }
    }
    Some(c::CategoryAxisDataChoice::NumberReference(reference)) => {
      push_number_reference_texts(texts, reference);
    }
    Some(c::CategoryAxisDataChoice::NumberLiteral(literal)) => {
      push_number_literal_texts(texts, literal)
    }
    Some(c::CategoryAxisDataChoice::StringReference(reference)) => {
      push_string_reference_texts(texts, reference);
    }
    Some(c::CategoryAxisDataChoice::StringLiteral(literal)) => {
      push_string_literal_texts(texts, literal)
    }
    None => {}
  }
}

fn push_values_texts(texts: &mut Vec<String>, values: &c::Values) {
  match values.values_choice.as_ref() {
    Some(c::ValuesChoice::NumberReference(reference)) => {
      push_number_reference_texts(texts, reference)
    }
    Some(c::ValuesChoice::NumberLiteral(literal)) => push_number_literal_texts(texts, literal),
    None => {}
  }
}

fn push_y_values_texts(texts: &mut Vec<String>, values: &c::YValues) {
  match values.y_values_choice.as_ref() {
    Some(c::YValuesChoice::NumberReference(reference)) => {
      push_number_reference_texts(texts, reference)
    }
    Some(c::YValuesChoice::NumberLiteral(literal)) => push_number_literal_texts(texts, literal),
    None => {}
  }
}

fn push_x_values_texts(texts: &mut Vec<String>, values: &c::XValues) {
  match values.x_values_choice.as_ref() {
    Some(c::XValuesChoice::MultiLevelStringReference(reference)) => {
      if let Some(cache) = reference.multi_level_string_cache.as_deref() {
        for level in &cache.level {
          for point in &level.string_point {
            push_unique_text(texts, &point.numeric_value);
          }
        }
      }
    }
    Some(c::XValuesChoice::NumberReference(reference)) => {
      push_number_reference_texts(texts, reference)
    }
    Some(c::XValuesChoice::NumberLiteral(literal)) => push_number_literal_texts(texts, literal),
    Some(c::XValuesChoice::StringReference(reference)) => {
      push_string_reference_texts(texts, reference)
    }
    Some(c::XValuesChoice::StringLiteral(literal)) => push_string_literal_texts(texts, literal),
    None => {}
  }
}

fn push_bubble_size_texts(texts: &mut Vec<String>, values: &c::BubbleSize) {
  match values.bubble_size_choice.as_ref() {
    Some(c::BubbleSizeChoice::NumberReference(reference)) => {
      push_number_reference_texts(texts, reference)
    }
    Some(c::BubbleSizeChoice::NumberLiteral(literal)) => push_number_literal_texts(texts, literal),
    None => {}
  }
}

fn push_data_label_texts(texts: &mut Vec<String>, data_labels: &c::DataLabels) {
  for label in &data_labels.data_label {
    if let Some(sequence) = label
      .data_label_choice
      .iter()
      .find_map(|choice| match choice {
        c::DataLabelChoice::Sequence(sequence) => Some(sequence.as_ref()),
        _ => None,
      })
      && let Some(chart_text) = sequence.chart_text.as_deref()
    {
      push_chart_text(texts, chart_text);
    }
  }
}

fn push_series_data_label_value_texts(
  texts: &mut Vec<String>,
  series: ChartSeriesRef<'_>,
  data_labels: &c::DataLabels,
) {
  let categories = series
    .category_axis_data
    .map(category_axis_text_values)
    .unwrap_or_default();
  let values = series.values.map(values_text_values).unwrap_or_default();
  if categories.is_empty() || values.is_empty() {
    return;
  }
  let group = match data_labels.data_labels_choice.as_ref() {
    Some(c::DataLabelsChoice::Sequence(sequence)) => Some(sequence),
    _ => None,
  };
  for label in &data_labels.data_label {
    let label_sequence = label
      .data_label_choice
      .iter()
      .find_map(|choice| match choice {
        c::DataLabelChoice::Sequence(sequence) => Some(sequence.as_ref()),
        _ => None,
      });
    let show_category = label_sequence
      .and_then(|sequence| sequence.show_category_name.as_ref())
      .and_then(|show| show.val)
      .or_else(|| {
        group
          .and_then(|sequence| sequence.show_category_name.as_ref())
          .and_then(|show| show.val)
      })
      .is_some_and(|value| value.as_bool());
    let show_value = label_sequence
      .and_then(|sequence| sequence.show_value.as_ref())
      .and_then(|show| show.val)
      .or_else(|| {
        group
          .and_then(|sequence| sequence.show_value.as_ref())
          .and_then(|show| show.val)
      })
      .is_some_and(|value| value.as_bool());
    if !show_category || !show_value {
      continue;
    }
    let index = label.index.val as usize;
    let Some(category) = categories.get(index) else {
      continue;
    };
    let Some(value) = values.get(index) else {
      continue;
    };
    let separator = label_sequence
      .and_then(|sequence| sequence.separator.as_deref())
      .or_else(|| group.and_then(|sequence| sequence.separator.as_deref()))
      .unwrap_or("; ");
    push_unique_text(texts, &format!("{category}{separator}{value}"));
  }
}

fn category_axis_text_values(data: &c::CategoryAxisData) -> Vec<String> {
  match data.category_axis_data_choice.as_ref() {
    Some(c::CategoryAxisDataChoice::StringReference(reference)) => reference
      .string_cache
      .as_deref()
      .map(string_cache_text_values)
      .unwrap_or_default(),
    Some(c::CategoryAxisDataChoice::StringLiteral(literal)) => literal
      .string_point
      .iter()
      .map(|point| point.numeric_value.trim().to_string())
      .collect(),
    _ => Vec::new(),
  }
}

fn values_text_values(values: &c::Values) -> Vec<String> {
  match values.values_choice.as_ref() {
    Some(c::ValuesChoice::NumberReference(reference)) => reference
      .numbering_cache
      .as_deref()
      .map(numbering_cache_text_values)
      .unwrap_or_default(),
    Some(c::ValuesChoice::NumberLiteral(literal)) => literal
      .numeric_point
      .iter()
      .map(|point| point.numeric_value.trim().to_string())
      .collect(),
    None => Vec::new(),
  }
}

fn string_cache_text_values(cache: &c::StringCache) -> Vec<String> {
  cache
    .string_point
    .iter()
    .map(|point| point.numeric_value.trim().to_string())
    .collect()
}

fn numbering_cache_text_values(cache: &c::NumberingCache) -> Vec<String> {
  cache
    .numeric_point
    .iter()
    .map(|point| point.numeric_value.trim().to_string())
    .collect()
}

fn push_string_reference_texts(texts: &mut Vec<String>, reference: &c::StringReference) {
  if let Some(cache) = reference.string_cache.as_deref() {
    push_string_cache_texts(texts, cache);
  }
}

fn push_string_cache_texts(texts: &mut Vec<String>, cache: &c::StringCache) {
  for point in &cache.string_point {
    push_unique_text(texts, &point.numeric_value);
  }
}

fn push_string_literal_texts(texts: &mut Vec<String>, literal: &c::StringLiteral) {
  for point in &literal.string_point {
    push_unique_text(texts, &point.numeric_value);
  }
}

fn push_number_reference_texts(texts: &mut Vec<String>, reference: &c::NumberReference) {
  if let Some(cache) = reference.numbering_cache.as_deref() {
    push_numbering_cache_texts(texts, cache);
  }
}

fn push_numbering_cache_texts(texts: &mut Vec<String>, cache: &c::NumberingCache) {
  for point in &cache.numeric_point {
    push_unique_text(texts, &point.numeric_value);
  }
}

fn push_number_literal_texts(texts: &mut Vec<String>, literal: &c::NumberLiteral) {
  for point in &literal.numeric_point {
    push_unique_text(texts, &point.numeric_value);
  }
}

fn push_unique_text(texts: &mut Vec<String>, value: &str) {
  let trimmed = value.trim();
  if trimmed.is_empty() || texts.iter().any(|text| text == trimmed) {
    return;
  }
  texts.push(trimmed.to_string());
}

#[cfg(test)]
mod tests {
  use super::{
    ChartTitleText, automatic_chart_title, automatic_series_title, chart_title_text,
    clustered_column_chart, clustered_column_slot, fixed_output_latin_font_family,
    fixed_output_texts_for_ui_language, format_chart_number, linear_axis_scale,
    ordinary_clustered_column_chart, pie_chart_model,
  };
  use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_chart as c;
  use ooxmlsdk::sdk::SdkType;

  #[test]
  fn automatic_title_uses_the_output_ui_language_not_the_chart_editing_language() {
    assert_eq!(automatic_chart_title(Some("zh-CN")), "图表标题");
    assert_eq!(automatic_chart_title(Some("zh-TW")), "圖表標題");
    assert_eq!(automatic_chart_title(Some("en-US")), "Chart Title");
  }

  #[test]
  fn automatic_series_title_uses_the_output_ui_language() {
    assert_eq!(automatic_series_title(Some("zh-CN"), 1), "系列 1");
    assert_eq!(automatic_series_title(Some("zh-TW"), 2), "數列 2");
    assert_eq!(automatic_series_title(Some("en-US"), 3), "Series 3");
  }

  #[test]
  fn general_number_format_uses_short_decimal_output_not_scientific_notation() {
    assert_eq!(format_chart_number(30.8, None), "30.8");
    assert_eq!(format_chart_number(66.79, Some("General")), "66.79");
    assert_eq!(format_chart_number(2.0e-9, Some("0.0E+00")), "2.0E-09");
  }

  #[test]
  fn fixed_output_prefers_chart_local_latin_typeface() {
    let chart_space = c::ChartSpace::from_bytes(
      br#"<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><c:chart><c:plotArea/></c:chart><c:txPr><a:bodyPr/><a:lstStyle/><a:p><a:pPr><a:defRPr><a:latin typeface="Arial"/></a:defRPr></a:pPr></a:p></c:txPr></c:chartSpace>"#,
    )
    .expect("chart space");

    assert_eq!(fixed_output_latin_font_family(&chart_space), Some("Arial"));
  }

  #[test]
  fn pie_percent_labels_use_largest_remainder_rounding() {
    let chart_space = c::ChartSpace::from_bytes(
      br#"<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"><c:chart><c:plotArea><c:pieChart><c:ser><c:idx val="0"/><c:order val="0"/><c:dLbls><c:showVal val="0"/><c:showCatName val="1"/><c:showPercent val="1"/></c:dLbls><c:cat><c:strLit><c:pt idx="0"><c:v>A</c:v></c:pt><c:pt idx="1"><c:v>B</c:v></c:pt><c:pt idx="2"><c:v>C</c:v></c:pt></c:strLit></c:cat><c:val><c:numLit><c:pt idx="0"><c:v>1</c:v></c:pt><c:pt idx="1"><c:v>1</c:v></c:pt><c:pt idx="2"><c:v>1</c:v></c:pt></c:numLit></c:val></c:ser></c:pieChart></c:plotArea></c:chart></c:chartSpace>"#,
    )
    .expect("chart space");

    assert_eq!(
      fixed_output_texts_for_ui_language(&chart_space, None),
      ["A 34%", "B 33%", "C 33%"]
    );
  }

  #[test]
  fn single_series_pie_uses_its_series_name_as_the_automatic_title() {
    let chart_space = c::ChartSpace::from_bytes(
      br#"<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"><c:chart><c:title/><c:autoTitleDeleted val="0"/><c:plotArea><c:pieChart><c:ser><c:idx val="0"/><c:order val="0"/><c:tx><c:v>Sales</c:v></c:tx><c:val><c:numLit><c:pt idx="0"><c:v>1</c:v></c:pt></c:numLit></c:val></c:ser></c:pieChart></c:plotArea></c:chart></c:chartSpace>"#,
    )
    .expect("chart space");

    let pie = pie_chart_model(&chart_space).expect("pie chart");
    assert_eq!(
      pie.title,
      Some(ChartTitleText::Explicit("Sales".to_string()))
    );
  }

  #[test]
  fn office_pie_model_uses_only_the_first_series_and_schema_defaults() {
    let chart_space = c::ChartSpace::from_bytes(
      br#"<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><c:chart><c:plotArea><c:pieChart><c:varyColors/><c:ser><c:idx val="0"/><c:order val="0"/><c:spPr><a:solidFill><a:srgbClr val="FFFF00"/></a:solidFill></c:spPr><c:dPt><c:idx val="1"/><c:spPr><a:solidFill><a:srgbClr val="FF0000"/></a:solidFill></c:spPr></c:dPt><c:cat><c:strLit><c:pt idx="0"><c:v>A</c:v></c:pt><c:pt idx="1"><c:v>B</c:v></c:pt></c:strLit></c:cat><c:val><c:numLit><c:pt idx="0"><c:v>3</c:v></c:pt><c:pt idx="1"><c:v>1</c:v></c:pt></c:numLit></c:val></c:ser><c:ser><c:idx val="1"/><c:order val="1"/><c:cat><c:strLit><c:pt idx="0"><c:v>Hidden</c:v></c:pt></c:strLit></c:cat><c:val><c:numLit><c:pt idx="0"><c:v>99</c:v></c:pt></c:numLit></c:val></c:ser></c:pieChart></c:plotArea><c:legend><c:legendEntry><c:idx val="1"/><c:delete/></c:legendEntry></c:legend></c:chart></c:chartSpace>"#,
    )
    .expect("chart space");

    let pie = pie_chart_model(&chart_space).expect("pie chart");
    assert_eq!(pie.categories, ["A", "B"]);
    assert_eq!(pie.values, [Some(3.0), Some(1.0)]);
    assert_eq!(pie.first_slice_angle_deg, 0.0);
    assert_eq!(pie.legend_position, Some(super::ChartLegendPosition::Right));
    assert_eq!(pie.visible_legend_indices, [0]);
    assert_eq!(
      fixed_output_texts_for_ui_language(&chart_space, None),
      ["A"]
    );
    assert!(!pie.vary_colors);
    assert_eq!(pie.data_point_fills.len(), 1);
  }

  #[test]
  fn pie_point_custom_text_replaces_the_inherited_value_label() {
    let chart_space = c::ChartSpace::from_bytes(
      br#"<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><c:chart><c:autoTitleDeleted val="1"/><c:plotArea><c:pieChart><c:ser><c:idx val="0"/><c:order val="0"/><c:dLbls><c:dLbl><c:idx val="1"/><c:tx><c:rich><a:bodyPr/><a:lstStyle/><a:p><a:r><a:t>custom</a:t></a:r></a:p></c:rich></c:tx><c:showVal val="1"/></c:dLbl><c:showVal val="1"/></c:dLbls><c:cat><c:strLit><c:pt idx="0"><c:v>A</c:v></c:pt><c:pt idx="1"><c:v>B</c:v></c:pt></c:strLit></c:cat><c:val><c:numLit><c:pt idx="0"><c:v>1</c:v></c:pt><c:pt idx="1"><c:v>2</c:v></c:pt></c:numLit></c:val></c:ser></c:pieChart></c:plotArea></c:chart></c:chartSpace>"#,
    )
    .expect("chart space");

    let pie = pie_chart_model(&chart_space).expect("pie chart");
    assert_eq!(
      pie
        .data_labels
        .iter()
        .map(|label| (label.point_index, label.text.as_str()))
        .collect::<Vec<_>>(),
      [(0, "1"), (1, "custom")]
    );
    assert_eq!(
      fixed_output_texts_for_ui_language(&chart_space, None),
      ["1", "custom"]
    );
  }

  #[test]
  fn pie_custom_percentage_field_resolves_from_the_point_share() {
    let chart_space = c::ChartSpace::from_bytes(
      br#"<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><c:chart><c:plotArea><c:pieChart><c:ser><c:idx val="0"/><c:order val="0"/><c:dLbls><c:dLbl><c:idx val="0"/><c:tx><c:rich><a:bodyPr/><a:lstStyle/><a:p><a:fld id="{00000000-0000-0000-0000-000000000000}" type="PERCENTAGE"><a:t>[PERCENTAGE]</a:t></a:fld></a:p></c:rich></c:tx><c:showPercent val="1"/></c:dLbl><c:showPercent val="1"/></c:dLbls><c:val><c:numLit><c:pt idx="0"><c:v>4.3</c:v></c:pt></c:numLit></c:val></c:ser></c:pieChart></c:plotArea></c:chart></c:chartSpace>"#,
    )
    .expect("chart space");

    let pie = pie_chart_model(&chart_space).expect("pie chart");
    assert_eq!(pie.data_labels[0].text, "100%");
  }

  #[test]
  fn fixed_output_ignores_later_pie_series_like_office() {
    let chart_space = c::ChartSpace::from_bytes(
      br#"<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"><c:chart><c:plotArea><c:pieChart><c:ser><c:idx val="0"/><c:order val="0"/><c:cat><c:strLit><c:pt idx="0"><c:v>Visible</c:v></c:pt></c:strLit></c:cat><c:val><c:numLit><c:pt idx="0"><c:v>1</c:v></c:pt></c:numLit></c:val></c:ser><c:ser><c:idx val="1"/><c:order val="1"/><c:dLbls><c:showSerName val="1"/></c:dLbls><c:tx><c:v>Hidden series</c:v></c:tx><c:cat><c:strLit><c:pt idx="0"><c:v>Hidden category</c:v></c:pt></c:strLit></c:cat><c:val><c:numLit><c:pt idx="0"><c:v>2</c:v></c:pt></c:numLit></c:val></c:ser></c:pieChart></c:plotArea><c:legend/></c:chart></c:chartSpace>"#,
    )
    .expect("chart space");

    assert_eq!(
      fixed_output_texts_for_ui_language(&chart_space, None),
      ["Visible"]
    );
  }

  #[test]
  fn automatic_title_uses_empty_title_placeholders_and_absent_title_marker() {
    let chart_space = c::ChartSpace::from_bytes(
      br#"<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"><c:chart><c:autoTitleDeleted val="0"/><c:plotArea/></c:chart></c:chartSpace>"#,
    )
    .expect("chart space");
    assert_eq!(
      chart_title_text(&chart_space.chart),
      Some(ChartTitleText::Automatic)
    );

    let empty_title = c::ChartSpace::from_bytes(
      br#"<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"><c:chart><c:title/><c:autoTitleDeleted val="0"/><c:plotArea/></c:chart></c:chartSpace>"#,
    )
    .expect("chart space");
    assert_eq!(chart_title_text(&empty_title.chart), None);

    let placeholder_title = c::ChartSpace::from_bytes(
      br#"<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><c:chart><c:title><c:layout/><c:txPr><a:bodyPr/><a:lstStyle/><a:p/></c:txPr></c:title><c:autoTitleDeleted val="0"/><c:plotArea/></c:chart></c:chartSpace>"#,
    )
    .expect("chart space");
    assert_eq!(chart_title_text(&placeholder_title.chart), None);
    assert!(super::has_powerpoint_automatic_title_placeholder(
      &placeholder_title.chart
    ));
    assert!(super::has_word_automatic_title_placeholder(
      &placeholder_title.chart
    ));
    assert!(!super::has_excel_automatic_title_placeholder(
      &placeholder_title.chart
    ));

    let omitted_marker = c::ChartSpace::from_bytes(
      br#"<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"><c:chart><c:plotArea/></c:chart></c:chartSpace>"#,
    )
    .expect("chart space");
    assert_eq!(chart_title_text(&omitted_marker.chart), None);
  }

  #[test]
  fn ordinary_chart_accepts_only_non_painting_chart_and_plot_area_properties() {
    let inert = c::ChartSpace::from_bytes(
      br#"<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><c:chart><c:plotArea><c:barChart><c:barDir val="col"/><c:ser><c:idx val="0"/><c:order val="0"/><c:val><c:numLit><c:pt idx="0"><c:v>1</c:v></c:pt></c:numLit></c:val></c:ser></c:barChart><c:spPr><a:noFill/><a:ln w="12700"><a:noFill/></a:ln></c:spPr></c:plotArea></c:chart><c:spPr><a:noFill/><a:ln><a:noFill/></a:ln></c:spPr></c:chartSpace>"#,
    )
    .expect("chart space");
    assert!(ordinary_clustered_column_chart(&inert).is_some());

    let painted = c::ChartSpace::from_bytes(
      br#"<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><c:chart><c:plotArea><c:barChart><c:barDir val="col"/><c:ser><c:idx val="0"/><c:order val="0"/><c:val><c:numLit><c:pt idx="0"><c:v>1</c:v></c:pt></c:numLit></c:val></c:ser></c:barChart></c:plotArea></c:chart><c:spPr><a:solidFill><a:srgbClr val="FF0000"/></a:solidFill></c:spPr></c:chartSpace>"#,
    )
    .expect("chart space");
    assert!(ordinary_clustered_column_chart(&painted).is_none());
  }

  #[test]
  fn series_data_label_settings_expand_to_points_before_point_deletes() {
    let chart_space = c::ChartSpace::from_bytes(
      br#"<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"><c:chart><c:plotArea><c:barChart><c:barDir val="col"/><c:ser><c:idx val="0"/><c:order val="0"/><c:tx><c:v>Revenue</c:v></c:tx><c:dLbls><c:dLbl><c:idx val="1"/><c:delete val="1"/></c:dLbl><c:dLblPos val="outEnd"/><c:showVal val="1"/><c:showCatName val="1"/><c:separator>, </c:separator></c:dLbls><c:cat><c:strLit><c:pt idx="0"><c:v>A</c:v></c:pt><c:pt idx="1"><c:v>B</c:v></c:pt></c:strLit></c:cat><c:val><c:numLit><c:pt idx="0"><c:v>1</c:v></c:pt><c:pt idx="1"><c:v>2</c:v></c:pt></c:numLit></c:val></c:ser></c:barChart></c:plotArea></c:chart></c:chartSpace>"#,
    )
    .expect("chart space");

    let chart = clustered_column_chart(&chart_space).expect("clustered chart");
    assert_eq!(chart.series[0].data_labels.len(), 1);
    assert_eq!(chart.series[0].data_labels[0].point_index, 0);
    assert_eq!(chart.series[0].data_labels[0].text, "A, 1");
  }

  #[test]
  fn linear_scale_expands_a_wide_positive_range_to_zero_and_past_border_value() {
    let scale = linear_axis_scale(
      [4.3, 2.5, 3.5, 4.5, 2.4, 4.4, 1.8, 2.8, 2.0, 3.0, 5.0],
      None,
      10,
    )
    .expect("finite values produce a scale");

    assert_eq!(scale.minimum, 0.0);
    assert_eq!(scale.maximum, 6.0);
    assert_eq!(scale.major_unit, 1.0);
  }

  #[test]
  fn linear_scale_expands_a_narrow_positive_range_toward_zero_before_alignment() {
    let scale = linear_axis_scale([0.3578542, 0.3578431, 0.3578942, 0.3578425], None, 10)
      .expect("finite values produce a scale");

    assert!((scale.minimum - 0.35781).abs() < 1.0e-12);
    assert!((scale.maximum - 0.35790).abs() < 1.0e-12);
    assert!((scale.major_unit - 0.00001).abs() < 1.0e-12);
  }

  #[test]
  fn clustered_column_slots_follow_gap_and_overlap_distances() {
    let first = clustered_column_slot(0, 0, 4, 3, 219.0, -27.0).expect("valid slot");
    let second = clustered_column_slot(0, 1, 4, 3, 219.0, -27.0).expect("valid slot");
    let next_category = clustered_column_slot(1, 0, 4, 3, 219.0, -27.0).expect("valid slot");

    assert!((first.width - 0.25 / 5.73).abs() < 1.0e-12);
    assert!((second.center - first.center - first.width * 1.27).abs() < 1.0e-12);
    assert!((next_category.center - first.center - 0.25).abs() < 1.0e-12);
  }
}
