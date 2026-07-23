use ooxmlsdk::schemas::schemas_microsoft_com_office_drawing_2014_chartex as cx;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use crate::model::{
  BorderStyle, LineItem, LineItemKind, PageItem, PdfTextSegmentation, RectItem, RgbColor, TextItem,
  TextStyle,
};
use crate::pptx::chart::ChartFrame;
use crate::render::chart::format_chart_number;

use super::import::ExcelImport;
use super::worksheet::{CellAddress, CellRange, CellRect};

const COLORS: [RgbColor; 6] = [
  RgbColor {
    r: 68,
    g: 114,
    b: 196,
  },
  RgbColor {
    r: 237,
    g: 125,
    b: 49,
  },
  RgbColor {
    r: 165,
    g: 165,
    b: 165,
  },
  RgbColor {
    r: 255,
    g: 192,
    b: 0,
  },
  RgbColor {
    r: 91,
    g: 155,
    b: 213,
  },
  RgbColor {
    r: 112,
    g: 173,
    b: 71,
  },
];

pub(crate) fn lower_extended_chart(
  import: &ExcelImport,
  chart_space: &cx::ChartSpace,
  frame: CellRect,
) -> Vec<PageItem> {
  let mut title_style = import.styles.default_chart_text_style();
  title_style.font_size_pt = 14.0;
  title_style.bold = true;
  let mut label_style = import.styles.default_chart_text_style();
  label_style.font_size_pt = 9.0;
  lower_extended_chart_with_styles(Some(import), chart_space, frame, title_style, label_style)
}

pub(crate) fn lower_extended_chart_cached(
  chart_space: &cx::ChartSpace,
  frame: ChartFrame,
  title_style: TextStyle,
  label_style: TextStyle,
) -> Vec<PageItem> {
  lower_extended_chart_with_styles(
    None,
    chart_space,
    CellRect {
      x_pt: frame.x_pt,
      y_pt: frame.y_pt,
      width_pt: frame.width_pt,
      height_pt: frame.height_pt,
    },
    title_style,
    label_style,
  )
}

fn lower_extended_chart_with_styles(
  import: Option<&ExcelImport>,
  chart_space: &cx::ChartSpace,
  frame: CellRect,
  title_style: TextStyle,
  label_style: TextStyle,
) -> Vec<PageItem> {
  let mut items = vec![rect(
    frame.x_pt,
    frame.y_pt,
    frame.width_pt,
    frame.height_pt,
    Some(RgbColor {
      r: 255,
      g: 255,
      b: 255,
    }),
  )];
  let title = chart_space
    .chart
    .chart_title
    .as_deref()
    .map(chart_title_text)
    .unwrap_or_default();
  if !title.is_empty() {
    push_text(
      &mut items,
      frame.x_pt + frame.width_pt * 0.08,
      frame.y_pt + frame.height_pt * 0.035,
      title,
      title_style,
    );
  }
  let title_band = if chart_space.chart.chart_title.is_some() {
    0.15
  } else {
    0.05
  };
  let plot = CellRect {
    x_pt: frame.x_pt + frame.width_pt * 0.1,
    y_pt: frame.y_pt + frame.height_pt * title_band,
    width_pt: frame.width_pt * 0.82,
    height_pt: frame.height_pt * (0.88 - title_band),
  };
  let series = &chart_space.chart.plot_area.plot_area_region.series;
  let visible_count = series
    .iter()
    .filter(|series| !series.hidden.is_some_and(|value| value.as_bool()))
    .count()
    .max(1);
  for (visible_index, series) in series
    .iter()
    .filter(|series| !series.hidden.is_some_and(|value| value.as_bool()))
    .enumerate()
  {
    let Some(data_id) = series.data_id.as_ref().map(|id| id.val) else {
      continue;
    };
    let values = numeric_values(import, chart_space, data_id);
    if values.is_empty() {
      continue;
    }
    let categories = category_values(import, chart_space, data_id, values.len());
    let color = COLORS[visible_index % COLORS.len()];
    match series.layout_id {
      cx::SeriesLayout::Funnel => {
        lower_funnel(&mut items, plot, &values, &categories, color, &label_style)
      }
      cx::SeriesLayout::Waterfall => {
        lower_waterfall(&mut items, plot, &values, &categories, color, &label_style)
      }
      cx::SeriesLayout::BoxWhisker => lower_box_whisker(
        &mut items,
        plot,
        &values,
        visible_index,
        visible_count,
        color,
      ),
      cx::SeriesLayout::Treemap | cx::SeriesLayout::Sunburst | cx::SeriesLayout::RegionMap => {
        lower_partition(&mut items, plot, &values, &categories, &label_style)
      }
      cx::SeriesLayout::ParetoLine => {
        lower_columns(&mut items, plot, &values, &categories, color, &label_style);
        lower_pareto(&mut items, plot, &values);
      }
      cx::SeriesLayout::ClusteredColumn => {
        lower_columns(&mut items, plot, &values, &categories, color, &label_style)
      }
    }
  }
  items
}

fn numeric_values(import: Option<&ExcelImport>, space: &cx::ChartSpace, id: u32) -> Vec<f64> {
  let Some(data) = space
    .chart_data
    .as_deref()
    .and_then(|data| data.data.iter().find(|data| data.id == id))
  else {
    return Vec::new();
  };
  for choice in &data.data_choice {
    let cx::DataChoice::NumericDimension(dimension) = choice else {
      continue;
    };
    if dimension.r#type != cx::NumericDimensionType::Val {
      continue;
    }
    let (levels, formula) = match dimension.numeric_dimension_choice.as_ref() {
      Some(cx::NumericDimensionChoice::Sequence(sequence)) => (
        sequence.numeric_level.as_slice(),
        sequence.formula.0.xml_content.as_deref(),
      ),
      Some(cx::NumericDimensionChoice::NumericLevel(level)) => {
        (std::slice::from_ref(level.as_ref()), None)
      }
      None => (&[][..], None),
    };
    if let Some(level) = levels.first()
      && !level.numeric_value.is_empty()
    {
      return level
        .numeric_value
        .iter()
        .filter_map(|point| point.xml_content)
        .collect();
    }
    if let (Some(import), Some(formula)) = (import, formula) {
      return defined_values(import, formula)
        .into_iter()
        .filter_map(|value| value.parse().ok())
        .collect();
    }
  }
  Vec::new()
}

fn category_values(
  import: Option<&ExcelImport>,
  space: &cx::ChartSpace,
  id: u32,
  count: usize,
) -> Vec<String> {
  let cached = space
    .chart_data
    .as_deref()
    .and_then(|data| data.data.iter().find(|data| data.id == id))
    .and_then(|data| {
      data.data_choice.iter().find_map(|choice| {
        let cx::DataChoice::StringDimension(dimension) = choice else {
          return None;
        };
        let (levels, formula) = match dimension.string_dimension_choice.as_ref() {
          Some(cx::StringDimensionChoice::Sequence(sequence)) => (
            sequence.string_level.as_slice(),
            sequence.formula.0.xml_content.as_deref(),
          ),
          Some(cx::StringDimensionChoice::StringLevel(level)) => {
            (std::slice::from_ref(level), None)
          }
          None => (&[][..], None),
        };
        if let Some(level) = levels.first()
          && !level.chart_string_value.is_empty()
        {
          return Some(
            level
              .chart_string_value
              .iter()
              .filter_map(|point| point.xml_content.clone())
              .collect(),
          );
        }
        import.and_then(|import| formula.map(|formula| defined_values(import, formula)))
      })
    })
    .unwrap_or_default();
  if cached.is_empty() {
    (1..=count).map(|index| index.to_string()).collect()
  } else {
    cached
  }
}

fn defined_values(import: &ExcelImport, name_or_formula: &str) -> Vec<String> {
  let formula = import
    .defined_names
    .records
    .iter()
    .find(|record| record.name == name_or_formula)
    .map_or(name_or_formula, |record| record.formula.as_str());
  let sheet_name = formula
    .rsplit_once('!')
    .map(|(sheet, _)| sheet.trim_matches('\''));
  let Some(sheet) = sheet_name
    .and_then(|name| import.sheets.iter().find(|sheet| sheet.name == name))
    .or_else(|| import.sheets.first())
  else {
    return Vec::new();
  };
  let Some(range) = CellRange::parse_a1_range(formula) else {
    return Vec::new();
  };
  let mut values = Vec::new();
  for row in range.start.row..=range.end.row {
    for col in range.start.col..=range.end.col {
      if let Some(cell) = sheet.cell_at(CellAddress { col, row }) {
        values.push(
          cell
            .cached_value
            .clone()
            .unwrap_or_else(|| cell.display_text.clone()),
        );
      }
    }
  }
  values
}

fn chart_title_text(title: &cx::ChartTitle) -> String {
  title
    .text
    .as_deref()
    .map(cx_text)
    .filter(|text| !text.is_empty())
    .or_else(|| {
      title
        .tx_pr_text_body
        .as_deref()
        .map(|body| paragraph_text(&body.paragraph))
    })
    .unwrap_or_default()
}

fn cx_text(text: &cx::Text) -> String {
  match text.text_choice.as_ref() {
    Some(cx::TextChoice::RichTextBody(body)) => paragraph_text(&body.paragraph),
    Some(cx::TextChoice::TextData(data)) => match data.text_data_choice.as_ref() {
      Some(cx::TextDataChoice::Sequence(sequence)) => {
        sequence.v_xsdstring.clone().unwrap_or_default()
      }
      Some(cx::TextDataChoice::VXsdstring(value)) => value.clone(),
      None => String::new(),
    },
    None => String::new(),
  }
}

fn paragraph_text(paragraphs: &[a::Paragraph]) -> String {
  paragraphs
    .iter()
    .map(|paragraph| {
      paragraph
        .paragraph_choice
        .iter()
        .filter_map(|choice| match choice {
          a::ParagraphChoice::Run(run) => Some(run.text.as_str()),
          a::ParagraphChoice::Field(field) => field.text.as_deref(),
          _ => None,
        })
        .collect::<String>()
    })
    .filter(|line| !line.trim().is_empty())
    .collect::<Vec<_>>()
    .join("\n")
}

fn lower_columns(
  items: &mut Vec<PageItem>,
  plot: CellRect,
  values: &[f64],
  categories: &[String],
  color: RgbColor,
  style: &TextStyle,
) {
  let maximum = values.iter().copied().fold(0.0_f64, f64::max).max(1.0);
  let slot = plot.width_pt / values.len() as f32;
  for (index, value) in values.iter().copied().enumerate() {
    let height = (value.max(0.0) / maximum) as f32 * plot.height_pt * 0.82;
    items.push(rect(
      plot.x_pt + slot * (index as f32 + 0.15),
      plot.y_pt + plot.height_pt * 0.84 - height,
      slot * 0.7,
      height.max(0.5),
      Some(color),
    ));
    if let Some(category) = categories.get(index) {
      push_text(
        items,
        plot.x_pt + slot * index as f32,
        plot.y_pt + plot.height_pt * 0.86,
        category.clone(),
        style.clone(),
      );
    }
  }
}

fn lower_funnel(
  items: &mut Vec<PageItem>,
  plot: CellRect,
  values: &[f64],
  categories: &[String],
  color: RgbColor,
  style: &TextStyle,
) {
  let maximum = values.iter().copied().fold(0.0_f64, f64::max).max(1.0);
  let height = plot.height_pt / values.len() as f32;
  for (index, value) in values.iter().copied().enumerate() {
    let width = plot.width_pt * (value.max(0.0) / maximum) as f32;
    let x = plot.x_pt + (plot.width_pt - width) * 0.5;
    items.push(rect(
      x,
      plot.y_pt + index as f32 * height,
      width.max(0.5),
      height * 0.9,
      Some(color),
    ));
    let label = categories.get(index).map_or_else(
      || format_chart_number(value, None),
      |category| format!("{category} {}", format_chart_number(value, None)),
    );
    push_text(
      items,
      x + 2.0,
      plot.y_pt + index as f32 * height,
      label,
      style.clone(),
    );
  }
}

fn lower_waterfall(
  items: &mut Vec<PageItem>,
  plot: CellRect,
  values: &[f64],
  categories: &[String],
  color: RgbColor,
  style: &TextStyle,
) {
  let totals = values
    .iter()
    .scan(0.0, |sum, value| {
      *sum += *value;
      Some(*sum)
    })
    .collect::<Vec<_>>();
  let minimum = totals.iter().copied().fold(0.0_f64, f64::min);
  let maximum = totals.iter().copied().fold(0.0_f64, f64::max);
  let span = (maximum - minimum).max(1.0);
  let slot = plot.width_pt / values.len() as f32;
  let mut cumulative = 0.0;
  for (index, value) in values.iter().copied().enumerate() {
    let start = cumulative;
    cumulative += value;
    let top = start.max(cumulative);
    let bottom = start.min(cumulative);
    let y = plot.y_pt + (maximum - top) as f32 / span as f32 * plot.height_pt * 0.82;
    let height = (top - bottom) as f32 / span as f32 * plot.height_pt * 0.82;
    items.push(rect(
      plot.x_pt + slot * (index as f32 + 0.15),
      y,
      slot * 0.7,
      height.max(0.75),
      Some(if value < 0.0 {
        RgbColor { r: 192, g: 0, b: 0 }
      } else {
        color
      }),
    ));
    if let Some(category) = categories.get(index) {
      push_text(
        items,
        plot.x_pt + slot * index as f32,
        plot.y_pt + plot.height_pt * 0.86,
        category.clone(),
        style.clone(),
      );
    }
  }
}

fn lower_box_whisker(
  items: &mut Vec<PageItem>,
  plot: CellRect,
  values: &[f64],
  index: usize,
  count: usize,
  color: RgbColor,
) {
  let mut values = values.to_vec();
  values.sort_by(f64::total_cmp);
  let min = values[0];
  let max = values[values.len() - 1];
  let q1 = values[values.len() / 4];
  let median = values[values.len() / 2];
  let q3 = values[values.len() * 3 / 4];
  let span = (max - min).max(1.0);
  let x = plot.x_pt + plot.width_pt * (index as f32 + 0.5) / count as f32;
  let y = |value: f64| plot.y_pt + (max - value) as f32 / span as f32 * plot.height_pt;
  push_line(items, x, y(max), x, y(min), color);
  items.push(rect(
    x - plot.width_pt / count as f32 * 0.18,
    y(q3),
    plot.width_pt / count as f32 * 0.36,
    (y(q1) - y(q3)).max(1.0),
    Some(color),
  ));
  push_line(
    items,
    x - plot.width_pt / count as f32 * 0.18,
    y(median),
    x + plot.width_pt / count as f32 * 0.18,
    y(median),
    color,
  );
}

fn lower_partition(
  items: &mut Vec<PageItem>,
  plot: CellRect,
  values: &[f64],
  categories: &[String],
  style: &TextStyle,
) {
  let total = values
    .iter()
    .map(|value| value.max(0.0))
    .sum::<f64>()
    .max(1.0);
  let mut x = plot.x_pt;
  for (index, value) in values.iter().copied().enumerate() {
    let width = plot.width_pt * (value.max(0.0) / total) as f32;
    items.push(rect(
      x,
      plot.y_pt,
      width.max(0.5),
      plot.height_pt,
      Some(COLORS[index % 6]),
    ));
    if let Some(category) = categories.get(index) {
      push_text(
        items,
        x + 2.0,
        plot.y_pt + 2.0,
        category.clone(),
        style.clone(),
      );
    }
    x += width;
  }
}

fn lower_pareto(items: &mut Vec<PageItem>, plot: CellRect, values: &[f64]) {
  let total = values.iter().sum::<f64>();
  if total.abs() <= f64::EPSILON {
    return;
  }
  let slot = plot.width_pt / values.len() as f32;
  let mut sum = 0.0;
  let mut previous = None;
  for (index, value) in values.iter().copied().enumerate() {
    sum += value;
    let point = (
      plot.x_pt + slot * (index as f32 + 0.5),
      plot.y_pt + plot.height_pt * (1.0 - (sum / total) as f32),
    );
    if let Some((x, y)) = previous {
      push_line(items, x, y, point.0, point.1, COLORS[1]);
    }
    previous = Some(point);
  }
}

fn rect(x: f32, y: f32, width: f32, height: f32, fill: Option<RgbColor>) -> PageItem {
  PageItem::Rect(RectItem {
    x_pt: x,
    y_pt: y,
    width_pt: width,
    height_pt: height,
    fill_color: fill,
    fill_opacity: 1.0,
    stroke: Some(BorderStyle {
      width_pt: 0.5,
      color: RgbColor {
        r: 255,
        g: 255,
        b: 255,
      },
      ..BorderStyle::default()
    }),
    stroke_opacity: 1.0,
  })
}

fn push_line(items: &mut Vec<PageItem>, x1: f32, y1: f32, x2: f32, y2: f32, color: RgbColor) {
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

fn push_text(items: &mut Vec<PageItem>, x: f32, y: f32, text: String, style: TextStyle) {
  items.push(PageItem::Text(TextItem {
    x_pt: x,
    y_pt: y,
    line_height_pt: style.font_size_pt * 1.2,
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
