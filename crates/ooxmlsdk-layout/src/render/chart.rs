use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_chart as c;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use crate::render::math::text_math_text;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChartKind {
  Pie,
  Bar,
  Area,
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

#[derive(Clone, Debug)]
pub struct ClusteredColumnSeries<'a> {
  pub name: String,
  pub values: Vec<Option<f64>>,
  pub solid_fill: Option<&'a a::SolidFill>,
  pub data_point_fills: Vec<ChartDataPointFill<'a>>,
  pub data_labels: Vec<ClusteredColumnDataLabel>,
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
  pub categories: Vec<String>,
  pub series: Vec<ClusteredColumnSeries<'a>>,
  pub gap_width_percent: f64,
  pub overlap_percent: f64,
  pub value_axis: Option<&'a c::ValueAxis>,
  pub legend_position: Option<ChartLegendPosition>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LinearAxisScale {
  pub minimum: f64,
  pub maximum: f64,
  pub major_unit: f64,
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

/// Extracts the first ordinary two-dimensional clustered column chart.
///
/// Cached category/value sequences are data sources, not inherently visible
/// text.  Keeping them in a typed chart model lets each renderer decide which
/// labels are visible from the chart/axis/data-label settings.
pub fn clustered_column_chart(chart_space: &c::ChartSpace) -> Option<ClusteredColumnChart<'_>> {
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
      .unwrap_or_else(|| default_series_label(series_ref, series_index + 1));
    if categories.is_empty() {
      categories = source
        .category_axis_data
        .as_deref()
        .map(indexed_category_axis_text_values)
        .unwrap_or_default();
    }
    let values = source
      .values
      .as_deref()
      .map(indexed_values)
      .unwrap_or_default();
    let solid_fill = source
      .chart_shape_properties
      .as_deref()
      .and_then(chart_shape_solid_fill);
    let mut data_point_fills = Vec::new();
    collect_data_point_solid_fills(&source.data_point, &mut data_point_fills);
    data_point_fills.sort_by_key(|fill| fill.index);
    let data_labels = clustered_column_data_labels(source, &values);
    series.push(ClusteredColumnSeries {
      name,
      values,
      solid_fill,
      data_point_fills,
      data_labels,
    });
  }

  let title = chart_title_text(&chart_space.chart);
  let value_axis = chart_space
    .chart
    .plot_area
    .plot_area_choice2
    .iter()
    .find_map(|choice| match choice {
      c::PlotAreaChoice2::ValueAxis(axis) => Some(axis.as_ref()),
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
    value_axis,
    legend_position,
  })
}

fn clustered_column_data_labels(
  series: &c::BarChartSeries,
  values: &[Option<f64>],
) -> Vec<ClusteredColumnDataLabel> {
  let Some(labels) = series.data_labels.as_deref() else {
    return Vec::new();
  };
  let group = match labels.data_labels_choice.as_ref() {
    Some(c::DataLabelsChoice::Sequence(sequence)) => Some(sequence.as_ref()),
    _ => None,
  };
  let mut result = Vec::new();
  for label in &labels.data_label {
    if label.data_label_choice.iter().any(|choice| {
      matches!(choice, c::DataLabelChoice::Delete(delete) if delete.val.is_none_or(|value| value.as_bool()))
    }) {
      continue;
    }
    let sequence = label
      .data_label_choice
      .iter()
      .find_map(|choice| match choice {
        c::DataLabelChoice::Sequence(sequence) => Some(sequence.as_ref()),
        _ => None,
      });
    let Ok(point_index) = usize::try_from(label.index.val) else {
      continue;
    };
    let Some(value) = values.get(point_index).copied().flatten() else {
      continue;
    };
    let show_value = sequence
      .and_then(|sequence| sequence.show_value.as_ref())
      .and_then(|show| show.val)
      .or_else(|| {
        group
          .and_then(|sequence| sequence.show_value.as_ref())
          .and_then(|show| show.val)
      })
      .is_some_and(|value| value.as_bool());
    let chart_text = sequence.and_then(|sequence| sequence.chart_text.as_deref());
    let text = chart_text
      .map(|text| data_label_chart_text(text, value))
      .filter(|text| !text.is_empty())
      .or_else(|| show_value.then(|| general_chart_number(value)));
    let Some(text) = text else {
      continue;
    };
    let position = sequence
      .and_then(|sequence| sequence.data_label_position.as_ref())
      .or_else(|| group.and_then(|sequence| sequence.data_label_position.as_ref()))
      .map(|position| position.val)
      .unwrap_or(c::DataLabelPositionValues::OutsideEnd);
    result.push(ClusteredColumnDataLabel {
      point_index,
      text,
      position,
    });
  }
  result.sort_by_key(|label| label.point_index);
  result
}

fn data_label_chart_text(chart_text: &c::ChartText, value: f64) -> String {
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
        a::ParagraphChoice::Field(field)
          if field
            .r#type
            .as_deref()
            .is_some_and(|field_type| field_type.eq_ignore_ascii_case("VALUE")) =>
        {
          result.push_str(&general_chart_number(value));
        }
        a::ParagraphChoice::Field(field) => {
          if let Some(text) = field.text.as_deref() {
            result.push_str(text);
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
  if value.fract().abs() < 1.0e-12 {
    format!("{value:.0}")
  } else {
    let mut result = format!("{value:.15}");
    while result.ends_with('0') {
      result.pop();
    }
    if result.ends_with('.') {
      result.pop();
    }
    result
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
  let mut temporary_minimum = explicit_minimum.unwrap_or(source_minimum);
  let mut temporary_maximum = explicit_maximum.unwrap_or(source_maximum);
  if temporary_minimum > temporary_maximum {
    std::mem::swap(&mut temporary_minimum, &mut temporary_maximum);
  }
  if explicit_minimum.is_none()
    && temporary_minimum > 0.0
    && (temporary_minimum == temporary_maximum || temporary_minimum / temporary_maximum < 5.0 / 6.0)
  {
    temporary_minimum = 0.0;
  }
  if temporary_minimum == temporary_maximum {
    if temporary_maximum == 0.0 {
      temporary_maximum = 1.0;
    } else {
      temporary_maximum *= 2.0;
    }
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
    .find_map(|choice| match choice {
      c::PlotAreaChoice::PieChart(_)
      | c::PlotAreaChoice::Pie3DChart(_)
      | c::PlotAreaChoice::DoughnutChart(_)
      | c::PlotAreaChoice::OfPieChart(_) => Some(ChartKind::Pie),
      c::PlotAreaChoice::BarChart(_) | c::PlotAreaChoice::Bar3DChart(_) => Some(ChartKind::Bar),
      c::PlotAreaChoice::AreaChart(_) | c::PlotAreaChoice::Area3DChart(_) => Some(ChartKind::Area),
      _ => None,
    })
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
  visible_texts_with_default_series_label(chart_space, default_series_label)
}

pub fn visible_texts_with_uncached_series_labels(chart_space: &c::ChartSpace) -> Vec<String> {
  visible_texts_with_default_series_label(chart_space, uncached_series_label)
}

fn visible_texts_with_default_series_label(
  chart_space: &c::ChartSpace,
  default_label: fn(ChartSeriesRef<'_>, usize) -> String,
) -> Vec<String> {
  let mut texts = Vec::new();
  let mut series_index = 0usize;

  if let Some(title) = chart_space.chart.title.as_deref() {
    push_title_texts(&mut texts, title);
    if title.chart_text.is_none()
      && chart_space
        .chart
        .auto_title_deleted
        .as_ref()
        .and_then(|value| value.val)
        .is_none_or(|value| !value.as_bool())
    {
      push_unique_text(&mut texts, "Chart Title");
    }
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

fn default_series_label(series: ChartSeriesRef<'_>, series_index: usize) -> String {
  // uses the localized STR_ROW_LABEL/STR_COLUMN_LABEL defaults when imported
  // chart data has no explicit series label. OOXML bar charts with a horizontal
  // value range map each series to a data row.
  if let Some(formula) = series_value_formula(series)
    && let Some(range) = parse_chart_a1_range(formula)
    && range.start_col == range.end_col
    && range.start_row != range.end_row
  {
    return format!("Column {series_index}");
  }
  format!("Row {series_index}")
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
    let Some(c::ChartShapePropertiesChoice2::SolidFill(fill)) = data_point
      .chart_shape_properties
      .as_deref()
      .and_then(|properties| properties.chart_shape_properties_choice2.as_ref())
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

fn chart_title_text(chart: &c::Chart) -> Option<ChartTitleText> {
  let title = chart.title.as_deref()?;
  if let Some(chart_text) = title.chart_text.as_deref() {
    let mut values = Vec::new();
    push_chart_text(&mut values, chart_text);
    let value = values.join(" ");
    if !value.is_empty() {
      return Some(ChartTitleText::Explicit(value));
    }
  }
  chart
    .auto_title_deleted
    .as_ref()
    .and_then(|value| value.val)
    .is_none_or(|value| !value.as_bool())
    .then_some(ChartTitleText::Automatic)
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
    Some(c::CategoryAxisDataChoice::NumberReference(reference)) => reference
      .numbering_cache
      .as_deref()
      .map(|cache| indexed_numeric_point_texts(&cache.numeric_point))
      .unwrap_or_default(),
    Some(c::CategoryAxisDataChoice::NumberLiteral(literal)) => {
      indexed_numeric_point_texts(&literal.numeric_point)
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
  if let Some(c::DataLabelsChoice::Sequence(sequence)) = data_labels.data_labels_choice.as_ref()
    && sequence
      .show_percent
      .as_ref()
      .and_then(|show| show.val)
      .is_some_and(|value| value.as_bool())
  {
    push_unique_text(texts, "100%");
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
  use super::{automatic_chart_title, clustered_column_slot, linear_axis_scale};

  #[test]
  fn automatic_title_uses_the_output_ui_language_not_the_chart_editing_language() {
    assert_eq!(automatic_chart_title(Some("zh-CN")), "图表标题");
    assert_eq!(automatic_chart_title(Some("zh-TW")), "圖表標題");
    assert_eq!(automatic_chart_title(Some("en-US")), "Chart Title");
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
  fn clustered_column_slots_follow_gap_and_overlap_distances() {
    let first = clustered_column_slot(0, 0, 4, 3, 219.0, -27.0).expect("valid slot");
    let second = clustered_column_slot(0, 1, 4, 3, 219.0, -27.0).expect("valid slot");
    let next_category = clustered_column_slot(1, 0, 4, 3, 219.0, -27.0).expect("valid slot");

    assert!((first.width - 0.25 / 5.73).abs() < 1.0e-12);
    assert!((second.center - first.center - first.width * 1.27).abs() < 1.0e-12);
    assert!((next_category.center - first.center - 0.25).abs() < 1.0e-12);
  }
}
