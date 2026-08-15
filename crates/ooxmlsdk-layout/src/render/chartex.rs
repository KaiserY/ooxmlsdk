//! Shared Office 2016 ChartEx lowering.
//!
//! ChartEx data is deliberately normalized here instead of in the DOCX,
//! PPTX, or XLSX hosts.  In particular, cached point indices stay sparse,
//! category levels stay hierarchical, and numeric dimensions keep their
//! semantic role.  [MS-ODRAWXML] 2.6.3.5 and 2.6.3.6 make all three
//! distinctions observable.

use std::collections::{HashMap, HashSet};
use std::f32::consts::{PI, TAU};
use std::io::Cursor;
use std::sync::Arc;

use bytes::Bytes;
use image::codecs::png::PngEncoder;
use image::{ColorType, ImageEncoder};
use ooxmlsdk::schemas::schemas_microsoft_com_office_drawing_2012_chart_style as cs;
use ooxmlsdk::schemas::schemas_microsoft_com_office_drawing_2014_chartex as cx;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use crate::localization::OfficeStringCatalog;
use crate::model::{
  BorderStyle, ImageCrop, ImageItem, LineItem, LineItemKind, PageItem, PdfTextSegmentation,
  RectItem, RgbColor, TextItem, TextStyle, common_point, common_rect, common_rgb,
};
use crate::pptx::chart::ChartFrame;
use crate::pptx::drawingml::color::{
  Color, ColorTransformation, ColorTransformationKind, ResolvedColor, RgbHexColor,
  apply_transformations, hsl_color, preset_color, rgb_hex_color, rgb_percent_color, scheme_color,
  system_color,
};
use crate::render::chart::{automatic_chart_title, automatic_series_title, format_chart_number};
use crate::text_metrics::TextMetrics;

const DRAWINGML_PERCENT_MAX: i32 = 100_000;
// Office's fixed-format output spaces linear ChartEx colors over a 42%
// brightness interval on either side of the authored base color. The
// brightness is then lowered as a DrawingML shade/tint in linear scRGB.
const OFFICE_LINEAR_COLOR_BRIGHTNESS: i32 = 42_000;
// Waterfall has four style-format groups: increase, decrease, total, and the
// connector line. The series-line style supplies the connector color, but it
// still participates in the automatic color-format count.
const WATERFALL_COLOR_FORMAT_COUNT: usize = 4;
const OFFICE_ACCENTS: [RgbColor; 6] = [
  rgb(68, 114, 196),
  rgb(237, 125, 49),
  rgb(165, 165, 165),
  rgb(255, 192, 0),
  rgb(91, 155, 213),
  rgb(112, 173, 71),
];
const WATERFALL_INCREASE: RgbColor = rgb(91, 155, 213);
const WATERFALL_DECREASE: RgbColor = rgb(237, 125, 49);
const WATERFALL_TOTAL: RgbColor = rgb(165, 165, 165);

const fn rgb(r: u8, g: u8, b: u8) -> RgbColor {
  RgbColor { r, g, b }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct ChartExTheme {
  pub(crate) dark1: RgbColor,
  pub(crate) light1: RgbColor,
  pub(crate) dark2: RgbColor,
  pub(crate) light2: RgbColor,
  pub(crate) accents: [RgbColor; 6],
  pub(crate) hyperlink: RgbColor,
  pub(crate) followed_hyperlink: RgbColor,
}

impl Default for ChartExTheme {
  fn default() -> Self {
    // The built-in Office theme.  Hosts replace these values when a package
    // supplies a theme; keeping the Office defaults here also covers packages
    // that legitimately omit one.
    Self {
      dark1: rgb(0, 0, 0),
      light1: rgb(255, 255, 255),
      dark2: rgb(68, 84, 106),
      light2: rgb(231, 230, 230),
      accents: OFFICE_ACCENTS,
      hyperlink: rgb(5, 99, 193),
      followed_hyperlink: rgb(149, 79, 114),
    }
  }
}

impl ChartExTheme {
  fn color(self, value: a::SchemeColorValues) -> Option<RgbColor> {
    Some(match value {
      a::SchemeColorValues::Dark1 | a::SchemeColorValues::Text1 => self.dark1,
      a::SchemeColorValues::Light1 | a::SchemeColorValues::Background1 => self.light1,
      a::SchemeColorValues::Dark2 | a::SchemeColorValues::Text2 => self.dark2,
      a::SchemeColorValues::Light2 | a::SchemeColorValues::Background2 => self.light2,
      a::SchemeColorValues::Accent1 => self.accents[0],
      a::SchemeColorValues::Accent2 => self.accents[1],
      a::SchemeColorValues::Accent3 => self.accents[2],
      a::SchemeColorValues::Accent4 => self.accents[3],
      a::SchemeColorValues::Accent5 => self.accents[4],
      a::SchemeColorValues::Accent6 => self.accents[5],
      a::SchemeColorValues::Hyperlink => self.hyperlink,
      a::SchemeColorValues::FollowedHyperlink => self.followed_hyperlink,
      a::SchemeColorValues::PhColor => return None,
    })
  }

  fn drawing_color(self, value: a::SchemeColorValues) -> Option<Color> {
    let color = self.color(value)?;
    Some(Color::RgbHex(RgbHexColor {
      value: format!("{:02X}{:02X}{:02X}", color.r, color.g, color.b),
      transformations: Vec::new(),
    }))
  }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct FormulaMatrix {
  pub(crate) rows: Vec<Vec<String>>,
}

impl FormulaMatrix {
  fn levels(&self, direction: cx::FormulaDirection) -> Vec<Vec<String>> {
    if direction == cx::FormulaDirection::Row {
      return self.rows.clone();
    }
    let width = self.rows.iter().map(Vec::len).max().unwrap_or(0);
    (0..width)
      .map(|column| {
        self
          .rows
          .iter()
          .map(|row| row.get(column).cloned().unwrap_or_default())
          .collect()
      })
      .collect()
  }
}

pub(crate) type FormulaResolver<'a> = dyn FnMut(&str) -> Option<FormulaMatrix> + 'a;

#[derive(Clone, Copy, Default)]
pub(crate) struct ChartExStyleResources<'a> {
  pub(crate) chart_styles: &'a [cs::ChartStyle],
  pub(crate) color_styles: &'a [cs::ColorStyle],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ChartExHost {
  Word,
  PowerPoint,
  Excel,
}

pub(crate) struct ChartExRenderOptions<'a> {
  pub(crate) host: ChartExHost,
  pub(crate) frame: ChartFrame,
  pub(crate) title_style: TextStyle,
  pub(crate) label_style: TextStyle,
  pub(crate) ui_language: Option<&'a str>,
  pub(crate) theme: ChartExTheme,
  pub(crate) resources: ChartExStyleResources<'a>,
}

#[derive(Clone, Debug)]
struct SparseLevel<T> {
  name: Option<String>,
  format_code: Option<String>,
  points: Vec<Option<T>>,
}

impl<T> SparseLevel<T> {
  fn len(&self) -> usize {
    self.points.len()
  }
}

#[derive(Clone, Debug)]
struct NumericDimension {
  role: cx::NumericDimensionType,
  levels: Vec<SparseLevel<f64>>,
}

#[derive(Clone, Debug)]
struct StringDimension {
  role: cx::StringDimensionType,
  levels: Vec<SparseLevel<String>>,
}

#[derive(Clone, Debug, Default)]
struct DataSet {
  numeric: Vec<NumericDimension>,
  strings: Vec<StringDimension>,
}

impl DataSet {
  fn numeric(&self, role: cx::NumericDimensionType) -> Option<&NumericDimension> {
    self.numeric.iter().find(|dimension| dimension.role == role)
  }

  fn strings(&self, role: cx::StringDimensionType) -> Option<&StringDimension> {
    self.strings.iter().find(|dimension| dimension.role == role)
  }
}

#[derive(Clone, Debug)]
struct SeriesModel<'a> {
  source: &'a cx::Series,
  source_index: usize,
  name: String,
  automatic_name: bool,
  layout: cx::SeriesLayout,
  values: Vec<Option<f64>>,
  number_format: Option<String>,
  categories: Vec<SparseLevel<String>>,
}

impl SeriesModel<'_> {
  fn count(&self) -> usize {
    self.values.len().max(
      self
        .categories
        .iter()
        .map(SparseLevel::len)
        .max()
        .unwrap_or(0),
    )
  }

  fn leaf_category(&self, index: usize) -> String {
    self
      .categories
      .first()
      .and_then(|level| level.points.get(index))
      .and_then(Option::as_deref)
      .filter(|value| !value.is_empty())
      .map(str::to_string)
      .unwrap_or_else(|| (index + 1).to_string())
  }

  fn category_path(&self, index: usize) -> Vec<String> {
    // ChartEx stores the leaf level first.  Empty levels mean that the next
    // populated level is itself the leaf; they are not anonymous tree nodes.
    self
      .categories
      .iter()
      .rev()
      .filter_map(|level| {
        level
          .points
          .get(index)
          .and_then(Option::as_deref)
          .filter(|value| !value.is_empty())
          .map(str::to_string)
      })
      .fold(Vec::<String>::new(), |mut path, value| {
        if path.last() != Some(&value) {
          path.push(value);
        }
        path
      })
  }
}

#[derive(Clone, Copy, Debug)]
struct PlotRect {
  x: f32,
  y: f32,
  width: f32,
  height: f32,
}

#[derive(Clone, Debug)]
struct Appearance {
  host: ChartExHost,
  chart_width_pt: f32,
  theme: ChartExTheme,
  chart_fill: RgbColor,
  chart_gradient: Option<a::GradientFill>,
  chart_stroke: Option<(RgbColor, f32)>,
  plot_fill: Option<RgbColor>,
  plot_gradient: Option<a::GradientFill>,
  data_point_gradient: Option<a::GradientFill>,
  data_point_effects: Option<ChartExEffectSource>,
  data_point_outline: Option<a::Outline>,
  data_point_line_outline: Option<a::Outline>,
  series_line_color: RgbColor,
  series_line_width: f32,
  axis_color: RgbColor,
  axis_width: f32,
  grid_color: RgbColor,
  grid_width: f32,
  title_style: TextStyle,
  axis_title_style: TextStyle,
  label_style: TextStyle,
  data_label_style: TextStyle,
  palette: Vec<RgbColor>,
  color_method: Option<cs::ColorStyleMethodEnum>,
}

#[derive(Clone, Debug)]
enum ChartExEffectSource {
  List(a::EffectList),
  Dag(a::EffectDag),
}

impl Appearance {
  fn point_color(&self, index: usize, count: usize) -> RgbColor {
    let method = self.color_method;
    let base = match method {
      Some(
        cs::ColorStyleMethodEnum::WithinLinear | cs::ColorStyleMethodEnum::WithinLinearReversed,
      ) => self.palette[0],
      _ => self.palette[index % self.palette.len()],
    };
    let reversed = match method {
      Some(
        cs::ColorStyleMethodEnum::WithinLinearReversed
        | cs::ColorStyleMethodEnum::AcrossLinearReversed,
      ) => true,
      Some(cs::ColorStyleMethodEnum::WithinLinear | cs::ColorStyleMethodEnum::AcrossLinear) => {
        false
      }
      _ => return base,
    };
    if count <= 1 {
      return base;
    }
    linear_point_color(base, index, count, reversed)
  }

  fn data_point_stroke(&self, point_color: RgbColor) -> Option<(RgbColor, f32)> {
    let outline = self.data_point_outline.as_ref()?;
    let color = resolve_outline(outline, self.theme, Some(point_color))?;
    let width = outline
      .width
      .map(|width| crate::units::emu_to_points(i64::from(width)))
      .unwrap_or(0.75);
    Some((color, width))
  }

  fn box_whisker_stroke(&self, point_color: RgbColor) -> (RgbColor, f32, Option<&a::Outline>) {
    if let Some(outline) = self.data_point_outline.as_ref() {
      let color = resolve_outline(outline, self.theme, Some(point_color))
        .map(|color| shade(color, 0.2))
        .unwrap_or_else(|| shade(point_color, 0.2));
      let width = outline
        .width
        .map(|width| crate::units::emu_to_points(i64::from(width)))
        .unwrap_or(0.75);
      return (color, width, Some(outline));
    }
    if let Some(outline) = self.data_point_line_outline.as_ref() {
      let color = resolve_outline(outline, self.theme, Some(point_color)).unwrap_or(point_color);
      let width = outline
        .width
        .map(|width| crate::units::emu_to_points(i64::from(width)))
        .unwrap_or(2.25);
      return (color, width, Some(outline));
    }
    (shade(point_color, 0.2), 0.75, None)
  }
}

fn linear_point_color(base: RgbColor, index: usize, count: usize, reversed: bool) -> RgbColor {
  if count <= 1 {
    return base;
  }
  let mut position = index.min(count - 1) as f32 / (count - 1) as f32;
  if reversed {
    position = 1.0 - position;
  }
  let brightness = ((position * 2.0 - 1.0) * OFFICE_LINEAR_COLOR_BRIGHTNESS as f32).round() as i32;
  if brightness < 0 {
    transform_rgb(
      base,
      ColorTransformationKind::Shade,
      DRAWINGML_PERCENT_MAX + brightness,
    )
  } else if brightness > 0 {
    transform_rgb(
      base,
      ColorTransformationKind::Tint,
      DRAWINGML_PERCENT_MAX - brightness,
    )
  } else {
    base
  }
}

pub(crate) fn lower_extended_chart(
  chart_space: &cx::ChartSpace,
  options: ChartExRenderOptions<'_>,
  mut formula_resolver: Option<&mut FormulaResolver<'_>>,
) -> Vec<PageItem> {
  let data_sets = extract_data_sets(chart_space, &mut formula_resolver);
  let mut series = Vec::new();
  for (source_index, source) in chart_space
    .chart
    .plot_area
    .plot_area_region
    .series
    .iter()
    .enumerate()
  {
    if source.hidden.is_some_and(|hidden| hidden.as_bool()) {
      continue;
    }
    let data = source
      .data_id
      .as_ref()
      .and_then(|id| data_sets.get(&id.val));
    series.push(series_model(
      source,
      source_index,
      data,
      options.ui_language,
    ));
  }

  let mut appearance = chart_appearance(chart_space, &options);
  apply_chartex_text_properties(
    &mut appearance.title_style,
    chart_space
      .chart
      .chart_title
      .as_deref()
      .and_then(|title| title.tx_pr_text_body.as_deref()),
    options.theme,
  );
  if options.host != ChartExHost::Word
    && chart_space
      .chart
      .chart_title
      .as_deref()
      .is_some_and(|title| chart_title_text(title).is_empty())
  {
    // Office's automatic ChartEx title resource uses the regular UI font;
    // bold is reserved for an authored title text style.
    appearance.title_style.bold = false;
  }
  let frame = options.frame;
  let mut items = Vec::new();
  push_gradient_or_solid_rect(
    &mut items,
    snap_office_chart_rect(PlotRect {
      x: frame.x_pt,
      y: frame.y_pt,
      width: frame.width_pt,
      height: frame.height_pt,
    }),
    appearance.chart_fill,
    appearance.chart_gradient.as_ref(),
    appearance.theme,
    None,
    crate::common::GradientInterpolation::PowerPointGammaSigma,
    appearance.chart_stroke,
  );

  let title = chart_space.chart.chart_title.as_deref().map(|title| {
    let explicit = chart_title_text(title);
    if explicit.is_empty() {
      automatic_chart_title(options.ui_language).to_string()
    } else {
      explicit
    }
  });
  let legend = chart_space.chart.legend.as_deref();
  let has_category_axis_title =
    category_axis(chart_space).is_some_and(|axis| axis.axis_title.is_some());
  let (plot, title_slot, legend_slot) = chart_bands(
    frame,
    chart_space.chart.chart_title.as_deref(),
    legend,
    options.host,
    appearance.label_style.font_size_pt,
    has_category_axis_title,
  );
  let has_funnel = series
    .iter()
    .any(|series| series.layout == cx::SeriesLayout::Funnel);
  let series_plot = if has_funnel {
    funnel_chart_plot(
      frame,
      plot,
      chart_space.chart.chart_title.as_deref(),
      legend,
      &appearance,
    )
  } else {
    plot
  };

  if let Some(title_text) = title.as_deref()
    && !title_text.is_empty()
    && let Some(slot) = title_slot
  {
    if has_funnel {
      let width = text_width(title_text, &appearance.title_style);
      // A rich ChartEx title (the PowerPoint host form) retains the text
      // body's centered-anchor inset, while Excel's direct txPr form starts
      // lower in the automatic title band. The two funnel corpus families
      // expose the distinction independently.
      let title_top_em = if chart_space
        .chart
        .chart_title
        .as_deref()
        .is_some_and(|title| title.text.is_some())
      {
        0.471
      } else {
        0.59
      };
      push_text(
        &mut items,
        frame.x_pt + (frame.width_pt - width) * 0.5,
        frame.y_pt + appearance.title_style.font_size_pt * title_top_em,
        title_text.to_string(),
        appearance.title_style.clone(),
      );
    } else if options.host == ChartExHost::PowerPoint
      && chart_space
        .chart
        .chart_title
        .as_deref()
        .is_some_and(|title| chart_title_text(title).is_empty())
    {
      // PowerPoint top-aligns an application-generated ChartEx title in the
      // reserved title band. The three Office 2016 ChartEx reference decks
      // share this geometry; vertically centering the one-line resource text
      // moves its baseline down by half of the unused 9%-band height.
      let width = TextMetrics::new().measure_text(title_text, &appearance.title_style);
      push_text(
        &mut items,
        slot.x + (slot.width - width) * 0.5,
        slot.y - 1.66,
        title_text.to_string(),
        appearance.title_style.clone(),
      );
    } else {
      push_centered_measured_text(
        &mut items,
        PlotRect {
          y: slot.y + 0.5,
          ..slot
        },
        title_text.to_string(),
        appearance.title_style.clone(),
      );
    }
  }
  if appearance.plot_gradient.is_some() || appearance.plot_fill.is_some() {
    push_gradient_or_solid_rect(
      &mut items,
      plot,
      appearance.plot_fill.unwrap_or(appearance.chart_fill),
      appearance.plot_gradient.as_ref(),
      appearance.theme,
      None,
      crate::common::GradientInterpolation::PowerPointGammaSigma,
      None,
    );
  }

  if series.is_empty() {
    return items;
  }

  let has_pareto = series
    .iter()
    .any(|series| series.layout == cx::SeriesLayout::ParetoLine);
  let has_hierarchy = series.iter().any(|series| {
    matches!(
      series.layout,
      cx::SeriesLayout::Treemap | cx::SeriesLayout::Sunburst
    )
  });
  let mut legend_entries = Vec::new();

  if has_pareto {
    lower_pareto_chart(
      &mut items,
      plot,
      &series,
      chart_space,
      &appearance,
      options.ui_language,
      &mut legend_entries,
    );
  } else if has_hierarchy {
    for series in &series {
      match series.layout {
        cx::SeriesLayout::Treemap => {
          lower_treemap(&mut items, plot, series, &appearance, &mut legend_entries)
        }
        cx::SeriesLayout::Sunburst => lower_sunburst(&mut items, plot, series, &appearance),
        _ => {}
      }
    }
  } else {
    lower_nonhierarchical(
      &mut items,
      series_plot,
      &series,
      chart_space,
      &appearance,
      options.ui_language,
      &mut legend_entries,
    );
  }

  if legend.is_some() {
    if legend_entries.is_empty() {
      legend_entries.extend(series.iter().map(|series| {
        (
          series_legend_name(series, options.ui_language),
          appearance.point_color(series.source_index, series.count()),
        )
      }));
    }
    if let Some(slot) = legend_slot {
      let slot = if has_funnel {
        funnel_legend_slot(
          frame,
          chart_space.chart.chart_title.as_deref(),
          legend,
          &appearance,
        )
        .unwrap_or(slot)
      } else {
        slot
      };
      lower_legend(
        &mut items,
        slot,
        &legend_entries,
        &appearance,
        has_funnel,
        has_category_axis_title,
      );
    }
  }
  reorder_chartex_text_items(
    &mut items,
    &series,
    chart_space,
    title.as_deref(),
    &legend_entries,
    options.ui_language,
  );
  items
}

fn extract_data_sets(
  chart_space: &cx::ChartSpace,
  formula_resolver: &mut Option<&mut FormulaResolver<'_>>,
) -> HashMap<u32, DataSet> {
  let mut result = HashMap::new();
  let Some(chart_data) = chart_space.chart_data.as_deref() else {
    return result;
  };
  for data in &chart_data.data {
    let mut data_set = DataSet::default();
    for choice in &data.data_choice {
      match choice {
        cx::DataChoice::NumericDimension(dimension) => {
          data_set.numeric.push(NumericDimension {
            role: dimension.r#type,
            levels: numeric_levels(dimension, formula_resolver),
          });
        }
        cx::DataChoice::StringDimension(dimension) => {
          data_set.strings.push(StringDimension {
            role: dimension.r#type,
            levels: string_levels(dimension, formula_resolver),
          });
        }
      }
    }
    result.insert(data.id, data_set);
  }
  result
}

fn numeric_levels(
  dimension: &cx::NumericDimension,
  formula_resolver: &mut Option<&mut FormulaResolver<'_>>,
) -> Vec<SparseLevel<f64>> {
  let (cached, formula, direction) = match dimension.numeric_dimension_choice.as_ref() {
    Some(cx::NumericDimensionChoice::Sequence(sequence)) => (
      sequence.numeric_level.as_slice(),
      sequence.formula.0.xml_content.as_deref(),
      sequence.formula.0.dir.unwrap_or_default(),
    ),
    Some(cx::NumericDimensionChoice::NumericLevel(level)) => (
      std::slice::from_ref(level.as_ref()),
      None,
      cx::FormulaDirection::Col,
    ),
    None => (&[][..], None, cx::FormulaDirection::Col),
  };
  if cached.iter().any(|level| !level.numeric_value.is_empty()) {
    return cached
      .iter()
      .map(|level| SparseLevel {
        name: level.name.clone(),
        format_code: level.format_code.clone(),
        points: sparse_points(
          level.pt_count as usize,
          level
            .numeric_value
            .iter()
            .map(|point| (point.idx as usize, point.xml_content)),
        ),
      })
      .collect();
  }
  let Some(formula) = formula else {
    return Vec::new();
  };
  let Some(matrix) = formula_resolver
    .as_deref_mut()
    .and_then(|resolver| resolver(formula))
  else {
    return Vec::new();
  };
  matrix
    .levels(direction)
    .into_iter()
    .map(|level| SparseLevel {
      name: None,
      format_code: None,
      points: level
        .into_iter()
        .map(|value| value.parse::<f64>().ok())
        .collect(),
    })
    .collect()
}

fn string_levels(
  dimension: &cx::StringDimension,
  formula_resolver: &mut Option<&mut FormulaResolver<'_>>,
) -> Vec<SparseLevel<String>> {
  let (cached, formula, direction) = match dimension.string_dimension_choice.as_ref() {
    Some(cx::StringDimensionChoice::Sequence(sequence)) => (
      sequence.string_level.as_slice(),
      sequence.formula.0.xml_content.as_deref(),
      sequence.formula.0.dir.unwrap_or_default(),
    ),
    Some(cx::StringDimensionChoice::StringLevel(level)) => {
      (std::slice::from_ref(level), None, cx::FormulaDirection::Col)
    }
    None => (&[][..], None, cx::FormulaDirection::Col),
  };
  if cached
    .iter()
    .any(|level| !level.chart_string_value.is_empty())
  {
    return cached
      .iter()
      .map(|level| SparseLevel {
        name: level.name.clone(),
        format_code: None,
        points: sparse_points(
          level.pt_count as usize,
          level
            .chart_string_value
            .iter()
            .map(|point| (point.index as usize, point.xml_content.clone())),
        ),
      })
      .collect();
  }
  let Some(formula) = formula else {
    return Vec::new();
  };
  let Some(matrix) = formula_resolver
    .as_deref_mut()
    .and_then(|resolver| resolver(formula))
  else {
    return Vec::new();
  };
  matrix
    .levels(direction)
    .into_iter()
    .map(|level| SparseLevel {
      name: None,
      format_code: None,
      points: level.into_iter().map(Some).collect(),
    })
    .collect()
}

fn sparse_points<T>(
  declared_count: usize,
  values: impl IntoIterator<Item = (usize, Option<T>)>,
) -> Vec<Option<T>> {
  let values = values.into_iter().collect::<Vec<_>>();
  let count = declared_count.max(values.iter().map(|(index, _)| index + 1).max().unwrap_or(0));
  let mut points = (0..count).map(|_| None).collect::<Vec<_>>();
  for (index, value) in values {
    if let Some(slot) = points.get_mut(index) {
      *slot = value;
    }
  }
  points
}

fn series_model<'a>(
  source: &'a cx::Series,
  source_index: usize,
  data: Option<&DataSet>,
  ui_language: Option<&str>,
) -> SeriesModel<'a> {
  let preferred_role = if matches!(
    source.layout_id,
    cx::SeriesLayout::Treemap | cx::SeriesLayout::Sunburst | cx::SeriesLayout::RegionMap
  ) {
    cx::NumericDimensionType::Size
  } else {
    cx::NumericDimensionType::Val
  };
  let dimension = data
    .and_then(|data| data.numeric(preferred_role))
    .or_else(|| data.and_then(|data| data.numeric(cx::NumericDimensionType::Val)))
    .or_else(|| data.and_then(|data| data.numeric(cx::NumericDimensionType::Y)));
  let level = dimension.and_then(|dimension| dimension.levels.first());
  let categories = data
    .and_then(|data| data.strings(cx::StringDimensionType::Cat))
    .map(|dimension| dimension.levels.clone())
    .unwrap_or_default();
  let explicit_name = source
    .text
    .as_deref()
    .map(cx_text)
    .filter(|name| !name.is_empty())
    .or_else(|| {
      level
        .and_then(|level| level.name.as_deref())
        .filter(|name| !name.is_empty())
        .map(str::to_string)
    });
  // Excel's statistical ChartEx families use the one-based source ordinal
  // for an unnamed numeric series. Other families retain the localized
  // automatic "Series N" resource (region maps are a corpus counterexample).
  let automatic_name = explicit_name.is_none();
  let name = explicit_name.unwrap_or_else(|| {
    if matches!(
      source.layout_id,
      cx::SeriesLayout::ClusteredColumn | cx::SeriesLayout::ParetoLine
    ) {
      (source_index + 1).to_string()
    } else {
      automatic_series_title(ui_language, source_index + 1)
    }
  });
  SeriesModel {
    source,
    source_index,
    name,
    automatic_name,
    layout: source.layout_id,
    values: level.map(|level| level.points.clone()).unwrap_or_default(),
    number_format: level.and_then(|level| level.format_code.clone()),
    categories,
  }
}

fn series_legend_name(series: &SeriesModel<'_>, ui_language: Option<&str>) -> String {
  if series.automatic_name {
    automatic_series_title(ui_language, series.source_index + 1)
  } else {
    series.name.clone()
  }
}

fn chart_appearance(
  chart_space: &cx::ChartSpace,
  options: &ChartExRenderOptions<'_>,
) -> Appearance {
  let chart_style = options.resources.chart_styles.first();
  let color_style = options.resources.color_styles.first();
  let base_palette = color_style
    .map(|style| {
      style
        .color_style_choice
        .iter()
        .filter_map(|choice| resolve_cs_color(choice, options.theme, None))
        .collect::<Vec<_>>()
    })
    .unwrap_or_default();
  let mut palette = if let Some(style) = color_style
    && !base_palette.is_empty()
    && !style.color_style_variation.is_empty()
  {
    // [MS-ODRAWXML] 2.8.3.2 defines the total color set as every authored
    // base color repeated for each variation, in variation-major order. An
    // empty first variation therefore represents the unmodified base set.
    style
      .color_style_variation
      .iter()
      .flat_map(|variation| {
        base_palette
          .iter()
          .map(move |base| apply_color_variation(*base, variation))
      })
      .collect()
  } else {
    base_palette
  };
  if palette.is_empty() {
    palette.extend(options.theme.accents);
  }
  if palette.is_empty() {
    palette.extend(OFFICE_ACCENTS);
  }

  let color_method = color_style.map(|style| color_method(style.method.as_str()));

  let mut title_style = options.title_style.clone();
  let mut axis_title_style = options.label_style.clone();
  let mut label_style = options.label_style.clone();
  let mut data_label_style = options.label_style.clone();
  // Office's ChartEx fixed-format path emits ordinary Latin characters as
  // independent glyphs. In particular, Calibri `ti` in "Cumulative" is not
  // replaced by the font's optional ligature. HarfRust enables `liga`/`clig`
  // by default, so make the Office chart default explicit for every ChartEx
  // text role; DrawingML run properties do not opt these categories back in.
  for style in [
    &mut title_style,
    &mut axis_title_style,
    &mut label_style,
    &mut data_label_style,
  ] {
    style.ligatures = Some(crate::common::OpenTypeLigatures::default());
  }
  let mut chart_fill = options.theme.light1;
  let mut chart_gradient = None;
  let mut chart_stroke = None;
  let mut plot_fill = None;
  let mut plot_gradient = None;
  let mut data_point_gradient = None;
  let mut data_point_effects = None;
  let mut data_point_outline = None;
  let mut data_point_line_outline = None;
  let mut series_line_color = rgb(217, 217, 217);
  let mut series_line_width = 0.75;
  let mut axis_color = rgb(127, 127, 127);
  let mut axis_width = 0.75;
  let mut grid_color = rgb(217, 217, 217);
  let mut grid_width = 0.75;

  if let Some(style) = chart_style {
    chart_gradient = style
      .chart_area
      .shape_properties
      .as_deref()
      .and_then(cs_shape_gradient);
    chart_fill = style
      .chart_area
      .shape_properties
      .as_deref()
      .and_then(|shape| resolve_cs_shape_fill(shape, options.theme, None))
      .unwrap_or(chart_fill);
    chart_stroke = style
      .chart_area
      .shape_properties
      .as_deref()
      .and_then(|shape| {
        resolve_cs_outline(shape, options.theme, None)
          .map(|color| (color, drawingml_outline_width(shape.outline.as_deref())))
      });
    plot_gradient = style
      .plot_area
      .shape_properties
      .as_deref()
      .and_then(cs_shape_gradient);
    plot_fill = style
      .plot_area
      .shape_properties
      .as_deref()
      .and_then(|shape| resolve_cs_shape_fill(shape, options.theme, None));
    data_point_gradient = style
      .data_point
      .shape_properties
      .as_deref()
      .and_then(cs_shape_gradient);
    data_point_effects = style
      .data_point
      .shape_properties
      .as_deref()
      .and_then(cs_shape_effect_source);
    data_point_outline = style
      .data_point
      .shape_properties
      .as_deref()
      .and_then(|shape| shape.outline.as_deref())
      .cloned();
    data_point_line_outline = style
      .data_point_line
      .shape_properties
      .as_deref()
      .and_then(|shape| shape.outline.as_deref())
      .cloned();
    if let Some(shape) = style.series_line.shape_properties.as_deref() {
      series_line_color =
        resolve_cs_outline(shape, options.theme, None).unwrap_or(series_line_color);
      series_line_width = drawingml_outline_width(shape.outline.as_deref());
    }
    if let Some(shape) = style.category_axis.shape_properties.as_deref()
      && let Some(color) = resolve_cs_outline_over(shape, options.theme, None, chart_fill)
    {
      axis_color = color;
      axis_width = drawingml_outline_width(shape.outline.as_deref());
    }
    if let Some(shape) = style.gridline_major.shape_properties.as_deref()
      && let Some(color) = resolve_cs_outline_over(shape, options.theme, None, chart_fill)
    {
      grid_color = color;
      grid_width = drawingml_outline_width(shape.outline.as_deref());
    }
    apply_style_text(
      &mut title_style,
      style.title_style.text_character_properties_type.as_deref(),
      &style.title_style.font_reference,
      options.theme,
    );
    apply_style_text(
      &mut axis_title_style,
      style.axis_title.text_character_properties_type.as_deref(),
      &style.axis_title.font_reference,
      options.theme,
    );
    apply_style_text(
      &mut label_style,
      style.legend_style.text_character_properties_type.as_deref(),
      &style.legend_style.font_reference,
      options.theme,
    );
    apply_style_text(
      &mut data_label_style,
      style.data_label.text_character_properties_type.as_deref(),
      &style.data_label.font_reference,
      options.theme,
    );
  }

  chart_fill = chart_space
    .shape_properties
    .as_deref()
    .and_then(|shape| resolve_cx_shape_fill(shape, options.theme, None))
    .unwrap_or(chart_fill);
  if let Some(shape) = chart_space.shape_properties.as_deref()
    && shape.shape_properties_choice2.is_some()
  {
    chart_gradient = cx_shape_gradient(shape);
  }
  let chart_area_allows_no_line = chart_style.is_some_and(|style| {
    style
      .chart_area
      .modifiers
      .as_ref()
      .is_some_and(|modifiers| modifiers.iter().any(|value| value == "allowNoLineOverride"))
  });
  match chart_space.shape_properties.as_deref() {
    Some(shape) if shape.outline.is_some() => {
      // An authored `a:noFill` line is an explicit no-line override; do not
      // fall back to the ChartStyle outline when color resolution returns
      // `None` for that case.
      chart_stroke = resolve_cx_outline(shape, options.theme, None)
        .map(|color| (color, drawingml_outline_width(shape.outline.as_deref())));
    }
    // [MS-ODRAWXML] 2.8.4.8 permits this ChartStyle line to be replaced with
    // no line. PowerPoint applies that host default to an unformatted ChartEx
    // chart area; its fixed-format Of16 and funnel references consequently
    // contain no outer chart-frame stroke.
    _ if options.host == ChartExHost::PowerPoint && chart_area_allows_no_line => {
      chart_stroke = None;
    }
    _ => {}
  }
  plot_fill = chart_space
    .chart
    .plot_area
    .shape_properties
    .as_deref()
    .and_then(|shape| resolve_cx_shape_fill(shape, options.theme, None))
    .or(plot_fill);
  if let Some(shape) = chart_space.chart.plot_area.shape_properties.as_deref()
    && shape.shape_properties_choice2.is_some()
  {
    plot_gradient = cx_shape_gradient(shape);
  }
  if data_point_gradient.is_some() {
    // Office selects a dark automatic foreground for labels embedded in the
    // lightened data-point gradient, even when the style's fallback fontRef
    // points at the light theme color.
    data_label_style.color = rgb(66, 66, 66);
    data_label_style.color_is_automatic = false;
  }

  Appearance {
    host: options.host,
    chart_width_pt: options.frame.width_pt,
    theme: options.theme,
    chart_fill,
    chart_gradient,
    chart_stroke,
    plot_fill,
    plot_gradient,
    data_point_gradient,
    data_point_effects,
    data_point_outline,
    data_point_line_outline,
    series_line_color,
    series_line_width,
    axis_color,
    axis_width,
    grid_color,
    grid_width,
    title_style,
    axis_title_style,
    label_style,
    data_label_style,
    palette,
    color_method,
  }
}

fn color_method(value: &str) -> cs::ColorStyleMethodEnum {
  match value {
    "cycle" => cs::ColorStyleMethodEnum::Cycle,
    "withinLinear" => cs::ColorStyleMethodEnum::WithinLinear,
    "acrossLinear" => cs::ColorStyleMethodEnum::AcrossLinear,
    "withinLinearReversed" => cs::ColorStyleMethodEnum::WithinLinearReversed,
    "acrossLinearReversed" => cs::ColorStyleMethodEnum::AcrossLinearReversed,
    // [MS-ODRAWXML] 2.8.4.1 requires unknown methods to behave as `cycle`.
    _ => cs::ColorStyleMethodEnum::Cycle,
  }
}

fn resolve_cs_color(
  choice: &cs::ColorStyleChoice,
  theme: ChartExTheme,
  placeholder: Option<RgbColor>,
) -> Option<RgbColor> {
  let color = match choice {
    cs::ColorStyleChoice::RgbColorModelPercentage(color) => rgb_percent_color(color),
    cs::ColorStyleChoice::RgbColorModelHex(color) => rgb_hex_color(color),
    cs::ColorStyleChoice::HslColor(color) => hsl_color(color),
    cs::ColorStyleChoice::SystemColor(color) => system_color(color),
    cs::ColorStyleChoice::SchemeColor(color) => scheme_color(color),
    cs::ColorStyleChoice::PresetColor(color) => preset_color(color),
  };
  resolve_color(&color, theme, placeholder)
}

fn resolve_font_reference(reference: &cs::FontReference, theme: ChartExTheme) -> Option<RgbColor> {
  let color = match reference.font_reference_choice.as_ref()? {
    cs::FontReferenceChoice::RgbColorModelPercentage(color) => rgb_percent_color(color),
    cs::FontReferenceChoice::RgbColorModelHex(color) => rgb_hex_color(color),
    cs::FontReferenceChoice::HslColor(color) => hsl_color(color),
    cs::FontReferenceChoice::SystemColor(color) => system_color(color),
    cs::FontReferenceChoice::SchemeColor(color)
    | cs::FontReferenceChoice::ChartStyleSchemeColor(color) => scheme_color(color),
    cs::FontReferenceChoice::PresetColor(color) => preset_color(color),
  };
  resolve_color(&color, theme, None)
}

fn apply_style_text(
  style: &mut TextStyle,
  properties: Option<&cs::TextCharacterPropertiesType>,
  font_reference: &cs::FontReference,
  theme: ChartExTheme,
) {
  if let Some(color) = resolve_font_reference(font_reference, theme) {
    style.color = color;
    style.color_is_automatic = false;
  }
  let Some(properties) = properties else {
    return;
  };
  if let Some(size) = properties.font_size {
    style.font_size_pt = size as f32 / 100.0;
  }
  if let Some(bold) = properties.bold {
    style.bold = bold.as_bool();
  }
  if let Some(italic) = properties.italic {
    style.italic = italic.as_bool();
  }
  let effects = match properties.text_character_properties_type_choice2.as_ref() {
    Some(cs::TextCharacterPropertiesTypeChoice2::EffectList(effects)) => {
      Some(ChartExEffectSource::List(effects.as_ref().clone()))
    }
    Some(cs::TextCharacterPropertiesTypeChoice2::EffectDag(effects)) => {
      Some(ChartExEffectSource::Dag(effects.as_ref().clone()))
    }
    None => None,
  };
  if let Some(effects) = effects {
    style.drawingml_text_effects = resolve_chart_effects(&effects, theme, Some(style.color));
  }
}

fn apply_chartex_text_properties(
  style: &mut TextStyle,
  properties: Option<&cx::TxPrTextBody>,
  theme: ChartExTheme,
) {
  let Some(properties) = properties else {
    return;
  };
  for paragraph in &properties.paragraph {
    if let Some(defaults) = paragraph
      .paragraph_properties
      .as_deref()
      .and_then(|properties| properties.default_run_properties.as_deref())
    {
      if let Some(size) = defaults.font_size {
        style.font_size_pt = size as f32 / 100.0;
      }
      if let Some(bold) = defaults.bold.as_ref() {
        style.bold = bold.as_bool();
      }
      if let Some(fill) = defaults.default_run_properties_choice1.as_ref()
        && let Some(color) = resolve_run_fill(fill, theme)
      {
        style.color = color;
        style.color_is_automatic = false;
      }
      if let Some(typeface) = defaults
        .latin_font
        .as_ref()
        .and_then(|font| font.typeface.as_deref())
        .filter(|typeface| !typeface.is_empty() && !typeface.starts_with('+'))
      {
        style.font_family = Some(Arc::from(typeface));
      }
    }
    if let Some(properties) = paragraph
      .paragraph_choice
      .iter()
      .find_map(|choice| match choice {
        a::ParagraphChoice::Run(run) => run.run_properties.as_deref(),
        a::ParagraphChoice::Field(field) => field.run_properties.as_deref(),
        a::ParagraphChoice::Break(_)
        | a::ParagraphChoice::TextMath(_)
        | a::ParagraphChoice::AlternateContent(_) => None,
      })
    {
      apply_chartex_run_properties(style, properties, theme);
    }
  }
}

fn apply_chartex_run_properties(
  style: &mut TextStyle,
  properties: &a::RunProperties,
  theme: ChartExTheme,
) {
  if let Some(size) = properties.font_size.filter(|size| *size > 0) {
    style.font_size_pt = size as f32 / 100.0;
  }
  if let Some(bold) = properties.bold.as_ref() {
    style.bold = bold.as_bool();
  }
  if let Some(italic) = properties.italic.as_ref() {
    style.italic = italic.as_bool();
  }
  if let Some(typeface) = properties
    .latin_font
    .as_ref()
    .and_then(|font| font.typeface.as_deref())
    .filter(|typeface| !typeface.is_empty() && !typeface.starts_with('+'))
  {
    style.font_family = Some(Arc::from(typeface));
  }
  if let Some(typeface) = properties
    .east_asian_font
    .as_ref()
    .and_then(|font| font.typeface.as_deref())
    .filter(|typeface| !typeface.is_empty() && !typeface.starts_with('+'))
  {
    style.east_asia_font_family = Some(Arc::from(typeface));
  }
  if let Some(a::RunPropertiesChoice::SolidFill(fill)) = properties.run_properties_choice1.as_ref()
    && let Some(color) = resolve_solid_fill(fill, theme, None)
  {
    style.color = color;
    style.color_is_automatic = false;
  }
}

fn resolve_run_fill(fill: &a::DefaultRunPropertiesChoice, theme: ChartExTheme) -> Option<RgbColor> {
  let a::DefaultRunPropertiesChoice::SolidFill(fill) = fill else {
    return None;
  };
  resolve_solid_fill(fill, theme, None)
}

fn cs_shape_gradient(shape: &cs::ShapeProperties) -> Option<a::GradientFill> {
  match shape.shape_properties_choice2.as_ref()? {
    cs::ShapePropertiesChoice2::GradientFill(fill) => Some(fill.as_ref().clone()),
    _ => None,
  }
}

fn cx_shape_gradient(shape: &cx::ShapeProperties) -> Option<a::GradientFill> {
  match shape.shape_properties_choice2.as_ref()? {
    cx::ShapePropertiesChoice2::GradientFill(fill) => Some(fill.as_ref().clone()),
    _ => None,
  }
}

fn cs_shape_effect_source(shape: &cs::ShapeProperties) -> Option<ChartExEffectSource> {
  match shape.shape_properties_choice3.as_ref()? {
    cs::ShapePropertiesChoice3::EffectList(effects) => {
      Some(ChartExEffectSource::List(effects.as_ref().clone()))
    }
    cs::ShapePropertiesChoice3::EffectDag(effects) => {
      Some(ChartExEffectSource::Dag(effects.as_ref().clone()))
    }
  }
}

struct ChartExEffectColorResolver {
  theme: ChartExTheme,
  placeholder: Option<Color>,
}

impl ChartExEffectColorResolver {
  fn resolve(
    &self,
    color: Option<Color>,
  ) -> Option<crate::common::drawingml_image_effects::ResolvedEffectColor> {
    let color = color?;
    let mut scheme_resolver = |value| self.theme.drawing_color(value);
    let resolved = color.resolve_rgb(&mut scheme_resolver, self.placeholder.as_ref())?;
    Some(
      crate::common::drawingml_image_effects::ResolvedEffectColor {
        color: rgb(resolved.r, resolved.g, resolved.b),
        alpha: ((resolved.alpha.clamp(0, 100_000) as f32 / 100_000.0) * 255.0).round() as u8,
      },
    )
  }
}

impl crate::common::drawingml_image_effects::ImageEffectColorResolver
  for ChartExEffectColorResolver
{
  fn alpha_inverse(
    &self,
    choice: &a::AlphaInverseChoice,
  ) -> Option<crate::common::drawingml_image_effects::ResolvedEffectColor> {
    self.resolve(Color::from_alpha_inverse_choice(choice))
  }

  fn color_from(
    &self,
    choice: &a::ColorFromChoice,
  ) -> Option<crate::common::drawingml_image_effects::ResolvedEffectColor> {
    self.resolve(Color::from_color_from_choice(choice))
  }

  fn color_to(
    &self,
    choice: &a::ColorToChoice,
  ) -> Option<crate::common::drawingml_image_effects::ResolvedEffectColor> {
    self.resolve(Color::from_color_to_choice(choice))
  }

  fn color_replacement(
    &self,
    choice: &a::ColorReplacementChoice,
  ) -> Option<crate::common::drawingml_image_effects::ResolvedEffectColor> {
    self.resolve(Color::from_color_replacement_choice(choice))
  }

  fn duotone(
    &self,
    choice: &a::DuotoneChoice,
  ) -> Option<crate::common::drawingml_image_effects::ResolvedEffectColor> {
    self.resolve(Color::from_duotone_choice(choice))
  }

  fn solid_fill(
    &self,
    choice: &a::SolidFillChoice,
  ) -> Option<crate::common::drawingml_image_effects::ResolvedEffectColor> {
    self.resolve(Color::from_solid_fill_choice(choice))
  }

  fn gradient_stop(
    &self,
    choice: &a::GradientStopChoice,
  ) -> Option<crate::common::drawingml_image_effects::ResolvedEffectColor> {
    self.resolve(Color::from_gradient_stop_choice(choice))
  }

  fn foreground(
    &self,
    choice: &a::ForegroundColorChoice,
  ) -> Option<crate::common::drawingml_image_effects::ResolvedEffectColor> {
    self.resolve(Color::from_foreground_color_choice(choice))
  }

  fn background(
    &self,
    choice: &a::BackgroundColorChoice,
  ) -> Option<crate::common::drawingml_image_effects::ResolvedEffectColor> {
    self.resolve(Color::from_background_color_choice(choice))
  }

  fn glow(
    &self,
    choice: &a::GlowChoice,
  ) -> Option<crate::common::drawingml_image_effects::ResolvedEffectColor> {
    self.resolve(Color::from_glow_choice(choice))
  }

  fn inner_shadow(
    &self,
    choice: &a::InnerShadowChoice,
  ) -> Option<crate::common::drawingml_image_effects::ResolvedEffectColor> {
    self.resolve(Color::from_inner_shadow_choice(choice))
  }

  fn outer_shadow(
    &self,
    choice: &a::OuterShadowChoice,
  ) -> Option<crate::common::drawingml_image_effects::ResolvedEffectColor> {
    self.resolve(Color::from_outer_shadow_choice(choice))
  }

  fn preset_shadow(
    &self,
    choice: &a::PresetShadowChoice,
  ) -> Option<crate::common::drawingml_image_effects::ResolvedEffectColor> {
    self.resolve(Color::from_preset_shadow_choice(choice))
  }
}

fn resolve_chart_effects(
  source: &ChartExEffectSource,
  theme: ChartExTheme,
  placeholder: Option<RgbColor>,
) -> Option<crate::common::drawingml_image_effects::ImageEffectContainer> {
  let placeholder = placeholder.map(|color| {
    Color::RgbHex(RgbHexColor {
      value: format!("{:02X}{:02X}{:02X}", color.r, color.g, color.b),
      transformations: Vec::new(),
    })
  });
  let resolver = ChartExEffectColorResolver { theme, placeholder };
  let effects = match source {
    ChartExEffectSource::List(source) => {
      crate::common::drawingml_image_effects::from_effect_list(source, None, &resolver)
    }
    ChartExEffectSource::Dag(source) => {
      crate::common::drawingml_image_effects::from_effect_dag(source, None, &resolver)
    }
  };
  (!effects.effects.is_empty()).then_some(effects)
}

fn resolve_cs_shape_fill(
  shape: &cs::ShapeProperties,
  theme: ChartExTheme,
  placeholder: Option<RgbColor>,
) -> Option<RgbColor> {
  match shape.shape_properties_choice2.as_ref()? {
    cs::ShapePropertiesChoice2::SolidFill(fill) => resolve_solid_fill(fill, theme, placeholder),
    cs::ShapePropertiesChoice2::GradientFill(fill) => {
      resolve_gradient_fill(fill, theme, placeholder)
    }
    _ => None,
  }
}

fn resolve_cx_shape_fill(
  shape: &cx::ShapeProperties,
  theme: ChartExTheme,
  placeholder: Option<RgbColor>,
) -> Option<RgbColor> {
  match shape.shape_properties_choice2.as_ref()? {
    cx::ShapePropertiesChoice2::SolidFill(fill) => resolve_solid_fill(fill, theme, placeholder),
    cx::ShapePropertiesChoice2::GradientFill(fill) => {
      resolve_gradient_fill(fill, theme, placeholder)
    }
    _ => None,
  }
}

fn resolve_cs_outline(
  shape: &cs::ShapeProperties,
  theme: ChartExTheme,
  placeholder: Option<RgbColor>,
) -> Option<RgbColor> {
  resolve_outline(shape.outline.as_deref()?, theme, placeholder)
}

fn resolve_outline(
  outline: &a::Outline,
  theme: ChartExTheme,
  placeholder: Option<RgbColor>,
) -> Option<RgbColor> {
  let fill = outline.outline_choice1.as_ref()?;
  let a::OutlineChoice::SolidFill(fill) = fill else {
    return None;
  };
  resolve_solid_fill(fill.as_ref(), theme, placeholder)
}

fn drawingml_outline_width(outline: Option<&a::Outline>) -> f32 {
  outline
    .and_then(|outline| outline.width)
    .map(|width| crate::units::emu_to_points(i64::from(width)))
    .unwrap_or(0.75)
}

fn resolve_cs_outline_over(
  shape: &cs::ShapeProperties,
  theme: ChartExTheme,
  placeholder: Option<RgbColor>,
  background: RgbColor,
) -> Option<RgbColor> {
  let fill = shape.outline.as_deref()?.outline_choice1.as_ref()?;
  let a::OutlineChoice::SolidFill(fill) = fill else {
    return None;
  };
  let choice = fill.solid_fill_choice.as_ref()?;
  let color = Color::from_solid_fill_choice(choice)?;
  let placeholder = placeholder.map(|color| {
    Color::RgbHex(RgbHexColor {
      value: format!("{:02X}{:02X}{:02X}", color.r, color.g, color.b),
      transformations: Vec::new(),
    })
  });
  let mut resolver = |value| theme.drawing_color(value);
  let resolved = color.resolve_rgb(&mut resolver, placeholder.as_ref())?;
  let opacity = resolved.alpha.clamp(0, 100_000) as f32 / 100_000.0;
  Some(rgb(
    (f32::from(background.r) * (1.0 - opacity) + f32::from(resolved.r) * opacity).round() as u8,
    (f32::from(background.g) * (1.0 - opacity) + f32::from(resolved.g) * opacity).round() as u8,
    (f32::from(background.b) * (1.0 - opacity) + f32::from(resolved.b) * opacity).round() as u8,
  ))
}

fn resolve_cx_outline(
  shape: &cx::ShapeProperties,
  theme: ChartExTheme,
  placeholder: Option<RgbColor>,
) -> Option<RgbColor> {
  resolve_outline(shape.outline.as_deref()?, theme, placeholder)
}

fn resolve_solid_fill(
  fill: &a::SolidFill,
  theme: ChartExTheme,
  placeholder: Option<RgbColor>,
) -> Option<RgbColor> {
  let choice = fill.solid_fill_choice.as_ref()?;
  let color = Color::from_solid_fill_choice(choice)?;
  resolve_color(&color, theme, placeholder)
}

fn resolve_gradient_fill(
  fill: &a::GradientFill,
  theme: ChartExTheme,
  placeholder: Option<RgbColor>,
) -> Option<RgbColor> {
  let stops = &fill.gradient_stop_list.as_ref()?.gradient_stop;
  let stop = stops
    .get(usize::from(stops.len() > 2))
    .or_else(|| stops.first())?;
  let color = Color::from_gradient_stop_choice(stop.gradient_stop_choice.as_ref()?)?;
  resolve_color(&color, theme, placeholder)
}

fn resolve_color(
  color: &Color,
  theme: ChartExTheme,
  placeholder: Option<RgbColor>,
) -> Option<RgbColor> {
  let placeholder = placeholder.map(|color| {
    Color::RgbHex(RgbHexColor {
      value: format!("{:02X}{:02X}{:02X}", color.r, color.g, color.b),
      transformations: Vec::new(),
    })
  });
  let mut resolver = |value| theme.drawing_color(value);
  let resolved = color.resolve_rgb(&mut resolver, placeholder.as_ref())?;
  Some(rgb(resolved.r, resolved.g, resolved.b))
}

fn apply_color_variation(color: RgbColor, variation: &cs::ColorStyleVariation) -> RgbColor {
  let transformations = variation
    .color_style_variation_choice
    .iter()
    .map(color_style_variation_transform)
    .collect::<Vec<_>>();
  let mut color = ResolvedColor::new(color.r, color.g, color.b);
  apply_transformations(&mut color, &transformations);
  rgb(color.r, color.g, color.b)
}

fn color_style_variation_transform(choice: &cs::ColorStyleVariationChoice) -> ColorTransformation {
  use ColorTransformationKind as Kind;
  use cs::ColorStyleVariationChoice as Choice;

  let (kind, value) = match choice {
    Choice::Tint(value) => (Kind::Tint, Some(drawingml_percent(value.val))),
    Choice::Shade(value) => (Kind::Shade, Some(drawingml_percent(value.val))),
    Choice::Complement => (Kind::Comp, None),
    Choice::Inverse => (Kind::Inv, None),
    Choice::Gray => (Kind::Gray, None),
    Choice::Alpha(value) => (Kind::Alpha, Some(drawingml_percent(value.val))),
    Choice::AlphaOffset(value) => (Kind::AlphaOff, Some(drawingml_percent(value.val))),
    Choice::AlphaModulation(value) => (Kind::AlphaMod, Some(drawingml_percent(value.val))),
    Choice::Hue(value) => (Kind::Hue, Some(value.val)),
    Choice::HueOffset(value) => (Kind::HueOff, Some(value.val)),
    Choice::HueModulation(value) => (Kind::HueMod, Some(drawingml_percent(value.val))),
    Choice::Saturation(value) => (Kind::Sat, Some(drawingml_percent(value.val))),
    Choice::SaturationOffset(value) => (Kind::SatOff, Some(drawingml_percent(value.val))),
    Choice::SaturationModulation(value) => (Kind::SatMod, Some(drawingml_percent(value.val))),
    Choice::Luminance(value) => (Kind::Lum, Some(drawingml_percent(value.val))),
    Choice::LuminanceOffset(value) => (Kind::LumOff, Some(drawingml_percent(value.val))),
    Choice::LuminanceModulation(value) => (Kind::LumMod, Some(drawingml_percent(value.val))),
    Choice::Red(value) => (Kind::Red, Some(drawingml_percent(value.val))),
    Choice::RedOffset(value) => (Kind::RedOff, Some(drawingml_percent(value.val))),
    Choice::RedModulation(value) => (Kind::RedMod, Some(drawingml_percent(value.val))),
    Choice::Green(value) => (Kind::Green, Some(drawingml_percent(value.val))),
    Choice::GreenOffset(value) => (Kind::GreenOff, Some(drawingml_percent(value.val))),
    Choice::GreenModulation(value) => (Kind::GreenMod, Some(drawingml_percent(value.val))),
    Choice::Blue(value) => (Kind::Blue, Some(drawingml_percent(value.val))),
    Choice::BlueOffset(value) => (Kind::BlueOff, Some(drawingml_percent(value.val))),
    Choice::BlueModulation(value) => (Kind::BlueMod, Some(drawingml_percent(value.val))),
    Choice::Gamma => (Kind::Gamma, None),
    Choice::InverseGamma => (Kind::InvGamma, None),
  };
  ColorTransformation { kind, value }
}

fn drawingml_percent(value: ooxmlsdk::simple_type::DrawingmlPercentageValue) -> i32 {
  value.as_drawingml_percent()
}

fn transform_rgb(color: RgbColor, kind: ColorTransformationKind, value: i32) -> RgbColor {
  let mut resolved = ResolvedColor::new(color.r, color.g, color.b);
  apply_transformations(
    &mut resolved,
    &[ColorTransformation {
      kind,
      value: Some(value),
    }],
  );
  rgb(resolved.r, resolved.g, resolved.b)
}

fn tint(color: RgbColor, amount: f32) -> RgbColor {
  let amount = amount.clamp(0.0, 1.0);
  rgb(
    (f32::from(color.r) + (255.0 - f32::from(color.r)) * amount).round() as u8,
    (f32::from(color.g) + (255.0 - f32::from(color.g)) * amount).round() as u8,
    (f32::from(color.b) + (255.0 - f32::from(color.b)) * amount).round() as u8,
  )
}

fn shade(color: RgbColor, amount: f32) -> RgbColor {
  let factor = (1.0 - amount).clamp(0.0, 1.0);
  rgb(
    (f32::from(color.r) * factor).round() as u8,
    (f32::from(color.g) * factor).round() as u8,
    (f32::from(color.b) * factor).round() as u8,
  )
}

fn chart_bands(
  frame: ChartFrame,
  title: Option<&cx::ChartTitle>,
  legend: Option<&cx::Legend>,
  host: ChartExHost,
  label_font_size_pt: f32,
  has_category_axis_title: bool,
) -> (PlotRect, Option<PlotRect>, Option<PlotRect>) {
  let mut plot = PlotRect {
    x: frame.x_pt,
    y: frame.y_pt + frame.height_pt * 0.02,
    width: frame.width_pt,
    height: frame.height_pt * 0.96,
  };
  let mut title_slot = None;
  let mut legend_slot = None;
  let top_title_with_top_legend = title.is_some_and(|title| {
    title.pos.unwrap_or(cx::SidePos::T) == cx::SidePos::T
      && !title.overlay.is_some_and(|overlay| overlay.as_bool())
  }) && legend.is_some_and(|legend| {
    legend.pos.unwrap_or(cx::SidePos::R) == cx::SidePos::T
      && !legend.overlay.is_some_and(|overlay| overlay.as_bool())
  });
  let excel_top_title_with_top_legend = host == ChartExHost::Excel && top_title_with_top_legend;
  let powerpoint_top_title_with_top_legend =
    host == ChartExHost::PowerPoint && top_title_with_top_legend;
  let automatic_title_with_top_legend = excel_top_title_with_top_legend
    && title.is_some_and(|title| chart_title_text(title).is_empty());
  if let Some(title) = title {
    let overlay = title.overlay.is_some_and(|value| value.as_bool());
    let side = title.pos.unwrap_or(cx::SidePos::T);
    let mut thickness = if matches!(side, cx::SidePos::T | cx::SidePos::B) {
      frame.height_pt
        * if automatic_title_with_top_legend && side == cx::SidePos::T {
          0.125
        } else if excel_top_title_with_top_legend && side == cx::SidePos::T {
          0.112
        } else if host == ChartExHost::PowerPoint
          && side == cx::SidePos::T
          && chart_title_text(title).is_empty()
        {
          // PowerPoint's empty ChartEx title reserves a compact one-line UI
          // resource band. The Office 2016 waterfall, box-whisker, and
          // sunburst decks all retain 95% of the inset plot height below it.
          0.05
        } else {
          0.09
        }
    } else {
      frame.width_pt * 0.14
    };
    if matches!(side, cx::SidePos::T | cx::SidePos::B) {
      // An authored title outline participates in Office's automatic title
      // box.  The stroke is centered on the shape boundary, so half of its
      // explicit width extends the reserved band. waterfall2.xlsx exposes
      // this independently with a 3pt dashed title outline.
      thickness += title
        .shape_properties
        .as_deref()
        .and_then(|shape| shape.outline.as_deref())
        .and_then(|outline| outline.width)
        .map(|width| crate::units::emu_to_points(i64::from(width)) * 0.5)
        .unwrap_or(0.0);
    }
    let slot = reserve_side(&mut plot, side, thickness, overlay);
    title_slot = Some(slot);
  }
  if let Some(legend) = legend {
    let overlay = legend.overlay.is_some_and(|value| value.as_bool());
    let side = legend.pos.unwrap_or(cx::SidePos::R);
    let thickness = if matches!(side, cx::SidePos::T | cx::SidePos::B) {
      frame.height_pt
        * if automatic_title_with_top_legend && side == cx::SidePos::T {
          0.14
        } else if excel_top_title_with_top_legend && side == cx::SidePos::T {
          0.145
        } else if powerpoint_top_title_with_top_legend && side == cx::SidePos::T {
          // PowerPoint's one-line ChartEx legend band is text-em based. The
          // compact 9pt style resolves to 2.3 ems, while the 11.97pt Of16-01
          // style reaches its physical profile at 2.2 ems. Keeping the band
          // tied to the resolved ChartStyle font preserves both chart sizes.
          label_font_size_pt * if label_font_size_pt <= 9.5 { 2.3 } else { 2.2 } / frame.height_pt
        } else if host == ChartExHost::Excel && side == cx::SidePos::B && has_category_axis_title {
          // Excel lays out a bottom legend after the category-axis title.
          // Its automatic band is a physical one-line frame with symmetric
          // title/legend separation, rather than the generic percentage of
          // the chart height.  The 9pt waterfall reference resolves to the
          // 28.6pt band visible after Excel's fixed-output grid rounding.
          label_font_size_pt * 3.173 / frame.height_pt
        } else {
          0.115
        }
    } else {
      frame.width_pt * 0.18
    };
    legend_slot = Some(reserve_side(&mut plot, side, thickness, overlay));
  }
  (plot, title_slot, legend_slot)
}

fn funnel_chart_plot(
  frame: ChartFrame,
  fallback: PlotRect,
  title: Option<&cx::ChartTitle>,
  legend: Option<&cx::Legend>,
  appearance: &Appearance,
) -> PlotRect {
  if title.is_some_and(|title| {
    title.pos.unwrap_or(cx::SidePos::T) != cx::SidePos::T
      || title.overlay.is_some_and(|overlay| overlay.as_bool())
  }) || legend.is_some_and(|legend| {
    legend.pos.unwrap_or(cx::SidePos::R) != cx::SidePos::T
      || legend.overlay.is_some_and(|overlay| overlay.as_bool())
  }) {
    return fallback;
  }

  // Office's funnel plot bands are text-em based rather than percentages of
  // the chart frame. This keeps the same 27.36pt title band in the two XLSX
  // counterexamples even though their chart heights differ substantially.
  let mut top = title.map_or(6.6, |_| appearance.title_style.font_size_pt * 1.954);
  if legend.is_some() {
    top += appearance.label_style.font_size_pt * 2.4;
  }
  let bottom = 6.6;
  PlotRect {
    x: frame.x_pt,
    y: frame.y_pt + top,
    width: frame.width_pt,
    height: (frame.height_pt - top - bottom).max(1.0),
  }
}

fn funnel_legend_slot(
  frame: ChartFrame,
  title: Option<&cx::ChartTitle>,
  legend: Option<&cx::Legend>,
  appearance: &Appearance,
) -> Option<PlotRect> {
  let title = title?;
  let legend = legend?;
  if title.pos.unwrap_or(cx::SidePos::T) != cx::SidePos::T
    || title.overlay.is_some_and(|overlay| overlay.as_bool())
    || legend.pos.unwrap_or(cx::SidePos::R) != cx::SidePos::T
    || legend.overlay.is_some_and(|overlay| overlay.as_bool())
  {
    return None;
  }

  // PowerPoint's top funnel legend is a one-line object positioned from the
  // title em box, not vertically centered in chart_bands' percentage slot.
  // funnel-pp1.pptx fixes the row at 2.22 title ems from the chart top.
  let label_size = appearance.label_style.font_size_pt;
  Some(PlotRect {
    x: frame.x_pt,
    y: frame.y_pt + appearance.title_style.font_size_pt * 2.22 - label_size * 0.0675,
    width: frame.width_pt,
    height: label_size,
  })
}

fn reserve_side(plot: &mut PlotRect, side: cx::SidePos, thickness: f32, overlay: bool) -> PlotRect {
  match side {
    cx::SidePos::T => {
      let slot = PlotRect {
        x: plot.x,
        y: plot.y,
        width: plot.width,
        height: thickness,
      };
      if !overlay {
        plot.y += thickness;
        plot.height = (plot.height - thickness).max(1.0);
      }
      slot
    }
    cx::SidePos::B => {
      let slot = PlotRect {
        x: plot.x,
        y: plot.y + plot.height - thickness,
        width: plot.width,
        height: thickness,
      };
      if !overlay {
        plot.height = (plot.height - thickness).max(1.0);
      }
      slot
    }
    cx::SidePos::L => {
      let slot = PlotRect {
        x: plot.x,
        y: plot.y,
        width: thickness,
        height: plot.height,
      };
      if !overlay {
        plot.x += thickness;
        plot.width = (plot.width - thickness).max(1.0);
      }
      slot
    }
    cx::SidePos::R => {
      let slot = PlotRect {
        x: plot.x + plot.width - thickness,
        y: plot.y,
        width: thickness,
        height: plot.height,
      };
      if !overlay {
        plot.width = (plot.width - thickness).max(1.0);
      }
      slot
    }
  }
}

fn lower_legend(
  items: &mut Vec<PageItem>,
  slot: PlotRect,
  entries: &[(String, RgbColor)],
  appearance: &Appearance,
  compact_funnel: bool,
  has_category_axis_title: bool,
) {
  if entries.is_empty() {
    return;
  }
  let style = &appearance.label_style;
  let horizontal = slot.width > slot.height * 2.0;
  let powerpoint_compact =
    appearance.host == ChartExHost::PowerPoint && horizontal && style.font_size_pt > 9.5;
  let marker_width = style.font_size_pt
    * if compact_funnel {
      0.59
    } else if powerpoint_compact {
      0.60
    } else {
      0.8
    };
  let marker_to_text = style.font_size_pt
    * if compact_funnel {
      0.33
    } else if powerpoint_compact {
      0.31
    } else {
      0.42
    };
  let entry_spacing = style.font_size_pt
    * if powerpoint_compact && !compact_funnel {
      0.61
    } else {
      0.8
    };
  let content_widths = entries
    .iter()
    .map(|(name, _)| marker_width + marker_to_text + text_width(name, style))
    .collect::<Vec<_>>();
  let total_horizontal_width =
    content_widths.iter().sum::<f32>() + entry_spacing * entries.len().saturating_sub(1) as f32;
  let step = if horizontal {
    0.0
  } else {
    (style.font_size_pt * 1.55).min(slot.height / entries.len() as f32)
  };
  // Office's automatic horizontal legend frame is offset from the nominal
  // chart band by its text-em inset. The marker and label share that frame,
  // so applying the offset to the complete row preserves entry spacing.
  let mut horizontal_x = slot.x
    + (slot.width - total_horizontal_width).max(0.0) * 0.5
    + if horizontal {
      style.font_size_pt
        * if compact_funnel || powerpoint_compact {
          0.13
        } else {
          0.2
        }
    } else {
      0.0
    };
  for (index, (name, color)) in entries.iter().enumerate() {
    let (x, y) = if horizontal {
      let x = horizontal_x;
      horizontal_x += content_widths[index] + entry_spacing;
      (
        x,
        slot.y + (slot.height - style.font_size_pt) * 0.5 + style.font_size_pt * 0.0675
          - if appearance.host == ChartExHost::Excel && has_category_axis_title {
            // Excel's bottom legend text frame is biased toward the adjacent
            // category title.  Keeping the optical offset text-em based also
            // moves the marker and label as one authored legend row.
            style.font_size_pt * 0.35
          } else {
            0.0
          },
      )
    } else {
      (slot.x, slot.y + step * index as f32)
    };
    push_data_point_rect(
      items,
      PlotRect {
        x,
        y: y
          + style.font_size_pt
            * if compact_funnel || powerpoint_compact {
              0.235
            } else {
              0.18
            },
        width: marker_width,
        height: marker_width,
      },
      *color,
      appearance,
      None,
      None,
    );
    let text_x = x + marker_width + marker_to_text;
    if let Some((prefix, ordinal)) = split_automatic_legend_name(name) {
      let prefix_width = TextMetrics::new().measure_text(prefix, style);
      push_text(items, text_x, y, prefix.to_string(), style.clone());
      push_text(
        items,
        text_x + prefix_width,
        y,
        ordinal.to_string(),
        style.clone(),
      );
    } else {
      push_text(items, text_x, y, name.clone(), style.clone());
    }
  }
}

fn split_automatic_legend_name(name: &str) -> Option<(&str, &str)> {
  let separator = name.rfind(' ')?;
  let (prefix, ordinal) = name.split_at(separator + 1);
  (!prefix.trim().is_empty()
    && !ordinal.is_empty()
    && ordinal.chars().all(|character| character.is_ascii_digit()))
  .then_some((prefix, ordinal))
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

fn lower_nonhierarchical(
  items: &mut Vec<PageItem>,
  plot: PlotRect,
  series: &[SeriesModel<'_>],
  chart_space: &cx::ChartSpace,
  appearance: &Appearance,
  ui_language: Option<&str>,
  legend_entries: &mut Vec<(String, RgbColor)>,
) {
  let primary = series[0].layout;
  match primary {
    cx::SeriesLayout::Funnel => {
      let series_count = series.len();
      for series in series {
        let series_color = appearance.point_color(series.source_index, series_count);
        lower_funnel(items, plot, series, series_color, appearance, chart_space);
        legend_entries.push((series_legend_name(series, ui_language), series_color));
      }
    }
    cx::SeriesLayout::Waterfall => {
      lower_waterfall_chart(
        items,
        plot,
        &series[0],
        chart_space,
        appearance,
        ui_language,
      );
      legend_entries.extend(waterfall_legend(ui_language, appearance));
    }
    cx::SeriesLayout::BoxWhisker => {
      lower_box_whisker_chart(items, plot, series, chart_space, appearance);
      legend_entries.extend(series.iter().map(|series| {
        (
          series_legend_name(series, ui_language),
          appearance.point_color(series.source_index, series.count()),
        )
      }));
    }
    cx::SeriesLayout::RegionMap => {
      lower_region_map(items, plot, &series[0], appearance, legend_entries);
    }
    cx::SeriesLayout::ClusteredColumn => {
      if series.iter().any(is_histogram_series) {
        lower_histogram_chart(items, plot, &series[0], chart_space, appearance);
      } else {
        lower_clustered_columns(items, plot, series, chart_space, appearance);
        legend_entries.extend(series.iter().map(|series| {
          (
            series_legend_name(series, ui_language),
            appearance.point_color(series.source_index, series.count()),
          )
        }));
      }
    }
    cx::SeriesLayout::ParetoLine | cx::SeriesLayout::Treemap | cx::SeriesLayout::Sunburst => {}
  }
}

#[derive(Clone, Copy, Debug)]
struct AxisScale {
  minimum: f64,
  maximum: f64,
  major: f64,
  divisor: f64,
}

const OFFICE_FIXED_CHART_CENTER_GRID_PT: f32 = 0.06;
const OFFICE_FIXED_CHART_EDGE_GRID_PT: f32 = 0.12;
const OFFICE_FIXED_CHART_EDGE_GRID_PHASE_PT: f32 = 0.06;
const AUTOMATIC_AXIS_TITLE_DISTANCE_RATIO: f32 = 0.02;

impl AxisScale {
  fn y(self, plot: PlotRect, value: f64) -> f32 {
    let span = (self.maximum - self.minimum).max(f64::EPSILON);
    let top = snap_office_chart_edge(plot.y);
    let bottom = snap_office_chart_edge(plot.y + plot.height);
    let plot_steps = ((bottom - top) / OFFICE_FIXED_CHART_EDGE_GRID_PT)
      .round()
      .max(1.0);
    let value_steps = (((value - self.minimum) / span) as f32 * plot_steps).round();
    bottom - value_steps * OFFICE_FIXED_CHART_EDGE_GRID_PT
  }
}

fn cartesian_plot(
  plot: PlotRect,
  has_value_title: bool,
  has_category_title: bool,
  has_out_end_labels: bool,
  scale: AxisScale,
  axis: Option<&cx::Axis>,
  appearance: &Appearance,
) -> PlotRect {
  cartesian_plot_with_top_inset(
    plot,
    CartesianPlotOptions {
      has_value_title,
      has_category_title,
      has_out_end_labels,
      scale,
      axis,
      appearance,
      top_inset: None,
      use_excel_minimum_gutter: true,
    },
  )
}

fn cartesian_plot_without_excel_label_floor(
  plot: PlotRect,
  has_value_title: bool,
  has_category_title: bool,
  has_out_end_labels: bool,
  scale: AxisScale,
  axis: Option<&cx::Axis>,
  appearance: &Appearance,
) -> PlotRect {
  cartesian_plot_with_top_inset(
    plot,
    CartesianPlotOptions {
      has_value_title,
      has_category_title,
      has_out_end_labels,
      scale,
      axis,
      appearance,
      top_inset: None,
      use_excel_minimum_gutter: false,
    },
  )
}

struct CartesianPlotOptions<'a> {
  has_value_title: bool,
  has_category_title: bool,
  has_out_end_labels: bool,
  scale: AxisScale,
  axis: Option<&'a cx::Axis>,
  appearance: &'a Appearance,
  top_inset: Option<f32>,
  use_excel_minimum_gutter: bool,
}

fn cartesian_plot_with_top_inset(plot: PlotRect, options: CartesianPlotOptions<'_>) -> PlotRect {
  let CartesianPlotOptions {
    has_value_title,
    has_category_title,
    has_out_end_labels,
    scale,
    axis,
    appearance,
    top_inset,
    use_excel_minimum_gutter,
  } = options;
  let style = &appearance.label_style;
  let mut widest_label = widest_value_axis_label(scale, axis, style);
  // Office keeps a stable value-axis gutter even when every visible tick is
  // a single digit. The shaped "0.0" template is the lower bound exposed by
  // paretoLine.xlsx; wider labels continue to grow the gutter normally.
  if appearance.host == ChartExHost::Excel && use_excel_minimum_gutter {
    widest_label = widest_label.max(TextMetrics::new().measure_text("0.0", style));
  }
  // Office's automatic ChartEx plot frame is expressed on its 0.05pt
  // fixed-layout grid. Keeping these insets on the same grid avoids moving
  // otherwise identical bar and gridline edges to the adjacent PDF pixel.
  let bottom = if has_category_title {
    38.05
  } else if appearance.host == ChartExHost::Excel {
    19.5
  } else if appearance.host == ChartExHost::PowerPoint && style.font_size_pt <= 9.5 {
    // The compact PowerPoint category-label band follows the 9pt style's
    // line box; the larger Office 2016 profile below is physically capped.
    style.font_size_pt * 1.64
  } else {
    17.95
  };
  let top = top_inset.unwrap_or_else(|| {
    if appearance.host == ChartExHost::Word && has_out_end_labels {
      3.0
    } else if appearance.host == ChartExHost::PowerPoint && has_out_end_labels {
      11.55
    } else if appearance.host == ChartExHost::Excel && has_out_end_labels {
      10.35
    } else if has_value_title || has_category_title {
      5.75
    } else if appearance.host == ChartExHost::Excel {
      12.2
    } else {
      8.5
    }
  });
  let right = if has_value_title || has_category_title {
    6.55
  } else {
    6.65
  };
  let data_plot_height = (plot.height - bottom - top).max(1.0);
  let wrapped_value_title_gutter = if has_value_title {
    excel_wrapped_value_axis_title_gutter(data_plot_height, axis, appearance)
  } else {
    0.0
  };
  let left =
    12.05 + widest_label + if has_value_title { 20.15 } else { 0.0 } + wrapped_value_title_gutter;
  PlotRect {
    x: plot.x + left,
    y: plot.y + top,
    width: (plot.width - left - right).max(1.0),
    height: data_plot_height,
  }
}

fn widest_value_axis_label(scale: AxisScale, axis: Option<&cx::Axis>, style: &TextStyle) -> f32 {
  let format = axis
    .and_then(|axis| axis.number_format.as_ref())
    .map(|format| format.format_code.as_str());
  let mut widest_label = 0.0_f32;
  let mut value = scale.minimum;
  let mut guard = 0;
  while value <= scale.maximum + scale.major * 0.001 && guard < 100 {
    widest_label = widest_label.max(text_width(
      &format_chart_number(value / scale.divisor, format),
      style,
    ));
    value += scale.major;
    guard += 1;
  }
  widest_label
}

fn excel_wrapped_value_axis_title_gutter(
  data_plot_height: f32,
  axis: Option<&cx::Axis>,
  appearance: &Appearance,
) -> f32 {
  if appearance.host != ChartExHost::Excel {
    return 0.0;
  }
  let Some(title) = axis_title(axis.and_then(|axis| axis.axis_title.as_deref()), None) else {
    return 0.0;
  };
  let style = &appearance.axis_title_style;
  let maximum_inline_width =
    (data_plot_height - style.font_size_pt * 4.0).max(style.font_size_pt * 2.0);
  let columns = wrap_chart_axis_title(&title, maximum_inline_width, style).len();
  // Each additional vertical line advances the automatic title frame by the
  // visible Calibri column width, while the first line is already covered by
  // the ordinary 20.15pt value-title gutter.  waterfall2.xlsx exposes the
  // resulting 8.36pt second-column reserve at 9pt.
  columns.saturating_sub(1) as f32 * style.font_size_pt * 0.929
}

fn has_out_end_labels(series: &SeriesModel<'_>) -> bool {
  series
    .source
    .data_labels
    .as_deref()
    .is_some_and(|labels| labels.pos == Some(cx::DataLabelPos::OutEnd))
}

fn snap_office_chart_center(value: f32) -> f32 {
  (value / OFFICE_FIXED_CHART_CENTER_GRID_PT).round() * OFFICE_FIXED_CHART_CENTER_GRID_PT
}

fn snap_office_chart_edge(value: f32) -> f32 {
  ((value - OFFICE_FIXED_CHART_EDGE_GRID_PHASE_PT) / OFFICE_FIXED_CHART_EDGE_GRID_PT).round()
    * OFFICE_FIXED_CHART_EDGE_GRID_PT
    + OFFICE_FIXED_CHART_EDGE_GRID_PHASE_PT
}

fn snap_office_chart_rect(rect: PlotRect) -> PlotRect {
  let left = snap_office_chart_edge(rect.x);
  let top = snap_office_chart_edge(rect.y);
  let right = snap_office_chart_edge(rect.x + rect.width);
  let bottom = snap_office_chart_edge(rect.y + rect.height);
  PlotRect {
    x: left,
    y: top,
    width: (right - left).max(OFFICE_FIXED_CHART_EDGE_GRID_PT),
    height: (bottom - top).max(OFFICE_FIXED_CHART_EDGE_GRID_PT),
  }
}

fn value_axis(chart_space: &cx::ChartSpace) -> Option<&cx::Axis> {
  chart_space
    .chart
    .plot_area
    .axis
    .iter()
    .find(|axis| matches!(axis.axis_choice, Some(cx::AxisChoice::ValueAxisScaling(_))))
}

fn category_axis(chart_space: &cx::ChartSpace) -> Option<&cx::Axis> {
  chart_space.chart.plot_area.axis.iter().find(|axis| {
    matches!(
      axis.axis_choice,
      Some(cx::AxisChoice::CategoryAxisScaling(_))
    )
  })
}

fn axis_title(title: Option<&cx::AxisTitle>, ui_language: Option<&str>) -> Option<String> {
  let title = title?;
  let explicit = title
    .text
    .as_deref()
    .map(cx_text)
    .filter(|value| !value.is_empty())
    .or_else(|| {
      title
        .tx_pr_text_body
        .as_deref()
        .map(|body| paragraph_text(&body.paragraph))
        .filter(|value| !value.is_empty())
    });
  explicit.or_else(|| Some(automatic_axis_title(ui_language).to_string()))
}

fn axis_title_offset_points(title: Option<&cx::AxisTitle>) -> (f32, f32) {
  let Some(offset) = title.and_then(|title| title.offset.as_ref()) else {
    return (0.0, 0.0);
  };
  // [MS-ODRAWXML] 2.24.3.66 defines these as signed inch offsets from the
  // automatic position, with positive left/top values moving right/down.
  let points = |inches: f64| {
    if inches.is_finite() {
      (inches * 72.0) as f32
    } else {
      0.0
    }
  };
  (points(offset.left), points(offset.top))
}

fn automatic_axis_title(ui_language: Option<&str>) -> &'static str {
  OfficeStringCatalog::for_ui_language(ui_language).chart_axis_title()
}

fn axis_scale(values: impl IntoIterator<Item = f64>, axis: Option<&cx::Axis>) -> AxisScale {
  axis_scale_with_profile(values, axis, 8.0, nice_number)
}

fn box_whisker_axis_scale(
  values: impl IntoIterator<Item = f64>,
  axis: Option<&cx::Axis>,
) -> AxisScale {
  // LibreOffice ScaleAutomatism starts a linear axis with at most ten main
  // increments and snaps the raw distance to the 1/2/5 sequence. Excel's
  // ChartEx box-and-whisker output exposes both rules: 1..9 resolves to
  // 0..10 by ones, while the wide -78..128 family resolves by fifties.
  axis_scale_with_profile(values, axis, 10.0, nice_number_125)
}

fn axis_scale_with_profile(
  values: impl IntoIterator<Item = f64>,
  axis: Option<&cx::Axis>,
  automatic_interval_count: f64,
  automatic_increment: fn(f64) -> f64,
) -> AxisScale {
  let mut minimum = f64::INFINITY;
  let mut maximum = f64::NEG_INFINITY;
  for value in values.into_iter().filter(|value| value.is_finite()) {
    minimum = minimum.min(value);
    maximum = maximum.max(value);
  }
  if !minimum.is_finite() || !maximum.is_finite() {
    minimum = 0.0;
    maximum = 1.0;
  }
  if minimum >= 0.0 {
    minimum = 0.0;
  }
  if maximum <= 0.0 {
    maximum = 0.0;
  }
  if minimum == maximum {
    let delta = minimum.abs().max(1.0);
    minimum -= delta;
    maximum += delta;
  }

  let scaling = axis.and_then(|axis| match axis.axis_choice.as_ref() {
    Some(cx::AxisChoice::ValueAxisScaling(scaling)) => Some(scaling.as_ref()),
    _ => None,
  });
  let explicit_minimum = scaling
    .and_then(|scaling| scaling.min.as_deref())
    .and_then(parse_axis_number);
  let explicit_maximum = scaling
    .and_then(|scaling| scaling.max.as_deref())
    .and_then(parse_axis_number);
  let explicit_major = scaling
    .and_then(|scaling| scaling.major_unit.as_deref())
    .and_then(parse_axis_number)
    .filter(|value| *value > 0.0);

  let span = maximum - minimum;
  let padded_minimum = if explicit_minimum.is_some() || minimum == 0.0 {
    minimum
  } else {
    minimum - span * 0.10
  };
  let padded_maximum = if explicit_maximum.is_some() {
    maximum
  } else {
    maximum + span * 0.10
  };
  let major = explicit_major.unwrap_or_else(|| {
    automatic_increment((padded_maximum - padded_minimum) / automatic_interval_count)
  });
  minimum = explicit_minimum.unwrap_or_else(|| (padded_minimum / major).floor() * major);
  maximum = explicit_maximum.unwrap_or_else(|| (padded_maximum / major).ceil() * major);
  if maximum <= minimum {
    maximum = minimum + major;
  }
  AxisScale {
    minimum,
    maximum,
    major,
    divisor: axis
      .and_then(|axis| axis.axis_units.as_deref())
      .and_then(|units| units.unit)
      .map(axis_unit_divisor)
      .unwrap_or(1.0),
  }
}

fn parse_axis_number(value: &str) -> Option<f64> {
  (!value.eq_ignore_ascii_case("auto"))
    .then(|| value.parse::<f64>().ok())
    .flatten()
}

fn nice_number(value: f64) -> f64 {
  if !value.is_finite() || value <= 0.0 {
    return 1.0;
  }
  let magnitude = 10_f64.powf(value.log10().floor());
  let normalized = value / magnitude;
  let step = if normalized <= 1.0 {
    1.0
  } else if normalized <= 2.0 {
    2.0
  } else if normalized <= 2.5 {
    // ScaleAutomatism treats the requested interval count as a soft target;
    // retaining the 2× rhythm is preferable to introducing a 2.5× rhythm.
    2.0
  } else if normalized <= 5.0 {
    5.0
  } else {
    10.0
  };
  step * magnitude
}

fn nice_number_125(value: f64) -> f64 {
  if !value.is_finite() || value <= 0.0 {
    return 1.0;
  }
  let magnitude = 10_f64.powf(value.log10().floor());
  let normalized = value / magnitude;
  let step = if normalized <= 1.0 {
    1.0
  } else if normalized <= 2.0 {
    2.0
  } else if normalized <= 5.0 {
    5.0
  } else {
    10.0
  };
  step * magnitude
}

fn axis_unit_divisor(unit: cx::AxisUnit) -> f64 {
  match unit {
    cx::AxisUnit::Hundreds => 100.0,
    cx::AxisUnit::Thousands => 1_000.0,
    cx::AxisUnit::TenThousands => 10_000.0,
    cx::AxisUnit::HundredThousands => 100_000.0,
    cx::AxisUnit::Millions => 1_000_000.0,
    cx::AxisUnit::TenMillions => 10_000_000.0,
    cx::AxisUnit::HundredMillions => 100_000_000.0,
    cx::AxisUnit::Billions => 1_000_000_000.0,
    cx::AxisUnit::Trillions => 1_000_000_000_000.0,
    cx::AxisUnit::Percentage => 0.01,
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AxisPaintPass {
  BackgroundGrid,
  Foreground,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum CategoryAxisCrossing {
  #[default]
  Zero,
  Minimum,
}

#[derive(Clone, Copy, Debug)]
struct AxisPaintOptions<'a> {
  ui_language: Option<&'a str>,
  pass: AxisPaintPass,
  category_crossing: CategoryAxisCrossing,
}

fn automatic_value_axis_title_center(
  data_plot: PlotRect,
  scale: AxisScale,
  axis: Option<&cx::Axis>,
  appearance: &Appearance,
  title: &str,
  style: &TextStyle,
) -> (f32, f32) {
  let show_ticks = axis.is_none_or(|axis| axis.tick_labels.is_some());
  let outer_axis_left = if show_ticks {
    data_plot.x - 6.25 - widest_value_axis_label(scale, axis, &appearance.label_style)
  } else {
    data_plot.x
  };
  let title_cross_extent = TextMetrics::new()
    .vertical_metrics_for_text(title, style)
    .ink_height_pt();
  // LibreOffice ChartView positions an automatic left-axis title outside the
  // complete diagram-plus-axes rectangle by 2% of the chart width. Office's
  // 216pt and 432pt Word chart references expose the same proportional gap.
  (
    outer_axis_left
      - appearance.chart_width_pt * AUTOMATIC_AXIS_TITLE_DISTANCE_RATIO
      - title_cross_extent * 0.5,
    data_plot.y + data_plot.height * 0.5,
  )
}

fn lower_axes(
  items: &mut Vec<PageItem>,
  data_plot: PlotRect,
  scale: AxisScale,
  categories: &[String],
  chart_space: &cx::ChartSpace,
  appearance: &Appearance,
  options: AxisPaintOptions<'_>,
) {
  let AxisPaintOptions {
    ui_language,
    pass,
    category_crossing,
  } = options;
  let value_axis = value_axis(chart_space);
  let category_axis = category_axis(chart_space);
  let value_axis_title = value_axis.and_then(|axis| axis.axis_title.as_deref());
  let category_axis_title = category_axis.and_then(|axis| axis.axis_title.as_deref());
  let value_hidden = value_axis
    .and_then(|axis| axis.hidden)
    .is_some_and(|hidden| hidden.as_bool());
  let category_hidden = category_axis
    .and_then(|axis| axis.hidden)
    .is_some_and(|hidden| hidden.as_bool());

  if pass == AxisPaintPass::BackgroundGrid {
    let show_grid =
      !value_hidden && value_axis.is_some_and(|axis| axis.major_gridlines_gridlines.is_some());
    if show_grid {
      let mut value = scale.minimum;
      if category_crossing == CategoryAxisCrossing::Minimum {
        // The category-axis stroke owns the minimum edge for statistical
        // charts. Painting a gridline there would double its weight.
        value += scale.major;
      }
      let mut guard = 0;
      while value <= scale.maximum + scale.major * 0.001 && guard < 100 {
        let y = scale.y(data_plot, value);
        push_line(
          items,
          data_plot.x,
          y,
          data_plot.x + data_plot.width,
          y,
          appearance.grid_color,
          appearance.grid_width,
        );
        value += scale.major;
        guard += 1;
      }
    }
    return;
  }

  let category_y = match category_crossing {
    CategoryAxisCrossing::Zero => scale.y(data_plot, 0.0_f64.clamp(scale.minimum, scale.maximum)),
    CategoryAxisCrossing::Minimum => scale.y(data_plot, scale.minimum),
  };

  if !value_hidden {
    push_line(
      items,
      data_plot.x,
      data_plot.y,
      data_plot.x,
      data_plot.y + data_plot.height,
      appearance.axis_color,
      appearance.axis_width,
    );
    let show_ticks = value_axis.is_none_or(|axis| axis.tick_labels.is_some());
    let format = value_axis
      .and_then(|axis| axis.number_format.as_ref())
      .map(|format| format.format_code.as_str());
    let mut value = scale.minimum;
    let mut guard = 0;
    while value <= scale.maximum + scale.major * 0.001 && guard < 100 {
      let y = scale.y(data_plot, value);
      if show_ticks {
        let label = format_chart_number(value / scale.divisor, format);
        // Excel's ChartEx value-label baseline is one fixed-output cell above
        // the generic host position. The compact two-axis-title profile uses
        // three cells after its plot-height quantization. Besides matching
        // the painted glyphs, this keeps chart ticks and unrelated worksheet
        // cells on the same distinct PDF text lines as Office.
        let titled_axis_profile = value_axis.is_some_and(|axis| axis.axis_title.is_some())
          && category_axis.is_some_and(|axis| axis.axis_title.is_some());
        let host_baseline_offset = if appearance.host == ChartExHost::Excel {
          -OFFICE_FIXED_CHART_EDGE_GRID_PT * if titled_axis_profile { 3.0 } else { 1.0 }
        } else {
          0.0
        };
        push_right_aligned_text(
          items,
          data_plot.x - 6.25,
          y - appearance.label_style.font_size_pt * 0.55 + host_baseline_offset,
          label,
          appearance.label_style.clone(),
        );
      }
      value += scale.major;
      guard += 1;
    }
    if let Some(title) = axis_title(value_axis_title, ui_language) {
      let mut style = appearance.axis_title_style.clone();
      style.rotation_deg = -90.0;
      let (offset_x, offset_y) = axis_title_offset_points(value_axis_title);
      let maximum_inline_width =
        (data_plot.height - style.font_size_pt * 4.0).max(style.font_size_pt * 2.0);
      let wrapped = wrap_chart_axis_title(&title, maximum_inline_width, &style);
      if appearance.host == ChartExHost::Excel && wrapped.len() > 1 {
        // DrawingML `bodyPr/@wrap="square"` wraps against the automatic
        // title box. Excel centers each resulting vertical line on the plot
        // and advances columns by the resolved font line height.
        let wrapped_gutter =
          excel_wrapped_value_axis_title_gutter(data_plot.height, value_axis, appearance);
        let block_center_x = data_plot.x
          - (12.05 + appearance.axis_title_style.font_size_pt * 1.62 + wrapped_gutter)
          + offset_x;
        // LibreOffice ChartView centers vertical titles on the plot area
        // excluding axes and data tables; Office fixed output follows the
        // same geometry, then snaps the rotated frame upward on its output
        // grid. push_centered_rotated_text owns the remaining glyph metrics.
        let center_y =
          data_plot.y + data_plot.height * 0.5 - OFFICE_FIXED_CHART_EDGE_GRID_PT * 2.5 + offset_y;
        let line_advance = appearance.axis_title_style.font_size_pt * 1.227;
        let line_center = (wrapped.len().saturating_sub(1)) as f32 * 0.5;
        let mut metrics = TextMetrics::new();
        for (index, line) in wrapped.into_iter().enumerate() {
          let width = metrics.measure_text(&line, &style);
          push_centered_rotated_text(
            items,
            (
              block_center_x + (index as f32 - line_center) * line_advance,
              center_y,
            ),
            width,
            line,
            style.clone(),
          );
        }
      } else {
        let (center_x, center_y) = automatic_value_axis_title_center(
          data_plot, scale, value_axis, appearance, &title, &style,
        );
        let width = TextMetrics::new().measure_text(&title, &style);
        push_centered_rotated_text(
          items,
          (center_x + offset_x, center_y + offset_y),
          width,
          title,
          style,
        );
      }
    }
  }

  if !category_hidden {
    push_line(
      items,
      data_plot.x,
      category_y,
      data_plot.x + data_plot.width,
      category_y,
      appearance.axis_color,
      appearance.axis_width,
    );
    if category_axis.is_none_or(|axis| axis.tick_labels.is_some()) && !categories.is_empty() {
      let slot = data_plot.width / categories.len() as f32;
      for (index, category) in categories.iter().enumerate() {
        push_centered_text(
          items,
          PlotRect {
            x: data_plot.x + slot * index as f32,
            y: data_plot.y + data_plot.height + 6.5,
            width: slot,
            height: appearance.label_style.font_size_pt * 1.4,
          },
          category.clone(),
          appearance.label_style.clone(),
        );
      }
    }
    if let Some(title) = axis_title(category_axis_title, ui_language) {
      let (offset_x, offset_y) = axis_title_offset_points(category_axis_title);
      push_centered_text(
        items,
        PlotRect {
          x: data_plot.x + offset_x,
          // The title follows the category-label line rather than sharing
          // its slot. Office reserves a 1.25-em label line and a 0.65-em
          // title gap after the fixed 6.5pt tick-label inset.
          y: data_plot.y
            + data_plot.height
            + 6.5
            + appearance.label_style.font_size_pt * 1.25
            + appearance.axis_title_style.font_size_pt * 0.65
            + offset_y,
          width: data_plot.width,
          height: appearance.axis_title_style.font_size_pt * 1.4,
        },
        title,
        appearance.axis_title_style.clone(),
      );
    }
  }
}

#[derive(Clone, Copy, Debug)]
struct WaterfallBar {
  start: f64,
  end: f64,
  value: f64,
  subtotal: bool,
}

fn waterfall_bars(series: &SeriesModel<'_>) -> Vec<Option<WaterfallBar>> {
  let subtotals = series
    .source
    .series_layout_properties
    .as_deref()
    .and_then(|properties| properties.subtotals.as_ref())
    .map(|subtotals| {
      subtotals
        .unsigned_integer_type
        .iter()
        .map(|index| index.val as usize)
        .collect::<HashSet<_>>()
    })
    .unwrap_or_default();
  let mut cumulative = 0.0;
  series
    .values
    .iter()
    .enumerate()
    .map(|(index, value)| {
      let value = (*value)?;
      if subtotals.contains(&index) {
        cumulative = value;
        Some(WaterfallBar {
          start: 0.0,
          end: value,
          value,
          subtotal: true,
        })
      } else {
        let start = cumulative;
        cumulative += value;
        Some(WaterfallBar {
          start,
          end: cumulative,
          value,
          subtotal: false,
        })
      }
    })
    .collect()
}

fn lower_waterfall_chart(
  items: &mut Vec<PageItem>,
  plot: PlotRect,
  series: &SeriesModel<'_>,
  chart_space: &cx::ChartSpace,
  appearance: &Appearance,
  ui_language: Option<&str>,
) {
  let bars = waterfall_bars(series);
  let categories = (0..series.count())
    .map(|index| series.leaf_category(index))
    .collect::<Vec<_>>();
  let value_title = value_axis(chart_space).is_some_and(|axis| axis.axis_title.is_some());
  let category_title = category_axis(chart_space).is_some_and(|axis| axis.axis_title.is_some());
  let extents = bars
    .iter()
    .flatten()
    .flat_map(|bar| [bar.start, bar.end])
    .collect::<Vec<_>>();
  let mut scale = axis_scale(extents.iter().copied(), value_axis(chart_space));
  let explicit_scaling = value_axis(chart_space).and_then(|axis| match axis.axis_choice.as_ref() {
    Some(cx::AxisChoice::ValueAxisScaling(scaling)) => Some(scaling.as_ref()),
    _ => None,
  });
  if explicit_scaling.is_none_or(|scaling| scaling.min.is_none() && scaling.max.is_none()) {
    let source_minimum = extents.iter().copied().fold(0.0_f64, f64::min);
    let source_maximum = extents.iter().copied().fold(0.0_f64, f64::max);
    let major = explicit_scaling
      .and_then(|scaling| scaling.major_unit.as_deref())
      .and_then(parse_axis_number)
      // Office targets ten intervals for a waterfall value axis. This keeps
      // the compact 0..45 German counterexample on 5-unit ticks while the
      // 0..170 Of16 deck remains on 20-unit ticks.
      .unwrap_or_else(|| nice_number((source_maximum - source_minimum) / 10.0));
    scale.minimum = (source_minimum / major).floor() * major;
    scale.maximum = (source_maximum / major).ceil() * major;
    scale.major = major;
    if series
      .source
      .data_labels
      .as_deref()
      .is_some_and(|labels| labels.pos == Some(cx::DataLabelPos::OutEnd))
      && (scale.maximum - source_maximum).abs() < major * 0.05
    {
      scale.maximum += major;
    }
  }
  let data_plot = cartesian_plot_without_excel_label_floor(
    plot,
    value_title,
    category_title,
    has_out_end_labels(series),
    scale,
    value_axis(chart_space),
    appearance,
  );
  lower_axes(
    items,
    data_plot,
    scale,
    &categories,
    chart_space,
    appearance,
    AxisPaintOptions {
      ui_language,
      pass: AxisPaintPass::BackgroundGrid,
      category_crossing: CategoryAxisCrossing::Zero,
    },
  );
  let count = bars.len().max(1);
  let gap = category_axis(chart_space)
    .and_then(|axis| match axis.axis_choice.as_ref() {
      Some(cx::AxisChoice::CategoryAxisScaling(scaling)) => scaling.gap_width.as_deref(),
      _ => None,
    })
    .and_then(|value| value.parse::<f32>().ok())
    .unwrap_or(0.5)
    .clamp(0.0, 5.0);
  let slot = data_plot.width / count as f32;
  let width = slot / (1.0 + gap).max(1.05);
  // Office fixes the category band centers independently from the stroked
  // axis edge. Across the complete band this is a 0.02pt leading offset at
  // the stock DOCX chart size; retaining it keeps both the first and last
  // point centered after fixed-output coordinate quantization.
  let point_band_x = data_plot.x
    + if appearance.host == ChartExHost::PowerPoint {
      0.12
    } else {
      -0.02
    };
  let point_bounds = |index: usize| {
    let center = snap_office_chart_center(point_band_x + slot * (index as f32 + 0.5));
    let left = snap_office_chart_edge(center - width * 0.5);
    let right = snap_office_chart_edge(center + width * 0.5);
    (left, (right - left).max(0.12))
  };
  let effect_clip = {
    // Office retains a small guard around the plot when clipping rasterized
    // point effects. The leading value-axis edge gets one additional
    // fixed-output cell so the centered axis stroke does not trim the blur.
    const EFFECT_GUARD_PT: f32 = 1.26;
    let left =
      snap_office_chart_edge(data_plot.x) - EFFECT_GUARD_PT - OFFICE_FIXED_CHART_EDGE_GRID_PT;
    let top = snap_office_chart_edge(data_plot.y) - EFFECT_GUARD_PT;
    let right = snap_office_chart_edge(data_plot.x + data_plot.width) + EFFECT_GUARD_PT;
    let bottom = snap_office_chart_edge(data_plot.y + data_plot.height) + EFFECT_GUARD_PT;
    PlotRect {
      x: left,
      y: top,
      width: right - left,
      height: bottom - top,
    }
  };
  let connectors = series
    .source
    .series_layout_properties
    .as_deref()
    .and_then(|properties| properties.series_element_visibilities.as_ref())
    .and_then(|visibility| visibility.connector_lines)
    .is_none_or(|value| value.as_bool());
  for (index, bar) in bars.iter().enumerate() {
    let Some(bar) = bar else {
      continue;
    };
    let (x, point_width) = point_bounds(index);
    let y1 = scale.y(data_plot, bar.start);
    let y2 = scale.y(data_plot, bar.end);
    let role = if bar.subtotal {
      WaterfallColorRole::Total
    } else if bar.value < 0.0 {
      WaterfallColorRole::Decrease
    } else {
      WaterfallColorRole::Increase
    };
    let color = waterfall_role_color(appearance, role);
    let color = series_color_override(series, index, appearance, color);
    push_data_point_rect(
      items,
      PlotRect {
        x,
        y: y1.min(y2),
        width: point_width,
        height: (y2 - y1).abs().max(0.75),
      },
      color,
      appearance,
      Some(effect_clip),
      // The shared point helper resolves an authored ChartStyle `dataPoint`
      // outline. Of16-05 carries only `gradFill`/`effectLst` and remains
      // fill-only, while style 395 uses a same-color 0.75pt outline.
      None,
    );
    if connectors && index + 1 < bars.len() && bars[index + 1].is_some() {
      let y = scale.y(data_plot, bar.end);
      let (next_x, _) = point_bounds(index + 1);
      push_line(
        items,
        x + point_width,
        y,
        next_x,
        y,
        appearance.series_line_color,
        appearance.series_line_width,
      );
    }
    if let Some(label) = data_label_text(series, index, bar.value) {
      let position = data_label_position(series, index);
      let mut label_y = match position {
        cx::DataLabelPos::InEnd => {
          if bar.end >= bar.start {
            y1.min(y2) + 2.0
          } else {
            y1.max(y2) - appearance.data_label_style.font_size_pt - 2.0
          }
        }
        cx::DataLabelPos::InBase => {
          if bar.end >= bar.start {
            y1.max(y2) - appearance.data_label_style.font_size_pt - 2.0
          } else {
            y1.min(y2) + 2.0
          }
        }
        _ => {
          if bar.end >= bar.start {
            y1.min(y2) - appearance.data_label_style.font_size_pt * 1.15
          } else {
            y1.max(y2) + 1.0
          }
        }
      };
      if appearance.host == ChartExHost::Excel {
        // Excel places waterfall labels on the same fixed-output lattice as
        // its worksheet text. A chart with both automatic axis-title bands
        // uses three additional grid cells of lift after those bands compact
        // the data plot; the untitled profile remains unchanged.
        let optical_lift_cells = if value_title && category_title {
          5.5
        } else {
          2.5
        };
        label_y -= OFFICE_FIXED_CHART_EDGE_GRID_PT * optical_lift_cells;
        if position == cx::DataLabelPos::OutEnd {
          // An out-end label below a negative bar is pulled back into the
          // plot when its line box would collide with the category labels.
          // Office leaves a 0.35pt optical overshoot below the axis edge.
          label_y = label_y
            .min(data_plot.y + data_plot.height - appearance.data_label_style.font_size_pt - 0.35);
        }
      }
      push_centered_text(
        items,
        PlotRect {
          x: data_plot.x + slot * index as f32,
          y: label_y,
          width: slot,
          height: appearance.data_label_style.font_size_pt * 1.2,
        },
        label,
        appearance.data_label_style.clone(),
      );
    }
  }
  lower_axes(
    items,
    data_plot,
    scale,
    &categories,
    chart_space,
    appearance,
    AxisPaintOptions {
      ui_language,
      pass: AxisPaintPass::Foreground,
      category_crossing: CategoryAxisCrossing::Zero,
    },
  );
}

fn wrap_chart_axis_title(text: &str, maximum_width: f32, style: &TextStyle) -> Vec<String> {
  let mut metrics = TextMetrics::new();
  if metrics.measure_text(text, style) <= maximum_width {
    return vec![text.to_string()];
  }
  let mut lines = Vec::new();
  let mut current = String::new();
  for word in text.split_whitespace() {
    let candidate = if current.is_empty() {
      word.to_string()
    } else {
      format!("{current} {word}")
    };
    if !current.is_empty() && metrics.measure_text(&candidate, style) > maximum_width {
      lines.push(std::mem::take(&mut current));
      current.push_str(word);
    } else {
      current = candidate;
    }
  }
  if !current.is_empty() {
    lines.push(current);
  }
  if lines.is_empty() {
    vec![text.to_string()]
  } else {
    lines
  }
}

#[derive(Clone, Copy, Debug)]
enum WaterfallColorRole {
  Increase,
  Decrease,
  Total,
}

impl WaterfallColorRole {
  fn index(self) -> usize {
    match self {
      Self::Increase => 0,
      Self::Decrease => 1,
      Self::Total => 2,
    }
  }

  fn fallback(self) -> RgbColor {
    match self {
      Self::Increase => WATERFALL_INCREASE,
      Self::Decrease => WATERFALL_DECREASE,
      Self::Total => WATERFALL_TOTAL,
    }
  }
}

fn waterfall_role_color(appearance: &Appearance, role: WaterfallColorRole) -> RgbColor {
  if appearance.color_method.is_some() {
    appearance.point_color(role.index(), WATERFALL_COLOR_FORMAT_COUNT)
  } else {
    role.fallback()
  }
}

fn waterfall_legend(ui_language: Option<&str>, appearance: &Appearance) -> Vec<(String, RgbColor)> {
  let names = OfficeStringCatalog::for_ui_language(ui_language).waterfall_legend();
  vec![
    (
      names[0].to_string(),
      waterfall_role_color(appearance, WaterfallColorRole::Increase),
    ),
    (
      names[1].to_string(),
      waterfall_role_color(appearance, WaterfallColorRole::Decrease),
    ),
    (
      names[2].to_string(),
      waterfall_role_color(appearance, WaterfallColorRole::Total),
    ),
  ]
}

fn series_color_override(
  series: &SeriesModel<'_>,
  index: usize,
  appearance: &Appearance,
  fallback: RgbColor,
) -> RgbColor {
  series
    .source
    .data_point
    .iter()
    .find(|point| point.idx as usize == index)
    .and_then(|point| point.shape_properties.as_deref())
    .and_then(|shape| resolve_cx_shape_fill(shape, appearance.theme, Some(fallback)))
    .or_else(|| {
      series
        .source
        .shape_properties
        .as_deref()
        .and_then(|shape| resolve_cx_shape_fill(shape, appearance.theme, Some(fallback)))
    })
    .unwrap_or(fallback)
}

fn series_stroke_override(
  series: &SeriesModel<'_>,
  index: usize,
  appearance: &Appearance,
  placeholder: RgbColor,
) -> Option<(RgbColor, f32)> {
  let point_shape = series
    .source
    .data_point
    .iter()
    .find(|point| point.idx as usize == index)
    .and_then(|point| point.shape_properties.as_deref());
  for shape in [point_shape, series.source.shape_properties.as_deref()]
    .into_iter()
    .flatten()
  {
    if shape.outline.is_some() {
      return resolve_cx_outline(shape, appearance.theme, Some(placeholder)).map(|color| {
        let width = shape
          .outline
          .as_deref()
          .and_then(|outline| outline.width)
          .map(|width| crate::units::emu_to_points(i64::from(width)))
          // A direct ChartEx data-point line without `w` uses Office's
          // 2.25pt chart-series default. This is distinct from the 0.75pt
          // default used by ordinary DrawingML shape outlines.
          .unwrap_or(2.25);
        (color, width)
      });
    }
  }
  appearance.data_point_stroke(placeholder)
}

#[derive(Clone, Copy, Debug, Default)]
struct LabelVisibility {
  series: bool,
  category: bool,
  value: bool,
}

fn visibility(
  source: Option<&cx::DataLabelVisibilities>,
  inherited: LabelVisibility,
) -> LabelVisibility {
  let Some(source) = source else {
    return inherited;
  };
  LabelVisibility {
    series: source
      .series_name
      .map(|value| value.as_bool())
      .unwrap_or(inherited.series),
    category: source
      .category_name
      .map(|value| value.as_bool())
      .unwrap_or(inherited.category),
    value: source
      .value
      .map(|value| value.as_bool())
      .unwrap_or(inherited.value),
  }
}

fn data_label_text(series: &SeriesModel<'_>, index: usize, value: f64) -> Option<String> {
  let labels = series.source.data_labels.as_deref()?;
  if labels
    .data_label_hidden
    .iter()
    .any(|hidden| hidden.idx as usize == index)
  {
    return None;
  }
  let inherited = visibility(
    labels.data_label_visibilities.as_ref(),
    LabelVisibility::default(),
  );
  let override_label = labels
    .data_label
    .iter()
    .find(|label| label.idx as usize == index);
  let visibility = visibility(
    override_label.and_then(|label| label.data_label_visibilities.as_ref()),
    inherited,
  );
  let separator = override_label
    .and_then(|label| label.separator_xsdstring.as_deref())
    .or(labels.separator_xsdstring.as_deref())
    .unwrap_or(", ");
  let format = override_label
    .and_then(|label| label.number_format.as_ref())
    .or(labels.number_format.as_ref())
    .map(|format| format.format_code.as_str())
    .or(series.number_format.as_deref());
  let mut parts = Vec::new();
  if visibility.series {
    parts.push(series.name.clone());
  }
  if visibility.category {
    parts.push(series.leaf_category(index));
  }
  if visibility.value {
    parts.push(format_chart_number(value, format));
  }
  (!parts.is_empty()).then(|| parts.join(separator))
}

fn data_label_position(series: &SeriesModel<'_>, index: usize) -> cx::DataLabelPos {
  let Some(labels) = series.source.data_labels.as_deref() else {
    return cx::DataLabelPos::BestFit;
  };
  labels
    .data_label
    .iter()
    .find(|label| label.idx as usize == index)
    .and_then(|label| label.pos)
    .or(labels.pos)
    .unwrap_or_default()
}

fn data_label_has_explicit_text_properties(series: &SeriesModel<'_>, index: usize) -> bool {
  let Some(labels) = series.source.data_labels.as_deref() else {
    return false;
  };
  labels.tx_pr_text_body.is_some()
    || labels
      .data_label
      .iter()
      .find(|label| label.idx as usize == index)
      .is_some_and(|label| label.tx_pr_text_body.is_some())
}

fn automatic_inside_data_label_color(fill: RgbColor) -> Option<RgbColor> {
  // Office's in-shape chart labels choose the light neutral for fills whose
  // perceived sRGB brightness is below the middle neutral. The one-code-point
  // light neutral matches Office fixed output's 254/255 white paint.
  let brightness =
    (u32::from(fill.r) * 299 + u32::from(fill.g) * 587 + u32::from(fill.b) * 114) / 1000;
  (brightness < 128).then(|| rgb(254, 254, 254))
}

fn lower_funnel(
  items: &mut Vec<PageItem>,
  plot: PlotRect,
  series: &SeriesModel<'_>,
  base_color: RgbColor,
  appearance: &Appearance,
  chart_space: &cx::ChartSpace,
) {
  let values = series
    .values
    .iter()
    .map(|value| value.unwrap_or(0.0).max(0.0))
    .collect::<Vec<_>>();
  let maximum = values.iter().copied().fold(0.0_f64, f64::max).max(1.0);
  let count = values.len().max(1);
  let axis = category_axis(chart_space);
  let axis_hidden = axis
    .and_then(|axis| axis.hidden)
    .is_some_and(|hidden| hidden.as_bool());
  let show_tick_labels = !axis_hidden && axis.is_some_and(|axis| axis.tick_labels.is_some());
  let format = axis
    .and_then(|axis| axis.number_format.as_ref())
    .map(|format| format.format_code.as_str());
  let has_explicit_categories = !series.categories.is_empty();
  let categories = (0..count)
    .map(|index| {
      if has_explicit_categories {
        series.leaf_category(index)
      } else {
        format_chart_number(index as f64 + 1.0, format)
      }
    })
    .collect::<Vec<_>>();
  let widest_category = if show_tick_labels {
    {
      categories
        .iter()
        .map(|category| text_width(category, &appearance.label_style))
        .fold(0.0_f32, f32::max)
    }
  } else {
    0.0
  };
  let left = if show_tick_labels {
    widest_category + 12.05
  } else {
    6.65
  };
  let right = 6.65;
  let top = 0.0;
  let bottom = 0.0;
  let data_plot = PlotRect {
    x: plot.x + left,
    y: plot.y + top,
    width: (plot.width - left - right).max(1.0),
    height: (plot.height - top - bottom).max(1.0),
  };
  let gap_width = axis
    .and_then(|axis| match axis.axis_choice.as_ref() {
      Some(cx::AxisChoice::CategoryAxisScaling(scaling)) => scaling
        .gap_width
        .as_deref()
        .and_then(parse_axis_number)
        .map(|value| value.max(0.0) as f32),
      _ => None,
    })
    // Excel's automatic category gap is 150% when ChartEx omits a value.
    .unwrap_or(1.5);
  let category_slot = data_plot.height / count as f32;
  // [MS-ODRAWXML] defines gapWidth as gap width / category width. The
  // category slot therefore consists of one bar plus that ratio of gap.
  let bar_height = category_slot / (1.0 + gap_width);
  for (index, value) in values.iter().copied().enumerate() {
    let width = data_plot.width * (value / maximum) as f32;
    let x = data_plot.x + (data_plot.width - width) * 0.5;
    let y = data_plot.y + category_slot * index as f32 + (category_slot - bar_height) * 0.5;
    let color = series_color_override(series, index, appearance, base_color);
    push_data_point_rect(
      items,
      PlotRect {
        x,
        y,
        width,
        height: bar_height,
      },
      color,
      appearance,
      None,
      series_stroke_override(series, index, appearance, color),
    );
    if let Some(label) = data_label_text(series, index, value) {
      let mut label_style = appearance.data_label_style.clone();
      if !data_label_has_explicit_text_properties(series, index)
        && matches!(
          data_label_position(series, index),
          cx::DataLabelPos::BestFit
            | cx::DataLabelPos::Ctr
            | cx::DataLabelPos::InBase
            | cx::DataLabelPos::InEnd
        )
        && let Some(color) = automatic_inside_data_label_color(color)
      {
        label_style.color = color;
        label_style.color_is_automatic = false;
      }
      push_centered_text(
        items,
        PlotRect {
          x,
          y,
          width,
          height: bar_height,
        },
        label,
        label_style,
      );
    }
  }

  if !axis_hidden && axis.is_some() {
    push_line(
      items,
      data_plot.x,
      data_plot.y,
      data_plot.x,
      data_plot.y + data_plot.height,
      appearance.axis_color,
      appearance.axis_width,
    );
    if let Some(tick_marks) = axis.and_then(|axis| axis.major_tick_marks_tick_marks.as_deref()) {
      let tick_type = tick_marks.r#type.unwrap_or_default();
      if tick_type != cx::TickMarksType::None {
        let (outside, inside) = match tick_type {
          cx::TickMarksType::In => (0.0, 2.88),
          cx::TickMarksType::Out => (2.88, 0.0),
          cx::TickMarksType::Cross => (2.88, 2.88),
          cx::TickMarksType::None => (0.0, 0.0),
        };
        for boundary in 0..=count {
          let y = data_plot.y + category_slot * boundary as f32;
          push_line(
            items,
            data_plot.x - outside,
            y,
            data_plot.x + inside,
            y,
            appearance.axis_color,
            appearance.axis_width,
          );
        }
      }
    }
    if show_tick_labels {
      for (index, category) in categories.into_iter().enumerate() {
        push_right_aligned_text(
          items,
          data_plot.x - 6.5,
          data_plot.y + category_slot * (index as f32 + 0.5)
            - appearance.label_style.font_size_pt * 0.60,
          category,
          appearance.label_style.clone(),
        );
      }
    }
  }
}

fn lower_clustered_columns(
  items: &mut Vec<PageItem>,
  plot: PlotRect,
  series: &[SeriesModel<'_>],
  chart_space: &cx::ChartSpace,
  appearance: &Appearance,
) {
  let count = series.iter().map(SeriesModel::count).max().unwrap_or(0);
  if count == 0 {
    return;
  }
  let categories = (0..count)
    .map(|index| {
      series
        .iter()
        .find(|series| index < series.count())
        .map(|series| series.leaf_category(index))
        .unwrap_or_else(|| (index + 1).to_string())
    })
    .collect::<Vec<_>>();
  let value_title = value_axis(chart_space).is_some_and(|axis| axis.axis_title.is_some());
  let category_title = category_axis(chart_space).is_some_and(|axis| axis.axis_title.is_some());
  let scale = axis_scale(
    series
      .iter()
      .flat_map(|series| series.values.iter().copied().flatten()),
    value_axis(chart_space),
  );
  let data_plot = cartesian_plot(
    plot,
    value_title,
    category_title,
    series.iter().any(has_out_end_labels),
    scale,
    value_axis(chart_space),
    appearance,
  );
  lower_axes(
    items,
    data_plot,
    scale,
    &categories,
    chart_space,
    appearance,
    AxisPaintOptions {
      ui_language: None,
      pass: AxisPaintPass::BackgroundGrid,
      category_crossing: CategoryAxisCrossing::Zero,
    },
  );
  let slot = data_plot.width / count as f32;
  let gap = category_axis(chart_space)
    .and_then(|axis| match axis.axis_choice.as_ref() {
      Some(cx::AxisChoice::CategoryAxisScaling(scaling)) => scaling.gap_width.as_deref(),
      _ => None,
    })
    .and_then(|value| value.parse::<f32>().ok())
    .unwrap_or(0.5)
    .max(0.0);
  // [MS-ODRAWXML] defines gapWidth as gap width / category width.
  let group_width = slot / (1.0 + gap);
  let bar_width = group_width / series.len().max(1) as f32;
  let zero = scale.y(data_plot, 0.0_f64.clamp(scale.minimum, scale.maximum));
  for (series_index, series) in series.iter().enumerate() {
    let base = appearance.point_color(series_index, series.count());
    for (index, value) in series.values.iter().copied().enumerate() {
      let Some(value) = value else {
        continue;
      };
      let x = data_plot.x + slot * (index as f32 + 0.5) - group_width * 0.5
        + bar_width * series_index as f32;
      let y = scale.y(data_plot, value);
      let color = series_color_override(series, index, appearance, base);
      items.push(rect(
        x,
        y.min(zero),
        bar_width.max(0.5),
        (y - zero).abs().max(0.5),
        Some(color),
        Some((appearance.chart_fill, 0.5)),
      ));
      if let Some(label) = data_label_text(series, index, value) {
        push_centered_text(
          items,
          PlotRect {
            x,
            y: if value >= 0.0 {
              y - appearance.data_label_style.font_size_pt * 1.15
            } else {
              y + 1.0
            },
            width: bar_width,
            height: appearance.data_label_style.font_size_pt * 1.2,
          },
          label,
          appearance.data_label_style.clone(),
        );
      }
    }
  }
  lower_axes(
    items,
    data_plot,
    scale,
    &categories,
    chart_space,
    appearance,
    AxisPaintOptions {
      ui_language: None,
      pass: AxisPaintPass::Foreground,
      category_crossing: CategoryAxisCrossing::Zero,
    },
  );
}

#[derive(Clone, Debug)]
struct BoxSummary {
  minimum_whisker: f64,
  q1: f64,
  median: f64,
  q3: f64,
  maximum_whisker: f64,
  mean: f64,
  outliers: Vec<f64>,
  inner: Vec<f64>,
}

fn box_summary(values: &[f64], method: cx::QuartileMethod) -> Option<BoxSummary> {
  let mut values = values
    .iter()
    .copied()
    .filter(|value| value.is_finite())
    .collect::<Vec<_>>();
  if values.is_empty() {
    return None;
  }
  values.sort_by(f64::total_cmp);
  let percentile = |p| match method {
    cx::QuartileMethod::Inclusive => percentile_inclusive(&values, p),
    cx::QuartileMethod::Exclusive => percentile_exclusive(&values, p),
  };
  let q1 = percentile(0.25);
  let median = percentile(0.5);
  let q3 = percentile(0.75);
  let iqr = q3 - q1;
  let lower_fence = q1 - 1.5 * iqr;
  let upper_fence = q3 + 1.5 * iqr;
  let minimum_whisker = values
    .iter()
    .copied()
    .find(|value| *value >= lower_fence)
    .unwrap_or(values[0]);
  let maximum_whisker = values
    .iter()
    .copied()
    .rev()
    .find(|value| *value <= upper_fence)
    .unwrap_or(*values.last().unwrap_or(&values[0]));
  let outliers = values
    .iter()
    .copied()
    .filter(|value| *value < minimum_whisker || *value > maximum_whisker)
    .collect();
  let inner = values
    .iter()
    .copied()
    .filter(|value| *value >= minimum_whisker && *value <= maximum_whisker)
    .collect();
  Some(BoxSummary {
    minimum_whisker,
    q1,
    median,
    q3,
    maximum_whisker,
    mean: values.iter().sum::<f64>() / values.len() as f64,
    outliers,
    inner,
  })
}

fn percentile_inclusive(values: &[f64], percentile: f64) -> f64 {
  if values.len() == 1 {
    return values[0];
  }
  let rank = (values.len() - 1) as f64 * percentile;
  interpolate_rank(values, rank)
}

fn percentile_exclusive(values: &[f64], percentile: f64) -> f64 {
  let rank = (values.len() + 1) as f64 * percentile - 1.0;
  interpolate_rank(values, rank.clamp(0.0, (values.len() - 1) as f64))
}

fn interpolate_rank(values: &[f64], rank: f64) -> f64 {
  let lower = rank.floor() as usize;
  let upper = rank.ceil() as usize;
  if lower == upper {
    values[lower]
  } else {
    values[lower] + (values[upper] - values[lower]) * (rank - lower as f64)
  }
}

fn lower_box_whisker_chart(
  items: &mut Vec<PageItem>,
  plot: PlotRect,
  series: &[SeriesModel<'_>],
  chart_space: &cx::ChartSpace,
  appearance: &Appearance,
) {
  let has_explicit_categories = series.iter().any(|series| !series.categories.is_empty());
  let mut categories = if has_explicit_categories {
    let mut categories = Vec::<String>::new();
    for series in series {
      for index in 0..series.count() {
        let category = series.leaf_category(index);
        if !categories.contains(&category) {
          categories.push(category);
        }
      }
    }
    categories
  } else {
    // A BoxWhisker series without a category dimension is one distribution,
    // not one category per observation. Multiple such series share Office's
    // single automatic category and are laid out side by side inside it.
    vec!["1".to_string()]
  };
  if categories.is_empty() {
    categories.push(String::new());
  }
  let value_title = value_axis(chart_space).is_some_and(|axis| axis.axis_title.is_some());
  let category_title = category_axis(chart_space).is_some_and(|axis| axis.axis_title.is_some());
  let scale = box_whisker_axis_scale(
    series
      .iter()
      .flat_map(|series| series.values.iter().copied().flatten()),
    value_axis(chart_space),
  );
  // Each host owns the residual inset below its independently sized automatic
  // title band; the lower category edge remains fixed.
  let top_inset = match appearance.host {
    ChartExHost::PowerPoint => 8.95,
    // Excel keeps the lower category edge fixed but reserves one additional
    // half-line above the ten-increment box-whisker scale. boxWhisker.xlsx
    // exposes this as a 124.3pt 0..10 grid instead of the generic 129.6pt
    // Cartesian grid. Word remains the counterexample and keeps the generic
    // statistical inset.
    ChartExHost::Excel => 11.40,
    ChartExHost::Word => 5.95,
  };
  let data_plot = cartesian_plot_with_top_inset(
    plot,
    CartesianPlotOptions {
      has_value_title: value_title,
      has_category_title: category_title,
      has_out_end_labels: series.iter().any(has_out_end_labels),
      scale,
      axis: value_axis(chart_space),
      appearance,
      top_inset: Some(top_inset),
      use_excel_minimum_gutter: false,
    },
  );
  lower_axes(
    items,
    data_plot,
    scale,
    &categories,
    chart_space,
    appearance,
    AxisPaintOptions {
      ui_language: None,
      pass: AxisPaintPass::BackgroundGrid,
      category_crossing: CategoryAxisCrossing::Minimum,
    },
  );

  let category_slot = data_plot.width / categories.len() as f32;
  let gap_width = category_axis(chart_space)
    .and_then(|axis| match axis.axis_choice.as_ref() {
      Some(cx::AxisChoice::CategoryAxisScaling(scaling)) => scaling.gap_width.as_deref(),
      _ => None,
    })
    .and_then(|value| value.parse::<f32>().ok())
    .filter(|value| value.is_finite() && *value >= 0.0)
    .unwrap_or(1.0);
  // [MS-ODRAWXML] defines this value as gap width divided by category
  // width. The category slot therefore consists of one data-point group plus
  // that ratio of empty space.
  let group_width = category_slot / (1.0 + gap_width);
  let series_slot = group_width / series.len().max(1) as f32;
  // Multiple statistical series retain a small intra-group separation. A
  // single series owns the complete category width; otherwise the very small
  // 0.06 gap in the Office counterexample remains visibly too narrow.
  let box_width = series_slot * if series.len() > 1 { 0.90 } else { 1.0 };
  let mut means_by_series = vec![Vec::<(f32, f32)>::new(); series.len()];

  for (category_index, category) in categories.iter().enumerate() {
    for (series_index, series) in series.iter().enumerate() {
      let values = if has_explicit_categories {
        (0..series.count())
          .filter(|index| series.leaf_category(*index) == *category)
          .filter_map(|index| series.values.get(index).copied().flatten())
          .collect::<Vec<_>>()
      } else {
        series.values.iter().copied().flatten().collect::<Vec<_>>()
      };
      let properties = series.source.series_layout_properties.as_deref();
      let method = properties
        .and_then(|properties| properties.statistics.as_ref())
        .and_then(|statistics| statistics.quartile_method)
        .unwrap_or_default();
      let Some(summary) = box_summary(&values, method) else {
        continue;
      };
      let visibility =
        properties.and_then(|properties| properties.series_element_visibilities.as_ref());
      let show_mean_marker = visibility
        .and_then(|visibility| visibility.mean_marker)
        .is_none_or(|value| value.as_bool());
      let show_mean_line = visibility
        .and_then(|visibility| visibility.mean_line)
        .is_some_and(|value| value.as_bool());
      let show_inner = visibility
        .and_then(|visibility| visibility.nonoutliers)
        .is_none_or(|value| value.as_bool());
      let show_outliers = visibility
        .and_then(|visibility| visibility.outliers)
        .is_none_or(|value| value.as_bool());
      let x = data_plot.x + category_slot * (category_index as f32 + 0.5) - group_width * 0.5
        + series_slot * (series_index as f32 + 0.5);
      let color = series_color_override(
        series,
        category_index,
        appearance,
        appearance.point_color(series_index, series.count()),
      );
      let y_min = scale.y(data_plot, summary.minimum_whisker);
      let y_q1 = scale.y(data_plot, summary.q1);
      let y_median = scale.y(data_plot, summary.median);
      let y_q3 = scale.y(data_plot, summary.q3);
      let y_max = scale.y(data_plot, summary.maximum_whisker);
      let y_mean = scale.y(data_plot, summary.mean);
      let (outline, outline_width, outline_source) = appearance.box_whisker_stroke(color);
      push_box_whisker_line(
        items,
        (x, y_max),
        (x, y_q3),
        outline,
        outline_width,
        outline_source,
      );
      push_box_whisker_line(
        items,
        (x, y_q1),
        (x, y_min),
        outline,
        outline_width,
        outline_source,
      );
      push_box_whisker_line(
        items,
        (x - box_width * 0.15, y_max),
        (x + box_width * 0.15, y_max),
        outline,
        outline_width,
        outline_source,
      );
      push_box_whisker_line(
        items,
        (x - box_width * 0.15, y_min),
        (x + box_width * 0.15, y_min),
        outline,
        outline_width,
        outline_source,
      );
      items.push(rect(
        x - box_width * 0.5,
        y_q3,
        box_width,
        (y_q1 - y_q3).max(outline_width),
        Some(color),
        Some((outline, outline_width)),
      ));
      push_box_whisker_line(
        items,
        (x - box_width * 0.5, y_median),
        (x + box_width * 0.5, y_median),
        outline,
        outline_width,
        outline_source,
      );
      if show_mean_marker {
        let size = 3.0_f32.min(box_width * 0.25);
        push_box_whisker_line(
          items,
          (x - size, y_mean - size),
          (x + size, y_mean + size),
          outline,
          outline_width,
          outline_source,
        );
        push_box_whisker_line(
          items,
          (x - size, y_mean + size),
          (x + size, y_mean - size),
          outline,
          outline_width,
          outline_source,
        );
      }
      if show_mean_line {
        means_by_series[series_index].push((x, y_mean));
      }
      if show_inner {
        let mut inner = summary
          .inner
          .iter()
          .copied()
          .filter(|value| *value > summary.minimum_whisker && *value < summary.maximum_whisker)
          .collect::<Vec<_>>();
        inner.dedup_by(|left, right| *left == *right);
        for value in inner {
          push_marker(items, x, scale.y(data_plot, value), 1.5, color);
        }
      }
      if show_outliers {
        let mut outliers = summary.outliers.clone();
        outliers.dedup_by(|left, right| *left == *right);
        for value in outliers {
          push_marker(items, x, scale.y(data_plot, value), 1.5, color);
        }
      }
      lower_box_whisker_data_labels(
        items,
        series,
        &values,
        &summary,
        x,
        box_width,
        data_plot,
        scale,
        show_mean_marker,
        appearance,
      );
    }
  }
  for (series_index, means) in means_by_series.iter().enumerate() {
    for pair in means.windows(2) {
      push_line(
        items,
        pair[0].0,
        pair[0].1,
        pair[1].0,
        pair[1].1,
        appearance.point_color(series_index, series.len()),
        0.75,
      );
    }
  }
  lower_axes(
    items,
    data_plot,
    scale,
    &categories,
    chart_space,
    appearance,
    AxisPaintOptions {
      ui_language: None,
      pass: AxisPaintPass::Foreground,
      category_crossing: CategoryAxisCrossing::Minimum,
    },
  );
}

fn push_box_whisker_line(
  items: &mut Vec<PageItem>,
  start: (f32, f32),
  end: (f32, f32),
  color: RgbColor,
  width: f32,
  outline: Option<&a::Outline>,
) {
  let (x1, y1) = start;
  let (x2, y2) = end;
  let mut stroke = crate::common::Stroke {
    width: crate::common::Pt(width),
    color: common_rgb(color, 1.0),
    ..Default::default()
  };
  if let Some(outline) = outline {
    crate::common::drawingml_stroke::apply_outline_style(&mut stroke, outline);
  }
  items.push(PageItem::Path(crate::common::PathItem {
    bounds: common_rect(x1.min(x2), y1.min(y2), (x2 - x1).abs(), (y2 - y1).abs()),
    points: Vec::new(),
    commands: vec![
      crate::common::PathCommand::MoveTo(common_point(x1, y1)),
      crate::common::PathCommand::LineTo(common_point(x2, y2)),
    ],
    closed: false,
    fill: crate::common::Fill::None,
    stroke: Some(stroke),
  }));
}

#[allow(clippy::too_many_arguments)]
fn lower_box_whisker_data_labels(
  items: &mut Vec<PageItem>,
  series: &SeriesModel<'_>,
  values: &[f64],
  summary: &BoxSummary,
  x: f32,
  box_width: f32,
  data_plot: PlotRect,
  scale: AxisScale,
  show_mean_marker: bool,
  appearance: &Appearance,
) {
  if series.source.data_labels.is_none() {
    return;
  }
  let mut labeled_values = values
    .iter()
    .copied()
    .enumerate()
    .filter_map(|(index, value)| data_label_text(series, index, value).map(|text| (value, text)))
    .collect::<Vec<_>>();
  labeled_values.sort_by(|left, right| left.0.total_cmp(&right.0));
  labeled_values.dedup_by(|left, right| left.0 == right.0 && left.1 == right.1);
  let style = &appearance.data_label_style;
  let label_y = |value| scale.y(data_plot, value) - style.font_size_pt * 0.42;
  for (value, text) in labeled_values {
    let at_whisker = value <= summary.minimum_whisker || value >= summary.maximum_whisker;
    let label_x = if at_whisker {
      x + box_width * 0.15
    } else {
      x + 1.5
    };
    push_text(items, label_x, label_y(value), text, style.clone());
  }

  for value in [summary.q1, summary.median, summary.q3] {
    if let Some(text) = inherited_data_label_text(series, value) {
      push_text(
        items,
        x + box_width * 0.5,
        label_y(value),
        text,
        style.clone(),
      );
    }
  }
  if show_mean_marker
    && let Some(text) =
      inherited_data_label_text(series, box_whisker_mean_label_value(summary.mean))
  {
    push_text(items, x + 3.0, label_y(summary.mean), text, style.clone());
  }
}

fn box_whisker_mean_label_value(value: f64) -> f64 {
  if value == 0.0 || !value.is_finite() {
    return value;
  }
  // Excel exposes computed ChartEx statistics to the data-label formatter at
  // ten significant decimal digits. Keep the full value for geometry while
  // removing calculation tails from the visible mean.
  let magnitude = value.abs().log10().floor() as i32;
  let factor = 10_f64.powi(9 - magnitude);
  (value * factor).round() / factor
}

fn inherited_data_label_text(series: &SeriesModel<'_>, value: f64) -> Option<String> {
  let labels = series.source.data_labels.as_deref()?;
  let visible = visibility(
    labels.data_label_visibilities.as_ref(),
    LabelVisibility::default(),
  );
  let format = labels
    .number_format
    .as_ref()
    .map(|format| format.format_code.as_str())
    .or(series.number_format.as_deref());
  let mut parts = Vec::new();
  if visible.series {
    parts.push(series.name.clone());
  }
  if visible.category {
    parts.push(series.leaf_category(0));
  }
  if visible.value {
    parts.push(format_chart_number(value, format));
  }
  (!parts.is_empty()).then(|| parts.join(labels.separator_xsdstring.as_deref().unwrap_or(", ")))
}

fn source_has_binning(series: &cx::Series) -> bool {
  series
    .series_layout_properties
    .as_deref()
    .and_then(|properties| properties.series_layout_properties_choice.as_ref())
    .is_some_and(|choice| matches!(choice, cx::SeriesLayoutPropertiesChoice::Binning(_)))
}

fn is_histogram_series(series: &SeriesModel<'_>) -> bool {
  source_has_binning(series.source)
}

pub(crate) fn is_histogram_chart_space(chart_space: &cx::ChartSpace) -> bool {
  let series = &chart_space.chart.plot_area.plot_area_region.series;
  !series
    .iter()
    .any(|series| series.layout_id == cx::SeriesLayout::ParetoLine)
    && series.iter().any(|series| {
      series.layout_id == cx::SeriesLayout::ClusteredColumn && source_has_binning(series)
    })
}

pub(crate) fn is_pareto_chart_space(chart_space: &cx::ChartSpace) -> bool {
  chart_space
    .chart
    .plot_area
    .plot_area_region
    .series
    .iter()
    .any(|series| series.layout_id == cx::SeriesLayout::ParetoLine)
}

pub(crate) fn is_waterfall_chart_space(chart_space: &cx::ChartSpace) -> bool {
  chart_space
    .chart
    .plot_area
    .plot_area_region
    .series
    .iter()
    .any(|series| series.layout_id == cx::SeriesLayout::Waterfall)
}

#[derive(Clone, Debug, PartialEq)]
struct HistogramBin {
  lower: Option<f64>,
  upper: Option<f64>,
  count: usize,
}

fn histogram_bins(values: &[f64], binning: Option<&cx::Binning>) -> Vec<HistogramBin> {
  let mut values = values
    .iter()
    .copied()
    .filter(|value| value.is_finite())
    .collect::<Vec<_>>();
  if values.is_empty() {
    return Vec::new();
  }
  values.sort_by(f64::total_cmp);
  let minimum = values[0];
  let maximum = *values.last().unwrap_or(&minimum);
  let underflow = binning
    .and_then(|binning| binning.underflow.as_deref())
    .and_then(parse_axis_number);
  let overflow = binning
    .and_then(|binning| binning.overflow.as_deref())
    .and_then(parse_axis_number);
  let (bin_count, width) = match binning.and_then(|binning| binning.binning_choice.as_ref()) {
    Some(cx::BinningChoice::Xsddouble(width)) => {
      let width = width.val.or(width.xml_content).filter(|width| *width > 0.0);
      let width = width.unwrap_or_else(|| rounded_scott_bin_width(&values));
      let count = ((maximum - minimum) / width).ceil().max(1.0) as usize;
      (count, width)
    }
    Some(cx::BinningChoice::BinCountXsdunsignedInt(count)) => {
      let count = count
        .val
        .or(count.xml_content)
        .map(|count| count as usize)
        .unwrap_or(1)
        .max(1);
      (count, ((maximum - minimum) / count as f64).max(1.0))
    }
    None => {
      let width = rounded_scott_bin_width(&values);
      let count = ((maximum - minimum) / width).ceil().max(1.0) as usize;
      (count, width)
    }
  };
  let start = underflow.unwrap_or(minimum);
  let mut bins = Vec::new();
  if let Some(boundary) = underflow {
    bins.push(HistogramBin {
      lower: None,
      upper: Some(boundary),
      count: values.iter().filter(|value| **value <= boundary).count(),
    });
  }
  for index in 0..bin_count {
    let lower = start + index as f64 * width;
    let upper = if index + 1 == bin_count {
      overflow
        .unwrap_or(start + (index + 1) as f64 * width)
        .max(maximum)
    } else {
      start + (index + 1) as f64 * width
    };
    let count = values
      .iter()
      .filter(|value| {
        let above_lower = if index == 0 && underflow.is_none() {
          **value >= lower
        } else {
          **value > lower
        };
        above_lower && **value <= upper
      })
      .count();
    bins.push(HistogramBin {
      lower: Some(lower),
      upper: Some(upper),
      count,
    });
  }
  if let Some(boundary) = overflow {
    bins.push(HistogramBin {
      lower: Some(boundary),
      upper: None,
      count: values.iter().filter(|value| **value > boundary).count(),
    });
  }
  bins
}

fn scott_bin_width(values: &[f64]) -> f64 {
  if values.len() < 2 {
    return 1.0;
  }
  let mean = values.iter().sum::<f64>() / values.len() as f64;
  let variance = values
    .iter()
    .map(|value| (value - mean).powi(2))
    .sum::<f64>()
    / (values.len() - 1) as f64;
  let width = 3.5 * variance.sqrt() / (values.len() as f64).cbrt();
  if width.is_finite() && width > 0.0 {
    width
  } else {
    1.0
  }
}

fn rounded_scott_bin_width(values: &[f64]) -> f64 {
  let width = scott_bin_width(values);
  if !width.is_finite() || width <= 0.0 {
    return 1.0;
  }
  // Excel's automatic Histogram/Pareto axis rounds Scott's result to two
  // significant digits before constructing the displayed boundaries. The
  // three Office corpus ranges independently expose 3.937 -> 3.9,
  // 2.166 -> 2.2, and 3.477 -> 3.5.
  let magnitude = 10_f64.powf(width.abs().log10().floor() - 1.0);
  (width / magnitude).round() * magnitude
}

fn histogram_boundary_digits(bin: &HistogramBin) -> usize {
  let width = match (bin.lower, bin.upper) {
    (Some(lower), Some(upper)) => (upper - lower).abs(),
    _ => 1.0,
  };
  if !width.is_finite() || width <= 0.0 {
    return 0;
  }
  (1 - width.log10().floor() as i32).max(0) as usize
}

fn histogram_boundary(value: f64, digits: usize) -> String {
  let factor = 10_f64.powi(digits as i32);
  format_chart_number((value * factor).round() / factor, None)
}

fn histogram_bin_label(bin: &HistogramBin, closed: cx::IntervalClosedSide) -> String {
  let digits = histogram_boundary_digits(bin);
  match (bin.lower, bin.upper) {
    (None, Some(upper)) => format!("≤{}", histogram_boundary(upper, digits)),
    (Some(lower), None) => format!(">{}", histogram_boundary(lower, digits)),
    (Some(lower), Some(upper)) => match closed {
      cx::IntervalClosedSide::L => format!(
        "[{}, {})",
        histogram_boundary(lower, digits),
        histogram_boundary(upper, digits)
      ),
      cx::IntervalClosedSide::R => format!(
        "({}, {}]",
        histogram_boundary(lower, digits),
        histogram_boundary(upper, digits)
      ),
    },
    (None, None) => String::new(),
  }
}

fn histogram_bin_labels(bins: &[HistogramBin], binning: Option<&cx::Binning>) -> Vec<String> {
  let closed = binning
    .and_then(|binning| binning.interval_closed)
    .unwrap_or(cx::IntervalClosedSide::R);
  bins
    .iter()
    .enumerate()
    .map(|(index, bin)| {
      let mut label = histogram_bin_label(bin, closed);
      if index == 0 && bin.lower.is_some() {
        label.replace_range(..1, "[");
      }
      label
    })
    .collect()
}

fn histogram_axis_scale(bins: &[HistogramBin], axis: Option<&cx::Axis>) -> AxisScale {
  let scaling = axis.and_then(|axis| match axis.axis_choice.as_ref() {
    Some(cx::AxisChoice::ValueAxisScaling(scaling)) => Some(scaling.as_ref()),
    _ => None,
  });
  if scaling.is_some_and(|scaling| {
    scaling.min.is_some() || scaling.max.is_some() || scaling.major_unit.is_some()
  }) {
    return axis_scale(bins.iter().map(|bin| bin.count as f64), axis);
  }
  let maximum_count = bins.iter().map(|bin| bin.count).max().unwrap_or(1).max(1) as f64;
  let major = nice_number(maximum_count / 8.0);
  AxisScale {
    minimum: 0.0,
    maximum: (maximum_count / major).ceil() * major + major,
    major,
    divisor: axis
      .and_then(|axis| axis.axis_units.as_deref())
      .and_then(|units| units.unit)
      .map(axis_unit_divisor)
      .unwrap_or(1.0),
  }
}

fn lower_histogram_chart(
  items: &mut Vec<PageItem>,
  plot: PlotRect,
  series: &SeriesModel<'_>,
  chart_space: &cx::ChartSpace,
  appearance: &Appearance,
) {
  let binning = series
    .source
    .series_layout_properties
    .as_deref()
    .and_then(|properties| properties.series_layout_properties_choice.as_ref())
    .and_then(|choice| match choice {
      cx::SeriesLayoutPropertiesChoice::Binning(binning) => Some(binning.as_ref()),
      _ => None,
    });
  let values = series.values.iter().copied().flatten().collect::<Vec<_>>();
  let bins = histogram_bins(&values, binning);
  if bins.is_empty() {
    return;
  }
  let categories = histogram_bin_labels(&bins, binning);
  let value_title = value_axis(chart_space).is_some_and(|axis| axis.axis_title.is_some());
  let category_title = category_axis(chart_space).is_some_and(|axis| axis.axis_title.is_some());
  let scale = histogram_axis_scale(&bins, value_axis(chart_space));
  let data_plot = cartesian_plot(
    plot,
    value_title,
    category_title,
    has_out_end_labels(series),
    scale,
    value_axis(chart_space),
    appearance,
  );
  lower_axes(
    items,
    data_plot,
    scale,
    &categories,
    chart_space,
    appearance,
    AxisPaintOptions {
      ui_language: None,
      pass: AxisPaintPass::BackgroundGrid,
      category_crossing: CategoryAxisCrossing::Zero,
    },
  );
  let slot = data_plot.width / bins.len() as f32;
  let zero = scale.y(data_plot, 0.0);
  let color = appearance.point_color(series.source_index, series.count());
  for (index, bin) in bins.iter().enumerate() {
    let y = scale.y(data_plot, bin.count as f64);
    items.push(rect(
      data_plot.x + slot * index as f32,
      y,
      slot.max(0.5),
      (zero - y).max(0.5),
      Some(series_color_override(series, index, appearance, color)),
      Some((appearance.chart_fill, 0.5)),
    ));
  }
  lower_axes(
    items,
    data_plot,
    scale,
    &categories,
    chart_space,
    appearance,
    AxisPaintOptions {
      ui_language: None,
      pass: AxisPaintPass::Foreground,
      category_crossing: CategoryAxisCrossing::Zero,
    },
  );
}

#[derive(Clone, Debug, Default)]
struct HierarchyNode {
  name: String,
  value: f64,
  source_index: Option<usize>,
  children: Vec<HierarchyNode>,
}

impl HierarchyNode {
  fn insert(&mut self, path: &[String], value: f64, source_index: usize) {
    self.value += value;
    let Some((name, rest)) = path.split_first() else {
      self.source_index = Some(source_index);
      return;
    };
    let child_index = self
      .children
      .iter()
      .position(|child| child.name == *name)
      .unwrap_or_else(|| {
        self.children.push(HierarchyNode {
          name: name.clone(),
          ..HierarchyNode::default()
        });
        self.children.len() - 1
      });
    self.children[child_index].insert(rest, value, source_index);
  }

  fn depth(&self) -> usize {
    self
      .children
      .iter()
      .map(HierarchyNode::depth)
      .max()
      .unwrap_or(0)
      + usize::from(!self.children.is_empty())
  }
}

fn hierarchy(series: &SeriesModel<'_>) -> HierarchyNode {
  let mut root = HierarchyNode::default();
  for (index, value) in series.values.iter().copied().enumerate() {
    let Some(value) = value.filter(|value| value.is_finite() && *value > 0.0) else {
      continue;
    };
    let mut path = series.category_path(index);
    if path.is_empty() {
      path.push((index + 1).to_string());
    }
    root.insert(&path, value, index);
  }
  root
}

#[derive(Clone, Copy, Debug)]
struct TreemapTile<'a> {
  node: &'a HierarchyNode,
  rect: PlotRect,
}

fn squarify<'a>(nodes: &'a [HierarchyNode], rect: PlotRect) -> Vec<TreemapTile<'a>> {
  let mut nodes = nodes
    .iter()
    .filter(|node| node.value > 0.0)
    .collect::<Vec<_>>();
  nodes.sort_by(|left, right| right.value.total_cmp(&left.value));
  let total = nodes.iter().map(|node| node.value).sum::<f64>();
  if nodes.is_empty() || total <= 0.0 || rect.width <= 0.0 || rect.height <= 0.0 {
    return Vec::new();
  }
  let scale = f64::from(rect.width * rect.height) / total;
  let mut remaining = rect;
  let mut row = Vec::<&HierarchyNode>::new();
  let mut result = Vec::new();
  while let Some(node) = nodes.first().copied() {
    let side = remaining.width.min(remaining.height).max(0.1);
    let current = worst_aspect(&row, side, scale);
    let mut candidate = row.clone();
    candidate.push(node);
    let next = worst_aspect(&candidate, side, scale);
    if row.is_empty() || next <= current * 1.2 {
      row.push(node);
      nodes.remove(0);
    } else {
      layout_treemap_row(&row, &mut remaining, scale, &mut result);
      row.clear();
    }
  }
  if !row.is_empty() {
    layout_treemap_row(&row, &mut remaining, scale, &mut result);
  }
  result
}

fn worst_aspect(nodes: &[&HierarchyNode], side: f32, scale: f64) -> f64 {
  if nodes.is_empty() {
    return f64::INFINITY;
  }
  let areas = nodes
    .iter()
    .map(|node| node.value * scale)
    .collect::<Vec<_>>();
  let sum = areas.iter().sum::<f64>();
  let minimum = areas.iter().copied().fold(f64::INFINITY, f64::min);
  let maximum = areas.iter().copied().fold(0.0_f64, f64::max);
  let side_squared = f64::from(side * side);
  ((side_squared * maximum) / (sum * sum))
    .max((sum * sum) / (side_squared * minimum.max(f64::EPSILON)))
}

fn layout_treemap_row<'a>(
  nodes: &[&'a HierarchyNode],
  remaining: &mut PlotRect,
  scale: f64,
  output: &mut Vec<TreemapTile<'a>>,
) {
  let area = nodes.iter().map(|node| node.value * scale).sum::<f64>() as f32;
  if remaining.width >= remaining.height {
    let width = (area / remaining.height.max(0.1)).min(remaining.width);
    let mut y = remaining.y;
    for node in nodes {
      let height = (node.value * scale) as f32 / width.max(0.1);
      output.push(TreemapTile {
        node,
        rect: PlotRect {
          x: remaining.x,
          y,
          width,
          height,
        },
      });
      y += height;
    }
    remaining.x += width;
    remaining.width = (remaining.width - width).max(0.0);
  } else {
    let height = (area / remaining.width.max(0.1)).min(remaining.height);
    let mut x = remaining.x;
    for node in nodes {
      let width = (node.value * scale) as f32 / height.max(0.1);
      output.push(TreemapTile {
        node,
        rect: PlotRect {
          x,
          y: remaining.y,
          width,
          height,
        },
      });
      x += width;
    }
    remaining.y += height;
    remaining.height = (remaining.height - height).max(0.0);
  }
}

fn lower_treemap(
  items: &mut Vec<PageItem>,
  plot: PlotRect,
  series: &SeriesModel<'_>,
  appearance: &Appearance,
  legend_entries: &mut Vec<(String, RgbColor)>,
) {
  let root = hierarchy(series);
  let parent_layout = series
    .source
    .series_layout_properties
    .as_deref()
    .and_then(|properties| properties.parent_label_layout.as_ref())
    .map(|layout| layout.parent_label_layout_val)
    .unwrap_or(cx::ParentLabelLayoutVal::None);
  let show_labels = hierarchy_category_labels(series);
  let label_position = series
    .source
    .data_labels
    .as_deref()
    .and_then(|labels| labels.pos)
    .unwrap_or_default();
  legend_entries.extend(root.children.iter().enumerate().map(|(index, node)| {
    (
      node.name.clone(),
      appearance.point_color(index, root.children.len()),
    )
  }));
  let layout_plot = PlotRect {
    x: plot.x + 5.4,
    y: plot.y + 0.27,
    width: (plot.width - 10.8).max(0.1),
    height: (plot.height - 0.54).max(0.1),
  };
  let tiles = squarify(&root.children, layout_plot);
  for tile in &tiles {
    let palette_index = root
      .children
      .iter()
      .position(|node| std::ptr::eq(node, tile.node))
      .unwrap_or(0);
    let color = appearance.point_color(palette_index, root.children.len());
    lower_treemap_node(
      items,
      tile.node,
      tile.rect,
      TreemapRenderContext {
        base: color,
        depth: 0,
        parent_layout,
        show_labels,
        label_position,
        appearance,
      },
    );
  }
}

#[derive(Clone, Copy)]
struct TreemapRenderContext<'a> {
  base: RgbColor,
  depth: usize,
  parent_layout: cx::ParentLabelLayoutVal,
  show_labels: bool,
  label_position: cx::DataLabelPos,
  appearance: &'a Appearance,
}

fn lower_treemap_node(
  items: &mut Vec<PageItem>,
  node: &HierarchyNode,
  rect: PlotRect,
  context: TreemapRenderContext<'_>,
) {
  let TreemapRenderContext {
    base,
    depth,
    parent_layout,
    show_labels,
    label_position,
    appearance,
  } = context;
  items.push(rect_item(
    rect,
    Some(base),
    Some((appearance.chart_fill, 1.5)),
  ));
  if node.children.is_empty() {
    if show_labels {
      let style = appearance.data_label_style.clone();
      let inset_x = 7.3;
      let y = if label_position == cx::DataLabelPos::InEnd {
        rect.y + rect.height - style.font_size_pt * 1.2 - 3.7
      } else {
        rect.y + 6.4
      };
      if rect.width >= text_width(&node.name, &style) + inset_x * 2.0
        && rect.height >= style.font_size_pt * 1.2 + 8.0
      {
        push_text(items, rect.x + inset_x, y, node.name.clone(), style);
      }
    }
    return;
  }

  let header_height = match parent_layout {
    cx::ParentLabelLayoutVal::Banner => appearance.data_label_style.font_size_pt * 1.45,
    cx::ParentLabelLayoutVal::Overlapping => appearance.data_label_style.font_size_pt * 1.15,
    cx::ParentLabelLayoutVal::None => 0.0,
  }
  .min(rect.height * 0.35);
  if show_labels && depth == 0 && parent_layout != cx::ParentLabelLayoutVal::None {
    push_text(
      items,
      rect.x + 7.3,
      rect.y + 6.4,
      node.name.clone(),
      appearance.data_label_style.clone(),
    );
  }
  let child_rect = PlotRect {
    x: rect.x,
    y: rect.y
      + if parent_layout == cx::ParentLabelLayoutVal::Banner {
        header_height
      } else {
        0.0
      },
    width: rect.width,
    height: (rect.height
      - if parent_layout == cx::ParentLabelLayoutVal::Banner {
        header_height
      } else {
        0.0
      })
    .max(0.1),
  };
  for tile in squarify(&node.children, child_rect) {
    lower_treemap_node(
      items,
      tile.node,
      tile.rect,
      TreemapRenderContext {
        base,
        depth: depth + 1,
        parent_layout,
        show_labels,
        label_position,
        appearance,
      },
    );
  }
}

fn hierarchy_category_labels(series: &SeriesModel<'_>) -> bool {
  series
    .source
    .data_labels
    .as_deref()
    .and_then(|labels| labels.data_label_visibilities.as_ref())
    .and_then(|visibility| visibility.category_name)
    .is_some_and(|value| value.as_bool())
}

fn lower_sunburst(
  items: &mut Vec<PageItem>,
  plot: PlotRect,
  series: &SeriesModel<'_>,
  appearance: &Appearance,
) {
  let root = hierarchy(series);
  let rings = root.depth().max(1);
  let (center_y_ratio, radius_ratio) = if appearance.host == ChartExHost::PowerPoint {
    // PowerPoint's Office 2016 sunburst uses the complete compact-title plot
    // and leaves the radial scene fractionally below its geometric centre.
    // The fixed-format Of16-03 reference resolves to a 194.48pt radius around
    // (479.96, 256.94), independently of Excel's worksheet/anchor profile.
    (0.506_16, 0.500_89)
  } else {
    (0.5, 0.496_6)
  };
  let center = (
    plot.x + plot.width * 0.5,
    plot.y + plot.height * center_y_ratio,
  );
  let radius = plot.width.min(plot.height) * radius_ratio;
  // Office reserves one radial band for the donut hole, then gives every
  // hierarchy level an equal-width ring.
  let ring_width = radius / (rings + 1) as f32;
  let show_labels = hierarchy_category_labels(series);
  let total = root.value.max(f64::EPSILON);
  let mut angle = -PI * 0.5;
  let mut top_nodes = root.children.iter().enumerate().collect::<Vec<_>>();
  top_nodes.sort_by(|left, right| {
    right
      .1
      .value
      .total_cmp(&left.1.value)
      .then_with(|| left.0.cmp(&right.0))
  });
  for (palette_index, node) in top_nodes {
    let sweep = TAU * (node.value / total) as f32;
    let color = appearance.point_color(palette_index, root.children.len());
    lower_sunburst_node(
      items,
      node,
      center,
      0,
      ring_width,
      angle,
      angle + sweep,
      color,
      show_labels,
      appearance,
    );
    angle += sweep;
  }
}

#[allow(clippy::too_many_arguments)]
fn lower_sunburst_node(
  items: &mut Vec<PageItem>,
  node: &HierarchyNode,
  center: (f32, f32),
  depth: usize,
  ring_width: f32,
  start: f32,
  end: f32,
  base: RgbColor,
  show_labels: bool,
  appearance: &Appearance,
) {
  let inner = ring_width * (depth + 1) as f32;
  // A point with a blank lower category remains in its own ring. Office
  // leaves the deeper rings empty rather than stretching that point outward.
  let outer = ring_width * (depth + 2) as f32;
  push_annular_sector(
    items,
    center,
    inner,
    outer,
    start,
    end,
    base,
    appearance
      .data_point_stroke(base)
      .unwrap_or((appearance.chart_fill, 0.75)),
  );
  if show_labels {
    let middle = (start + end) * 0.5;
    let label_radius = (inner + outer) * 0.5;
    let arc = (end - start).abs() * label_radius;
    // Sunburst labels read radially: the ring width owns the text length,
    // while the arc owns only the glyph height. Testing those dimensions in
    // the opposite order drops Office's narrow Leaf 2/6/12/13/16 sectors.
    let mut style = appearance.data_label_style.clone();
    let width = TextMetrics::new().measure_text(&node.name, &style);
    let line_height = style.font_size_pt * 1.2;
    if arc >= line_height + 3.0 && width <= (ring_width - 6.0).max(0.0) {
      let x = center.0 + middle.cos() * label_radius;
      let y = center.1 + middle.sin() * label_radius;
      // Office's fixed-format writer emits rotated sunburst labels as glyph
      // outlines.  Keeping them out of the PDF text layer also prevents the
      // radial reading order from corrupting document extraction.
      style.pdf_glyph_outlines = true;
      let degrees = middle.to_degrees();
      style.rotation_deg = if (90.0..270.0).contains(&degrees.rem_euclid(360.0)) {
        degrees + 180.0
      } else {
        degrees
      };
      push_centered_rotated_text(items, (x, y), width, node.name.clone(), style);
    }
  }
  if node.children.is_empty() {
    return;
  }
  let total = node.children.iter().map(|child| child.value).sum::<f64>();
  if total <= 0.0 {
    return;
  }
  let mut child_start = start;
  let mut children = node.children.iter().enumerate().collect::<Vec<_>>();
  children.sort_by(|left, right| {
    right
      .1
      .value
      .total_cmp(&left.1.value)
      .then_with(|| left.0.cmp(&right.0))
  });
  for (_, child) in children {
    let sweep = (end - start) * (child.value / total) as f32;
    lower_sunburst_node(
      items,
      child,
      center,
      depth + 1,
      ring_width,
      child_start,
      child_start + sweep,
      base,
      show_labels,
      appearance,
    );
    child_start += sweep;
  }
}

fn lower_pareto_chart(
  items: &mut Vec<PageItem>,
  plot: PlotRect,
  series: &[SeriesModel<'_>],
  chart_space: &cx::ChartSpace,
  appearance: &Appearance,
  ui_language: Option<&str>,
  legend_entries: &mut Vec<(String, RgbColor)>,
) {
  let column = series
    .iter()
    .find(|series| series.layout == cx::SeriesLayout::ClusteredColumn)
    .or_else(|| series.first())
    .unwrap();
  let line = series
    .iter()
    .find(|series| series.layout == cx::SeriesLayout::ParetoLine);
  let binning = column
    .source
    .series_layout_properties
    .as_deref()
    .and_then(|properties| properties.series_layout_properties_choice.as_ref())
    .and_then(|choice| match choice {
      cx::SeriesLayoutPropertiesChoice::Binning(binning) => Some(binning.as_ref()),
      _ => None,
    });
  let pareto_bins = binning.map(|binning| {
    histogram_bins(
      &column.values.iter().copied().flatten().collect::<Vec<_>>(),
      Some(binning),
    )
  });
  let mut aggregate = if let Some(bins) = pareto_bins.as_deref() {
    histogram_bin_labels(bins, binning)
      .into_iter()
      .zip(bins.iter().map(|bin| bin.count as f64))
      .collect::<Vec<_>>()
  } else {
    let mut aggregate = Vec::<(String, f64)>::new();
    for (index, value) in column.values.iter().copied().enumerate() {
      let Some(value) = value.filter(|value| value.is_finite()) else {
        continue;
      };
      let category = column.leaf_category(index);
      if let Some((_, total)) = aggregate.iter_mut().find(|entry| entry.0 == category) {
        *total += value;
      } else {
        aggregate.push((category, value));
      }
    }
    aggregate
  };
  aggregate.sort_by(|left, right| right.1.total_cmp(&left.1));
  if aggregate.is_empty() {
    return;
  }
  let categories = aggregate
    .iter()
    .map(|(category, _)| category.clone())
    .collect::<Vec<_>>();
  let value_title = value_axis(chart_space).is_some_and(|axis| axis.axis_title.is_some());
  let category_title = category_axis(chart_space).is_some_and(|axis| axis.axis_title.is_some());
  let scale = pareto_bins.as_deref().map_or_else(
    || {
      axis_scale(
        aggregate.iter().map(|(_, value)| *value),
        value_axis(chart_space),
      )
    },
    |bins| histogram_axis_scale(bins, value_axis(chart_space)),
  );
  let mut data_plot = cartesian_plot(
    plot,
    value_title,
    category_title,
    series.iter().any(has_out_end_labels),
    scale,
    value_axis(chart_space),
    appearance,
  );
  if let Some(axis) = line.and_then(|line| pareto_value_axis(line, chart_space))
    && !axis.hidden.is_some_and(|hidden| hidden.as_bool())
    && axis.tick_labels.is_some()
  {
    let scaling = match axis.axis_choice.as_ref() {
      Some(cx::AxisChoice::ValueAxisScaling(scaling)) => Some(scaling.as_ref()),
      _ => None,
    };
    let maximum = scaling
      .and_then(|scaling| scaling.max.as_deref())
      .and_then(parse_axis_number)
      .unwrap_or(1.0);
    let percentage =
      axis.axis_units.as_deref().and_then(|units| units.unit) == Some(cx::AxisUnit::Percentage);
    let terminal_label = if percentage {
      format!("{}%", format_chart_number(maximum * 100.0, None))
    } else {
      format_chart_number(maximum, None)
    };
    let terminal_width = TextMetrics::new().measure_text(&terminal_label, &appearance.label_style);
    let current_right = if value_title || category_title {
      6.55
    } else {
      6.65
    };
    // The secondary-axis label sits 6.25pt beyond the plot edge and Office
    // retains the same 12.05pt outer chart gutter used on the primary side.
    let required_right = 6.25 + terminal_width + 12.05;
    data_plot.width = (data_plot.width - (required_right - current_right).max(0.0)).max(1.0);
  }
  lower_axes(
    items,
    data_plot,
    scale,
    &categories,
    chart_space,
    appearance,
    AxisPaintOptions {
      ui_language: None,
      pass: AxisPaintPass::BackgroundGrid,
      category_crossing: CategoryAxisCrossing::Zero,
    },
  );
  let slot = data_plot.width / aggregate.len() as f32;
  let gap = category_axis(chart_space)
    .and_then(|axis| match axis.axis_choice.as_ref() {
      Some(cx::AxisChoice::CategoryAxisScaling(scaling)) => scaling.gap_width.as_deref(),
      _ => None,
    })
    .and_then(|value| value.parse::<f32>().ok())
    .unwrap_or(0.0)
    .max(0.0);
  let column_width = slot / (1.0 + gap);
  let zero = scale.y(data_plot, 0.0_f64.clamp(scale.minimum, scale.maximum));
  let column_color = appearance.point_color(column.source_index, series.len());
  let line_color =
    appearance.point_color(line.map_or(1, |series| series.source_index), series.len());
  let total = aggregate.iter().map(|(_, value)| *value).sum::<f64>();
  let mut cumulative = 0.0;
  let mut previous: Option<(f32, f32)> = None;
  for (index, (_, value)) in aggregate.iter().enumerate() {
    let y = scale.y(data_plot, *value);
    let x = data_plot.x + slot * (index as f32 + 0.5) - column_width * 0.5;
    items.push(rect(
      x,
      y.min(zero),
      column_width,
      (zero - y).abs().max(0.5),
      Some(column_color),
      Some((appearance.chart_fill, 0.5)),
    ));
    if let Some(label) = data_label_text(column, index, *value) {
      let position = data_label_position(column, index);
      let mut label_style = appearance.data_label_style.clone();
      if !data_label_has_explicit_text_properties(column, index)
        && matches!(position, cx::DataLabelPos::InEnd)
        && let Some(color) = automatic_inside_data_label_color(column_color)
      {
        label_style.color = color;
        label_style.color_is_automatic = false;
      }
      push_centered_text(
        items,
        PlotRect {
          x,
          y: if matches!(position, cx::DataLabelPos::InEnd) {
            y + 2.0
          } else {
            y - appearance.data_label_style.font_size_pt * 1.15
          },
          width: column_width,
          height: appearance.data_label_style.font_size_pt * 1.2,
        },
        label,
        label_style,
      );
    }
    cumulative += *value;
    let point = (
      data_plot.x + slot * (index as f32 + 0.5),
      data_plot.y + data_plot.height * (1.0 - (cumulative / total.max(f64::EPSILON)) as f32),
    );
    if let Some(previous) = previous {
      push_line(
        items, previous.0, previous.1, point.0, point.1, line_color, 2.25,
      );
    }
    previous = Some(point);
  }
  lower_axes(
    items,
    data_plot,
    scale,
    &categories,
    chart_space,
    appearance,
    AxisPaintOptions {
      ui_language: None,
      pass: AxisPaintPass::Foreground,
      category_crossing: CategoryAxisCrossing::Zero,
    },
  );
  if let Some(line) = line {
    lower_pareto_percentage_axis(items, data_plot, line, chart_space, appearance);
  }
  legend_entries.push((series_legend_name(column, ui_language), column_color));
}

fn lower_pareto_percentage_axis(
  items: &mut Vec<PageItem>,
  data_plot: PlotRect,
  line: &SeriesModel<'_>,
  chart_space: &cx::ChartSpace,
  appearance: &Appearance,
) {
  let Some(axis) = pareto_value_axis(line, chart_space) else {
    return;
  };
  if axis.hidden.is_some_and(|hidden| hidden.as_bool()) {
    return;
  }
  push_line(
    items,
    data_plot.x + data_plot.width,
    data_plot.y,
    data_plot.x + data_plot.width,
    data_plot.y + data_plot.height,
    appearance.axis_color,
    appearance.axis_width,
  );
  if axis.tick_labels.is_none() {
    return;
  }
  let scaling = match axis.axis_choice.as_ref() {
    Some(cx::AxisChoice::ValueAxisScaling(scaling)) => Some(scaling.as_ref()),
    _ => None,
  };
  let minimum = scaling
    .and_then(|scaling| scaling.min.as_deref())
    .and_then(parse_axis_number)
    .unwrap_or(0.0);
  let maximum = scaling
    .and_then(|scaling| scaling.max.as_deref())
    .and_then(parse_axis_number)
    .unwrap_or(1.0);
  let automatic_interval_count = if data_plot.height < appearance.label_style.font_size_pt * 13.0 {
    5.0
  } else {
    10.0
  };
  let major = scaling
    .and_then(|scaling| scaling.major_unit.as_deref())
    .and_then(parse_axis_number)
    .filter(|major| *major > 0.0)
    .unwrap_or((maximum - minimum) / automatic_interval_count);
  let percentage =
    axis.axis_units.as_deref().and_then(|units| units.unit) == Some(cx::AxisUnit::Percentage);
  let mut value = minimum;
  let mut guard = 0;
  while value <= maximum + major * 0.001 && guard < 100 {
    let ratio = ((value - minimum) / (maximum - minimum).max(f64::EPSILON)) as f32;
    let y = data_plot.y + data_plot.height * (1.0 - ratio);
    let label = if percentage {
      format!("{}%", format_chart_number(value * 100.0, None))
    } else {
      format_chart_number(value, None)
    };
    push_text(
      items,
      data_plot.x + data_plot.width + 6.25,
      y - appearance.label_style.font_size_pt * 0.55,
      label,
      appearance.label_style.clone(),
    );
    value += major;
    guard += 1;
  }
}

fn pareto_value_axis<'a>(
  line: &SeriesModel<'_>,
  chart_space: &'a cx::ChartSpace,
) -> Option<&'a cx::Axis> {
  let axis_id = line.source.axis_id.first().and_then(|axis| axis.val)?;
  chart_space
    .chart
    .plot_area
    .axis
    .iter()
    .find(|axis| axis.id == axis_id)
}

fn lower_region_map(
  items: &mut Vec<PageItem>,
  plot: PlotRect,
  series: &SeriesModel<'_>,
  appearance: &Appearance,
  legend_entries: &mut Vec<(String, RgbColor)>,
) {
  let geography = series
    .source
    .series_layout_properties
    .as_deref()
    .and_then(|properties| properties.geography.as_deref());
  let clear = geography
    .and_then(|geography| geography.geo_cache.as_ref())
    .and_then(|cache| {
      cache
        .geo_cache_choice
        .iter()
        .find_map(|choice| match choice {
          cx::GeoCacheChoice::Clear(clear) => Some(clear.as_ref()),
          _ => None,
        })
    });
  let geo_data = clear
    .and_then(|clear| clear.geo_data_entity_query_results.as_ref())
    .map(|results| {
      results
        .geo_data_entity_query_result
        .iter()
        .filter_map(|result| result.geo_data.as_deref())
        .collect::<Vec<_>>()
    })
    .unwrap_or_default();
  let locations = clear
    .and_then(|clear| clear.geo_location_query_results.as_ref())
    .map(|results| {
      results
        .geo_location_query_result
        .iter()
        .filter_map(|result| result.geo_locations.as_deref()?.geo_location.as_deref())
        .collect::<Vec<_>>()
    })
    .unwrap_or_default();

  items.push(rect_item(
    plot,
    Some(tint(appearance.theme.light2, 0.45)),
    Some((appearance.axis_color, 0.55)),
  ));
  let values = series.values.iter().copied().flatten().collect::<Vec<_>>();
  let minimum = values.iter().copied().fold(f64::INFINITY, f64::min);
  let maximum = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
  let color_for_value = |value: f64| {
    let ratio = if maximum > minimum {
      ((value - minimum) / (maximum - minimum)) as f32
    } else {
      0.5
    };
    blend(
      tint(appearance.point_color(0, 1), 0.75),
      shade(appearance.point_color(0, 1), 0.25),
      ratio,
    )
  };

  if !geo_data.is_empty() {
    let west = geo_data
      .iter()
      .map(|data| data.west)
      .fold(f64::INFINITY, f64::min);
    let east = geo_data
      .iter()
      .map(|data| data.east)
      .fold(f64::NEG_INFINITY, f64::max);
    let south = geo_data
      .iter()
      .map(|data| data.south)
      .fold(f64::INFINITY, f64::min);
    let north = geo_data
      .iter()
      .map(|data| data.north)
      .fold(f64::NEG_INFINITY, f64::max);
    for (index, data) in geo_data.iter().enumerate() {
      let value_index = (0..series.count())
        .find(|index| series.leaf_category(*index) == data.entity_name)
        .unwrap_or(index.min(series.values.len().saturating_sub(1)));
      let value = series
        .values
        .get(value_index)
        .copied()
        .flatten()
        .unwrap_or(0.0);
      let x1 = longitude_x(plot, data.west, west, east);
      let x2 = longitude_x(plot, data.east, west, east);
      let y1 = latitude_y(plot, data.north, south, north);
      let y2 = latitude_y(plot, data.south, south, north);
      let bounds = PlotRect {
        x: x1.min(x2),
        y: y1.min(y2),
        width: (x2 - x1).abs().max(1.0),
        height: (y2 - y1).abs().max(1.0),
      };
      items.push(rect_item(
        bounds,
        Some(color_for_value(value)),
        Some((appearance.chart_fill, 0.45)),
      ));
      push_text_if_fits(
        items,
        bounds,
        data.entity_name.clone(),
        appearance.data_label_style.clone(),
        1.5,
      );
    }
  } else if !locations.is_empty() {
    for (index, location) in locations.iter().enumerate() {
      let (Some(latitude), Some(longitude)) = (location.latitude, location.longitude) else {
        continue;
      };
      let value = series.values.get(index).copied().flatten().unwrap_or(0.0);
      let x = longitude_x(plot, longitude, -180.0, 180.0);
      let y = latitude_y(plot, latitude, -85.0, 85.0);
      push_marker(items, x, y, 3.0, color_for_value(value));
      push_text(
        items,
        x + 4.0,
        y - appearance.data_label_style.font_size_pt * 0.5,
        location.entity_name.clone(),
        appearance.data_label_style.clone(),
      );
    }
  } else {
    // A binary geography cache cannot be decoded without the provider's
    // polygon payload.  Preserve the chart's quantitative and geographic
    // semantics as a tiled map key instead of misrepresenting it as a
    // treemap.
    let count = series.count().max(1);
    let columns = (count as f32).sqrt().ceil() as usize;
    let rows = count.div_ceil(columns);
    let width = plot.width / columns as f32;
    let height = plot.height / rows as f32;
    for index in 0..count {
      let value = series.values.get(index).copied().flatten().unwrap_or(0.0);
      let tile = PlotRect {
        x: plot.x + (index % columns) as f32 * width,
        y: plot.y + (index / columns) as f32 * height,
        width,
        height,
      };
      items.push(rect_item(
        tile,
        Some(color_for_value(value)),
        Some((appearance.chart_fill, 0.5)),
      ));
      push_text_if_fits(
        items,
        tile,
        series.leaf_category(index),
        appearance.data_label_style.clone(),
        2.0,
      );
    }
  }
  legend_entries.push((
    format_chart_number(minimum.finite_or_zero(), series.number_format.as_deref()),
    tint(appearance.point_color(0, 1), 0.75),
  ));
  legend_entries.push((
    format_chart_number(maximum.finite_or_zero(), series.number_format.as_deref()),
    shade(appearance.point_color(0, 1), 0.25),
  ));
}

trait FiniteOrZero {
  fn finite_or_zero(self) -> f64;
}

impl FiniteOrZero for f64 {
  fn finite_or_zero(self) -> f64 {
    if self.is_finite() { self } else { 0.0 }
  }
}

fn longitude_x(plot: PlotRect, longitude: f64, west: f64, east: f64) -> f32 {
  let span = (east - west).abs().max(f64::EPSILON);
  plot.x + ((longitude - west) / span).clamp(0.0, 1.0) as f32 * plot.width
}

fn latitude_y(plot: PlotRect, latitude: f64, south: f64, north: f64) -> f32 {
  let mercator = |latitude: f64| {
    let latitude = latitude.clamp(-85.0, 85.0).to_radians();
    (0.5 * ((1.0 + latitude.sin()) / (1.0 - latitude.sin())).ln()).clamp(-PI as f64, PI as f64)
  };
  let south = mercator(south);
  let north = mercator(north);
  let value = mercator(latitude);
  plot.y + ((north - value) / (north - south).abs().max(f64::EPSILON)) as f32 * plot.height
}

fn blend(left: RgbColor, right: RgbColor, ratio: f32) -> RgbColor {
  let ratio = ratio.clamp(0.0, 1.0);
  let channel = |left: u8, right: u8| {
    (f32::from(left) * (1.0 - ratio) + f32::from(right) * ratio).round() as u8
  };
  rgb(
    channel(left.r, right.r),
    channel(left.g, right.g),
    channel(left.b, right.b),
  )
}

fn reorder_chartex_text_items(
  items: &mut Vec<PageItem>,
  series: &[SeriesModel<'_>],
  chart_space: &cx::ChartSpace,
  title: Option<&str>,
  legend_entries: &[(String, RgbColor)],
  ui_language: Option<&str>,
) {
  let mut geometry = Vec::with_capacity(items.len());
  let mut pool = Vec::<TextItem>::new();
  for item in std::mem::take(items) {
    match item {
      PageItem::Text(text) => pool.push(text),
      item => geometry.push(item),
    }
  }
  if pool.is_empty() {
    *items = geometry;
    return;
  }

  // Reserve title and legend objects before matching duplicate category
  // labels (treemap top-level labels commonly repeat in the legend).
  let title_item = title.and_then(|title| take_matching_text(&mut pool, title, false));
  let mut legend_items = Vec::new();
  for (name, _) in legend_entries {
    if let Some(text) = take_matching_text(&mut pool, name, true) {
      legend_items.push(text);
    } else if let Some((prefix, ordinal)) = split_automatic_legend_name(name) {
      if let Some(text) = take_matching_text(&mut pool, prefix, true) {
        legend_items.push(text);
      }
      if let Some(text) = take_matching_text(&mut pool, ordinal, true) {
        legend_items.push(text);
      }
    }
  }
  let mut ordered = Vec::new();
  let primary = series.first().map(|series| series.layout);

  match primary {
    Some(cx::SeriesLayout::Waterfall) => {
      for series in series {
        for (index, value) in series.values.iter().copied().enumerate() {
          if let Some(label) = value.and_then(|value| data_label_text(series, index, value))
            && let Some(text) = take_matching_text(&mut pool, &label, true)
          {
            ordered.push(text);
          }
        }
      }
      take_category_axis_title(
        &mut pool,
        &mut ordered,
        category_axis(chart_space),
        ui_language,
        true,
      );
      take_series_categories(&mut pool, &mut ordered, &series[0]);
      take_category_axis_title(
        &mut pool,
        &mut ordered,
        value_axis(chart_space),
        ui_language,
        false,
      );
      take_numeric_texts(&mut pool, &mut ordered);
    }
    Some(cx::SeriesLayout::BoxWhisker) => {
      let has_data_labels = series
        .iter()
        .any(|series| series.source.data_labels.is_some());
      for series in series
        .iter()
        .filter(|series| series.source.data_labels.is_some())
      {
        let values = series.values.iter().copied().flatten().collect::<Vec<_>>();
        let method = series
          .source
          .series_layout_properties
          .as_deref()
          .and_then(|properties| properties.statistics.as_ref())
          .and_then(|statistics| statistics.quartile_method)
          .unwrap_or_default();
        let Some(summary) = box_summary(&values, method) else {
          continue;
        };
        let mut labels = values
          .iter()
          .copied()
          .enumerate()
          .filter_map(|(index, value)| {
            data_label_text(series, index, value).map(|text| (value, text))
          })
          .collect::<Vec<_>>();
        labels.sort_by(|left, right| left.0.total_cmp(&right.0));
        labels.dedup_by(|left, right| left.0 == right.0 && left.1 == right.1);
        for (_, label) in labels {
          if let Some(text) = take_matching_text(&mut pool, &label, false) {
            ordered.push(text);
          }
        }
        for value in [
          summary.q1,
          summary.median,
          summary.q3,
          box_whisker_mean_label_value(summary.mean),
        ] {
          if let Some(label) = inherited_data_label_text(series, value)
            && let Some(text) = take_matching_text(&mut pool, &label, false)
          {
            ordered.push(text);
          }
        }
      }
      let has_explicit_categories = series.iter().any(|series| !series.categories.is_empty());
      let mut categories = if has_explicit_categories {
        let mut categories = Vec::new();
        for series in series {
          for index in 0..series.count() {
            let category = series.leaf_category(index);
            if !categories.contains(&category) {
              categories.push(category);
            }
          }
        }
        categories
      } else {
        vec!["1".to_string()]
      };
      if categories.is_empty() {
        categories.push(String::new());
      }
      for category in categories {
        if let Some(text) = take_matching_text(&mut pool, &category, has_data_labels) {
          ordered.push(text);
        }
      }
      take_numeric_texts(&mut pool, &mut ordered);
    }
    Some(cx::SeriesLayout::Treemap) => {
      let tree = hierarchy(&series[0]);
      let mut names = Vec::new();
      for node in &tree.children {
        treemap_text_order(node, 0, &mut names);
      }
      for name in names {
        if let Some(text) = take_matching_text(&mut pool, &name, false) {
          ordered.push(text);
        }
      }
    }
    Some(cx::SeriesLayout::Sunburst) => {}
    Some(cx::SeriesLayout::Funnel) => {
      for series in series {
        for index in 0..series.count() {
          let value = series.values.get(index).copied().flatten().unwrap_or(0.0);
          let label = data_label_text(series, index, value).unwrap_or_else(|| {
            format!(
              "{} {}",
              series.leaf_category(index),
              format_chart_number(value, series.number_format.as_deref())
            )
          });
          if let Some(text) = take_matching_text(&mut pool, &label, true) {
            ordered.push(text);
          }
        }
      }
    }
    Some(cx::SeriesLayout::ClusteredColumn | cx::SeriesLayout::ParetoLine) => {
      let has_pareto = series
        .iter()
        .any(|series| series.layout == cx::SeriesLayout::ParetoLine);
      let column = series
        .iter()
        .find(|series| series.layout == cx::SeriesLayout::ClusteredColumn)
        .unwrap_or(&series[0]);
      let binning = column
        .source
        .series_layout_properties
        .as_deref()
        .and_then(|properties| properties.series_layout_properties_choice.as_ref())
        .and_then(|choice| match choice {
          cx::SeriesLayoutPropertiesChoice::Binning(binning) => Some(binning.as_ref()),
          _ => None,
        });
      if let Some(binning) = binning {
        let bins = histogram_bins(
          &column.values.iter().copied().flatten().collect::<Vec<_>>(),
          Some(binning),
        );
        let mut categories = histogram_bin_labels(&bins, Some(binning));
        if has_pareto {
          let mut paired = categories
            .into_iter()
            .zip(bins.iter().map(|bin| bin.count))
            .collect::<Vec<_>>();
          paired.sort_by_key(|item| std::cmp::Reverse(item.1));
          for (index, (_, count)) in paired.iter().enumerate() {
            if let Some(label) = data_label_text(column, index, *count as f64)
              && let Some(text) = take_matching_text(&mut pool, &label, false)
            {
              ordered.push(text);
            }
          }
          categories = paired.into_iter().map(|(category, _)| category).collect();
        }
        for category in categories {
          if let Some(text) = take_matching_text(&mut pool, &category, false) {
            ordered.push(text);
          }
        }
      } else {
        for series in series {
          for (index, value) in series.values.iter().copied().enumerate() {
            if let Some(label) = value.and_then(|value| data_label_text(series, index, value))
              && let Some(text) = take_matching_text(&mut pool, &label, true)
            {
              ordered.push(text);
            }
          }
        }
        take_series_categories(&mut pool, &mut ordered, &series[0]);
      }
      take_numeric_texts(&mut pool, &mut ordered);
    }
    Some(cx::SeriesLayout::RegionMap) | None => {}
  }

  // Any authored text not covered by the automatic order remains visible and
  // keeps its source order.  This includes unusual per-point overrides.
  ordered.append(&mut pool);
  if let Some(title) = title_item {
    ordered.push(title);
  }
  ordered.extend(legend_items);
  geometry.extend(ordered.into_iter().map(PageItem::Text));
  *items = geometry;
}

fn take_matching_text(
  pool: &mut Vec<TextItem>,
  target: &str,
  prefer_last: bool,
) -> Option<TextItem> {
  let index = if prefer_last {
    pool.iter().rposition(|text| text.text == target)
  } else {
    pool.iter().position(|text| text.text == target)
  }?;
  Some(pool.remove(index))
}

fn take_category_axis_title(
  pool: &mut Vec<TextItem>,
  ordered: &mut Vec<TextItem>,
  axis: Option<&cx::Axis>,
  ui_language: Option<&str>,
  prefer_last: bool,
) {
  let Some(title) = axis_title(
    axis.and_then(|axis| axis.axis_title.as_deref()),
    ui_language,
  ) else {
    return;
  };
  if let Some(text) = take_matching_text(pool, &title, prefer_last) {
    ordered.push(text);
  }
}

fn take_series_categories(
  pool: &mut Vec<TextItem>,
  ordered: &mut Vec<TextItem>,
  series: &SeriesModel<'_>,
) {
  for index in 0..series.count() {
    let category = series.leaf_category(index);
    if let Some(text) = take_matching_text(pool, &category, false) {
      ordered.push(text);
    }
  }
}

fn take_numeric_texts(pool: &mut Vec<TextItem>, ordered: &mut Vec<TextItem>) {
  let mut numeric = Vec::<(f64, usize, TextItem)>::new();
  let mut remainder = Vec::new();
  for (index, text) in std::mem::take(pool).into_iter().enumerate() {
    if let Some(value) = parse_display_number(&text.text) {
      numeric.push((value, index, text));
    } else {
      remainder.push(text);
    }
  }
  numeric.sort_by(|left, right| {
    left
      .0
      .total_cmp(&right.0)
      .then_with(|| left.1.cmp(&right.1))
  });
  ordered.extend(numeric.into_iter().map(|(_, _, text)| text));
  *pool = remainder;
}

fn parse_display_number(value: &str) -> Option<f64> {
  let value = value.trim().replace(',', "");
  value
    .strip_suffix('%')
    .unwrap_or(&value)
    .parse::<f64>()
    .ok()
}

fn treemap_text_order(node: &HierarchyNode, depth: usize, output: &mut Vec<String>) {
  if depth == 0 || node.children.is_empty() {
    output.push(node.name.clone());
  }
  for child in &node.children {
    treemap_text_order(child, depth + 1, output);
  }
}

fn resolved_common_gradient(
  fill: &a::GradientFill,
  theme: ChartExTheme,
  placeholder: Option<RgbColor>,
  bounds: PlotRect,
  interpolation: crate::common::GradientInterpolation,
) -> Option<crate::common::GradientFill<'static>> {
  let placeholder = placeholder.map(|color| {
    Color::RgbHex(RgbHexColor {
      value: format!("{:02X}{:02X}{:02X}", color.r, color.g, color.b),
      transformations: Vec::new(),
    })
  });
  let mut stops = fill
    .gradient_stop_list
    .as_ref()?
    .gradient_stop
    .iter()
    .filter_map(|stop| {
      let color = Color::from_gradient_stop_choice(stop.gradient_stop_choice.as_ref()?)?;
      let mut resolver = |value| theme.drawing_color(value);
      let color = color.resolve_rgb(&mut resolver, placeholder.as_ref())?;
      Some(crate::common::GradientStop {
        position: stop.position.as_ratio() as f32,
        color: crate::common::Color {
          r: color.r,
          g: color.g,
          b: color.b,
          a: ((color.alpha.clamp(0, 100_000) as f32 / 100_000.0) * 255.0).round() as u8,
        },
        scheme: None,
      })
    })
    .collect::<Vec<_>>();
  stops.sort_by(|left, right| left.position.total_cmp(&right.position));
  if stops.is_empty() {
    return None;
  }
  let definition_bounds = common_rect(bounds.x, bounds.y, bounds.width, bounds.height);
  let (angle_degrees, scaled, path) = match fill.gradient_fill_choice.as_ref()? {
    a::GradientFillChoice::LinearGradientFill(linear) => (
      Some(linear.angle.unwrap_or_default() as f32 / 60_000.0),
      linear.scaled.as_ref().is_some_and(|value| value.as_bool()),
      None,
    ),
    a::GradientFillChoice::PathGradientFill(path) => {
      let mut path = crate::common::drawingml_gradient::resolve_path_gradient(
        fill,
        path,
        crate::common::Transform {
          m11: bounds.width,
          m12: 0.0,
          m21: 0.0,
          m22: bounds.height,
          dx: crate::common::Pt(bounds.x),
          dy: crate::common::Pt(bounds.y),
        },
      );
      if path.kind == crate::common::GradientPathKind::Circle {
        path.transform = crate::common::office_circle_gradient_transform(path.transform);
      }
      (None, false, Some(path))
    }
  };
  Some(crate::common::GradientFill {
    stops,
    angle_degrees,
    definition_bounds: Some(definition_bounds),
    line: None,
    interpolation,
    scaled,
    rotate_with_shape: None,
    path,
  })
}

#[allow(clippy::too_many_arguments)]
fn push_gradient_or_solid_rect(
  items: &mut Vec<PageItem>,
  bounds: PlotRect,
  fallback: RgbColor,
  gradient: Option<&a::GradientFill>,
  theme: ChartExTheme,
  placeholder: Option<RgbColor>,
  interpolation: crate::common::GradientInterpolation,
  stroke: Option<(RgbColor, f32)>,
) {
  let Some(gradient) = gradient
    .and_then(|fill| resolved_common_gradient(fill, theme, placeholder, bounds, interpolation))
  else {
    items.push(rect(
      bounds.x,
      bounds.y,
      bounds.width,
      bounds.height,
      Some(fallback),
      stroke,
    ));
    return;
  };
  items.push(PageItem::Path(crate::common::PathItem {
    bounds: common_rect(bounds.x, bounds.y, bounds.width, bounds.height),
    points: vec![
      common_point(bounds.x, bounds.y),
      common_point(bounds.x + bounds.width, bounds.y),
      common_point(bounds.x + bounds.width, bounds.y + bounds.height),
      common_point(bounds.x, bounds.y + bounds.height),
    ],
    commands: Vec::new(),
    closed: true,
    fill: crate::common::Fill::Gradient(gradient),
    stroke: stroke.map(|(color, width)| crate::common::Stroke {
      width: crate::common::Pt(width),
      color: common_rgb(color, 1.0),
      ..Default::default()
    }),
  }));
}

fn push_data_point_rect(
  items: &mut Vec<PageItem>,
  bounds: PlotRect,
  color: RgbColor,
  appearance: &Appearance,
  effect_clip: Option<PlotRect>,
  stroke: Option<(RgbColor, f32)>,
) {
  if let Some(source) = appearance.data_point_effects.as_ref()
    && let Some(effects) = resolve_chart_effects(source, appearance.theme, Some(color))
    && let Some(backdrop) =
      crate::common::drawingml_image_effects::unchanged_foreground_backdrop(&effects)
    && let Some(image) = data_point_effect_backdrop(bounds, backdrop, effect_clip)
  {
    items.push(PageItem::Image(image));
  }
  let stroke = stroke.or_else(|| appearance.data_point_stroke(color));
  push_gradient_or_solid_rect(
    items,
    bounds,
    color,
    appearance.data_point_gradient.as_ref(),
    appearance.theme,
    Some(color),
    // Office writes ChartEx data-point gradients as ordinary piecewise-linear
    // PDF functions. Its fixed-format writer mirrors the authored stop
    // sequence over a two-shape-height brush, but the half clipped by the
    // point is exactly the original sequence.
    crate::common::GradientInterpolation::LinearSrgb,
    stroke,
  );
}

fn data_point_effect_backdrop(
  bounds: PlotRect,
  mut effects: crate::common::drawingml_image_effects::ImageEffectContainer,
  clip: Option<PlotRect>,
) -> Option<ImageItem> {
  let output = crate::common::drawingml_image_effects::container_output_bounds(
    &effects,
    bounds.width,
    bounds.height,
  )?;
  // Office's fixed-output ChartEx shape-effect XObjects are 100 DPI (the
  // Of16-05 bars retain that density across seven different aspect ratios).
  const OFFICE_CHART_EFFECT_PIXELS_PER_POINT: f32 = 100.0 / 72.0;
  const OFFICE_CHART_EFFECT_CANVAS_PADDING_PX: f32 = 2.0;
  const OFFICE_CHART_OUTER_SHADOW_FILTER_SCALE: f32 = 2.0 / 3.0;
  let canvas_padding_pt =
    OFFICE_CHART_EFFECT_CANVAS_PADDING_PX / OFFICE_CHART_EFFECT_PIXELS_PER_POINT;
  let relative_left = output.left_pt.min(0.0) - canvas_padding_pt;
  let relative_top = output.top_pt.min(0.0) - canvas_padding_pt;
  let relative_right = output.right_pt.max(bounds.width) + canvas_padding_pt;
  let relative_bottom = output.bottom_pt.max(bounds.height) + canvas_padding_pt;
  if ![relative_left, relative_top, relative_right, relative_bottom]
    .iter()
    .all(|value| value.is_finite())
  {
    return None;
  }
  let raster_bounds = common_rect(
    bounds.x + relative_left,
    bounds.y + relative_top,
    relative_right - relative_left,
    relative_bottom - relative_top,
  );
  if raster_bounds.size.width.0 <= f32::EPSILON || raster_bounds.size.height.0 <= f32::EPSILON {
    return None;
  }
  let source = crate::common::DisplayItem::Rect(crate::common::RectItem {
    bounds: common_rect(bounds.x, bounds.y, bounds.width, bounds.height),
    fill: crate::common::Fill::Solid(crate::common::Color {
      r: 255,
      g: 255,
      b: 255,
      a: 255,
    }),
    stroke: None,
  });
  crate::common::drawingml_image_effects::scale_outer_shadow_filter_radius(
    &mut effects,
    OFFICE_CHART_OUTER_SHADOW_FILTER_SCALE,
  );
  let mut raster =
    crate::common::drawingml_shape_raster::rasterize_vector_items_for_effects_at_pixels_per_point(
      std::slice::from_ref(&source),
      raster_bounds,
      &effects,
      OFFICE_CHART_EFFECT_PIXELS_PER_POINT,
    )?;
  crate::common::drawingml_image_effects::scale_container_pixel_lengths(
    &mut effects,
    raster.pixels_per_point / (96.0 / 72.0),
  );
  crate::common::drawingml_image_effects::apply_container_to_padded_image_with_sources(
    &mut raster.image,
    &effects,
    -relative_left * raster.pixels_per_point,
    -relative_top * raster.pixels_per_point,
    bounds.width * raster.pixels_per_point,
    bounds.height * raster.pixels_per_point,
    crate::common::drawingml_image_effects::ImageEffectSourceImages {
      fill: raster.fill_image.as_ref(),
      line: raster.line_image.as_ref(),
      fill_line: raster.fill_line_image.as_ref(),
      children: raster.children_image.as_ref(),
    },
  );
  let mut png = Cursor::new(Vec::new());
  PngEncoder::new(&mut png)
    .write_image(
      raster.image.as_raw(),
      raster.image.width(),
      raster.image.height(),
      ColorType::Rgba8.into(),
    )
    .ok()?;
  Some(ImageItem {
    x_pt: raster_bounds.origin.x.0,
    y_pt: raster_bounds.origin.y.0,
    width_pt: raster_bounds.size.width.0,
    height_pt: raster_bounds.size.height.0,
    crop: ImageCrop::default(),
    clip_path: clip.map_or_else(Vec::new, |clip| {
      vec![
        crate::common::PathCommand::MoveTo(common_point(clip.x, clip.y)),
        crate::common::PathCommand::LineTo(common_point(clip.x + clip.width, clip.y)),
        crate::common::PathCommand::LineTo(common_point(clip.x + clip.width, clip.y + clip.height)),
        crate::common::PathCommand::LineTo(common_point(clip.x, clip.y + clip.height)),
        crate::common::PathCommand::Close,
      ]
    }),
    rotation_deg: 0.0,
    flip_horizontal: false,
    flip_vertical: false,
    data: Bytes::from(png.into_inner()),
    content_type: Some("image/png".to_string()),
    metafile_monochrome_dib_palette_override: None,
    metafile_background_color: None,
    metafile_external_header: None,
    metafile_semantic_text_includes_raster_backdrop: false,
    alt_text: None,
    hyperlink_url: None,
    floating: false,
    behind_text: false,
  })
}

fn rect(
  x: f32,
  y: f32,
  width: f32,
  height: f32,
  fill: Option<RgbColor>,
  stroke: Option<(RgbColor, f32)>,
) -> PageItem {
  PageItem::Rect(RectItem {
    x_pt: x,
    y_pt: y,
    width_pt: width,
    height_pt: height,
    fill_color: fill,
    fill_opacity: 1.0,
    stroke: stroke.map(|(color, width_pt)| BorderStyle {
      width_pt,
      color,
      ..BorderStyle::default()
    }),
    stroke_opacity: 1.0,
  })
}

fn rect_item(rect: PlotRect, fill: Option<RgbColor>, stroke: Option<(RgbColor, f32)>) -> PageItem {
  self::rect(rect.x, rect.y, rect.width, rect.height, fill, stroke)
}

fn push_line(
  items: &mut Vec<PageItem>,
  x1: f32,
  y1: f32,
  x2: f32,
  y2: f32,
  color: RgbColor,
  width: f32,
) {
  items.push(PageItem::Line(LineItem {
    x1_pt: x1,
    y1_pt: y1,
    x2_pt: x2,
    y2_pt: y2,
    width_pt: width,
    color,
    kind: LineItemKind::Stroke,
  }));
}

fn push_polygon(
  items: &mut Vec<PageItem>,
  points: &[(f32, f32)],
  fill: RgbColor,
  stroke: Option<(RgbColor, f32)>,
) {
  if points.len() < 3 {
    return;
  }
  let minimum_x = points
    .iter()
    .map(|point| point.0)
    .fold(f32::INFINITY, f32::min);
  let maximum_x = points
    .iter()
    .map(|point| point.0)
    .fold(f32::NEG_INFINITY, f32::max);
  let minimum_y = points
    .iter()
    .map(|point| point.1)
    .fold(f32::INFINITY, f32::min);
  let maximum_y = points
    .iter()
    .map(|point| point.1)
    .fold(f32::NEG_INFINITY, f32::max);
  items.push(PageItem::Path(crate::common::PathItem {
    bounds: common_rect(
      minimum_x,
      minimum_y,
      maximum_x - minimum_x,
      maximum_y - minimum_y,
    ),
    points: points.iter().map(|(x, y)| common_point(*x, *y)).collect(),
    commands: Vec::new(),
    closed: true,
    fill: crate::common::Fill::Solid(common_rgb(fill, 1.0)),
    stroke: stroke.map(|(color, width)| crate::common::Stroke {
      width: crate::common::Pt(width),
      color: common_rgb(color, 1.0),
      dash: None,
      source_style_id: None,
      ..Default::default()
    }),
  }));
}

#[allow(clippy::too_many_arguments)]
fn push_annular_sector(
  items: &mut Vec<PageItem>,
  center: (f32, f32),
  inner: f32,
  outer: f32,
  start: f32,
  end: f32,
  fill: RgbColor,
  stroke: (RgbColor, f32),
) {
  let outer_start = (
    center.0 + start.cos() * outer,
    center.1 + start.sin() * outer,
  );
  let mut commands = vec![crate::common::PathCommand::MoveTo(common_point(
    outer_start.0,
    outer_start.1,
  ))];
  push_circular_arc_commands(&mut commands, center, outer, start, end);
  if inner <= 0.01 {
    commands.push(crate::common::PathCommand::LineTo(common_point(
      center.0, center.1,
    )));
  } else {
    let inner_end = (center.0 + end.cos() * inner, center.1 + end.sin() * inner);
    commands.push(crate::common::PathCommand::LineTo(common_point(
      inner_end.0,
      inner_end.1,
    )));
    push_circular_arc_commands(&mut commands, center, inner, end, start);
  }
  commands.push(crate::common::PathCommand::Close);
  items.push(PageItem::Path(crate::common::PathItem {
    bounds: common_rect(center.0 - outer, center.1 - outer, outer * 2.0, outer * 2.0),
    points: Vec::new(),
    commands,
    closed: true,
    fill: crate::common::Fill::Solid(common_rgb(fill, 1.0)),
    stroke: Some(crate::common::Stroke {
      width: crate::common::Pt(stroke.1),
      color: common_rgb(stroke.0, 1.0),
      dash: None,
      source_style_id: None,
      ..Default::default()
    }),
  }));
}

fn push_circular_arc_commands(
  commands: &mut Vec<crate::common::PathCommand>,
  center: (f32, f32),
  radius: f32,
  start: f32,
  end: f32,
) {
  let sweep = end - start;
  let segments = ((sweep.abs() / (PI * 0.5)).ceil() as usize).max(1);
  let step = sweep / segments as f32;
  for segment in 0..segments {
    let angle0 = start + step * segment as f32;
    let angle1 = angle0 + step;
    let tangent = (step * 0.25).tan() * (4.0 / 3.0) * radius;
    let control1 = (
      center.0 + angle0.cos() * radius - angle0.sin() * tangent,
      center.1 + angle0.sin() * radius + angle0.cos() * tangent,
    );
    let control2 = (
      center.0 + angle1.cos() * radius + angle1.sin() * tangent,
      center.1 + angle1.sin() * radius - angle1.cos() * tangent,
    );
    let end = (
      center.0 + angle1.cos() * radius,
      center.1 + angle1.sin() * radius,
    );
    commands.push(crate::common::PathCommand::CubicTo {
      control1: common_point(control1.0, control1.1),
      control2: common_point(control2.0, control2.1),
      end: common_point(end.0, end.1),
    });
  }
}

fn push_centered_rotated_text(
  items: &mut Vec<PageItem>,
  center: (f32, f32),
  width: f32,
  text: String,
  style: TextStyle,
) {
  if text.is_empty() {
    return;
  }
  let line_height = style.font_size_pt * 1.2;
  items.push(PageItem::Text(TextItem {
    x_pt: center.0 - width * 0.5,
    y_pt: center.1 - line_height * 0.5,
    line_height_pt: line_height,
    paint_clip: None,
    discard_if_horizontally_clipped: false,
    text,
    style: Box::new(style),
    rotation_center_pt: Some(center),
    hyperlink_url: None,
    form_widget_id: None,
    paragraph_bidi: false,
    preserve_text_portion: true,
    pdf_text_segmentation: PdfTextSegmentation::Line,
    source_path: Vec::new(),
  }));
}

fn push_marker(items: &mut Vec<PageItem>, x: f32, y: f32, radius: f32, color: RgbColor) {
  let points = (0..12)
    .map(|index| {
      let angle = TAU * index as f32 / 12.0;
      (x + angle.cos() * radius, y + angle.sin() * radius)
    })
    .collect::<Vec<_>>();
  push_polygon(items, &points, color, None);
}

fn push_text(items: &mut Vec<PageItem>, x: f32, y: f32, text: String, style: TextStyle) {
  if text.is_empty() {
    return;
  }
  items.push(PageItem::Text(TextItem {
    x_pt: x,
    y_pt: y,
    line_height_pt: style.font_size_pt * 1.2,
    paint_clip: None,
    discard_if_horizontally_clipped: false,
    text,
    style: Box::new(style),
    rotation_center_pt: None,
    hyperlink_url: None,
    form_widget_id: None,
    paragraph_bidi: false,
    preserve_text_portion: true,
    pdf_text_segmentation: PdfTextSegmentation::Line,
    source_path: Vec::new(),
  }));
}

fn text_width(text: &str, style: &TextStyle) -> f32 {
  text
    .chars()
    .map(|character| {
      let em = if character.is_ascii_digit() {
        0.507
      } else if character.is_ascii_uppercase() {
        0.60
      } else if character.is_ascii_lowercase() {
        0.43
      } else if character.is_ascii_whitespace() {
        0.23
      } else if character.is_ascii_punctuation() {
        0.31
      } else {
        1.0
      };
      style.font_size_pt * em
    })
    .sum()
}

fn push_centered_text(items: &mut Vec<PageItem>, rect: PlotRect, text: String, style: TextStyle) {
  let width = text_width(&text, &style);
  push_text(
    items,
    rect.x + (rect.width - width) * 0.5,
    rect.y + (rect.height - style.font_size_pt * 1.2) * 0.5,
    text,
    style,
  );
}

fn push_centered_measured_text(
  items: &mut Vec<PageItem>,
  rect: PlotRect,
  text: String,
  style: TextStyle,
) {
  let width = TextMetrics::new().measure_text(&text, &style);
  push_text(
    items,
    rect.x + (rect.width - width) * 0.5,
    rect.y + (rect.height - style.font_size_pt * 1.2) * 0.5,
    text,
    style,
  );
}

fn push_right_aligned_text(
  items: &mut Vec<PageItem>,
  right: f32,
  y: f32,
  text: String,
  style: TextStyle,
) {
  let width = text_width(&text, &style);
  push_text(items, right - width, y, text, style);
}

fn push_text_if_fits(
  items: &mut Vec<PageItem>,
  rect: PlotRect,
  text: String,
  style: TextStyle,
  inset: f32,
) {
  if rect.width < text_width(&text, &style) + inset * 2.0
    || rect.height < style.font_size_pt * 1.15 + inset * 2.0
  {
    return;
  }
  push_text(items, rect.x + inset, rect.y + inset, text, style);
}

#[cfg(test)]
mod tests {
  use super::*;

  fn test_series<'a>(
    source: &'a cx::Series,
    values: Vec<Option<f64>>,
    categories: Vec<SparseLevel<String>>,
  ) -> SeriesModel<'a> {
    SeriesModel {
      source,
      source_index: 0,
      name: "Series 1".to_string(),
      automatic_name: false,
      layout: source.layout_id,
      values,
      number_format: Some("General".to_string()),
      categories,
    }
  }

  #[test]
  fn sparse_cache_indices_are_not_compacted() {
    let points = sparse_points(5, [(1, Some(10)), (4, Some(40))]);
    assert_eq!(points, vec![None, Some(10), None, None, Some(40)]);
  }

  #[test]
  fn formula_direction_preserves_rows_and_columns() {
    let matrix = FormulaMatrix {
      rows: vec![vec!["a".into(), "b".into()], vec!["c".into(), "d".into()]],
    };
    assert_eq!(
      matrix.levels(cx::FormulaDirection::Col),
      vec![vec!["a", "c"], vec!["b", "d"]]
    );
    assert_eq!(
      matrix.levels(cx::FormulaDirection::Row),
      vec![vec!["a", "b"], vec!["c", "d"]]
    );
  }

  #[test]
  fn waterfall_subtotals_start_at_zero_and_reset_the_running_total() {
    let mut source = cx::Series {
      layout_id: cx::SeriesLayout::Waterfall,
      ..cx::Series::default()
    };
    source.series_layout_properties = Some(Box::new(cx::SeriesLayoutProperties {
      subtotals: Some(cx::Subtotals {
        unsigned_integer_type: [0_u32, 4, 7]
          .into_iter()
          .map(|val| cx::UnsignedIntegerType { val })
          .collect(),
      }),
      ..cx::SeriesLayoutProperties::default()
    }));
    let series = test_series(
      &source,
      [100.0, 20.0, 50.0, -40.0, 130.0, -60.0, 70.0, 140.0]
        .into_iter()
        .map(Some)
        .collect(),
      Vec::new(),
    );
    let bars = waterfall_bars(&series)
      .into_iter()
      .map(Option::unwrap)
      .collect::<Vec<_>>();
    assert_eq!(
      bars.iter().map(|bar| bar.end).collect::<Vec<_>>(),
      vec![100.0, 120.0, 170.0, 130.0, 130.0, 70.0, 140.0, 140.0]
    );
    assert_eq!(bars[4].start, 0.0);
    assert!(bars[4].subtotal);
  }

  #[test]
  fn waterfall_linear_color_roles_match_office_scrgb_variations() {
    let accent = rgb(0x5B, 0x9B, 0xD5);
    assert_eq!(
      linear_point_color(accent, 0, WATERFALL_COLOR_FORMAT_COUNT, false),
      rgb(0x46, 0x79, 0xA7)
    );
    assert_eq!(
      linear_point_color(accent, 1, WATERFALL_COLOR_FORMAT_COUNT, false),
      rgb(0x55, 0x91, 0xC7)
    );
    assert_eq!(
      linear_point_color(accent, 2, WATERFALL_COLOR_FORMAT_COUNT, false),
      rgb(0x84, 0xAE, 0xDC)
    );
    assert_eq!(
      linear_point_color(accent, 1, WATERFALL_COLOR_FORMAT_COUNT, true),
      linear_point_color(accent, 2, WATERFALL_COLOR_FORMAT_COUNT, false)
    );
  }

  #[test]
  fn inclusive_and_exclusive_quartiles_follow_excel_rank_rules() {
    let values = (1..=9).map(f64::from).collect::<Vec<_>>();
    let inclusive = box_summary(&values, cx::QuartileMethod::Inclusive).unwrap();
    let exclusive = box_summary(&values, cx::QuartileMethod::Exclusive).unwrap();
    assert_eq!(
      (inclusive.q1, inclusive.median, inclusive.q3),
      (3.0, 5.0, 7.0)
    );
    assert_eq!(
      (exclusive.q1, exclusive.median, exclusive.q3),
      (2.5, 5.0, 7.5)
    );
  }

  #[test]
  fn hierarchy_reverses_cached_leaf_to_root_levels_and_skips_blanks() {
    let source = cx::Series {
      layout_id: cx::SeriesLayout::Sunburst,
      ..cx::Series::default()
    };
    let series = test_series(
      &source,
      vec![Some(10.0), Some(5.0)],
      vec![
        SparseLevel {
          name: None,
          format_code: None,
          points: vec![Some("Leaf 1".into()), Some(String::new())],
        },
        SparseLevel {
          name: None,
          format_code: None,
          points: vec![Some("Stem 1".into()), Some("Leaf 2".into())],
        },
        SparseLevel {
          name: None,
          format_code: None,
          points: vec![Some("Branch 1".into()), Some("Branch 1".into())],
        },
      ],
    );
    assert_eq!(
      series.category_path(0),
      vec!["Branch 1", "Stem 1", "Leaf 1"]
    );
    assert_eq!(series.category_path(1), vec!["Branch 1", "Leaf 2"]);
    let tree = hierarchy(&series);
    assert_eq!(tree.children[0].value, 15.0);
    assert_eq!(tree.depth(), 3);
  }

  #[test]
  fn squarify_preserves_total_area() {
    let nodes = [6.0, 3.0, 1.0]
      .into_iter()
      .enumerate()
      .map(|(index, value)| HierarchyNode {
        name: index.to_string(),
        value,
        ..HierarchyNode::default()
      })
      .collect::<Vec<_>>();
    let tiles = squarify(
      &nodes,
      PlotRect {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 50.0,
      },
    );
    let area = tiles
      .iter()
      .map(|tile| tile.rect.width * tile.rect.height)
      .sum::<f32>();
    assert!((area - 5_000.0).abs() < 0.5);
    assert!(
      tiles
        .iter()
        .all(|tile| tile.rect.width > 0.0 && tile.rect.height > 0.0)
    );
  }

  #[test]
  fn histogram_fixed_count_uses_left_open_right_closed_bins() {
    let binning = cx::Binning {
      binning_choice: Some(cx::BinningChoice::BinCountXsdunsignedInt(
        cx::BinCountXsdunsignedInt {
          val: Some(2),
          xml_content: None,
        },
      )),
      ..cx::Binning::default()
    };
    let bins = histogram_bins(&[0.0, 1.0, 2.0, 3.0], Some(&binning));
    assert_eq!(bins.len(), 2);
    assert_eq!(
      bins.iter().map(|bin| bin.count).collect::<Vec<_>>(),
      vec![2, 2]
    );
  }

  #[test]
  fn axis_scale_matches_office_box_whisker_tick_extent() {
    let scale = axis_scale([-78.0, 128.0], None);
    assert_eq!(
      (scale.minimum, scale.maximum, scale.major),
      (-100.0, 150.0, 50.0)
    );
  }

  #[test]
  fn axis_title_offset_converts_signed_inches_to_points() {
    let title = cx::AxisTitle {
      offset: Some(cx::Offset {
        left: -0.25,
        top: 0.5,
      }),
      ..cx::AxisTitle::default()
    };
    assert_eq!(axis_title_offset_points(Some(&title)), (-18.0, 36.0));
    assert_eq!(axis_title_offset_points(None), (0.0, 0.0));
  }
}
