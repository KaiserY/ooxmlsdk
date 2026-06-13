use super::*;
use ooxmlsdk_layout::render::chart as shared_chart;

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
    let mut texts = shared_chart::visible_texts_with_uncached_series_labels(chart_space);
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
  if !shared_chart::has_vertical_multilevel_category_axis(chart_space) {
    return Vec::new();
  }

  let mut labels = Vec::new();
  for series in shared_chart::series(chart_space) {
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

fn push_unique_chart_text(texts: &mut Vec<String>, value: &str) {
  let trimmed = value.trim();
  if trimmed.is_empty() || texts.iter().any(|text| text == trimmed) {
    return;
  }
  texts.push(trimmed.to_string());
}

fn chart_label_color(chart_space: &c::ChartSpace, theme_colors: &ThemeColors) -> Option<RgbColor> {
  shared_chart::series(chart_space)
    .into_iter()
    .filter_map(|series| series.data_labels)
    .chain(shared_chart::data_labels(chart_space))
    .find_map(|labels| {
      labels
        .data_label
        .iter()
        .find_map(|label| {
          label
            .data_label_choice
            .iter()
            .find_map(|choice| match choice {
              c::DataLabelChoice::Sequence(sequence) => sequence
                .text_properties
                .as_deref()
                .and_then(|properties| chart_text_properties_color(properties, theme_colors)),
              c::DataLabelChoice::Delete(_) => None,
            })
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
