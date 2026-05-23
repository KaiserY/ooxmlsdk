use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_chart as c;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ChartKind {
  Pie,
  Bar,
  Area,
  Other,
}

#[derive(Clone, Copy)]
pub(crate) struct ChartSeriesRef<'a> {
  pub(crate) series_text: Option<&'a c::SeriesText>,
  pub(crate) category_axis_data: Option<&'a c::CategoryAxisData>,
  pub(crate) values: Option<&'a c::Values>,
  pub(crate) y_values: Option<&'a c::YValues>,
  pub(crate) x_values: Option<&'a c::XValues>,
  pub(crate) bubble_size: Option<&'a c::BubbleSize>,
  pub(crate) data_labels: Option<&'a c::DataLabels>,
}

#[derive(Clone, Copy)]
pub(crate) struct ChartDataPointFill<'a> {
  pub(crate) index: u32,
  pub(crate) fill: &'a a::SolidFill,
}

pub(crate) fn kind(chart_space: &c::ChartSpace) -> ChartKind {
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

pub(crate) fn has_values(chart_space: &c::ChartSpace, expected: &[&str]) -> bool {
  let values = visible_texts(chart_space);
  expected
    .iter()
    .all(|expected| values.iter().any(|value| value == expected))
}

pub(crate) fn has_vertical_multilevel_category_axis(chart_space: &c::ChartSpace) -> bool {
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

pub(crate) fn visible_texts(chart_space: &c::ChartSpace) -> Vec<String> {
  let mut texts = Vec::new();
  let mut series_index = 0usize;

  if let Some(title) = chart_space.chart.title.as_deref() {
    push_title_texts(&mut texts, title);
  }
  for title in axis_titles(chart_space) {
    push_title_texts(&mut texts, title);
  }

  for series in series(chart_space) {
    series_index += 1;
    if let Some(series_text) = series.series_text {
      push_series_text(&mut texts, series_text);
    } else {
      texts.push(format!("Series{series_index}"));
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
    }
  }

  for data_labels in data_labels(chart_space) {
    push_data_label_texts(&mut texts, data_labels);
  }
  texts
}

pub(crate) fn axis_titles(chart_space: &c::ChartSpace) -> impl Iterator<Item = &c::Title> {
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

pub(crate) fn series(chart_space: &c::ChartSpace) -> Vec<ChartSeriesRef<'_>> {
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

pub(crate) fn data_labels(chart_space: &c::ChartSpace) -> impl Iterator<Item = &c::DataLabels> {
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

pub(crate) fn data_point_solid_fills(chart_space: &c::ChartSpace) -> Vec<ChartDataPointFill<'_>> {
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

pub(crate) fn scheme_color_token(
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
        a::ParagraphChoice::Break(_) | a::ParagraphChoice::TextMath(_) => {}
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
    if let Some(c::DataLabelChoice::Sequence(sequence)) = label.data_label_choice.as_ref()
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
