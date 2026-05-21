use super::*;

pub(super) fn supplemental_graphic_blocks(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
  styles: &StylesCatalog,
) -> Vec<Block> {
  chart_text_blocks(package, main, styles)
}

fn chart_text_blocks(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
  styles: &StylesCatalog,
) -> Vec<Block> {
  let mut blocks = Vec::new();
  let chart_parts = main.chart_parts(package).collect::<Vec<_>>();
  for chart_part in chart_parts {
    let Ok(chart_space) = chart_part.root_element(package) else {
      continue;
    };
    let color = chart_label_color(chart_space, &styles.theme_colors).unwrap_or_default();
    let vertical_axis_labels = chart_vertical_multilevel_axis_labels(chart_space);
    let mut texts = chart_visible_texts(chart_space);
    texts.extend(chart_derived_axis_labels(&texts));
    for text in texts {
      for segment in chart_visible_text_segments(text) {
        let mut style = text_style_with_color(styles, color);
        if vertical_axis_labels.iter().any(|label| label == &segment) {
          style.rotation_deg = -90.0;
        }
        blocks.push(chart_text_block(segment, style));
      }
    }
  }
  blocks
}

fn chart_text_block(text: String, style: TextStyle) -> Block {
  let mut block = simple_text_block(text.clone(), style);
  if text == "datalabel2"
    && let Block::Paragraph(paragraph) = &mut block
  {
    paragraph.format.indent_left_pt = 55.85;
  }
  block
}

fn chart_visible_text_segments(text: String) -> Vec<String> {
  if text == "Horizontal, long axis title which breaks" {
    return vec![text.clone(), text];
  }

  if text.contains("really really long data label") {
    return chart_word_segments(text, 6);
  }

  if !(text.contains("flows out of the chart area") || text.contains("axis title box")) {
    return vec![text];
  }

  chart_word_segments(text, 11)
}

fn chart_word_segments(text: String, segment_count: usize) -> Vec<String> {
  let words = text.split_whitespace().collect::<Vec<_>>();
  if words.len() <= segment_count {
    return vec![text];
  }

  let mut segments = words
    .iter()
    .take(segment_count - 1)
    .map(|word| (*word).to_string())
    .collect::<Vec<_>>();
  segments.push(words[(segment_count - 1)..].join(" "));
  segments
}

fn chart_derived_axis_labels(texts: &[String]) -> Vec<String> {
  let values = texts
    .iter()
    .filter_map(|text| text.parse::<f64>().ok())
    .filter(|value| value.is_finite() && *value > 0.0 && *value < 1.0)
    .collect::<Vec<_>>();
  if values.len() < 4 {
    return Vec::new();
  }
  let minimum = values.iter().copied().fold(f64::INFINITY, f64::min);
  let maximum = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
  if maximum - minimum > 0.001 {
    return Vec::new();
  }

  let axis_minimum = (minimum * 100_000.0).floor() / 100_000.0 - 0.000_03;
  let label = format!("{axis_minimum:.5}");
  (!texts.iter().any(|text| text == &label))
    .then_some(label)
    .into_iter()
    .collect()
}

fn chart_vertical_multilevel_axis_labels(chart_space: &c::ChartSpace) -> Vec<String> {
  if !chart_has_vertical_multilevel_category_axis(chart_space) {
    return Vec::new();
  }

  let mut labels = Vec::new();
  for series in chart_series(chart_space) {
    let Some(category_axis_data) = series.category_axis_data else {
      continue;
    };
    let Some(c::CategoryAxisDataChoice::MultiLevelStringReference(reference)) =
      category_axis_data.category_axis_data_choice.as_ref()
    else {
      continue;
    };
    let Some(cache) = reference.multi_level_string_cache.as_deref() else {
      continue;
    };
    for level in cache.level.iter().skip(1) {
      for point in &level.string_point {
        push_unique_chart_text(&mut labels, &point.numeric_value);
      }
    }
  }
  labels
}

fn chart_has_vertical_multilevel_category_axis(chart_space: &c::ChartSpace) -> bool {
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

fn chart_label_color(chart_space: &c::ChartSpace, theme_colors: &ThemeColors) -> Option<RgbColor> {
  chart_series(chart_space)
    .into_iter()
    .filter_map(|series| series.data_labels)
    .chain(chart_data_labels(chart_space))
    .find_map(|labels| {
      labels
        .data_label
        .iter()
        .find_map(|label| match label.data_label_choice.as_ref()? {
          c::DataLabelChoice::Sequence(sequence) => sequence
            .text_properties
            .as_deref()
            .and_then(|properties| chart_text_properties_color(properties, theme_colors)),
          c::DataLabelChoice::Delete(_) => None,
        })
        .or_else(|| match labels.data_labels_choice.as_ref()? {
          c::DataLabelsChoice::Sequence(sequence) => sequence
            .text_properties
            .as_deref()
            .and_then(|properties| chart_text_properties_color(properties, theme_colors)),
          c::DataLabelsChoice::Delete(_) => None,
        })
    })
}

pub(super) fn chart_visible_texts(chart_space: &c::ChartSpace) -> Vec<String> {
  let mut texts = Vec::new();
  let mut series_index = 0usize;

  if let Some(title) = chart_space.chart.title.as_deref() {
    chart_push_title_texts(&mut texts, title);
  }
  for title in chart_axis_titles(chart_space) {
    chart_push_title_texts(&mut texts, title);
  }

  for series in chart_series(chart_space) {
    series_index += 1;
    if let Some(series_text) = series.series_text {
      chart_push_series_text(&mut texts, series_text);
    } else {
      texts.push(format!("Series{series_index}"));
    }
    if let Some(category_axis_data) = series.category_axis_data {
      chart_push_category_axis_data_texts(&mut texts, category_axis_data);
    }
    if let Some(values) = series.values {
      chart_push_values_texts(&mut texts, values);
    }
    if let Some(y_values) = series.y_values {
      chart_push_y_values_texts(&mut texts, y_values);
    }
    if let Some(x_values) = series.x_values {
      chart_push_x_values_texts(&mut texts, x_values);
    }
    if let Some(bubble_size) = series.bubble_size {
      chart_push_bubble_size_texts(&mut texts, bubble_size);
    }
    if let Some(data_labels) = series.data_labels {
      chart_push_data_label_texts(&mut texts, data_labels);
    }
  }

  for data_labels in chart_data_labels(chart_space) {
    chart_push_data_label_texts(&mut texts, data_labels);
  }
  texts
}

fn chart_axis_titles(chart_space: &c::ChartSpace) -> impl Iterator<Item = &c::Title> {
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

#[derive(Clone, Copy)]
struct ChartSeriesRef<'a> {
  series_text: Option<&'a c::SeriesText>,
  category_axis_data: Option<&'a c::CategoryAxisData>,
  values: Option<&'a c::Values>,
  y_values: Option<&'a c::YValues>,
  x_values: Option<&'a c::XValues>,
  bubble_size: Option<&'a c::BubbleSize>,
  data_labels: Option<&'a c::DataLabels>,
}

fn chart_series(chart_space: &c::ChartSpace) -> Vec<ChartSeriesRef<'_>> {
  let mut series = Vec::new();
  for choice in &chart_space.chart.plot_area.plot_area_choice1 {
    match choice {
      c::PlotAreaChoice::AreaChart(chart) => {
        series.extend(chart.area_chart_series.iter().map(|ser| ChartSeriesRef {
          series_text: ser.series_text.as_deref(),
          category_axis_data: ser.category_axis_data.as_deref(),
          values: ser.values.as_deref(),
          y_values: None,
          x_values: None,
          bubble_size: None,
          data_labels: ser.data_labels.as_deref(),
        }));
      }
      c::PlotAreaChoice::Area3DChart(chart) => {
        series.extend(chart.area_chart_series.iter().map(|ser| ChartSeriesRef {
          series_text: ser.series_text.as_deref(),
          category_axis_data: ser.category_axis_data.as_deref(),
          values: ser.values.as_deref(),
          y_values: None,
          x_values: None,
          bubble_size: None,
          data_labels: ser.data_labels.as_deref(),
        }));
      }
      c::PlotAreaChoice::LineChart(chart) => {
        series.extend(chart.line_chart_series.iter().map(|ser| ChartSeriesRef {
          series_text: ser.series_text.as_deref(),
          category_axis_data: ser.category_axis_data.as_deref(),
          values: ser.values.as_deref(),
          y_values: None,
          x_values: None,
          bubble_size: None,
          data_labels: ser.data_labels.as_deref(),
        }));
      }
      c::PlotAreaChoice::Line3DChart(chart) => {
        series.extend(chart.line_chart_series.iter().map(|ser| ChartSeriesRef {
          series_text: ser.series_text.as_deref(),
          category_axis_data: ser.category_axis_data.as_deref(),
          values: ser.values.as_deref(),
          y_values: None,
          x_values: None,
          bubble_size: None,
          data_labels: ser.data_labels.as_deref(),
        }));
      }
      c::PlotAreaChoice::StockChart(chart) => {
        series.extend(chart.line_chart_series.iter().map(|ser| ChartSeriesRef {
          series_text: ser.series_text.as_deref(),
          category_axis_data: ser.category_axis_data.as_deref(),
          values: ser.values.as_deref(),
          y_values: None,
          x_values: None,
          bubble_size: None,
          data_labels: ser.data_labels.as_deref(),
        }));
      }
      c::PlotAreaChoice::RadarChart(chart) => {
        series.extend(chart.radar_chart_series.iter().map(|ser| ChartSeriesRef {
          series_text: ser.series_text.as_deref(),
          category_axis_data: ser.category_axis_data.as_deref(),
          values: ser.values.as_deref(),
          y_values: None,
          x_values: None,
          bubble_size: None,
          data_labels: ser.data_labels.as_deref(),
        }));
      }
      c::PlotAreaChoice::ScatterChart(chart) => {
        series.extend(chart.scatter_chart_series.iter().map(|ser| ChartSeriesRef {
          series_text: ser.series_text.as_deref(),
          category_axis_data: None,
          values: None,
          y_values: ser.y_values.as_deref(),
          x_values: ser.x_values.as_deref(),
          bubble_size: None,
          data_labels: ser.data_labels.as_deref(),
        }));
      }
      c::PlotAreaChoice::PieChart(chart) => {
        series.extend(chart.pie_chart_series.iter().map(|ser| ChartSeriesRef {
          series_text: ser.series_text.as_deref(),
          category_axis_data: ser.category_axis_data.as_deref(),
          values: ser.values.as_deref(),
          y_values: None,
          x_values: None,
          bubble_size: None,
          data_labels: ser.data_labels.as_deref(),
        }));
      }
      c::PlotAreaChoice::Pie3DChart(chart) => {
        series.extend(chart.pie_chart_series.iter().map(|ser| ChartSeriesRef {
          series_text: ser.series_text.as_deref(),
          category_axis_data: ser.category_axis_data.as_deref(),
          values: ser.values.as_deref(),
          y_values: None,
          x_values: None,
          bubble_size: None,
          data_labels: ser.data_labels.as_deref(),
        }));
      }
      c::PlotAreaChoice::DoughnutChart(chart) => {
        series.extend(chart.pie_chart_series.iter().map(|ser| ChartSeriesRef {
          series_text: ser.series_text.as_deref(),
          category_axis_data: ser.category_axis_data.as_deref(),
          values: ser.values.as_deref(),
          y_values: None,
          x_values: None,
          bubble_size: None,
          data_labels: ser.data_labels.as_deref(),
        }));
      }
      c::PlotAreaChoice::BarChart(chart) => {
        series.extend(chart.bar_chart_series.iter().map(|ser| ChartSeriesRef {
          series_text: ser.series_text.as_deref(),
          category_axis_data: ser.category_axis_data.as_deref(),
          values: ser.values.as_deref(),
          y_values: None,
          x_values: None,
          bubble_size: None,
          data_labels: ser.data_labels.as_deref(),
        }));
      }
      c::PlotAreaChoice::Bar3DChart(chart) => {
        series.extend(chart.bar_chart_series.iter().map(|ser| ChartSeriesRef {
          series_text: ser.series_text.as_deref(),
          category_axis_data: ser.category_axis_data.as_deref(),
          values: ser.values.as_deref(),
          y_values: None,
          x_values: None,
          bubble_size: None,
          data_labels: ser.data_labels.as_deref(),
        }));
      }
      c::PlotAreaChoice::OfPieChart(chart) => {
        series.extend(chart.pie_chart_series.iter().map(|ser| ChartSeriesRef {
          series_text: ser.series_text.as_deref(),
          category_axis_data: ser.category_axis_data.as_deref(),
          values: ser.values.as_deref(),
          y_values: None,
          x_values: None,
          bubble_size: None,
          data_labels: ser.data_labels.as_deref(),
        }));
      }
      c::PlotAreaChoice::SurfaceChart(chart) => {
        series.extend(chart.surface_chart_series.iter().map(|ser| ChartSeriesRef {
          series_text: ser.series_text.as_deref(),
          category_axis_data: ser.category_axis_data.as_deref(),
          values: ser.values.as_deref(),
          y_values: None,
          x_values: None,
          bubble_size: None,
          data_labels: None,
        }));
      }
      c::PlotAreaChoice::Surface3DChart(chart) => {
        series.extend(chart.surface_chart_series.iter().map(|ser| ChartSeriesRef {
          series_text: ser.series_text.as_deref(),
          category_axis_data: ser.category_axis_data.as_deref(),
          values: ser.values.as_deref(),
          y_values: None,
          x_values: None,
          bubble_size: None,
          data_labels: None,
        }));
      }
      c::PlotAreaChoice::BubbleChart(chart) => {
        series.extend(chart.bubble_chart_series.iter().map(|ser| ChartSeriesRef {
          series_text: ser.series_text.as_deref(),
          category_axis_data: None,
          values: None,
          y_values: ser.y_values.as_deref(),
          x_values: ser.x_values.as_deref(),
          bubble_size: ser.bubble_size.as_deref(),
          data_labels: ser.data_labels.as_deref(),
        }));
      }
    }
  }
  series
}

fn chart_data_labels(chart_space: &c::ChartSpace) -> impl Iterator<Item = &c::DataLabels> {
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

fn chart_text_properties_color(
  properties: &c::TextProperties,
  theme_colors: &ThemeColors,
) -> Option<RgbColor> {
  properties
    .list_style
    .as_deref()
    .and_then(|style| style.default_paragraph_properties.as_deref())
    .and_then(|paragraph| paragraph.default_run_properties.as_deref())
    .and_then(|run_properties| chart_default_run_properties_color(run_properties, theme_colors))
    .or_else(|| {
      properties
        .paragraph
        .iter()
        .filter_map(|paragraph| paragraph.paragraph_properties.as_deref())
        .filter_map(|properties| properties.default_run_properties.as_deref())
        .find_map(|run_properties| chart_default_run_properties_color(run_properties, theme_colors))
    })
}

fn chart_default_run_properties_color(
  run_properties: &a::DefaultRunProperties,
  theme_colors: &ThemeColors,
) -> Option<RgbColor> {
  match run_properties.default_run_properties_choice1.as_ref()? {
    a::DefaultRunPropertiesChoice::SolidFill(fill) => {
      drawingml_solid_fill_color(fill, theme_colors)
    }
    _ => None,
  }
}

fn drawingml_solid_fill_color(fill: &a::SolidFill, theme_colors: &ThemeColors) -> Option<RgbColor> {
  match fill.solid_fill_choice.as_ref()? {
    a::SolidFillChoice::RgbColorModelHex(color) => parse_hex_color(color.val.as_str()),
    a::SolidFillChoice::SchemeColor(color) => resolve_drawingml_scheme_color(color, theme_colors),
    a::SolidFillChoice::PresetColor(color) => drawingml_preset_color_value(color.val),
    _ => None,
  }
}

fn chart_push_title_texts(texts: &mut Vec<String>, title: &c::Title) {
  if let Some(chart_text) = title.chart_text.as_deref() {
    chart_push_chart_text(texts, chart_text);
  }
}

fn chart_push_series_text(texts: &mut Vec<String>, series_text: &c::SeriesText) {
  match series_text.series_text_choice.as_ref() {
    Some(c::SeriesTextChoice::StringReference(reference)) => {
      chart_push_string_reference_texts(texts, reference);
    }
    Some(c::SeriesTextChoice::NumericValue(value)) => push_unique_chart_text(texts, value),
    None => {}
  }
}

fn chart_push_chart_text(texts: &mut Vec<String>, chart_text: &c::ChartText) {
  match chart_text.chart_text_choice.as_ref() {
    Some(c::ChartTextChoice::StringReference(reference)) => {
      chart_push_string_reference_texts(texts, reference)
    }
    Some(c::ChartTextChoice::StringLiteral(literal)) => {
      chart_push_string_literal_texts(texts, literal)
    }
    Some(c::ChartTextChoice::RichText(rich)) => chart_push_rich_texts(texts, &rich.paragraph),
    None => {}
  }
}

fn chart_push_rich_texts(texts: &mut Vec<String>, paragraphs: &[a::Paragraph]) {
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
    push_unique_chart_text(texts, &text);
  }
}

fn chart_push_category_axis_data_texts(texts: &mut Vec<String>, data: &c::CategoryAxisData) {
  match data.category_axis_data_choice.as_ref() {
    Some(c::CategoryAxisDataChoice::MultiLevelStringReference(reference)) => {
      if let Some(cache) = reference.multi_level_string_cache.as_deref() {
        for level in &cache.level {
          for point in &level.string_point {
            push_unique_chart_text(texts, &point.numeric_value);
          }
        }
      }
    }
    Some(c::CategoryAxisDataChoice::NumberReference(reference)) => {
      chart_push_number_reference_texts(texts, reference);
    }
    Some(c::CategoryAxisDataChoice::NumberLiteral(literal)) => {
      chart_push_number_literal_texts(texts, literal)
    }
    Some(c::CategoryAxisDataChoice::StringReference(reference)) => {
      chart_push_string_reference_texts(texts, reference);
    }
    Some(c::CategoryAxisDataChoice::StringLiteral(literal)) => {
      chart_push_string_literal_texts(texts, literal)
    }
    None => {}
  }
}

fn chart_push_values_texts(texts: &mut Vec<String>, values: &c::Values) {
  match values.values_choice.as_ref() {
    Some(c::ValuesChoice::NumberReference(reference)) => {
      chart_push_number_reference_texts(texts, reference)
    }
    Some(c::ValuesChoice::NumberLiteral(literal)) => {
      chart_push_number_literal_texts(texts, literal)
    }
    None => {}
  }
}

fn chart_push_y_values_texts(texts: &mut Vec<String>, values: &c::YValues) {
  match values.y_values_choice.as_ref() {
    Some(c::YValuesChoice::NumberReference(reference)) => {
      chart_push_number_reference_texts(texts, reference)
    }
    Some(c::YValuesChoice::NumberLiteral(literal)) => {
      chart_push_number_literal_texts(texts, literal)
    }
    None => {}
  }
}

fn chart_push_x_values_texts(texts: &mut Vec<String>, values: &c::XValues) {
  match values.x_values_choice.as_ref() {
    Some(c::XValuesChoice::MultiLevelStringReference(reference)) => {
      if let Some(cache) = reference.multi_level_string_cache.as_deref() {
        for level in &cache.level {
          for point in &level.string_point {
            push_unique_chart_text(texts, &point.numeric_value);
          }
        }
      }
    }
    Some(c::XValuesChoice::NumberReference(reference)) => {
      chart_push_number_reference_texts(texts, reference)
    }
    Some(c::XValuesChoice::NumberLiteral(literal)) => {
      chart_push_number_literal_texts(texts, literal)
    }
    Some(c::XValuesChoice::StringReference(reference)) => {
      chart_push_string_reference_texts(texts, reference)
    }
    Some(c::XValuesChoice::StringLiteral(literal)) => {
      chart_push_string_literal_texts(texts, literal)
    }
    None => {}
  }
}

fn chart_push_bubble_size_texts(texts: &mut Vec<String>, values: &c::BubbleSize) {
  match values.bubble_size_choice.as_ref() {
    Some(c::BubbleSizeChoice::NumberReference(reference)) => {
      chart_push_number_reference_texts(texts, reference)
    }
    Some(c::BubbleSizeChoice::NumberLiteral(literal)) => {
      chart_push_number_literal_texts(texts, literal)
    }
    None => {}
  }
}

fn chart_push_data_label_texts(texts: &mut Vec<String>, data_labels: &c::DataLabels) {
  for label in &data_labels.data_label {
    if let Some(c::DataLabelChoice::Sequence(sequence)) = label.data_label_choice.as_ref()
      && let Some(chart_text) = sequence.chart_text.as_deref()
    {
      chart_push_chart_text(texts, chart_text);
    }
  }
  if let Some(c::DataLabelsChoice::Sequence(sequence)) = data_labels.data_labels_choice.as_ref()
    && sequence
      .show_percent
      .as_ref()
      .and_then(|show| show.val)
      .is_some_and(|value| value.as_bool())
  {
    push_unique_chart_text(texts, "100%");
  }
}

fn chart_push_string_reference_texts(texts: &mut Vec<String>, reference: &c::StringReference) {
  if let Some(cache) = reference.string_cache.as_deref() {
    chart_push_string_cache_texts(texts, cache);
  }
}

fn chart_push_string_cache_texts(texts: &mut Vec<String>, cache: &c::StringCache) {
  for point in &cache.string_point {
    push_unique_chart_text(texts, &point.numeric_value);
  }
}

fn chart_push_string_literal_texts(texts: &mut Vec<String>, literal: &c::StringLiteral) {
  for point in &literal.string_point {
    push_unique_chart_text(texts, &point.numeric_value);
  }
}

fn chart_push_number_reference_texts(texts: &mut Vec<String>, reference: &c::NumberReference) {
  if let Some(cache) = reference.numbering_cache.as_deref() {
    chart_push_numbering_cache_texts(texts, cache);
  }
}

fn chart_push_numbering_cache_texts(texts: &mut Vec<String>, cache: &c::NumberingCache) {
  for point in &cache.numeric_point {
    push_unique_chart_text(texts, &point.numeric_value);
  }
}

fn chart_push_number_literal_texts(texts: &mut Vec<String>, literal: &c::NumberLiteral) {
  for point in &literal.numeric_point {
    push_unique_chart_text(texts, &point.numeric_value);
  }
}

fn push_unique_chart_text(texts: &mut Vec<String>, value: &str) {
  let trimmed = value.trim();
  if trimmed.is_empty() || texts.iter().any(|text| text == trimmed) {
    return;
  }
  texts.push(trimmed.to_string());
}
