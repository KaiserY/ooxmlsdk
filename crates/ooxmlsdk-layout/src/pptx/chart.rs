use crate::model::{
  BorderStyle, LineItem, LineItemKind, PageItem, PdfTextSegmentation, RectItem, RgbColor, TextItem,
  TextStyle, common_point, common_rect, common_rgb,
};
use crate::render::chart::{
  Chart3DView, ChartCategoryTick, ChartErrorBarValues, ChartLegendPosition, ChartSeriesGrouping,
  ChartSeriesKind, ChartTitleText, ClusteredColumnChart, LinearAxisScaleOptions, PieChartModel,
  RadialChartKind, SurfaceChartGroup, axis_interval_count, clustered_column_slot,
  date_axis_data_position, date_axis_minor_tick_positions_with_maximum_auto_increment_count,
  date_axis_ticks, date_axis_ticks_with_maximum_auto_increment_count,
  horizontal_axis_number_format_code, linear_axis_scale_with_options, trendline_legend_title,
  value_axis_display_unit, value_axis_display_unit_label_text, vertical_axis_number_format_code,
};
use crate::text_metrics::TextMetrics;
use kurbo::BezPath;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_chart as c;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use ooxmlsdk_fonts::{FontSize, TextScript, script_direction_runs};
use std::{borrow::Cow, sync::Arc};

use crate::render::chart_layout_profiles as profiles;

use crate::common::drawingml_geometry::bez_path_commands;

const TEXT_LINE_HEIGHT_SCALE: f32 = 1.2;
// ECMA-376 Part 1 §20.1.7.1 DrawingML text-body schema defaults.
const DRAWINGML_DEFAULT_HORIZONTAL_TEXT_BODY_INSET_EMU: i64 = 91_440;
const DRAWINGML_DEFAULT_VERTICAL_TEXT_BODY_INSET_EMU: i64 = 45_720;
// LibreOffice chart2 `AreaChart.cxx` passes a fixed 100 mm100 offset to
// `createDataLabel` after moving a top label above the marker.
const CARTESIAN_DATA_LABEL_OFFSET_PT: f32 = 72.0 / 25.4;
// LibreOffice raises the corresponding 3-D bar/column offset to 260 mm100,
// but Microsoft Office fixed output retains the ordinary 100 mm100 clearance
// after transforming the complete 3-D anchor. Keeping the LO value here puts
// every outside label exactly 1.6 mm too far from its marker.
const BAR_3D_DATA_LABEL_OFFSET_PT: f32 = 72.0 / 25.4;
// LibreOffice chart2/source/view/charttypes/BubbleChart.cxx sizes the largest
// bubble to one quarter of the smaller final plot extent. ECMA-376
// §21.2.2.21 then applies c:bubbleScale as 0..=300% of that default size.
const MAXIMUM_BUBBLE_DIAMETER_RATIO: f32 = 0.25;
// Word's legacy chart fixed-output pipeline quantizes data-marker edges to
// 1/600 inch. Axis strokes retain their higher-precision automatic-layout
// coordinates, so this grid belongs to series geometry rather than PlotRect.
const WORD_FIXED_CHART_DATA_EDGE_GRID_PT: f32 = 72.0 / 600.0;

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
  pub chart_style_id: u8,
  pub modern_excel_profile: bool,
  pub stroke_scale: f32,
  pub automatic_line_width_pt: f32,
  pub has_explicit_title: bool,
  pub title_top_adjustment_ratio: f32,
  pub title: TextStyle,
  pub title_fill_color: Option<RgbColor>,
  pub label: TextStyle,
  pub legend: TextStyle,
  pub category_axis_title: TextStyle,
  pub value_axis_title: TextStyle,
  pub additional_axis_titles: Vec<TextStyle>,
  pub category_label: TextStyle,
  pub value_label: TextStyle,
  pub series_label: TextStyle,
  pub data_label: TextStyle,
  pub data_label_styles: Vec<Vec<Option<TextStyle>>>,
  /// Host-resolved styles for each retained custom data-label DrawingML run,
  /// indexed as series -> label -> run.
  pub data_label_rich_text_styles: Vec<Vec<Vec<TextStyle>>>,
  pub gridline_color: RgbColor,
  pub value_gridline_width_pt: Option<f32>,
  pub axis_line_width_pt: Option<f32>,
  pub category_major_gridline: Option<(RgbColor, f32)>,
  pub category_minor_gridline: Option<(RgbColor, f32)>,
  pub series_colors: Vec<RgbColor>,
  /// Automatic per-point colors for a single `varyColors` cartesian series.
  /// Explicit `c:dPt/c:spPr` paint in `series_point_styles` still wins.
  pub series_point_colors: Vec<Vec<Option<RgbColor>>>,
  pub series_styles: Vec<crate::common::ShapeStyle<'static>>,
  pub trendline_styles: Vec<Vec<crate::common::ShapeStyle<'static>>>,
  /// Host-resolved line formatting for each series' `c:errBars` records,
  /// retaining X/Y order independently from trendline and series paint.
  pub error_bar_styles: Vec<Vec<crate::common::ShapeStyle<'static>>>,
  /// Host-resolved chart-group decorations in the same order as
  /// `ClusteredColumnChart::group_decorations`.
  pub group_decoration_styles: Vec<CartesianChartGroupDecorationStyle>,
  pub series_point_styles: Vec<Vec<Option<crate::common::ShapeStyle<'static>>>>,
  /// Surface-chart value-band colors keyed by `c:bandFmt/c:idx`, one vector
  /// per surface plot-area group.
  pub surface_band_colors: Vec<Vec<(u32, RgbColor)>>,
  pub data_label_fill_colors: Vec<Vec<Option<RgbColor>>>,
  pub chart_area_style: crate::common::ShapeStyle<'static>,
  pub plot_area_style: crate::common::ShapeStyle<'static>,
  /// Host-resolved formatting for the three authored c:chart 3-D planes.
  /// These are independent of c:plotArea/c:spPr: Office permits every plane
  /// to carry its own fill and outline, including an explicit noFill.
  pub floor_style: crate::common::ShapeStyle<'static>,
  pub side_wall_style: crate::common::ShapeStyle<'static>,
  pub back_wall_style: crate::common::ShapeStyle<'static>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct CartesianChartGroupDecorationStyle {
  pub drop_lines: crate::common::ShapeStyle<'static>,
  pub high_low_lines: crate::common::ShapeStyle<'static>,
  pub up_bars: crate::common::ShapeStyle<'static>,
  pub down_bars: crate::common::ShapeStyle<'static>,
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

fn maximum_auto_main_increment_count(
  available_axis_length_pt: f32,
  label_shape_extent_pt: f32,
) -> usize {
  // LibreOffice VCartesianAxis::estimateMaximumAutoMainIncrementCount divides
  // the final center-to-center label line by the maximum recorded label shape
  // extent, then ScaleAutomatism clamps that result to 2..10. The first and
  // last generated shapes each occupy half an extent at the ends of the full
  // axis line, so their centers are one complete extent closer together.
  // Axis tick labels are auto-growing text shapes built from character
  // properties; DrawingML text body insets belong to authored text boxes and
  // are not mapped into these generated shapes. Bubble marker bounds expand
  // the numeric data domain before this pass; they are not part of the label
  // extent either.
  let label_shape_extent_pt = label_shape_extent_pt.max(1.0);
  ((available_axis_length_pt - label_shape_extent_pt).max(0.0) / label_shape_extent_pt)
    .floor()
    .clamp(2.0, 10.0) as usize
}

fn suppress_duplicate_formatted_tick_budget(
  physical_budget: usize,
  tick_labels: &[(f64, String)],
) -> usize {
  // LibreOffice VCartesianAxis::estimateMaximumAutoMainIncrementCount also
  // protects against a number format collapsing adjacent numeric ticks to the
  // same visible string (tdf#48041).  The longest equal-label run determines
  // how far the scale must be thinned; date axes have their own time-unit
  // estimator and do not enter this helper.
  let mut longest_run = 1_usize;
  let mut current_run = 0_usize;
  let mut previous = None::<&str>;
  for (_, label) in tick_labels {
    if previous == Some(label.as_str()) {
      current_run += 1;
    } else {
      current_run = 1;
      previous = Some(label);
    }
    longest_run = longest_run.max(current_run);
  }
  if longest_run <= 1 {
    return physical_budget;
  }
  physical_budget
    .min(tick_labels.len() / longest_run)
    .clamp(2, 10)
}

fn maximum_tick_label_axis_pitch(
  tick_labels: &[(f64, String)],
  style: &TextStyle,
  rotation_degrees: f32,
  horizontal_axis: bool,
  metrics: &mut TextMetrics,
) -> f32 {
  // LibreOffice VCartesianAxis records the bounds of the generated label
  // shapes and divides the final axis length by that painted extent. The
  // c:txPr/a:bodyPr values describe the text body used to format those
  // generated shapes; they are not an additional collision margin around
  // every tick label. Adding even one schema-default inset here makes short
  // Excel axes select a coarser scale than their fixed output.
  let (sin, cos) = rotation_degrees.to_radians().sin_cos();
  let sin = sin.abs();
  let cos = cos.abs();
  tick_labels
    .iter()
    .map(|(_, label)| {
      let width = metrics.measure_text(label, style);
      // Axis labels are laid out in the same 1.2-em line boxes that are later
      // emitted to the page. Font ink/face metrics can be substantially
      // shorter (notably Calibri digits), and using them here admits labels
      // that the generated text shapes cannot actually fit.
      let height = metrics
        .inline_text_box_height_for_text(label, style)
        .max(line_height(style));
      if horizontal_axis {
        width * cos + height * sin
      } else {
        width * sin + height * cos
      }
    })
    .fold(0.0_f32, f32::max)
    .max(line_height(style))
}

fn word_automatic_titled_bottom_layout(
  chart: &ClusteredColumnChart<'_>,
  style: &ClusteredColumnStyle,
) -> bool {
  style.layout_profile == ChartLayoutProfile::Word
    && chart.legend_position == Some(ChartLegendPosition::Bottom)
    && !chart.legend_overlay
    && chart.legend_layout.is_none()
    && chart.title.is_some()
    && !chart.title_overlay
    && chart.plot_layout.is_none()
    && chart.view_3d.is_none()
}

fn east_asian_title_script(title: &str, style: &TextStyle) -> Option<TextScript> {
  script_direction_runs(title, FontSize(style.font_size_pt), style.small_caps)
    .into_iter()
    .map(|run| run.script)
    .find(|script| {
      matches!(
        script,
        TextScript::Han | TextScript::Hiragana | TextScript::Katakana | TextScript::Hangul
      )
    })
}

#[derive(Clone, Debug)]
pub(crate) struct RadialChartStyle {
  pub layout_profile: ChartLayoutProfile,
  pub title: TextStyle,
  pub legend: TextStyle,
  pub data_label: TextStyle,
  pub data_label_styles: Vec<Option<TextStyle>>,
  /// Host-resolved styles for each retained custom data-label DrawingML run,
  /// indexed as label -> run.
  pub data_label_rich_text_styles: Vec<Vec<TextStyle>>,
  pub point_colors: Vec<RgbColor>,
  pub point_styles: Vec<crate::common::ShapeStyle<'static>>,
  pub data_label_fill_colors: Vec<Option<RgbColor>>,
  pub leader_line_style: crate::common::ShapeStyle<'static>,
  pub chart_area_style: crate::common::ShapeStyle<'static>,
  pub plot_area_style: crate::common::ShapeStyle<'static>,
}

pub(crate) fn solid_chart_shape_style(
  fill_color: Option<RgbColor>,
  stroke: Option<(RgbColor, f32)>,
) -> crate::common::ShapeStyle<'static> {
  crate::common::ShapeStyle {
    fill: fill_color.map_or(crate::common::ShapeStyleValue::Unspecified, |color| {
      crate::common::ShapeStyleValue::Paint(crate::common::Fill::Solid(common_rgb(color, 1.0)))
    }),
    stroke: stroke.map_or(
      crate::common::ShapeStyleValue::Unspecified,
      |(color, width_pt)| {
        crate::common::ShapeStyleValue::Paint(crate::common::Stroke {
          width: crate::common::Pt(width_pt),
          color: common_rgb(color, 1.0),
          ..Default::default()
        })
      },
    ),
  }
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

fn powerpoint_derived_single_series_title_layout(
  chart: &ClusteredColumnChart<'_>,
  style: &ClusteredColumnStyle,
) -> bool {
  style.layout_profile == ChartLayoutProfile::PowerPoint
    && !style.has_explicit_title
    && chart.has_automatic_title_marker
    && matches!(
      (chart.title.as_ref(), chart.series.as_slice()),
      (Some(ChartTitleText::Explicit(title)), [series])
        if series.has_nonempty_explicit_name && title == &series.name
    )
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

fn excel_legacy_default_single_series_side_title_layout(
  chart: &ClusteredColumnChart<'_>,
  style: &ClusteredColumnStyle,
) -> bool {
  style.layout_profile == ChartLayoutProfile::Excel
    && style.has_explicit_title
    && style.title_top_adjustment_ratio.abs() > f32::EPSILON
    && matches!(chart.title.as_ref(), Some(ChartTitleText::Explicit(_)))
    && chart.series.len() == 1
    && matches!(
      chart.legend_position,
      Some(ChartLegendPosition::Left | ChartLegendPosition::Right | ChartLegendPosition::TopRight)
    )
    && !chart.legend_overlay
    && chart.plot_layout.is_none()
}

fn excel_vary_colors_data_table_layout(
  chart: &ClusteredColumnChart<'_>,
  style: &ClusteredColumnStyle,
) -> bool {
  style.layout_profile == ChartLayoutProfile::Excel
    && chart.data_table.is_some()
    && chart.vary_colors_by_point
    && chart.series.len() == 1
    && matches!(
      chart.legend_position,
      Some(ChartLegendPosition::Left | ChartLegendPosition::Right | ChartLegendPosition::TopRight)
    )
    && !chart.legend_overlay
    && chart.title.is_none()
    && chart.plot_layout.is_none()
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
  // LibreOffice ChartView performs an initial ten-increment autoscale before
  // maximum label extents and the final inner plot rectangle are known.
  // A second, size-dependent pass is performed after the plot is laid out.
  let axis_scales =
    cartesian_axis_scales(chart, style.layout_profile, category_count, 10, 10, None);
  let Some(scale) = axis_scales.first().map(|axes| axes.y) else {
    return Vec::new();
  };
  if scale.maximum <= scale.minimum {
    return Vec::new();
  }
  let percent_stacked = axis_set_is_percent_stacked(chart, 0);

  let mut metrics = TextMetrics::new();
  let title_text = match chart.title.as_ref() {
    Some(ChartTitleText::Explicit(title)) => Some(title.as_str()),
    Some(ChartTitleText::Automatic) => Some(automatic_title),
    None => None,
  };
  let has_layout_title = title_text.is_some() && !chart.title_overlay;
  let has_layout_explicit_title = has_layout_title && style.has_explicit_title;
  let has_powerpoint_derived_single_series_title =
    powerpoint_derived_single_series_title_layout(chart, style);
  let title_line_height = line_height(&style.title);
  let title_reservation_height = title_text.map_or(title_line_height, |title| {
    let rotation_radians = (style.title.rotation_deg + chart.title_rotation_deg).to_radians();
    if rotation_radians.abs() <= f32::EPSILON {
      title_line_height
    } else {
      let title_width = metrics.measure_text(title, &style.title);
      title_width * rotation_radians.sin().abs() + title_line_height * rotation_radians.cos().abs()
    }
  });
  let category_label_line_height = line_height(&style.category_label);
  let value_label_line_height = line_height(&style.value_label);
  let label_line_height = category_label_line_height
    .max(value_label_line_height)
    .max(line_height(&style.label));
  let legend_line_height = line_height(&style.legend);
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
  let has_explicit_title_indexed_scatter_layout = style.layout_profile == ChartLayoutProfile::Excel
    && scatter_only
    && !chart.title_overlay
    && style.has_explicit_title
    && matches!(chart.title.as_ref(), Some(ChartTitleText::Explicit(_)))
    && chart.legend_position.is_none()
    && scatter_uses_index_x_values(chart)
    && chart.plot_layout.is_none();
  let has_legacy_single_series_title_layout = style.layout_profile == ChartLayoutProfile::Excel
    && !style.has_explicit_title
    && matches!(chart.title.as_ref(), Some(ChartTitleText::Explicit(_)))
    && chart.series.len() == 1
    && chart.legend_position.is_none();
  let has_modern_single_series_title_layout =
    has_legacy_single_series_title_layout && style.modern_excel_profile;
  let has_modern_single_series_scatter_title_layout =
    has_modern_single_series_title_layout && scatter_only;
  let modern_single_series_title_adjustment = if has_modern_single_series_title_layout {
    if scatter_only {
      profiles::EXCEL_MODERN_SINGLE_SERIES_SCATTER_TITLE
    } else {
      profiles::EXCEL_MODERN_SINGLE_SERIES_TITLE
    }
  } else {
    profiles::CartesianLayoutAdjustment::default()
  };
  let has_derived_single_series_side_title_layout =
    excel_derived_single_series_side_title_layout(chart, style);
  let has_explicit_single_series_side_title_layout =
    excel_explicit_single_series_side_title_layout(chart, style);
  let has_legacy_default_single_series_side_title_layout =
    excel_legacy_default_single_series_side_title_layout(chart, style);
  let legacy_default_single_series_adjustment =
    if has_legacy_default_single_series_side_title_layout {
      profiles::EXCEL_LEGACY_DEFAULT_SINGLE_SERIES_SIDE_TITLE
    } else {
      profiles::CartesianLayoutAdjustment::default()
    };
  let has_automatic_untitled_layout = chart.has_automatic_title_marker
    || (chart.title_overlay && matches!(chart.title.as_ref(), Some(ChartTitleText::Automatic)))
    || has_indexed_scatter_automatic_layout;
  let value_axis_visible = chart.value_axis.is_none_or(value_axis_is_visible);
  let primary_value_axis_on_right = chart.value_axis.is_some_and(value_axis_is_on_right);
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
  let (date_ticks, date_axis_increment_budget) =
    if category_tick_labels_visible && chart.date_axis.is_some() {
      let preliminary_ticks = date_axis_ticks(chart).unwrap_or_default();
      let budget = estimated_date_axis_maximum_auto_main_increment_count(
        style.layout_profile,
        chart,
        frame.width_pt,
        &preliminary_ticks,
        &style.category_label,
        &mut metrics,
      );
      (
        date_axis_ticks_with_maximum_auto_increment_count(chart, budget),
        budget,
      )
    } else {
      (None, 500)
    };
  let category_label_texts: Vec<String> = if category_tick_labels_visible {
    if let Some(ticks) = date_ticks.as_ref() {
      ticks.iter().map(|tick| tick.text.clone()).collect()
    } else {
      chart.categories.clone()
    }
  } else {
    Vec::new()
  };
  let category_axis_text_properties = chart
    .category_axis
    .and_then(|axis| axis.text_properties.as_deref())
    .or_else(|| {
      chart
        .date_axis
        .and_then(|axis| axis.text_properties.as_deref())
    });
  let unwrapped_category_label_maximum_width = category_label_texts
    .iter()
    .map(|label| metrics.measure_text(label, &style.category_label))
    .fold(0.0f32, f32::max);
  let category_label_rotation = category_axis_text_rotation_degrees_for_layout(
    style.layout_profile,
    category_axis_text_properties,
    frame.width_pt,
    category_label_texts.len(),
    unwrapped_category_label_maximum_width,
    style.category_label.font_size_pt,
  );
  let category_label_lines: Vec<Vec<String>> = if category_label_rotation.abs() > f32::EPSILON {
    // A rotated chart label owns one text line. Wrapping it against the
    // unrotated category slot before applying the transform turns
    // `Sep 2013` into two independently rotated objects and under-reserves
    // the category band. Excel and PowerPoint retain the complete line and
    // let its rotated extent determine the automatic plot reservation.
    category_label_texts
      .into_iter()
      .map(|label| vec![label])
      .collect()
  } else {
    let slot_width = frame.width_pt / category_count as f32 * 0.9;
    category_label_texts
      .iter()
      .map(|label| wrap_chart_label(label, slot_width, &style.category_label, &mut metrics))
      .collect()
  };
  let category_label_line_count =
    category_label_lines.iter().map(Vec::len).max().unwrap_or(1) as f32;
  let category_label_unrotated_height = category_label_line_height * category_label_line_count;
  let category_label_maximum_width = category_label_lines
    .iter()
    .flatten()
    .map(|line| metrics.measure_text(line, &style.category_label))
    .fold(0.0f32, f32::max);
  let category_label_height = if category_label_rotation.abs() <= f32::EPSILON {
    category_label_unrotated_height
  } else {
    let rotation = category_label_rotation.to_radians();
    category_label_maximum_width * rotation.sin().abs()
      + category_label_unrotated_height * rotation.cos().abs()
  };
  let legend_position = chart.legend_position;
  let has_bottom_legend =
    legend_position == Some(ChartLegendPosition::Bottom) && !chart.legend_overlay;
  let has_untitled_bottom_column_layout = excel_untitled_bottom_column_layout(chart, style);
  let has_untitled_bottom_line_no_marker_layout =
    excel_untitled_bottom_line_no_marker_layout(chart, style);
  let has_explicit_bottom_column_layout = excel_explicit_bottom_column_layout(chart, style);
  let has_explicit_powerpoint_title =
    style.layout_profile == ChartLayoutProfile::PowerPoint && has_layout_explicit_title;
  let has_powerpoint_generated_title_bottom_layout = style.layout_profile
    == ChartLayoutProfile::PowerPoint
    && has_bottom_legend
    && has_layout_title
    && !style.has_explicit_title
    && matches!(chart.title.as_ref(), Some(ChartTitleText::Automatic))
    && !chart.title_layout_container_present
    && chart.plot_layout.is_none();
  let has_powerpoint_generated_title_no_legend_layout = style.layout_profile
    == ChartLayoutProfile::PowerPoint
    && legend_position.is_none()
    && has_layout_title
    && has_powerpoint_derived_single_series_title
    && !chart.title_layout_container_present
    && chart.plot_layout.is_none()
    && chart.view_3d.is_none()
    && !radar_only
    && !horizontal_bar_only
    && !scatter_only;
  let has_top_legend = legend_position == Some(ChartLegendPosition::Top) && !chart.legend_overlay;
  let has_side_legend = matches!(
    legend_position,
    Some(ChartLegendPosition::Left | ChartLegendPosition::Right | ChartLegendPosition::TopRight)
  ) && !chart.legend_overlay;
  let has_excel_explicit_title_side_legend_layout = style.layout_profile
    == ChartLayoutProfile::Excel
    && has_side_legend
    && has_layout_explicit_title
    && chart.plot_layout.is_none();
  let has_excel_explicit_date_line_top_right_overlay_layout = style.layout_profile
    == ChartLayoutProfile::Excel
    && legend_position == Some(ChartLegendPosition::TopRight)
    && chart.legend_overlay
    && has_layout_title
    && chart.plot_layout.is_none()
    && chart.date_axis.is_some()
    && chart.category_axis_title.is_some()
    && chart.value_axis_title.is_some()
    && chart.series.len() >= 2
    && chart
      .series
      .iter()
      .all(|series| series.kind == ChartSeriesKind::Line);
  let has_excel_title_only_layout = style.layout_profile == ChartLayoutProfile::Excel
    && legend_position.is_none()
    && has_layout_explicit_title
    && chart.plot_layout.is_none()
    && !has_explicit_title_indexed_scatter_layout;
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
  let has_shifted_category_empty_side_legend_layout = style.layout_profile
    == ChartLayoutProfile::Excel
    && has_side_legend
    && !has_layout_title
    && chart.title.is_none()
    && !chart.has_automatic_title_marker
    && chart.has_explicit_categories
    && chart.category_axis_shifted
    && chart.plot_layout.is_none()
    && chart.series.len() == 1
    && chart.series[0].kind == ChartSeriesKind::Column
    && !chart.series[0].has_explicit_name
    && (chart.gap_width_percent - 100.0).abs() < f64::EPSILON
    && chart.overlap_percent.abs() < f64::EPSILON;
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
  } else if style.layout_profile == ChartLayoutProfile::Word
    && has_side_legend
    && !has_layout_title
    && chart.plot_layout.is_none()
  {
    profiles::WORD_UNTITLED_SIDE_LEGEND
  } else {
    profiles::CartesianLayoutAdjustment::default()
  };
  let word_no_legend_adjustment = if style.layout_profile == ChartLayoutProfile::Word
    && legend_position.is_none()
    && !has_layout_title
    && chart.plot_layout.is_none()
  {
    profiles::WORD_UNTITLED_NO_LEGEND
  } else {
    profiles::CartesianLayoutAdjustment::default()
  };
  let has_word_titled_bottom_layout = word_automatic_titled_bottom_layout(chart, style);
  let word_titled_bottom_adjustment = if has_word_titled_bottom_layout {
    profiles::WORD_TITLED_BOTTOM_LEGEND
  } else {
    profiles::CartesianLayoutAdjustment::default()
  };
  let powerpoint_derived_title_adjustment =
    if has_powerpoint_derived_single_series_title && has_side_legend && chart.plot_layout.is_none()
    {
      if horizontal_bar_only {
        profiles::POWERPOINT_DERIVED_SERIES_TITLE_HORIZONTAL_BAR
      } else if radar_only {
        profiles::CartesianLayoutAdjustment::default()
      } else {
        profiles::POWERPOINT_DERIVED_SERIES_TITLE_SIDE_LEGEND
      }
    } else {
      profiles::CartesianLayoutAdjustment::default()
    };
  let powerpoint_generated_title_bottom_adjustment = if has_powerpoint_generated_title_bottom_layout
  {
    profiles::POWERPOINT_GENERATED_TITLE_BOTTOM_LEGEND
  } else {
    profiles::CartesianLayoutAdjustment::default()
  };
  let powerpoint_generated_title_no_legend_adjustment =
    if has_powerpoint_generated_title_no_legend_layout {
      profiles::POWERPOINT_GENERATED_TITLE_NO_LEGEND
    } else {
      profiles::CartesianLayoutAdjustment::default()
    };
  let excel_side_adjustment = if has_excel_explicit_title_side_legend_layout {
    profiles::EXCEL_EXPLICIT_TITLE_SIDE_LEGEND
  } else {
    profiles::CartesianLayoutAdjustment::default()
  };
  let excel_explicit_date_line_top_right_overlay_adjustment =
    if has_excel_explicit_date_line_top_right_overlay_layout {
      profiles::EXCEL_EXPLICIT_DATE_LINE_TOP_RIGHT_OVERLAY
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
  let has_excel_vary_colors_data_table_layout = excel_vary_colors_data_table_layout(chart, style);
  let excel_vary_colors_data_table_adjustment = if has_excel_vary_colors_data_table_layout {
    profiles::EXCEL_VARY_COLORS_DATA_TABLE
  } else {
    profiles::CartesianLayoutAdjustment::default()
  };
  // Excel's automatic cartesian layout uses a separate title reservation when
  // no legend is authored. Office fixed-output evidence keeps both the plot
  // and category-label bands lower than the side-legend profile.
  let excel_title_only_adjustment = if has_explicit_title_indexed_scatter_layout {
    profiles::EXCEL_EXPLICIT_TITLE_INDEXED_SCATTER
  } else if has_excel_title_only_layout {
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
    + frame.height_pt * style.title_top_adjustment_ratio
    + if has_derived_single_series_side_title_layout {
      frame.height_pt * profiles::EXCEL_DERIVED_SINGLE_SERIES_SIDE_TITLE.title_top_ratio
    } else if has_explicit_single_series_side_title_layout {
      frame.height_pt * profiles::EXCEL_EXPLICIT_SINGLE_SERIES_SIDE_TITLE.title_top_ratio
    } else if has_explicit_bottom_column_layout {
      frame.height_pt * profiles::EXCEL_EXPLICIT_BOTTOM_COLUMN.title_top_ratio
    } else {
      0.0
    }
    + frame.height_pt * powerpoint_generated_title_bottom_adjustment.title_top_ratio;
  let title_top =
    title_top + frame.height_pt * powerpoint_generated_title_no_legend_adjustment.title_top_ratio;
  let legend_bottom_margin = style.legend.font_size_pt * 0.81;
  let legend_top = frame.y_pt + frame.height_pt - legend_bottom_margin - legend_line_height;
  let category_bottom_ratio = host_defaults.category_bottom_ratio;
  let mut category_top = if chart.data_table.is_some() {
    frame.y_pt + frame.height_pt
      - data_table_height
      - if has_bottom_legend {
        legend_line_height + frame.height_pt * profiles::DATA_TABLE_BOTTOM_LEGEND_GAP_RATIO
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
    + frame.height_pt * word_no_legend_adjustment.category_top_ratio
    + frame.height_pt * word_titled_bottom_adjustment.category_top_ratio
    + frame.height_pt * powerpoint_derived_title_adjustment.category_top_ratio
    + frame.height_pt * powerpoint_generated_title_bottom_adjustment.category_top_ratio
    + frame.height_pt * powerpoint_generated_title_no_legend_adjustment.category_top_ratio
    + frame.height_pt * excel_side_adjustment.category_top_ratio
    + frame.height_pt * excel_explicit_date_line_top_right_overlay_adjustment.category_top_ratio
    + frame.height_pt * excel_title_only_adjustment.category_top_ratio
    + frame.height_pt * excel_untitled_side_adjustment.category_top_ratio
    + frame.height_pt * excel_vary_colors_data_table_adjustment.category_top_ratio
    + frame.height_pt * legacy_default_single_series_adjustment.category_top_ratio;
  if !horizontal_bar_only && chart.category_axis_title.is_some() && chart.data_table.is_none() {
    // Office's automatic bottom stack reserves the category-axis title before
    // sizing the final plot. A bottom legend also needs the two inter-band
    // gaps around that title; without a legend the title line and its normal
    // leading still move the category labels and plot upward together.
    category_top -= line_height(&style.label) * if has_bottom_legend { 2.25 } else { 1.1 };
  }
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
  category_top += frame.height_pt * modern_single_series_title_adjustment.category_top_ratio;
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
    title_top + title_reservation_height + label_line_height * 0.9
  } else {
    frame.y_pt + frame.height_pt * untitled_plot_top_ratio
  } + frame.height_pt * word_side_adjustment.plot_top_ratio
    + frame.height_pt * word_no_legend_adjustment.plot_top_ratio
    + frame.height_pt * word_titled_bottom_adjustment.plot_top_ratio
    + frame.height_pt * powerpoint_derived_title_adjustment.plot_top_ratio
    + frame.height_pt * powerpoint_generated_title_bottom_adjustment.plot_top_ratio
    + frame.height_pt * powerpoint_generated_title_no_legend_adjustment.plot_top_ratio
    + frame.height_pt * excel_side_adjustment.plot_top_ratio
    + frame.height_pt * excel_explicit_date_line_top_right_overlay_adjustment.plot_top_ratio
    + frame.height_pt * excel_title_only_adjustment.plot_top_ratio
    + frame.height_pt * excel_untitled_side_adjustment.plot_top_ratio
    + frame.height_pt * excel_vary_colors_data_table_adjustment.plot_top_ratio
    + frame.height_pt * legacy_default_single_series_adjustment.plot_top_ratio;
  if has_word_titled_bottom_layout
    && let Some(title) = title_text
    && let Some(script) = east_asian_title_script(title, &style.title)
  {
    let metrics = metrics.vertical_metrics_for_script(&style.title, script);
    plot_top += frame.height_pt * profiles::WORD_BOTTOM_LEGEND_EAST_ASIAN_TITLE_EXTRA_RATIO
      + (metrics.line_height_pt() - style.title.font_size_pt).max(0.0);
  }
  if has_independent_axis_text_layout {
    plot_top += frame.height_pt * profiles::EXCEL_INDEPENDENT_AXIS_TEXT.plot_top_ratio;
  } else if has_legacy_indexed_scatter_layout {
    plot_top += frame.height_pt * profiles::EXCEL_LEGACY_INDEXED_SCATTER.plot_top_ratio;
  }
  plot_top += frame.height_pt * modern_single_series_title_adjustment.plot_top_ratio;
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
    plot_top += legend_line_height
      + frame.height_pt
        * if style.layout_profile == ChartLayoutProfile::Word {
          profiles::WORD_TOP_LEGEND_GAP_RATIO
        } else {
          profiles::DEFAULT_TOP_LEGEND_GAP_RATIO
        };
  }
  if chart.view_3d.is_some() && has_bottom_legend {
    // VDiagram preserves the projected scene aspect ratio inside the
    // axis/legend rectangle. With a bottom legend that fit leaves one value
    // label em above the scene; without it the 3-D back wall intrudes into an
    // overlay title even though the title itself does not resize the plot.
    plot_top += style.value_label.font_size_pt;
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
    ChartLayoutProfile::Excel
      if has_explicit_single_series_side_title_layout
        || has_legacy_default_single_series_side_title_layout =>
    {
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
  plot_bottom += frame.height_pt
    * (word_side_adjustment.plot_bottom_ratio
      + word_no_legend_adjustment.plot_bottom_ratio
      + word_titled_bottom_adjustment.plot_bottom_ratio
      + powerpoint_derived_title_adjustment.plot_bottom_ratio
      + powerpoint_generated_title_bottom_adjustment.plot_bottom_ratio);
  plot_bottom +=
    frame.height_pt * powerpoint_generated_title_no_legend_adjustment.plot_bottom_ratio;
  plot_bottom += frame.height_pt * excel_vary_colors_data_table_adjustment.plot_bottom_ratio;
  plot_bottom +=
    frame.height_pt * excel_explicit_date_line_top_right_overlay_adjustment.plot_bottom_ratio;
  plot_bottom += frame.height_pt * modern_single_series_title_adjustment.plot_bottom_ratio;
  plot_bottom += frame.height_pt * legacy_default_single_series_adjustment.plot_bottom_ratio;
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

  let value_number_format =
    vertical_axis_number_format_code(chart, 0).or(percent_stacked.then_some("0%"));
  let value_display_unit = chart.value_axis.map_or(1.0, value_axis_display_unit);
  let tick_labels = scale_tick_labels(
    scale.minimum,
    scale.maximum,
    scale.major_unit,
    value_number_format,
    scale.logarithmic_base,
    value_display_unit,
  );
  let maximum_tick_width = if value_tick_labels_visible {
    tick_labels
      .iter()
      .map(|(_, label)| metrics.measure_text(label, &style.value_label))
      .fold(0.0_f32, f32::max)
  } else {
    0.0
  };
  let secondary_value_tick_sets = axis_scales
    .iter()
    .enumerate()
    .skip(1)
    .filter_map(|(axis_set_index, axes)| {
      let axis = axis_set_value_axis(chart, axis_set_index)?;
      if !value_axis_is_visible(axis)
        || axis
          .tick_label_position
          .as_ref()
          .is_some_and(|position| position.val == Some(c::TickLabelPositionValues::None))
        || radar_only
        || horizontal_bar_only
      {
        return None;
      }
      let format_code = vertical_axis_number_format_code(chart, axis_set_index)
        .or_else(|| axis_set_is_percent_stacked(chart, axis_set_index).then_some("0%"));
      let labels = scale_tick_labels(
        axes.y.minimum,
        axes.y.maximum,
        axes.y.major_unit,
        format_code,
        axes.y.logarithmic_base,
        value_axis_display_unit(axis),
      );
      let width = labels
        .iter()
        .map(|(_, label)| metrics.measure_text(label, &style.value_label))
        .fold(0.0_f32, f32::max);
      Some((
        axis_set_index,
        axis,
        axes.y,
        labels,
        width,
        value_axis_is_on_right(axis),
      ))
    })
    .collect::<Vec<_>>();
  let secondary_left_value_axis_band_width = secondary_value_tick_sets
    .iter()
    .filter(|(_, _, _, _, _, on_right)| !on_right)
    .map(|(_, _, _, _, width, _)| *width + frame.height_pt * profiles::DEFAULT_TICK_GAP_RATIO)
    .sum::<f32>();
  let secondary_right_value_axis_band_width = secondary_value_tick_sets
    .iter()
    .filter(|(_, _, _, _, _, on_right)| *on_right)
    .map(|(_, _, _, _, width, _)| *width + frame.height_pt * profiles::DEFAULT_TICK_GAP_RATIO)
    .sum::<f32>();
  let side_legend_width = if has_side_legend {
    vertical_legend_width(chart, style, scale, &mut metrics)
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
    }
    + if has_legacy_default_single_series_side_title_layout {
      // The compact automatic label (`Series1` / `系列1`) does not donate its
      // removed word-space to the plot; Excel retains that half-em as part of
      // the side legend reservation.
      style.legend.font_size_pt * profiles::EXCEL_LEGACY_DEFAULT_SINGLE_SERIES_LEGEND_RESERVATION_EM
    } else {
      0.0
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
    if has_explicit_title_indexed_scatter_layout {
      profiles::DEFAULT_TICK_LEFT_RATIO
    } else if style.layout_profile == ChartLayoutProfile::Excel
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
  let primary_value_axis_title_band_width =
    if !horizontal_bar_only && chart.value_axis_title.is_some() {
      line_height(&style.label) * 2.0
    } else {
      0.0
    };
  let tick_left = frame.x_pt
    + frame.height_pt * tick_left_ratio
    + if primary_value_axis_on_right {
      0.0
    } else {
      primary_value_axis_title_band_width
    }
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
  let mut tick_left = tick_left
    + frame.height_pt * word_titled_bottom_adjustment.tick_left_ratio
    + frame.height_pt * powerpoint_derived_title_adjustment.tick_left_ratio
    + frame.height_pt * powerpoint_generated_title_no_legend_adjustment.tick_left_ratio
    + if has_indexed_scatter_automatic_layout {
      frame.height_pt * profiles::EXCEL_AUTOMATIC_INDEXED_SCATTER.tick_left_ratio
    } else if has_legacy_indexed_scatter_layout {
      frame.height_pt * profiles::EXCEL_LEGACY_INDEXED_SCATTER.tick_left_ratio
    } else {
      0.0
    }
    + frame.height_pt * excel_vary_colors_data_table_adjustment.tick_left_ratio;
  tick_left +=
    frame.height_pt * excel_explicit_date_line_top_right_overlay_adjustment.tick_left_ratio;
  let tick_gap = if value_tick_labels_visible {
    frame.height_pt
      * if has_side_legend {
        if has_shifted_category_empty_side_legend_layout {
          profiles::EXCEL_SHIFTED_CATEGORY_EMPTY_SIDE_LEGEND_TICK_GAP_RATIO
        } else if has_unshifted_side_line_layout {
          profiles::EXCEL_UNSHIFTED_LINE_SIDE_TICK_GAP_RATIO
        } else {
          host_side_legend_bands.tick_gap_ratio
        }
      } else {
        if has_modern_single_series_scatter_title_layout
          && chart
            .plot_layout
            .is_some_and(|layout| layout.targets_inner_plot)
        {
          profiles::EXCEL_MANUAL_INNER_SCATTER_TICK_GAP_RATIO
        } else if has_bottom_legend && has_explicit_powerpoint_title {
          profiles::POWERPOINT_TITLED_BOTTOM_TICK_GAP_RATIO
        } else {
          profiles::DEFAULT_TICK_GAP_RATIO
        }
      }
  } else {
    0.0
  };
  let primary_value_axis_band_width = maximum_tick_width + tick_gap;
  let left_value_axis_band_width = secondary_left_value_axis_band_width
    + if primary_value_axis_on_right {
      0.0
    } else {
      primary_value_axis_band_width
    };
  let right_value_axis_band_width = secondary_right_value_axis_band_width
    + if primary_value_axis_on_right {
      primary_value_axis_band_width
    } else {
      0.0
    };
  let mut plot_left = tick_left + left_value_axis_band_width;
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
    }
    - right_value_axis_band_width
    - if primary_value_axis_on_right {
      primary_value_axis_title_band_width
    } else {
      0.0
    };
  plot_left += frame.height_pt
    * (word_side_adjustment.plot_left_ratio
      + word_no_legend_adjustment.plot_left_ratio
      + word_titled_bottom_adjustment.plot_left_ratio
      + powerpoint_derived_title_adjustment.plot_left_ratio
      + powerpoint_generated_title_no_legend_adjustment.plot_left_ratio
      + legacy_default_single_series_adjustment.plot_left_ratio);
  plot_right += frame.height_pt
    * (word_side_adjustment.plot_right_ratio
      + word_no_legend_adjustment.plot_right_ratio
      + word_titled_bottom_adjustment.plot_right_ratio
      + powerpoint_derived_title_adjustment.plot_right_ratio
      + powerpoint_generated_title_no_legend_adjustment.plot_right_ratio
      + legacy_default_single_series_adjustment.plot_right_ratio);
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
  plot_left += frame.height_pt * excel_vary_colors_data_table_adjustment.plot_left_ratio;
  plot_right += frame.height_pt * excel_vary_colors_data_table_adjustment.plot_right_ratio;
  plot_left +=
    frame.height_pt * excel_explicit_date_line_top_right_overlay_adjustment.plot_left_ratio;
  plot_right +=
    frame.height_pt * excel_explicit_date_line_top_right_overlay_adjustment.plot_right_ratio;
  plot_left += frame.height_pt * modern_single_series_title_adjustment.plot_left_ratio;
  plot_right += frame.height_pt * modern_single_series_title_adjustment.plot_right_ratio;
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
  if has_shifted_category_empty_side_legend_layout {
    let adjustment = profiles::EXCEL_SHIFTED_CATEGORY_EMPTY_SIDE_LEGEND;
    category_top += frame.height_pt * adjustment.category_top_ratio;
    plot_left += frame.height_pt * adjustment.plot_left_ratio;
    plot_right += frame.height_pt * adjustment.plot_right_ratio;
    plot_top += frame.height_pt * adjustment.plot_top_ratio;
    plot_bottom += frame.height_pt * adjustment.plot_bottom_ratio;
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
    if layout.targets_inner_plot {
      // ECMA-376 Part 1 §21.2.2.89 defines the inner target as the
      // rectangle excluding axes and axis labels. Keep the automatic
      // label-to-axis gaps, but anchor those outer bands to the authored
      // inner rectangle instead of leaving them at the automatic plot.
      if !primary_value_axis_on_right {
        tick_left = plot_left - maximum_tick_width - tick_gap;
      }
      category_top = plot_bottom + (category_top - automatic.top - automatic.height);
    }
  }
  let mut projection_3d = None;
  if let Some(view) = chart.view_3d {
    let preferred_model_aspect = cartesian_3d_preferred_model_aspect(chart, category_count);
    let mut scene_plot = PlotRect {
      left: plot_left,
      top: plot_top,
      width: plot_right - plot_left,
      height: plot_bottom - plot_top,
    };
    if !horizontal_bar_only {
      // LibreOffice VDiagram::adjustPosAndSize_3d fits the projected scene
      // while preserving its aspect ratio, then centers that scene in the
      // available rectangle. Axis text is the stable physical scale for the
      // residual 2-D band around that scene.
      // Word's fixed-format layout leaves one value-label em between the
      // automatic outer plot and the fitted 3-D scene. PowerPoint and Excel
      // retain the wider 1.2 em host band. Treating all hosts alike narrows a
      // Word scene symmetrically by 0.4 em, compressing every category slot.
      let horizontal_inset = style.value_label.font_size_pt
        * if style.layout_profile == ChartLayoutProfile::Word {
          1.0
        } else {
          1.2
        };
      let vertical_inset = style.value_label.font_size_pt * 0.25;
      if scene_plot.width > horizontal_inset * 2.0 {
        scene_plot.left += horizontal_inset;
        scene_plot.width -= horizontal_inset * 2.0;
      }
      if scene_plot.height > vertical_inset * 2.0 {
        scene_plot.top += vertical_inset;
        scene_plot.height -= vertical_inset * 2.0;
      }
    }
    if let Some((reserve_on_right, reservation)) = cartesian_3d_series_axis_reservation(
      chart,
      scene_plot,
      view,
      preferred_model_aspect,
      style,
      &mut metrics,
    ) {
      // Office balances the projected scene and the series-axis label band as
      // one automatic-layout unit. The scene retains its physical box and is
      // translated by half the label width; shrinking the projection itself
      // incorrectly compresses the category spacing.
      let direction = if reserve_on_right { -1.0 } else { 1.0 };
      scene_plot.left += direction * reservation * 0.5;
      if style.layout_profile == ChartLayoutProfile::PowerPoint {
        scene_plot.top += frame.height_pt * profiles::POWERPOINT_CARTESIAN_3D_SERIES_AXIS_TOP_RATIO;
      }
    }
    plot_left = scene_plot.left;
    plot_top = scene_plot.top;
    plot_right = scene_plot.left + scene_plot.width;
    plot_bottom = scene_plot.top + scene_plot.height;
    projection_3d = Some(cartesian_3d_projection(
      view,
      scene_plot,
      style.layout_profile,
      preferred_model_aspect.0,
      preferred_model_aspect.1,
    ));
  }
  if plot_right <= plot_left || plot_bottom <= plot_top {
    return Vec::new();
  }
  let mut plot_width = plot_right - plot_left;
  let plot_height = plot_bottom - plot_top;
  // Axis text shares the same scene-to-screen transform as the grid and
  // markers. Right-angle axes are oblique, not unprojected; bypassing their
  // transform places top value ticks at the scene rectangle rather than on
  // the visible front edge and shifts every category label sideways.
  let axis_text_projection_3d = projection_3d;
  let (mut primary_value_axis_x, primary_value_axis_depth) = projection_3d.map_or(
    (
      if primary_value_axis_on_right {
        plot_right
      } else {
        plot_left
      },
      0.0,
    ),
    |projection| {
      projection.vertical_edge_for_visual_side(
        PlotRect {
          left: plot_left,
          top: plot_top,
          width: plot_width,
          height: plot_height,
        },
        primary_value_axis_on_right,
      )
    },
  );
  let (mut primary_value_label_axis_x, primary_value_label_axis_depth) = axis_text_projection_3d
    .map_or(
      (
        if primary_value_axis_on_right {
          plot_right
        } else {
          plot_left
        },
        0.0,
      ),
      |projection| {
        projection.vertical_edge_for_visual_side(
          PlotRect {
            left: plot_left,
            top: plot_top,
            width: plot_width,
            height: plot_height,
          },
          primary_value_axis_on_right,
        )
      },
    );
  let has_excel_outer_value_label_band = has_legacy_single_series_title_layout
    || has_untitled_bottom_column_layout
    || has_untitled_bottom_line_no_marker_layout
    || has_explicit_bottom_column_layout
    || has_titled_indexed_scatter_layout
    || has_indexed_scatter_automatic_layout
    || has_excel_explicit_title_side_legend_layout
    || has_excel_title_only_layout
    || has_excel_vary_colors_data_table_layout;
  // LibreOffice's TickFactory2D keeps the fixed 1 mm
  // AXIS2D_TICKLABELSPACING separate from the projected tick geometry. The
  // ordinary 2D profiles already absorb that spacing into their calibrated
  // band, but a 3D axis is transformed as a line and needs the fixed distance
  // restored after projection. Otherwise every projected value label sits one
  // millimetre too close to its tick.
  let projected_tick_label_spacing = if axis_text_projection_3d.is_some() {
    CARTESIAN_DATA_LABEL_OFFSET_PT
  } else {
    0.0
  };
  let primary_value_label_gap = if primary_value_axis_on_right {
    tick_gap + frame.height_pt * 0.012_59 + projected_tick_label_spacing
  } else if has_excel_outer_value_label_band && axis_text_projection_3d.is_none() {
    // These Excel automatic profiles own an outer value-label band
    // independently of the residual plot inset. Deriving labels from
    // `plot_left` with only the generic gap incorrectly carries that residual
    // into every label. Office fixed output for both `smoothed_series`
    // generations, `tdf115012`, `no_marker`, `chart_title`, the three
    // `testChartTitleProperties*Fill` variants, and `autotitledel_2013`,
    // `dispBlanksAs_2013`, and `ser_labels` instead keep the widest label at
    // `tick_left` while the plot remains at its separately adjusted edge.
    (plot_left - tick_left - maximum_tick_width).max(0.0)
  } else {
    tick_gap + projected_tick_label_spacing
  };
  let plot = PlotRect {
    left: plot_left,
    top: plot_top,
    width: plot_width,
    height: plot_height,
  };
  let value_axis_is_horizontal = horizontal_bar_only;
  let available_value_axis_length = if radar_only {
    // A radar value axis runs from the center to the outer polygon. Its label
    // budget is therefore the radius, not the full plot height used by a
    // cartesian value axis.
    plot_width.min(plot_height) * 0.46
  } else if value_axis_is_horizontal {
    plot_width
  } else {
    projection_3d.map_or(plot_height, |projection| {
      projection.vertical_axis_length(plot, primary_value_axis_on_right)
    })
  };
  let value_axis_text_properties = chart
    .value_axis
    .and_then(|axis| axis.text_properties.as_deref());
  let value_axis_label_rotation = category_axis_text_rotation_degrees(value_axis_text_properties);
  let value_axis_label_extent = maximum_tick_label_axis_pitch(
    &tick_labels,
    &style.value_label,
    value_axis_label_rotation,
    value_axis_is_horizontal,
    &mut metrics,
  );
  let maximum_auto_increment_count = suppress_duplicate_formatted_tick_budget(
    maximum_auto_main_increment_count(available_value_axis_length, value_axis_label_extent),
    &tick_labels,
  );
  // Polar radius labels and horizontal bar value labels do not use the same
  // Cartesian vertical-label capacity. LibreOffice gives the polar radius a
  // dedicated estimator; PowerPoint fixed output likewise retains five main
  // radar intervals even when the radius is shorter than five complete text
  // bodies. Its horizontal bar family uses the same five-interval automatic
  // scale preference, still reduced when the horizontal labels truly do not
  // fit. Keep these typed axis policies out of ordinary column/line/scatter
  // axes, whose capacity is measured from their generated label shapes.
  let maximum_auto_value_increment_count =
    if style.layout_profile == ChartLayoutProfile::PowerPoint && radar_only {
      5
    } else if style.layout_profile == ChartLayoutProfile::PowerPoint && horizontal_bar_only {
      maximum_auto_increment_count.min(5)
    } else if radar_only {
      maximum_auto_increment_count.min(5)
    } else {
      maximum_auto_increment_count
    };
  let maximum_auto_horizontal_increment_count = axis_scales
    .first()
    .and_then(|axes| axes.x)
    .map(|horizontal_scale| {
      let labels = scale_tick_labels(
        horizontal_scale.minimum,
        horizontal_scale.maximum,
        horizontal_scale.major_unit,
        horizontal_axis_number_format_code(chart, 0),
        horizontal_scale.logarithmic_base,
        axis_set_horizontal_value_axis(chart, 0).map_or(1.0, value_axis_display_unit),
      );
      let text_properties =
        axis_set_horizontal_value_axis(chart, 0).and_then(|axis| axis.text_properties.as_deref());
      let rotation = category_axis_text_rotation_degrees(text_properties);
      suppress_duplicate_formatted_tick_budget(
        maximum_auto_main_increment_count(
          plot_width,
          maximum_tick_label_axis_pitch(
            &labels,
            &style.category_label,
            rotation,
            true,
            &mut metrics,
          ),
        ),
        &labels,
      )
    })
    .unwrap_or(maximum_auto_increment_count);
  let axis_scales = cartesian_axis_scales(
    chart,
    style.layout_profile,
    category_count,
    maximum_auto_value_increment_count,
    maximum_auto_horizontal_increment_count,
    Some((plot_width, plot_height)),
  );
  let Some(scale) = axis_scales.first().map(|axes| axes.y) else {
    return Vec::new();
  };
  if scale.maximum <= scale.minimum {
    return Vec::new();
  }
  let tick_labels = scale_tick_labels(
    scale.minimum,
    scale.maximum,
    scale.major_unit,
    value_number_format,
    scale.logarithmic_base,
    value_display_unit,
  );
  let secondary_value_tick_sets = axis_scales
    .iter()
    .enumerate()
    .skip(1)
    .filter_map(|(axis_set_index, axes)| {
      let axis = axis_set_value_axis(chart, axis_set_index)?;
      if !value_axis_is_visible(axis)
        || axis
          .tick_label_position
          .as_ref()
          .is_some_and(|position| position.val == Some(c::TickLabelPositionValues::None))
        || radar_only
        || horizontal_bar_only
      {
        return None;
      }
      let format_code = vertical_axis_number_format_code(chart, axis_set_index)
        .or_else(|| axis_set_is_percent_stacked(chart, axis_set_index).then_some("0%"));
      let labels = scale_tick_labels(
        axes.y.minimum,
        axes.y.maximum,
        axes.y.major_unit,
        format_code,
        axes.y.logarithmic_base,
        value_axis_display_unit(axis),
      );
      let width = labels
        .iter()
        .map(|(_, label)| metrics.measure_text(label, &style.value_label))
        .fold(0.0_f32, f32::max);
      Some((
        axis_set_index,
        axis,
        axes.y,
        labels,
        width,
        value_axis_is_on_right(axis),
      ))
    })
    .collect::<Vec<_>>();
  if projection_3d.is_none() {
    let final_maximum_tick_width = if value_tick_labels_visible {
      tick_labels
        .iter()
        .map(|(_, label)| metrics.measure_text(label, &style.value_label))
        .fold(0.0_f32, f32::max)
    } else {
      0.0
    };
    let final_secondary_left_value_axis_band_width = secondary_value_tick_sets
      .iter()
      .filter(|(_, _, _, _, _, on_right)| !on_right)
      .map(|(_, _, _, _, width, _)| *width + frame.height_pt * profiles::DEFAULT_TICK_GAP_RATIO)
      .sum::<f32>();
    let final_secondary_right_value_axis_band_width = secondary_value_tick_sets
      .iter()
      .filter(|(_, _, _, _, _, on_right)| *on_right)
      .map(|(_, _, _, _, width, _)| *width + frame.height_pt * profiles::DEFAULT_TICK_GAP_RATIO)
      .sum::<f32>();
    let final_primary_value_axis_band_width = final_maximum_tick_width + tick_gap;
    let final_left_value_axis_band_width = final_secondary_left_value_axis_band_width
      + if primary_value_axis_on_right {
        0.0
      } else {
        final_primary_value_axis_band_width
      };
    let final_right_value_axis_band_width = final_secondary_right_value_axis_band_width
      + if primary_value_axis_on_right {
        final_primary_value_axis_band_width
      } else {
        0.0
      };
    if chart
      .plot_layout
      .is_some_and(|layout| layout.targets_inner_plot)
    {
      if !primary_value_axis_on_right {
        tick_left = plot_left - final_maximum_tick_width - tick_gap;
      }
    } else {
      // ChartView lays out maximum labels with an initial scale, recalculates
      // the automatic scale for the final axis length, then lets the final
      // label boxes resize the residual plot rectangle. Keep that second
      // horizontal reflow: a change from multi-character half-unit labels to
      // single-character unit labels must move both the plot and its category
      // geometry, not merely replace the painted strings.
      plot_left += final_left_value_axis_band_width - left_value_axis_band_width;
      plot_right -= final_right_value_axis_band_width - right_value_axis_band_width;
    }
    if plot_right <= plot_left {
      return Vec::new();
    }
    plot_width = plot_right - plot_left;
    primary_value_axis_x = if primary_value_axis_on_right {
      plot_right
    } else {
      plot_left
    };
    primary_value_label_axis_x = primary_value_axis_x;
  }
  let painted_category_top = if horizontal_bar_only && chart.data_table.is_some() {
    projection_3d.map_or(category_top, |projection| {
      // A horizontal bar chart swaps the category/value coordinate system.
      // In a 3-D scene Office anchors the bottom value axis and its data
      // table to the projected back edge, not to the unprojected front
      // rectangle. Preserve the automatic 2-D gap between plot and table
      // after moving that edge through the authored view3D transform.
      let (_, projected_bottom) = projection.project(plot_left, plot_bottom, 1.0);
      projected_bottom + (category_top - plot_bottom)
    })
  } else {
    category_top
  };
  let axis_line_width = style
    .axis_line_width_pt
    .unwrap_or_else(|| automatic_axis_line_width_pt(style));
  let value_gridline_width = style.value_gridline_width_pt.unwrap_or(axis_line_width);
  let zero_y = value_y(
    0.0_f64.clamp(scale.minimum, scale.maximum),
    scale,
    plot_top,
    plot_height,
  );

  let mut items = Vec::new();
  push_chart_shape_rect(
    &mut items,
    frame.x_pt,
    frame.y_pt,
    frame.width_pt,
    frame.height_pt,
    Some(&style.chart_area_style.fill),
    None,
    None,
    1.0,
  );
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
  push_chart_shape_rect(
    &mut items,
    outline_x,
    outline_y,
    outline_width,
    outline_height,
    None,
    Some(&style.chart_area_style.stroke),
    None,
    1.0,
  );
  if let Some(projection) = projection_3d {
    lower_cartesian_3d_walls(
      &mut items,
      PlotRect {
        left: plot_left,
        top: plot_top,
        width: plot_width,
        height: plot_height,
      },
      projection,
      style,
    );
  } else {
    push_chart_shape_rect(
      &mut items,
      plot_left,
      plot_top,
      plot_width,
      plot_height,
      Some(&style.plot_area_style.fill),
      Some(&style.plot_area_style.stroke),
      None,
      1.0,
    );
  }
  if value_gridlines_visible {
    for (value, _) in &tick_labels {
      if matches!(
        style.layout_profile,
        ChartLayoutProfile::Word | ChartLayoutProfile::PowerPoint
      ) && category_axis_visible
        && value.abs() < f64::EPSILON
      {
        // The category-axis stroke owns the zero line in Office fixed output;
        // painting the coincident major gridline would darken it and duplicate
        // the path in the PDF content stream.
        continue;
      }
      let y = value_y(*value, scale, plot_top, plot_height);
      if let Some(projection) = projection_3d {
        let (front_x, front_y) = projection.project(plot_left, y, 0.0);
        let (back_left_x, back_left_y) = projection.project(plot_left, y, 1.0);
        let (back_right_x, back_right_y) = projection.project(plot_right, y, 1.0);
        items.push(PageItem::Line(LineItem {
          x1_pt: back_left_x,
          y1_pt: back_left_y,
          x2_pt: back_right_x,
          y2_pt: back_right_y,
          width_pt: value_gridline_width,
          color: style.gridline_color,
          kind: LineItemKind::Stroke,
        }));
        items.push(PageItem::Line(LineItem {
          x1_pt: front_x,
          y1_pt: front_y,
          x2_pt: back_left_x,
          y2_pt: back_left_y,
          width_pt: value_gridline_width,
          color: style.gridline_color,
          kind: LineItemKind::Stroke,
        }));
      } else {
        items.push(PageItem::Line(LineItem {
          x1_pt: plot_left,
          y1_pt: y,
          x2_pt: plot_right,
          y2_pt: y,
          width_pt: value_gridline_width,
          color: style.gridline_color,
          kind: LineItemKind::Stroke,
        }));
      }
    }
  }
  for (_, axis, secondary_scale, labels, _, _) in &secondary_value_tick_sets {
    if axis.major_gridlines.is_none() {
      continue;
    }
    for (value, _) in labels {
      let y = value_y(*value, *secondary_scale, plot_top, plot_height);
      if let Some(projection) = projection_3d {
        let (back_left_x, back_left_y) = projection.project(plot_left, y, 1.0);
        let (back_right_x, back_right_y) = projection.project(plot_right, y, 1.0);
        items.push(PageItem::Line(LineItem {
          x1_pt: back_left_x,
          y1_pt: back_left_y,
          x2_pt: back_right_x,
          y2_pt: back_right_y,
          width_pt: value_gridline_width,
          color: style.gridline_color,
          kind: LineItemKind::Stroke,
        }));
      } else {
        items.push(PageItem::Line(LineItem {
          x1_pt: plot_left,
          y1_pt: y,
          x2_pt: plot_right,
          y2_pt: y,
          width_pt: value_gridline_width,
          color: style.gridline_color,
          kind: LineItemKind::Stroke,
        }));
      }
    }
  }
  if let Some(ticks) = date_ticks.as_ref() {
    if let Some((color, width_pt)) = style.category_major_gridline {
      for tick in ticks.iter().filter(|tick| {
        tick.gridline_position > f64::EPSILON && tick.gridline_position < 1.0 + f64::EPSILON
      }) {
        let x = plot_left + tick.gridline_position as f32 * plot_width;
        items.push(PageItem::Line(LineItem {
          x1_pt: x,
          y1_pt: plot_top,
          x2_pt: x,
          y2_pt: plot_bottom,
          width_pt,
          color,
          kind: LineItemKind::Stroke,
        }));
      }
    }
    if let Some((color, width_pt)) = style.category_minor_gridline
      && let Some(positions) = date_axis_minor_tick_positions_with_maximum_auto_increment_count(
        chart,
        date_axis_increment_budget,
      )
    {
      for position in positions {
        if position <= f64::EPSILON || position >= 1.0 + f64::EPSILON {
          continue;
        }
        let x = plot_left + position as f32 * plot_width;
        items.push(PageItem::Line(LineItem {
          x1_pt: x,
          y1_pt: plot_top,
          x2_pt: x,
          y2_pt: plot_bottom,
          width_pt,
          color,
          kind: LineItemKind::Stroke,
        }));
      }
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
      has_powerpoint_derived_single_series_title,
    );
  } else if horizontal_bar_only {
    lower_horizontal_bar_axes(
      &mut items,
      chart,
      &tick_labels,
      HorizontalAxisGeometry {
        plot: PlotRect {
          left: plot_left,
          top: plot_top,
          width: plot_width,
          height: plot_height,
        },
        scale,
        projection_3d,
        draw_gridlines: true,
        draw_labels: false,
      },
      style,
      &mut metrics,
      has_powerpoint_derived_single_series_title,
    );
  } else if scatter_only && let Some(horizontal_scale) = axis_scales.first().and_then(|axes| axes.x)
  {
    lower_scatter_x_axis(
      &mut items,
      chart,
      PlotRect {
        left: plot_left,
        top: plot_top,
        width: plot_width,
        height: plot_height,
      },
      horizontal_scale,
      style,
      &mut metrics,
      true,
      false,
    );
  }

  let powerpoint_2d_cartesian_axes = style.layout_profile == ChartLayoutProfile::PowerPoint
    && projection_3d.is_none()
    && !radar_only
    && !horizontal_bar_only;
  if powerpoint_2d_cartesian_axes {
    // PowerPoint's fixed output paints automatic 2-D axes and their ticks
    // after gridlines but before the data series. The axis XML controls the
    // tick direction; c:majorTickMark="none" suppresses ticks without
    // suppressing the main axis line.
    if category_axis_visible {
      items.push(PageItem::Line(LineItem {
        x1_pt: plot_left,
        y1_pt: zero_y,
        x2_pt: plot_right,
        y2_pt: zero_y,
        width_pt: axis_line_width,
        color: style.gridline_color,
        kind: LineItemKind::Stroke,
      }));
      lower_powerpoint_horizontal_axis_major_ticks(
        &mut items,
        chart,
        PowerPointHorizontalAxisTickContext {
          plot: PlotRect {
            left: plot_left,
            top: plot_top,
            width: plot_width,
            height: plot_height,
          },
          axis_y: zero_y,
          x_scale: axis_scales.first().and_then(|axes| axes.x),
          date_ticks: date_ticks.as_deref(),
          category_count,
          width_pt: axis_line_width,
          color: style.gridline_color,
        },
      );
    }
    if value_axis_visible {
      items.push(PageItem::Line(LineItem {
        x1_pt: primary_value_axis_x,
        y1_pt: plot_top,
        x2_pt: primary_value_axis_x,
        y2_pt: plot_bottom,
        width_pt: axis_line_width,
        color: style.gridline_color,
        kind: LineItemKind::Stroke,
      }));
      lower_powerpoint_vertical_axis_major_ticks(
        &mut items,
        &tick_labels,
        PowerPointVerticalAxisTickContext {
          axis: chart.value_axis,
          axis_x: primary_value_axis_x,
          axis_on_right: primary_value_axis_on_right,
          scale,
          plot_top,
          plot_height,
          width_pt: axis_line_width,
          color: style.gridline_color,
        },
      );
    }
  }

  lower_chart_group_decorations(
    &mut items,
    chart,
    PlotRect {
      left: plot_left,
      top: plot_top,
      width: plot_width,
      height: plot_height,
    },
    &axis_scales,
    projection_3d,
    style,
    category_count,
  );
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
    &axis_scales,
    category_count,
    projection_3d,
  );
  if let Some(projection) = projection_3d {
    lower_series_axes(
      &mut items,
      chart,
      PlotRect {
        left: plot_left,
        top: plot_top,
        width: plot_width,
        height: plot_height,
      },
      projection,
      style,
      &mut metrics,
    );
  }
  if category_axis_visible && !radar_only && !horizontal_bar_only && !powerpoint_2d_cartesian_axes {
    let (axis_start, axis_end) =
      projection_3d.map_or(((plot_left, zero_y), (plot_right, zero_y)), |projection| {
        (
          projection.project(plot_left, zero_y, 0.0),
          projection.project(plot_right, zero_y, 0.0),
        )
      });
    items.push(PageItem::Line(LineItem {
      x1_pt: axis_start.0,
      y1_pt: axis_start.1,
      x2_pt: axis_end.0,
      y2_pt: axis_end.1,
      width_pt: axis_line_width,
      color: style.gridline_color,
      kind: LineItemKind::Stroke,
    }));
  }
  if style.layout_profile == ChartLayoutProfile::Word {
    let tick_length = frame.height_pt * 0.012_59;
    if value_axis_visible {
      let axis_x = primary_value_axis_x;
      let axis_top = projection_3d.map_or((axis_x, plot_top), |projection| {
        projection.project(axis_x, plot_top, primary_value_axis_depth)
      });
      let axis_zero = projection_3d.map_or((axis_x, zero_y), |projection| {
        projection.project(axis_x, zero_y, primary_value_axis_depth)
      });
      items.push(PageItem::Line(LineItem {
        x1_pt: axis_top.0,
        y1_pt: axis_top.1,
        x2_pt: axis_zero.0,
        y2_pt: axis_zero.1,
        width_pt: axis_line_width,
        color: style.gridline_color,
        kind: LineItemKind::Stroke,
      }));
      if chart.value_axis.is_none_or(value_axis_has_major_ticks) {
        for (value, _) in &tick_labels {
          let y = value_y(*value, scale, plot_top, plot_height);
          let point = projection_3d.map_or((axis_x, y), |projection| {
            projection.project(axis_x, y, primary_value_axis_depth)
          });
          items.push(PageItem::Line(LineItem {
            x1_pt: if primary_value_axis_on_right {
              point.0
            } else {
              point.0 - tick_length
            },
            y1_pt: point.1,
            x2_pt: if primary_value_axis_on_right {
              point.0 + tick_length
            } else {
              point.0
            },
            y2_pt: point.1,
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
      let tick_positions = date_ticks
        .as_ref()
        .map(|ticks| ticks.iter().map(|tick| tick.position).collect::<Vec<_>>())
        .unwrap_or_else(|| {
          (0..=category_count)
            .map(|boundary| boundary as f64 / category_count as f64)
            .collect()
        });
      for position in tick_positions {
        let position = if chart.category_axis_reversed {
          1.0 - position
        } else {
          position
        };
        let x = plot_left + position as f32 * plot_width;
        let point =
          projection_3d.map_or((x, zero_y), |projection| projection.project(x, zero_y, 0.0));
        items.push(PageItem::Line(LineItem {
          x1_pt: point.0,
          y1_pt: point.1,
          x2_pt: point.0,
          y2_pt: point.1 + tick_length,
          width_pt: axis_line_width,
          color: style.gridline_color,
          kind: LineItemKind::Stroke,
        }));
      }
    }
  }
  let secondary_tick_gap = frame.height_pt * profiles::DEFAULT_TICK_GAP_RATIO;
  let mut left_axis_offset = if !primary_value_axis_on_right && value_tick_labels_visible {
    primary_value_axis_band_width
  } else {
    0.0
  };
  let mut right_axis_offset = if primary_value_axis_on_right && value_tick_labels_visible {
    primary_value_axis_band_width
  } else {
    0.0
  };
  for (_, axis, secondary_scale, labels, width, on_right) in &secondary_value_tick_sets {
    let axis_x = if *on_right {
      plot_right + right_axis_offset
    } else {
      plot_left - left_axis_offset
    };
    items.push(PageItem::Line(LineItem {
      x1_pt: axis_x,
      y1_pt: plot_top,
      x2_pt: axis_x,
      y2_pt: plot_bottom,
      width_pt: axis_line_width,
      color: style.gridline_color,
      kind: LineItemKind::Stroke,
    }));
    if value_axis_has_major_ticks(axis) {
      let tick_length = frame.height_pt * 0.012_59;
      for (value, _) in labels {
        let y = value_y(*value, *secondary_scale, plot_top, plot_height);
        items.push(PageItem::Line(LineItem {
          x1_pt: if *on_right {
            axis_x
          } else {
            axis_x - tick_length
          },
          y1_pt: y,
          x2_pt: if *on_right {
            axis_x + tick_length
          } else {
            axis_x
          },
          y2_pt: y,
          width_pt: axis_line_width,
          color: style.gridline_color,
          kind: LineItemKind::Stroke,
        }));
      }
    }
    if *on_right {
      right_axis_offset += *width + secondary_tick_gap;
    } else {
      left_axis_offset += *width + secondary_tick_gap;
    }
  }

  let defer_data_labels = !scatter_only && !horizontal_bar_only && !radar_only;
  let mut deferred_data_label_items = Vec::new();
  for (series_index, series) in chart.series.iter().enumerate() {
    for (label_index, label) in series.data_labels.iter().enumerate() {
      let data_label_style = style
        .data_label_styles
        .get(series_index)
        .and_then(|styles| styles.get(label_index))
        .and_then(Option::as_ref)
        .unwrap_or(&style.data_label);
      let data_label_line_height = line_height(data_label_style);
      let axes = axis_scales
        .get(series.axis_set_index)
        .unwrap_or(&axis_scales[0]);
      let series_zero_y = value_y(
        0.0_f64.clamp(axes.y.minimum, axes.y.maximum),
        axes.y,
        plot_top,
        plot_height,
      );
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
        *axes,
        series_zero_y,
        category_count,
      ) else {
        continue;
      };
      let three_dimensional_label_offset =
        if series.is_3d && label.position != c::DataLabelPositionValues::Center {
          BAR_3D_DATA_LABEL_OFFSET_PT
        } else {
          0.0
        };
      let anchor = project_3d_data_label_anchor(chart, series_index, anchor, projection_3d);
      let rich_text_styles = style
        .data_label_rich_text_styles
        .get(series_index)
        .and_then(|styles| styles.get(label_index))
        .map(Vec::as_slice)
        .unwrap_or_default();
      let text_frame = resolved_data_label_text_frame(frame, label);
      let (width, data_label_height) = data_label_text_dimensions(
        &mut metrics,
        label,
        data_label_style,
        rich_text_styles,
        text_frame,
      );
      let (x, y) = match label.position {
        c::DataLabelPositionValues::Center => (
          (anchor.x + anchor.base_x) * 0.5 - width * 0.5,
          (anchor.y + anchor.base_y) * 0.5 - data_label_height * 0.5,
        ),
        c::DataLabelPositionValues::InsideBase => (
          anchor.base_x - width * 0.5,
          anchor.base_y - data_label_height,
        ),
        c::DataLabelPositionValues::InsideEnd => (anchor.x - width * 0.5, anchor.y),
        c::DataLabelPositionValues::Bottom => (
          anchor.x - width * 0.5,
          anchor.y + data_label_line_height * 0.15,
        ),
        c::DataLabelPositionValues::Left => (
          anchor.x - width - data_label_style.font_size_pt * 0.2,
          anchor.y - data_label_height * 0.5,
        ),
        c::DataLabelPositionValues::Right => (
          anchor.x
            + if has_unshifted_side_line_layout {
              chart_marker_size(&chart.series[series_index])
                .map(|size| size * style.stroke_scale)
                .unwrap_or(0.0)
            } else {
              0.0
            }
            + data_label_style.font_size_pt * 0.2,
          anchor.y - data_label_height * 0.5,
        ),
        c::DataLabelPositionValues::OutsideEnd | c::DataLabelPositionValues::BestFit => (
          anchor.x - width * 0.5,
          anchor.y
            - data_label_height
            - three_dimensional_label_offset
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
            chart_marker_size(&chart.series[series_index])
              .map(|size| size * style.stroke_scale * 0.5)
              .unwrap_or(0.0)
              + CARTESIAN_DATA_LABEL_OFFSET_PT
          } else {
            three_dimensional_label_offset
          };
          (
            anchor.x - width * 0.5,
            anchor.y - data_label_height - marker_clearance,
          )
        }
      };
      let (x, y) = label.layout.map_or((x, y), |layout| {
        let bounds = apply_manual_text_layout(
          frame,
          PlotRect {
            left: x,
            top: y,
            width,
            height: data_label_height,
          },
          layout,
        );
        (bounds.left, bounds.top)
      });
      if let Some(fill_color) = style
        .data_label_fill_colors
        .get(series_index)
        .and_then(|colors| colors.get(label_index))
        .copied()
        .flatten()
      {
        let horizontal_padding = data_label_style.font_size_pt * 0.25;
        let vertical_padding = data_label_style.font_size_pt * 0.26;
        let target = if defer_data_labels {
          &mut deferred_data_label_items
        } else {
          &mut items
        };
        target.push(PageItem::Rect(RectItem {
          x_pt: x - horizontal_padding,
          y_pt: y - vertical_padding,
          width_pt: width + horizontal_padding * 2.0,
          height_pt: data_label_height + vertical_padding * 2.0,
          fill_color: Some(fill_color),
          fill_opacity: 1.0,
          stroke: None,
          stroke_opacity: 1.0,
        }));
      }
      let target = if defer_data_labels {
        &mut deferred_data_label_items
      } else {
        &mut items
      };
      push_data_label_text_components(
        target,
        &mut metrics,
        x,
        y,
        label,
        data_label_style,
        rich_text_styles,
        text_frame,
      );
    }
  }
  if horizontal_bar_only {
    // Horizontal bar data labels are part of the series stream. Office emits
    // the projected value/category axis text after those labels, while the
    // grid geometry remains behind the series.
    lower_horizontal_bar_axes(
      &mut items,
      chart,
      &tick_labels,
      HorizontalAxisGeometry {
        plot: PlotRect {
          left: plot_left,
          top: plot_top,
          width: plot_width,
          height: plot_height,
        },
        scale,
        projection_3d: axis_text_projection_3d,
        draw_gridlines: false,
        draw_labels: true,
      },
      style,
      &mut metrics,
      has_powerpoint_derived_single_series_title,
    );
  }

  // Ordinary Office chart streams emit value ticks before categories and
  // legends. Legacy PowerPoint automatic-series-title charts instead retain
  // X/category-axis text before Y/value-axis text, so defer the latter for
  // that structural profile. A chart data table is emitted before the ticks.
  let mut deferred_value_tick_label_items = Vec::new();
  if value_tick_labels_visible && chart.data_table.is_none() {
    let target = if has_powerpoint_derived_single_series_title {
      &mut deferred_value_tick_label_items
    } else {
      &mut items
    };
    lower_cartesian_value_tick_labels(
      target,
      &tick_labels,
      ValueTickLabelContext {
        axis_x: primary_value_label_axis_x,
        labels_on_right: primary_value_axis_on_right,
        label_gap: primary_value_label_gap,
        scale,
        plot_top,
        plot_height,
        axis_depth: primary_value_label_axis_depth,
        value_label_line_height,
        tick_top_offset: frame.height_pt * excel_title_only_adjustment.tick_top_ratio,
        projection_3d: axis_text_projection_3d,
      },
      style,
      &mut metrics,
    );
  }
  if chart.data_table.is_none() {
    let secondary_tick_label_offset = secondary_tick_gap + frame.height_pt * 0.012_59;
    let mut left_axis_offset = if !primary_value_axis_on_right && value_tick_labels_visible {
      primary_value_axis_band_width
    } else {
      0.0
    };
    let mut right_axis_offset = if primary_value_axis_on_right && value_tick_labels_visible {
      primary_value_axis_band_width
    } else {
      0.0
    };
    for (_, _, secondary_scale, labels, width, on_right) in &secondary_value_tick_sets {
      let axis_x = if *on_right {
        plot_right + right_axis_offset
      } else {
        plot_left - left_axis_offset
      };
      for (value, label) in labels {
        let label_width = metrics.measure_text(label, &style.value_label);
        let target = if has_powerpoint_derived_single_series_title {
          &mut deferred_value_tick_label_items
        } else {
          &mut items
        };
        push_text(
          target,
          if *on_right {
            axis_x + secondary_tick_label_offset
          } else {
            axis_x - secondary_tick_label_offset - label_width
          },
          value_y(*value, *secondary_scale, plot_top, plot_height) - value_label_line_height / 2.0,
          label.clone(),
          style.value_label.clone(),
        );
      }
      if *on_right {
        right_axis_offset += *width + secondary_tick_gap;
      } else {
        left_axis_offset += *width + secondary_tick_gap;
      }
    }
  }
  if scatter_only && let Some(horizontal_scale) = axis_scales.first().and_then(|axes| axes.x) {
    // Excel emits scatter data labels first, followed by value-axis labels
    // and then category-axis labels. Keep the vertical gridline geometry
    // behind the plotted series above, but defer its text to the shared
    // Office chart-text ordering used here.
    lower_scatter_x_axis(
      &mut items,
      chart,
      PlotRect {
        left: plot_left,
        top: plot_top,
        width: plot_width,
        height: plot_height,
      },
      horizontal_scale,
      style,
      &mut metrics,
      false,
      true,
    );
  }
  let mut painted_category_label_style = style.category_label.clone();
  painted_category_label_style.rotation_deg = category_label_rotation;
  if style.layout_profile == ChartLayoutProfile::Word
    && category_tick_labels_visible
    && let Some(projection) = axis_text_projection_3d
  {
    painted_category_label_style.rotation_deg = chart_3d_category_label_rotation(
      chart,
      &category_label_lines,
      PlotRect {
        left: plot_left,
        top: plot_top,
        width: plot_width,
        height: plot_height,
      },
      zero_y,
      projection,
      &style.category_label,
      &mut metrics,
    );
    if style.layout_profile == ChartLayoutProfile::Word
      && painted_category_label_style.rotation_deg.abs() > f32::EPSILON
    {
      // Word's fixed-format writer emits automatically rotated 3-D tick
      // labels as vector glyph outlines (the visible dates in chart-Area are
      // absent from the PDF text layer). Keep them visible without inventing
      // a searchable semantic overlay.
      painted_category_label_style.pdf_glyph_outlines = true;
      painted_category_label_style.pdf_glyph_outline_options =
        Some(Arc::new(crate::common::PdfGlyphOutlineOptions {
          semantic_text_overlay: false,
          ..crate::common::PdfGlyphOutlineOptions::default()
        }));
    }
  }
  if category_tick_labels_visible {
    for (category_index, lines) in category_label_lines.iter().enumerate() {
      let center = if let Some(ticks) = date_ticks.as_ref() {
        let position = if chart.category_axis_reversed {
          1.0 - ticks[category_index].position
        } else {
          ticks[category_index].position
        };
        plot_left + position as f32 * plot_width
      } else {
        let display_index = category_display_index(chart, category_index, category_count);
        category_point_x(
          chart,
          display_index,
          category_count,
          PlotRect {
            left: plot_left,
            top: plot_top,
            width: plot_width,
            height: plot_height,
          },
        )
      };
      for (line_index, line) in lines.iter().enumerate() {
        let width = metrics.measure_text(line, &painted_category_label_style);
        let (x, y, rotation_center) = axis_text_projection_3d.map_or_else(
          || {
            if painted_category_label_style.rotation_deg.abs() <= f32::EPSILON {
              return (
                center - width / 2.0,
                category_top + line_index as f32 * category_label_line_height,
                None,
              );
            }
            // DrawingML rotates the complete tick-label rectangle about its
            // center. Anchor the resulting axis-aligned box at category_top;
            // rotating around the text origin instead makes a -90 degree
            // label climb into the plot by roughly its full advance width.
            let angle = painted_category_label_style.rotation_deg.to_radians();
            let rotated_height =
              width * angle.sin().abs() + category_label_line_height * angle.cos().abs();
            let center_y =
              category_top + line_index as f32 * category_label_line_height + rotated_height * 0.5;
            (
              center - width / 2.0,
              center_y - category_label_line_height * 0.5,
              Some((center, center_y)),
            )
          },
          |projection| {
            let point = projection.project(center, zero_y, 0.0);
            let label_gap_em = if style.layout_profile == ChartLayoutProfile::PowerPoint
              && cartesian_3d_series_axis_labels_visible(chart)
            {
              profiles::POWERPOINT_CARTESIAN_3D_CATEGORY_LABEL_GAP_EM
            } else {
              0.45
            };
            // The em term accounts for the projected major-tick reach. Keep
            // LO's independent 1 mm tick-to-text spacing as a physical gap,
            // just as for the projected value axis above.
            let label_gap = painted_category_label_style.font_size_pt * label_gap_em
              + CARTESIAN_DATA_LABEL_OFFSET_PT;
            if painted_category_label_style.rotation_deg.abs() > f32::EPSILON {
              (
                point.0 - painted_category_label_style.font_size_pt * 0.15,
                point.1 + label_gap + line_index as f32 * category_label_line_height,
                None,
              )
            } else {
              (
                point.0 - width / 2.0,
                point.1 + label_gap + line_index as f32 * category_label_line_height,
                None,
              )
            }
          },
        );
        push_text_with_rotation_center(
          &mut items,
          x,
          y,
          line.clone(),
          painted_category_label_style.clone(),
          rotation_center,
        );
      }
    }
  }
  items.append(&mut deferred_value_tick_label_items);
  items.append(&mut deferred_data_label_items);
  if let Some(axis) = chart.value_axis
    && let Some(label) = value_axis_display_unit_label_text(axis, chart.ui_language.as_deref())
  {
    let mut unit_style = style.value_label.clone();
    // LibreOffice's source-backed OBJECTTYPE_AXISUNIT uses the same automatic
    // text role as an axis title (ObjectFormatter::spAxisTitleTexts), whose
    // default is bold. Office fixed output likewise strokes the synthesized
    // CJK face for an otherwise empty c:dispUnitsLbl.
    unit_style.bold = true;
    unit_style.rotation_deg = -90.0;
    push_text(
      &mut items,
      if primary_value_axis_on_right {
        plot_right + tick_gap + unit_style.font_size_pt
      } else {
        plot_left - tick_gap - unit_style.font_size_pt
      },
      value_y(
        (scale.maximum - scale.major_unit).max(scale.minimum),
        scale,
        plot_top,
        plot_height,
      ) - value_label_line_height / 2.0,
      label,
      unit_style,
    );
  }
  let mut left_axis_offset = if !primary_value_axis_on_right && value_tick_labels_visible {
    primary_value_axis_band_width
  } else {
    0.0
  };
  let mut right_axis_offset = if primary_value_axis_on_right && value_tick_labels_visible {
    primary_value_axis_band_width
  } else {
    0.0
  };
  for (_, axis, secondary_scale, _, width, on_right) in &secondary_value_tick_sets {
    let axis_x = if *on_right {
      plot_right + right_axis_offset
    } else {
      plot_left - left_axis_offset
    };
    if let Some(label) = value_axis_display_unit_label_text(axis, chart.ui_language.as_deref()) {
      let mut unit_style = style.value_label.clone();
      unit_style.bold = true;
      unit_style.rotation_deg = -90.0;
      push_text(
        &mut items,
        if *on_right {
          axis_x + secondary_tick_gap + unit_style.font_size_pt
        } else {
          axis_x - secondary_tick_gap - unit_style.font_size_pt
        },
        value_y(
          (secondary_scale.maximum - secondary_scale.major_unit).max(secondary_scale.minimum),
          *secondary_scale,
          plot_top,
          plot_height,
        ) - value_label_line_height / 2.0,
        label,
        unit_style,
      );
    }
    if *on_right {
      right_axis_offset += *width + secondary_tick_gap;
    } else {
      left_axis_offset += *width + secondary_tick_gap;
    }
  }
  if chart.data_table.is_some() {
    lower_axis_titles(
      &mut items,
      AxisTitleGeometry {
        frame,
        plot: PlotRect {
          left: plot_left,
          top: plot_top,
          width: plot_width,
          height: plot_height,
        },
        value_label_band_left: tick_left,
        category_band_top: painted_category_top,
        category_label_height,
        data_table_height,
        projection_3d,
      },
      chart,
      style,
      &mut metrics,
    );
  }
  if let Some(data_table) = chart.data_table {
    lower_data_table(
      &mut items,
      chart,
      data_table,
      PlotRect {
        left: plot_left,
        top: painted_category_top,
        width: plot_width,
        height: data_table_height,
      },
      style,
      &mut metrics,
    );
    if value_tick_labels_visible {
      lower_cartesian_value_tick_labels(
        &mut items,
        &tick_labels,
        ValueTickLabelContext {
          axis_x: primary_value_label_axis_x,
          labels_on_right: primary_value_axis_on_right,
          label_gap: primary_value_label_gap,
          scale,
          plot_top,
          plot_height,
          axis_depth: primary_value_label_axis_depth,
          value_label_line_height,
          tick_top_offset: frame.height_pt * excel_title_only_adjustment.tick_top_ratio,
          projection_3d: axis_text_projection_3d,
        },
        style,
        &mut metrics,
      );
    }
    let secondary_tick_label_offset = secondary_tick_gap + frame.height_pt * 0.012_59;
    let mut left_axis_offset = if !primary_value_axis_on_right && value_tick_labels_visible {
      primary_value_axis_band_width
    } else {
      0.0
    };
    let mut right_axis_offset = if primary_value_axis_on_right && value_tick_labels_visible {
      primary_value_axis_band_width
    } else {
      0.0
    };
    for (_, _, secondary_scale, labels, width, on_right) in &secondary_value_tick_sets {
      let axis_x = if *on_right {
        plot_right + right_axis_offset
      } else {
        plot_left - left_axis_offset
      };
      for (value, label) in labels {
        let label_width = metrics.measure_text(label, &style.value_label);
        push_text(
          &mut items,
          if *on_right {
            axis_x + secondary_tick_label_offset
          } else {
            axis_x - secondary_tick_label_offset - label_width
          },
          value_y(*value, *secondary_scale, plot_top, plot_height) - value_label_line_height / 2.0,
          label.clone(),
          style.value_label.clone(),
        );
      }
      if *on_right {
        right_axis_offset += *width + secondary_tick_gap;
      } else {
        left_axis_offset += *width + secondary_tick_gap;
      }
    }
  }
  if chart.data_table.is_none() {
    lower_axis_titles(
      &mut items,
      AxisTitleGeometry {
        frame,
        plot: PlotRect {
          left: plot_left,
          top: plot_top,
          width: plot_width,
          height: plot_height,
        },
        value_label_band_left: tick_left,
        category_band_top: painted_category_top,
        category_label_height,
        data_table_height,
        projection_3d,
      },
      chart,
      style,
      &mut metrics,
    );
  }
  if let Some(title) = title_text {
    let automatic_title_top = title_top
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
    let mut title_style = style.title.clone();
    title_style.rotation_deg += chart.title_rotation_deg;
    if style.layout_profile == ChartLayoutProfile::Excel
      && chart.title_rotation_deg.abs() > f32::EPSILON
    {
      // Excel's fixed-format writer emits independently rotated chart-title
      // text as glyph outlines, just as it does for rotated worksheet-shape
      // text. Keep the visual title but do not invent extractable PDF text.
      title_style.pdf_glyph_outlines = true;
      let mut options = title_style
        .pdf_glyph_outline_options
        .as_deref()
        .cloned()
        .unwrap_or_default();
      options.semantic_text_overlay = false;
      title_style.pdf_glyph_outline_options = Some(Arc::new(options));
    }
    let width = metrics.measure_text(title, &title_style);
    let automatic_title_x = frame.x_pt + (frame.width_pt - width) / 2.0
      - if has_explicit_single_series_side_title_layout {
        frame.height_pt * 0.004_76
      } else if has_explicit_bottom_column_layout {
        frame.height_pt * 0.003_23
      } else {
        0.0
      };
    let title_bounds = chart.title_layout.map_or(
      PlotRect {
        left: automatic_title_x,
        top: automatic_title_top,
        width,
        height: title_line_height,
      },
      |layout| {
        apply_manual_text_layout(
          frame,
          PlotRect {
            left: automatic_title_x,
            top: automatic_title_top,
            width,
            height: title_line_height,
          },
          layout,
        )
      },
    );
    let title_x = title_bounds.left;
    let painted_title_top = title_bounds.top;
    if let Some(color) = style.title_fill_color {
      let horizontal_padding = title_style.font_size_pt * 0.162;
      let vertical_padding = title_style.font_size_pt * 0.092;
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
      title_style,
    );
  }
  if let Some(layout) = chart.legend_layout {
    lower_manual_legend(&mut items, frame, layout, chart, style, scale);
  } else {
    match legend_position {
      Some(ChartLegendPosition::Bottom) => lower_horizontal_legend(
        &mut items,
        frame,
        tick_left,
        legend_top,
        chart,
        style,
        scale,
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
        scale,
        &mut metrics,
      ),
      Some(ChartLegendPosition::Left) => lower_vertical_legend(
        &mut items,
        frame.x_pt + side_legend_outer_margin,
        frame,
        false,
        chart,
        style,
        scale,
      ),
      Some(ChartLegendPosition::Right | ChartLegendPosition::TopRight) => lower_vertical_legend(
        &mut items,
        frame.x_pt + frame.width_pt - side_legend_outer_margin - side_legend_width
          + if has_explicit_single_series_side_title_layout {
            -frame.height_pt * 0.009_84
          } else if has_excel_vary_colors_data_table_layout {
            frame.height_pt * profiles::EXCEL_VARY_COLORS_DATA_TABLE_LEGEND_X_RATIO
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
        scale,
      ),
      None => {}
    }
  }
  items
}

#[derive(Clone, Copy)]
struct ValueTickLabelContext {
  axis_x: f32,
  labels_on_right: bool,
  label_gap: f32,
  scale: crate::render::chart::LinearAxisScale,
  plot_top: f32,
  plot_height: f32,
  axis_depth: f32,
  value_label_line_height: f32,
  tick_top_offset: f32,
  projection_3d: Option<Chart3DProjection>,
}

fn lower_cartesian_value_tick_labels(
  items: &mut Vec<PageItem>,
  tick_labels: &[(f64, String)],
  context: ValueTickLabelContext,
  style: &ClusteredColumnStyle,
  metrics: &mut TextMetrics,
) {
  for (value, label) in tick_labels {
    let width = metrics.measure_text(label, &style.value_label);
    let model_y = value_y(*value, context.scale, context.plot_top, context.plot_height);
    let (axis_x, axis_y) = context
      .projection_3d
      .map_or((context.axis_x, model_y), |projection| {
        projection.project(context.axis_x, model_y, context.axis_depth)
      });
    push_text(
      items,
      if context.labels_on_right {
        axis_x + context.label_gap
      } else {
        axis_x - context.label_gap - width
      },
      axis_y - context.value_label_line_height / 2.0
        + context.tick_top_offset
        + if context.projection_3d.is_some() {
          // The projected tick is the centre of the font em box. Our chart
          // line box has 1.2 em leading, so centring that whole box biases the
          // glyphs upward by half of the extra 0.2 em. Two-dimensional host
          // profiles already include this in their calibrated axis bands.
          style.value_label.font_size_pt * 0.1
        } else {
          0.0
        },
      label.clone(),
      style.value_label.clone(),
    );
  }
}

fn lower_cartesian_3d_walls(
  items: &mut Vec<PageItem>,
  plot: PlotRect,
  projection: Chart3DProjection,
  style: &ClusteredColumnStyle,
) {
  let front_top_left = projection.project(plot.left, plot.top, 0.0);
  let front_top_right = projection.project(plot.left + plot.width, plot.top, 0.0);
  let front_bottom_left = projection.project(plot.left, plot.top + plot.height, 0.0);
  let front_bottom_right = projection.project(plot.left + plot.width, plot.top + plot.height, 0.0);
  let back_top_left = projection.project(plot.left, plot.top, 1.0);
  let back_top_right = projection.project(plot.left + plot.width, plot.top, 1.0);
  let back_bottom_left = projection.project(plot.left, plot.top + plot.height, 1.0);
  let back_bottom_right = projection.project(plot.left + plot.width, plot.top + plot.height, 1.0);

  let back_color = (style.chart_style_id > 32)
    .then(|| chart_style_fill_fallback_color(&style.plot_area_style))
    .flatten()
    .or((style.chart_style_id > 32).then_some(RgbColor {
      r: 250,
      g: 250,
      b: 247,
    }));
  let (outline_color, outline_width) = chart_style_stroke_fallback(&style.plot_area_style)
    .unwrap_or((style.gridline_color, 0.75 * style.stroke_scale));
  push_chart_styled_polygon(
    items,
    &[
      back_top_left,
      back_top_right,
      back_bottom_right,
      back_bottom_left,
    ],
    &style.back_wall_style,
    back_color,
    Some((outline_color, outline_width)),
    style.stroke_scale,
  );

  let (depth_x, _) = projection.depth_vector();
  let (side_front_top, side_front_bottom, side_back_top, side_back_bottom) = if depth_x >= 0.0 {
    (
      front_top_left,
      front_bottom_left,
      back_top_left,
      back_bottom_left,
    )
  } else {
    (
      front_top_right,
      front_bottom_right,
      back_top_right,
      back_bottom_right,
    )
  };
  push_chart_styled_polygon(
    items,
    &[
      side_front_top,
      side_back_top,
      side_back_bottom,
      side_front_bottom,
    ],
    &style.side_wall_style,
    back_color.map(|color| shade_chart_color(color, 0.72)),
    Some((outline_color, outline_width)),
    style.stroke_scale,
  );
  push_chart_styled_polygon(
    items,
    &[
      front_bottom_left,
      back_bottom_left,
      back_bottom_right,
      front_bottom_right,
    ],
    &style.floor_style,
    back_color.map(|color| shade_chart_color(color, 0.88)),
    Some((outline_color, outline_width)),
    style.stroke_scale,
  );
}

fn push_chart_styled_polygon(
  items: &mut Vec<PageItem>,
  points: &[(f32, f32)],
  style: &crate::common::ShapeStyle<'static>,
  fallback_fill_color: Option<RgbColor>,
  fallback_stroke: Option<(RgbColor, f32)>,
  stroke_width_scale: f32,
) {
  if points.len() < 3 {
    return;
  }
  let (left, top, right, bottom) = points.iter().fold(
    (
      f32::INFINITY,
      f32::INFINITY,
      f32::NEG_INFINITY,
      f32::NEG_INFINITY,
    ),
    |(left, top, right, bottom), (x, y)| (left.min(*x), top.min(*y), right.max(*x), bottom.max(*y)),
  );
  let bounds = common_rect(left, top, right - left, bottom - top);
  let fill = match &style.fill {
    crate::common::ShapeStyleValue::Paint(fill) => bind_chart_fill_to_bounds(fill, bounds),
    crate::common::ShapeStyleValue::NoPaint => crate::common::Fill::None,
    crate::common::ShapeStyleValue::Unspecified => fallback_fill_color
      .map(|color| crate::common::Fill::Solid(common_rgb(color, 1.0)))
      .unwrap_or(crate::common::Fill::None),
  };
  let stroke = match &style.stroke {
    crate::common::ShapeStyleValue::Paint(stroke) => Some(bind_chart_stroke_to_bounds(
      stroke,
      bounds,
      stroke_width_scale,
    )),
    crate::common::ShapeStyleValue::NoPaint => None,
    crate::common::ShapeStyleValue::Unspecified => {
      fallback_stroke.map(|(color, width_pt)| crate::common::Stroke {
        width: crate::common::Pt(width_pt),
        color: common_rgb(color, 1.0),
        ..Default::default()
      })
    }
  };
  if fill == crate::common::Fill::None && stroke.is_none() {
    return;
  }
  items.push(PageItem::Path(crate::common::PathItem {
    bounds,
    points: points.iter().map(|(x, y)| common_point(*x, *y)).collect(),
    commands: Vec::new(),
    closed: true,
    fill,
    stroke,
  }));
}

fn push_chart_polygon(
  items: &mut Vec<PageItem>,
  points: &[(f32, f32)],
  color: RgbColor,
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
    fill: crate::common::Fill::Solid(common_rgb(color, 1.0)),
    stroke: stroke.map(|(color, width_pt)| crate::common::Stroke {
      width: crate::common::Pt(width_pt),
      color: common_rgb(color, 1.0),
      dash: None,
      source_style_id: None,
      ..Default::default()
    }),
  }));
}

fn shade_chart_color(color: RgbColor, factor: f32) -> RgbColor {
  let channel = |value: u8| (f32::from(value) * factor).round().clamp(0.0, 255.0) as u8;
  RgbColor {
    r: channel(color.r),
    g: channel(color.g),
    b: channel(color.b),
  }
}

fn tint_chart_color(color: RgbColor, factor: f32) -> RgbColor {
  let channel = |value: u8| {
    (f32::from(value) + (255.0 - f32::from(value)) * factor)
      .round()
      .clamp(0.0, 255.0) as u8
  };
  RgbColor {
    r: channel(color.r),
    g: channel(color.g),
    b: channel(color.b),
  }
}

fn bind_chart_gradient_to_bounds(
  gradient: &crate::common::GradientFill<'static>,
  bounds: crate::common::Rect,
) -> crate::common::GradientFill<'static> {
  let mut gradient = gradient.clone();
  gradient.definition_bounds = Some(bounds);
  if let Some(path) = gradient.path.as_mut() {
    path.transform =
      crate::common::drawingml_gradient::bind_path_transform_to_bounds(path.transform, bounds);
    if path.kind == crate::common::GradientPathKind::Circle {
      path.transform = crate::common::office_circle_gradient_transform(path.transform);
    }
  }
  gradient
}

fn bind_chart_fill_to_bounds(
  fill: &crate::common::Fill<'static>,
  bounds: crate::common::Rect,
) -> crate::common::Fill<'static> {
  match fill {
    crate::common::Fill::Gradient(gradient) => {
      crate::common::Fill::Gradient(bind_chart_gradient_to_bounds(gradient, bounds))
    }
    fill => fill.clone(),
  }
}

fn bind_chart_stroke_to_bounds(
  stroke: &crate::common::Stroke<'static>,
  bounds: crate::common::Rect,
  width_scale: f32,
) -> crate::common::Stroke<'static> {
  let mut stroke = stroke.clone();
  stroke.width.0 *= width_scale;
  if let Some(gradient) = stroke.gradient.as_mut() {
    *gradient = bind_chart_gradient_to_bounds(gradient, bounds);
  }
  stroke
}

fn chart_common_color(color: crate::common::Color) -> RgbColor {
  RgbColor {
    r: color.r,
    g: color.g,
    b: color.b,
  }
}

fn chart_fill_fallback_color(fill: &crate::common::Fill<'_>) -> Option<RgbColor> {
  match fill {
    crate::common::Fill::Solid(color) => Some(chart_common_color(*color)),
    crate::common::Fill::Gradient(gradient) => gradient
      .stops
      .first()
      .map(|stop| chart_common_color(stop.color)),
    crate::common::Fill::Pattern(pattern) => Some(chart_common_color(pattern.foreground)),
    crate::common::Fill::None
    | crate::common::Fill::Theme(_)
    | crate::common::Fill::Image { .. } => None,
  }
}

fn chart_style_fill_fallback_color(style: &crate::common::ShapeStyle<'_>) -> Option<RgbColor> {
  match &style.fill {
    crate::common::ShapeStyleValue::Paint(fill) => chart_fill_fallback_color(fill),
    crate::common::ShapeStyleValue::Unspecified | crate::common::ShapeStyleValue::NoPaint => None,
  }
}

fn chart_style_stroke_fallback(style: &crate::common::ShapeStyle<'_>) -> Option<(RgbColor, f32)> {
  match &style.stroke {
    crate::common::ShapeStyleValue::Paint(stroke) => {
      Some((chart_common_color(stroke.color), stroke.width.0))
    }
    crate::common::ShapeStyleValue::Unspecified | crate::common::ShapeStyleValue::NoPaint => None,
  }
}

fn push_chart_shape_rect(
  items: &mut Vec<PageItem>,
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  fill: Option<&crate::common::ShapeStyleValue<crate::common::Fill<'static>>>,
  stroke: Option<&crate::common::ShapeStyleValue<crate::common::Stroke<'static>>>,
  fallback_fill_color: Option<RgbColor>,
  stroke_width_scale: f32,
) {
  if width_pt <= 0.0 || height_pt <= 0.0 {
    return;
  }
  let bounds = common_rect(x_pt, y_pt, width_pt, height_pt);
  let fill = match fill {
    Some(crate::common::ShapeStyleValue::Paint(fill)) => bind_chart_fill_to_bounds(fill, bounds),
    Some(crate::common::ShapeStyleValue::NoPaint) => crate::common::Fill::None,
    Some(crate::common::ShapeStyleValue::Unspecified) | None => fallback_fill_color
      .map(|color| crate::common::Fill::Solid(common_rgb(color, 1.0)))
      .unwrap_or(crate::common::Fill::None),
  };
  let stroke = match stroke {
    Some(crate::common::ShapeStyleValue::Paint(stroke)) => Some(bind_chart_stroke_to_bounds(
      stroke,
      bounds,
      stroke_width_scale,
    )),
    Some(crate::common::ShapeStyleValue::NoPaint | crate::common::ShapeStyleValue::Unspecified)
    | None => None,
  };
  if fill == crate::common::Fill::None && stroke.is_none() {
    return;
  }
  items.push(PageItem::Path(crate::common::PathItem {
    bounds,
    points: vec![
      common_point(x_pt, y_pt),
      common_point(x_pt + width_pt, y_pt),
      common_point(x_pt + width_pt, y_pt + height_pt),
      common_point(x_pt, y_pt + height_pt),
    ],
    commands: Vec::new(),
    closed: true,
    fill,
    stroke,
  }));
}

fn push_chart_shape_ellipse(
  items: &mut Vec<PageItem>,
  center_x_pt: f32,
  center_y_pt: f32,
  diameter_pt: f32,
  fill: Option<&crate::common::ShapeStyleValue<crate::common::Fill<'static>>>,
  stroke: Option<&crate::common::ShapeStyleValue<crate::common::Stroke<'static>>>,
  fallback_fill_color: Option<RgbColor>,
  stroke_width_scale: f32,
) {
  if diameter_pt <= 0.0 {
    return;
  }
  let left = center_x_pt - diameter_pt * 0.5;
  let top = center_y_pt - diameter_pt * 0.5;
  let bounds = common_rect(left, top, diameter_pt, diameter_pt);
  let fill = match fill {
    Some(crate::common::ShapeStyleValue::Paint(fill)) => bind_chart_fill_to_bounds(fill, bounds),
    Some(crate::common::ShapeStyleValue::NoPaint) => crate::common::Fill::None,
    Some(crate::common::ShapeStyleValue::Unspecified) | None => fallback_fill_color
      .map(|color| crate::common::Fill::Solid(common_rgb(color, 1.0)))
      .unwrap_or(crate::common::Fill::None),
  };
  let stroke = match stroke {
    Some(crate::common::ShapeStyleValue::Paint(stroke)) => Some(bind_chart_stroke_to_bounds(
      stroke,
      bounds,
      stroke_width_scale,
    )),
    Some(crate::common::ShapeStyleValue::NoPaint | crate::common::ShapeStyleValue::Unspecified)
    | None => None,
  };
  if fill == crate::common::Fill::None && stroke.is_none() {
    return;
  }
  const SEGMENT_COUNT: usize = 32;
  let radius = diameter_pt * 0.5;
  let points = (0..SEGMENT_COUNT)
    .map(|index| {
      let angle = std::f32::consts::TAU * index as f32 / SEGMENT_COUNT as f32;
      common_point(
        center_x_pt + radius * angle.cos(),
        center_y_pt + radius * angle.sin(),
      )
    })
    .collect();
  items.push(PageItem::Path(crate::common::PathItem {
    bounds,
    points,
    commands: Vec::new(),
    closed: true,
    fill,
    stroke,
  }));
}

fn push_chart_styled_line(
  items: &mut Vec<PageItem>,
  start: (f32, f32),
  end: (f32, f32),
  stroke: Option<&crate::common::ShapeStyleValue<crate::common::Stroke<'static>>>,
  fallback_color: RgbColor,
  fallback_width_pt: f32,
  stroke_width_scale: f32,
) {
  let bounds = common_rect(
    start.0.min(end.0),
    start.1.min(end.1),
    (end.0 - start.0).abs(),
    (end.1 - start.1).abs(),
  );
  let stroke = match stroke {
    Some(crate::common::ShapeStyleValue::NoPaint) => return,
    Some(crate::common::ShapeStyleValue::Paint(stroke)) => {
      bind_chart_stroke_to_bounds(stroke, bounds, stroke_width_scale)
    }
    Some(crate::common::ShapeStyleValue::Unspecified) | None => crate::common::Stroke {
      width: crate::common::Pt(fallback_width_pt),
      color: common_rgb(fallback_color, 1.0),
      ..Default::default()
    },
  };
  items.push(PageItem::Path(crate::common::PathItem {
    bounds,
    points: vec![common_point(start.0, start.1), common_point(end.0, end.1)],
    commands: Vec::new(),
    closed: false,
    fill: crate::common::Fill::None,
    stroke: Some(stroke),
  }));
}

fn push_chart_styled_polyline(
  items: &mut Vec<PageItem>,
  points: &[(f32, f32)],
  stroke: Option<&crate::common::ShapeStyleValue<crate::common::Stroke<'static>>>,
  fallback_color: RgbColor,
  fallback_width_pt: f32,
  stroke_width_scale: f32,
) {
  if points.len() < 2 {
    return;
  }
  let (left, top, right, bottom) = points.iter().fold(
    (
      f32::INFINITY,
      f32::INFINITY,
      f32::NEG_INFINITY,
      f32::NEG_INFINITY,
    ),
    |(left, top, right, bottom), (x, y)| (left.min(*x), top.min(*y), right.max(*x), bottom.max(*y)),
  );
  let bounds = common_rect(left, top, right - left, bottom - top);
  let stroke = match stroke {
    Some(crate::common::ShapeStyleValue::NoPaint) => return,
    Some(crate::common::ShapeStyleValue::Paint(stroke)) => {
      bind_chart_stroke_to_bounds(stroke, bounds, stroke_width_scale)
    }
    Some(crate::common::ShapeStyleValue::Unspecified) | None => crate::common::Stroke {
      width: crate::common::Pt(fallback_width_pt),
      color: common_rgb(fallback_color, 1.0),
      ..Default::default()
    },
  };
  items.push(PageItem::Path(crate::common::PathItem {
    bounds,
    points: points.iter().map(|(x, y)| common_point(*x, *y)).collect(),
    commands: Vec::new(),
    closed: false,
    fill: crate::common::Fill::None,
    stroke: Some(stroke),
  }));
}

/// Clips a chart polyline to the visible plot rectangle using the same
/// Liang-Barsky segment algorithm as LibreOffice chart2 `Clipping.cxx`.
/// Separate visible pieces remain separate paths so a stroke cannot bridge an
/// interval that lies wholly outside the authored axis window.
fn clip_chart_polyline_to_plot(points: &[(f32, f32)], plot: PlotRect) -> Vec<Vec<(f32, f32)>> {
  let mut clipped_runs = Vec::new();
  let mut current = Vec::new();
  for segment in points.windows(2) {
    let Some((start, end)) = clip_chart_line_segment_to_plot(segment[0], segment[1], plot) else {
      if current.len() >= 2 {
        clipped_runs.push(std::mem::take(&mut current));
      } else {
        current.clear();
      }
      continue;
    };
    if start == end {
      continue;
    }
    if current.last().copied() == Some(start) {
      current.push(end);
    } else {
      if current.len() >= 2 {
        clipped_runs.push(std::mem::take(&mut current));
      } else {
        current.clear();
      }
      current.extend([start, end]);
    }
  }
  if current.len() >= 2 {
    clipped_runs.push(current);
  }
  clipped_runs
}

fn clip_chart_line_segment_to_plot(
  start: (f32, f32),
  end: (f32, f32),
  plot: PlotRect,
) -> Option<((f32, f32), (f32, f32))> {
  let direction = (end.0 - start.0, end.1 - start.1);
  if direction.0 == 0.0 && direction.1 == 0.0 {
    return chart_point_inside_plot(start, plot).then_some((start, end));
  }
  let mut enter = 0.0_f32;
  let mut leave = 1.0_f32;
  for (denominator, numerator) in [
    (direction.0, plot.left - start.0),
    (-direction.0, start.0 - (plot.left + plot.width)),
    (direction.1, plot.top - start.1),
    (-direction.1, start.1 - (plot.top + plot.height)),
  ] {
    if denominator > 0.0 {
      let ratio = numerator / denominator;
      if ratio > leave {
        return None;
      }
      enter = enter.max(ratio);
    } else if denominator < 0.0 {
      let ratio = numerator / denominator;
      if ratio < enter {
        return None;
      }
      leave = leave.min(ratio);
    } else if numerator > 0.0 {
      return None;
    }
  }
  Some((
    (start.0 + enter * direction.0, start.1 + enter * direction.1),
    (start.0 + leave * direction.0, start.1 + leave * direction.1),
  ))
}

fn chart_point_inside_plot(point: (f32, f32), plot: PlotRect) -> bool {
  point.0 >= plot.left
    && point.0 <= plot.left + plot.width
    && point.1 >= plot.top
    && point.1 <= plot.top + plot.height
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
      .map(|text| metrics.measure_text(text, &style.legend))
      .fold(0.0_f32, f32::max);
    maximum_label_width + style.legend.font_size_pt * host_defaults.side_legend_width_em
  } else {
    0.0
  };
  let legend_height = if bottom_legend || top_legend {
    line_height(&style.legend) * 1.8
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
  let has_powerpoint_automatic_radial_plot = style.layout_profile == ChartLayoutProfile::PowerPoint
    && chart.plot_layout.is_none()
    && matches!(chart.kind, RadialChartKind::Pie | RadialChartKind::Doughnut);
  if has_powerpoint_automatic_radial_plot {
    // LibreOffice ChartView gives automatic pie diagrams a fixed 3.5 mm
    // four-sided inset and then reduces the diagram to its preferred 1:1
    // aspect. This is intentionally applied after title/legend bands have
    // claimed their space and never overrides an authored manual layout.
    let inset = profiles::POWERPOINT_AUTOMATIC_PIE_FIXED_INSET_PT;
    plot.left += inset;
    plot.top += inset;
    plot.width = (plot.width - inset * 2.0).max(0.0);
    plot.height = (plot.height - inset * 2.0).max(0.0);
    let side = plot.width.min(plot.height);
    plot.left += (plot.width - side) * 0.5;
    plot.top += (plot.height - side) * 0.5;
    plot.width = side;
    plot.height = side;
  } else if let Some(layout) = chart.plot_layout {
    plot = apply_manual_layout(frame, plot, layout);
  }
  if plot.width <= 0.0 || plot.height <= 0.0 {
    return Vec::new();
  }

  let compact_side_legend_plot =
    style.layout_profile == ChartLayoutProfile::PowerPoint && title.is_some() && side_legend;
  let has_best_fit_data_labels = chart
    .data_labels
    .iter()
    .any(|label| label.position == c::DataLabelPositionValues::BestFit);
  let compact_label_fit_plot = style.layout_profile == ChartLayoutProfile::Excel
    && title.is_some()
    && bottom_legend
    && has_best_fit_data_labels;
  let radius_scale = if has_powerpoint_automatic_radial_plot {
    0.5
  } else if compact_side_legend_plot || compact_label_fit_plot {
    host_defaults.compact_radius_scale
  } else if style.layout_profile == ChartLayoutProfile::Excel && title.is_some() && bottom_legend {
    // Excel's ordinary titled bottom-legend pie keeps the larger automatic
    // plot. The smaller compact profile belongs to visible best-fit labels,
    // whose complete boxes must be rearranged inside the sectors; a c:dLbls
    // container with every show flag disabled does not reserve that region.
    host_defaults.titled_bottom_legend_radius_scale
  } else {
    host_defaults.radius_scale
  };
  let hole_ratio = (chart.hole_size_percent / 100.0).clamp(0.0, 0.9) as f32;
  // Excel keeps a circular 2-D pie inside the plot height. PowerPoint and
  // Word use a 4:3-expanded height basis.
  let radius_basis = plot
    .width
    .min(plot.height * host_defaults.radius_height_basis_scale);
  let powerpoint_pie_3d = (chart.kind == RadialChartKind::Pie3D
    && style.layout_profile == ChartLayoutProfile::PowerPoint)
    .then_some(profiles::POWERPOINT_PIE_3D_PROJECTION);
  let view_3d = chart.view_3d.unwrap_or_default();
  let maximum_explosion = chart
    .point_explosion_percent
    .iter()
    .copied()
    .flatten()
    .fold(chart.series_explosion_percent, f64::max)
    .clamp(0.0, 100.0) as f32
    / 100.0;
  let scene_fit_scale = powerpoint_pie_3d.map_or(1.0, |profile| {
    1.0 / (1.0 + maximum_explosion * profile.explosion_scale)
  });
  let exploded_geometry_scale = powerpoint_pie_3d.map_or(1.0, |profile| {
    if maximum_explosion > f32::EPSILON {
      profile.exploded_geometry_scale
    } else {
      1.0
    }
  });
  let projected_scene_scale = scene_fit_scale * exploded_geometry_scale;
  let radius_x = if let Some(profile) = powerpoint_pie_3d {
    radius_basis * profile.radius_x_scale * projected_scene_scale
  } else {
    radius_basis * radius_scale
  };
  let radius_y = if let Some(profile) = powerpoint_pie_3d {
    radius_x
      * view_3d
        .rotate_x_deg
        .abs()
        .to_radians()
        .sin()
        .clamp(0.05, 1.0)
      * profile.vertical_tilt_scale
  } else if chart.kind == RadialChartKind::Pie3D {
    radius_x * 0.62
  } else {
    radius_x
  };
  let unfitted_depth = if let Some(profile) = powerpoint_pie_3d {
    let rotation_scale = (view_3d.rotate_x_deg.to_radians().cos() / 30.0_f32.to_radians().cos())
      .abs()
      .clamp(0.25, 2.0);
    plot.height
      * profile.depth_height_scale
      * (view_3d.height_percent / 100.0).clamp(0.05, 5.0)
      * rotation_scale
  } else if chart.kind == RadialChartKind::Pie3D {
    plot.height * 0.09
  } else {
    0.0
  };
  let depth = unfitted_depth * projected_scene_scale;
  let center_x = plot.left + plot.width * 0.5;
  let center_y = plot.top
    + (plot.height - unfitted_depth) * 0.5
    + powerpoint_pie_3d.map_or(0.0, |profile| {
      plot.height * profile.center_y_offset_height_ratio
        + if maximum_explosion > f32::EPSILON {
          plot.height * profile.exploded_center_y_offset_height_ratio
        } else {
          0.0
        }
    });
  let mut items = Vec::new();
  push_chart_shape_rect(
    &mut items,
    frame.x_pt,
    frame.y_pt,
    frame.width_pt,
    frame.height_pt,
    Some(&style.chart_area_style.fill),
    Some(&style.chart_area_style.stroke),
    None,
    1.0,
  );
  push_chart_shape_rect(
    &mut items,
    plot.left,
    plot.top,
    plot.width,
    plot.height,
    Some(&style.plot_area_style.fill),
    Some(&style.plot_area_style.stroke),
    None,
    1.0,
  );
  let mut start_angle = chart.first_slice_angle_deg.to_radians() as f32;

  if matches!(
    chart.kind,
    RadialChartKind::PieOfPie | RadialChartKind::BarOfPie
  ) && !chart.secondary_indices.is_empty()
  {
    lower_of_pie_geometry(&mut items, plot, chart, style);
  } else {
    struct RadialSlice {
      index: usize,
      center: (f32, f32),
      perspective_plane_offset: (f32, f32),
      start_angle: f32,
      sweep: f32,
      explosion: f32,
      color: RgbColor,
    }
    let mut slices = Vec::new();
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
        .flatten()
        .unwrap_or(chart.series_explosion_percent)
        / 100.0)
        .clamp(0.0, 1.0) as f32;
      // Excel interprets c:explosion as approximately the percentage of the
      // pie radius. The host profile retains its smaller Word/PowerPoint
      // displacement policy.
      let explosion_scale = powerpoint_pie_3d.map_or(host_defaults.explosion_scale, |profile| {
        profile.explosion_scale
      });
      let perspective_plane_offset = if powerpoint_pie_3d.is_some() {
        (
          mid.sin() * explosion * explosion_scale / exploded_geometry_scale,
          -mid.cos() * explosion * explosion_scale / exploded_geometry_scale,
        )
      } else {
        (0.0, 0.0)
      };
      let offset_x = if powerpoint_pie_3d.is_some() {
        0.0
      } else {
        mid.sin() * radius_x * explosion * explosion_scale
      };
      let offset_y = if powerpoint_pie_3d.is_some() {
        0.0
      } else {
        -mid.cos() * radius_y * explosion * explosion_scale
      };
      let color = style.point_colors[index % style.point_colors.len()];
      slices.push(RadialSlice {
        index,
        center: (center_x + offset_x, center_y + offset_y),
        perspective_plane_offset,
        start_angle,
        sweep,
        explosion,
        color,
      });
      start_angle += sweep;
    }
    // A 3-D pie is one extruded scene. Paint every side face before any top
    // face; interleaving a translated full sector with its top lets a later
    // sector's side overwrite an earlier top and creates false translucent
    // wedges through the centre. LibreOffice likewise creates independent
    // 3-D extrusions and leaves face visibility to the scene painter.
    if depth > 0.0 {
      for slice in &slices {
        if slice.explosion <= f32::EPSILON {
          continue;
        }
        let projection =
          RadialPerspectiveProjection::new(slice.center, (radius_x, radius_y), view_3d)
            .with_strength_scale(projected_scene_scale)
            .with_plane_offset(slice.perspective_plane_offset);
        // A separated extrusion exposes only the radial cut plane whose
        // outward normal faces the camera. Painting a translated complete
        // sector here would invent a triangular underside beyond the slice
        // hub, most visibly on the back green and purple points.
        if slice.start_angle.sin() < -f32::EPSILON {
          items.push(radial_3d_cut_face_path(
            projection,
            depth,
            slice.start_angle,
            slice.color,
          ));
        }
        let end_angle = slice.start_angle + slice.sweep;
        if end_angle.sin() > f32::EPSILON {
          items.push(radial_3d_cut_face_path(
            projection,
            depth,
            end_angle,
            slice.color,
          ));
        }
      }
      for slice in &slices {
        let projection =
          RadialPerspectiveProjection::new(slice.center, (radius_x, radius_y), view_3d)
            .with_strength_scale(projected_scene_scale)
            .with_plane_offset(slice.perspective_plane_offset);
        items.extend(radial_3d_outer_wall_paths(
          projection,
          depth,
          (slice.start_angle, slice.sweep),
          slice.color,
          projection.horizontal_bounds(),
        ));
      }
    }
    for slice in slices {
      items.push(radial_segment_path(
        slice.center,
        (radius_x, radius_y),
        hole_ratio,
        (slice.start_angle, slice.sweep),
        (slice.color, 1.0),
        chart.kind != RadialChartKind::Pie3D,
        style.point_styles.get(slice.index),
        (chart.kind == RadialChartKind::Pie3D).then(|| {
          RadialPerspectiveProjection::new(slice.center, (radius_x, radius_y), view_3d)
            .with_strength_scale(projected_scene_scale)
            .with_plane_offset(slice.perspective_plane_offset)
        }),
      ));
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
    let data_label_style = style
      .data_label_styles
      .get(label_index)
      .and_then(Option::as_ref)
      .unwrap_or(&style.data_label);
    let rich_text_styles = style
      .data_label_rich_text_styles
      .get(label_index)
      .map(Vec::as_slice)
      .unwrap_or_default();
    let text_frame = resolved_data_label_text_frame(frame, label);
    let (width, label_height) = data_label_text_dimensions(
      &mut metrics,
      label,
      data_label_style,
      rich_text_styles,
      text_frame,
    );
    let text_body_insets = chart_text_body_insets(label.text_body_properties);
    let mut best_fit_outside = false;
    let (label_x, label_y) = if chart.kind == RadialChartKind::Pie
      && label.position == c::DataLabelPositionValues::BestFit
      && style.layout_profile == ChartLayoutProfile::Excel
    {
      excel_best_fit_pie_label_position(
        (center_x, center_y),
        (radius_x, radius_y),
        (angle, value / total * std::f64::consts::TAU),
        (width, label_height),
      )
    } else if chart.kind == RadialChartKind::Pie
      && label.position == c::DataLabelPositionValues::BestFit
      && style.layout_profile == ChartLayoutProfile::PowerPoint
    {
      let outer_width = if text_frame.is_fully_sized() {
        width
      } else {
        width + text_body_insets.left + text_body_insets.right
      };
      let outer_height = if text_frame.is_fully_sized() {
        label_height
      } else {
        label_height + text_body_insets.top + text_body_insets.bottom
      };
      powerpoint_best_fit_pie_label_position(
        (center_x, center_y),
        (radius_x, radius_y),
        (angle, value / total * std::f64::consts::TAU),
        (outer_width, outer_height),
        !label.rich_text_runs.is_empty()
          || label.text_components.len() > 1
          || text_frame.is_fully_sized(),
      )
      .map(|(x, y)| {
        if text_frame.is_fully_sized() {
          (x, y)
        } else {
          (x + text_body_insets.left, y + text_body_insets.top)
        }
      })
      .unwrap_or_else(|| {
        best_fit_outside = true;
        let outer_origin = outside_radial_label_position(
          (center_x, center_y),
          (radius_x, radius_y),
          angle,
          (outer_width, outer_height),
        );
        if text_frame.is_fully_sized() {
          outer_origin
        } else {
          (
            outer_origin.0 + text_body_insets.left,
            outer_origin.1 + text_body_insets.top,
          )
        }
      })
    } else if label.position == c::DataLabelPositionValues::OutsideEnd {
      outside_radial_label_position(
        (center_x, center_y),
        (radius_x, radius_y),
        angle,
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
    let (label_x, label_y) = label.layout.map_or((label_x, label_y), |layout| {
      let bounds = apply_manual_text_layout(
        frame,
        PlotRect {
          left: label_x,
          top: label_y,
          width,
          height: label_height,
        },
        layout,
      );
      (bounds.left, bounds.top)
    });
    let leader_start = (
      center_x + angle.sin() * radius_x,
      center_y - angle.cos() * radius_y,
    );
    let leader_end = (
      leader_start.0.clamp(label_x, label_x + width),
      leader_start.1.clamp(label_y, label_y + label_height),
    );
    let leader_edge_distance =
      (leader_start.0 - center_x).powi(2) + (leader_start.1 - center_y).powi(2);
    let leader_label_distance =
      (leader_end.0 - center_x).powi(2) + (leader_end.1 - center_y).powi(2);
    let leader_length = (leader_end.0 - leader_start.0).hypot(leader_end.1 - leader_start.1);
    // LibreOffice PieChart::createTextLabelShape connects the sector edge to
    // the closest point of a custom label rectangle.  It deliberately omits
    // the line when a manual label has been moved inside the pie and when the
    // remaining segment is less than one percent of the chart diagonal.
    let label_is_outside_pie = leader_label_distance > leader_edge_distance + f32::EPSILON;
    let leader_is_visible = leader_length >= frame.width_pt.hypot(frame.height_pt) * 0.01;
    if chart.show_leader_lines
      && label_is_outside_pie
      && leader_is_visible
      && (outside || best_fit_outside || label.layout.is_some())
    {
      push_chart_styled_line(
        &mut items,
        leader_start,
        leader_end,
        Some(&style.leader_line_style.stroke),
        style.data_label.color,
        0.75,
        1.0,
      );
    }
    if let Some(fill_color) = style
      .data_label_fill_colors
      .get(label_index)
      .copied()
      .flatten()
    {
      let horizontal_padding = data_label_style.font_size_pt * 0.3;
      let vertical_padding = data_label_style.font_size_pt * 0.4;
      items.push(PageItem::Rect(RectItem {
        x_pt: label_x - horizontal_padding,
        y_pt: label_y - vertical_padding,
        width_pt: width + horizontal_padding * 2.0,
        height_pt: label_height + vertical_padding * 2.0,
        fill_color: Some(fill_color),
        fill_opacity: 1.0,
        stroke: None,
        stroke_opacity: 1.0,
      }));
    }
    push_data_label_text_components(
      &mut items,
      &mut metrics,
      label_x,
      label_y,
      label,
      data_label_style,
      rich_text_styles,
      text_frame,
    );
  }

  if let Some(title) = title {
    let mut title_style = style.title.clone();
    title_style.rotation_deg += chart.title_rotation_deg;
    if style.layout_profile == ChartLayoutProfile::Excel
      && chart.title_rotation_deg.abs() > f32::EPSILON
    {
      title_style.pdf_glyph_outlines = true;
      let mut options = title_style
        .pdf_glyph_outline_options
        .as_deref()
        .cloned()
        .unwrap_or_default();
      options.semantic_text_overlay = false;
      title_style.pdf_glyph_outline_options = Some(Arc::new(options));
    }
    let width = metrics.measure_text(title, &title_style);
    let automatic = PlotRect {
      left: frame.x_pt + (frame.width_pt - width) * 0.5,
      top: frame.y_pt
        + frame.height_pt * profiles::RADIAL_TITLE_TOP_RATIO
        + if style.layout_profile == ChartLayoutProfile::Excel && bottom_legend {
          title_style.font_size_pt * profiles::EXCEL_BOTTOM_LEGEND_TITLE_OFFSET_EM
        } else {
          0.0
        },
      width,
      height: line_height(&title_style),
    };
    let bounds = chart.title_layout.map_or(automatic, |layout| {
      apply_manual_text_layout(frame, automatic, layout)
    });
    push_text(
      &mut items,
      bounds.left,
      bounds.top,
      title.to_string(),
      title_style,
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

fn powerpoint_best_fit_pie_label_position(
  center: (f32, f32),
  radii: (f32, f32),
  angles: (f32, f64),
  label_size: (f32, f32),
  seek_outer_edge: bool,
) -> Option<(f32, f32)> {
  let (clockwise_from_top, sweep) = angles;
  let radius_x = f64::from(radii.0);
  let radius_y = f64::from(radii.1);
  if label_size.0 <= 0.0
    || label_size.1 <= 0.0
    || radius_x <= 0.0
    || radius_y <= 0.0
    || sweep <= 0.0
  {
    return None;
  }

  let angle = f64::from(clockwise_from_top);
  let initial_center = [angle.sin() * 0.5, angle.cos() * 0.5];
  if sweep > std::f64::consts::PI && !seek_outer_edge {
    return Some((
      center.0 + (initial_center[0] * radius_x) as f32 - label_size.0 * 0.5,
      center.1 - (initial_center[1] * radius_y) as f32 - label_size.1 * 0.5,
    ));
  }

  // `bestFit` starts at PowerPoint's ordinary centered label position. Move
  // that complete rectangle by the minimum Euclidean distance needed to keep
  // every corner inside both radial boundaries and the pie circle. This uses
  // the same rectangle/sector/circle constraints as LibreOffice
  // PieChart::performLabelBestFitInnerPlacement, but not LibreOffice's
  // incompatible choice to push every accepted rectangle all the way to the
  // arc. The immutable percentage-number-formats.pptx Office output is the
  // counterexample: its narrow label remains near the half-radius position.
  #[derive(Clone, Copy)]
  enum Constraint {
    HalfPlane { normal: [f64; 2], minimum: f64 },
    Circle { corner: [f64; 2], radius: f64 },
  }

  let half_width = f64::from(label_size.0) * 0.5 / radius_x;
  let half_height = f64::from(label_size.1) * 0.5 / radius_y;
  if seek_outer_edge
    && f64::from(label_size.0).hypot(f64::from(label_size.1)) > radius_x.min(radius_y) * 0.975
  {
    // PieChart::performLabelBestFitInnerPlacement rejects an inner label
    // when its full nearest-to-farthest box diagonal exceeds the usable pie
    // radius. This deliberately conservative test sends the large fixed
    // c15 frames in tdf146487 to the source-defined outside placement.
    return None;
  }
  let corners = [
    [-half_width, -half_height],
    [-half_width, half_height],
    [half_width, -half_height],
    [half_width, half_height],
  ];
  if seek_outer_edge && sweep > std::f64::consts::PI {
    let direction = [angle.sin(), angle.cos()];
    let mut upper = f64::INFINITY;
    for corner in corners {
      let projection = direction[0] * corner[0] + direction[1] * corner[1];
      let discriminant = projection * projection
        - (corner[0] * corner[0] + corner[1] * corner[1] - 0.975_f64.powi(2));
      if discriminant < 0.0 {
        return None;
      }
      upper = upper.min(-projection + discriminant.sqrt());
    }
    if upper.is_finite() && upper >= 0.0 {
      return Some((
        center.0 + (direction[0] * upper * radius_x) as f32 - label_size.0 * 0.5,
        center.1 - (direction[1] * upper * radius_y) as f32 - label_size.1 * 0.5,
      ));
    }
    return None;
  }
  let alpha = std::f64::consts::FRAC_PI_2 - angle;
  let half_sweep = sweep * 0.5;
  let lower = alpha - half_sweep;
  let upper = alpha + half_sweep;
  let lower_normal = [-lower.sin(), lower.cos()];
  let upper_normal = [upper.sin(), -upper.cos()];
  let mut constraints = Vec::with_capacity(12);
  for corner in corners {
    constraints.push(Constraint::HalfPlane {
      normal: lower_normal,
      minimum: -(lower_normal[0] * corner[0] + lower_normal[1] * corner[1]),
    });
    constraints.push(Constraint::HalfPlane {
      normal: upper_normal,
      minimum: -(upper_normal[0] * corner[0] + upper_normal[1] * corner[1]),
    });
    constraints.push(Constraint::Circle {
      corner,
      // LibreOffice keeps the label rectangle 2.5% inside the arc. Retain
      // that source-backed safety boundary while projecting from the Office
      // center position instead of selecting LibreOffice's maximal radius.
      radius: 0.975,
    });
  }

  if seek_outer_edge && sweep <= std::f64::consts::PI {
    // LibreOffice's source-backed inner-placement geometry moves a complete
    // compound label frame as far as possible toward the slice arc. Office
    // fixed output uses that edge-seeking policy for compound/custom labels
    // (tdf125444 and tdf146487), while short scalar labels provide a direct
    // counterexample and use the minimum-displacement branch below.
    let direction = [angle.sin(), angle.cos()];
    let mut lower = 0.0_f64;
    let mut upper = f64::INFINITY;
    for constraint in constraints.iter().copied() {
      match constraint {
        Constraint::HalfPlane { normal, minimum } => {
          let coefficient = normal[0] * direction[0] + normal[1] * direction[1];
          if coefficient > 1.0e-12 {
            lower = lower.max(minimum / coefficient);
          } else if coefficient < -1.0e-12 {
            upper = upper.min(minimum / coefficient);
          } else if minimum > 1.0e-12 {
            return None;
          }
        }
        Constraint::Circle { corner, radius } => {
          let projection = direction[0] * corner[0] + direction[1] * corner[1];
          let discriminant = projection * projection
            - (corner[0] * corner[0] + corner[1] * corner[1] - radius * radius);
          if discriminant < 0.0 {
            return None;
          }
          let root = discriminant.sqrt();
          lower = lower.max(-projection - root);
          upper = upper.min(-projection + root);
        }
      }
    }
    if lower <= upper && upper.is_finite() && upper >= 0.0 {
      let box_center = [direction[0] * upper, direction[1] * upper];
      return Some((
        center.0 + (box_center[0] * radius_x) as f32 - label_size.0 * 0.5,
        center.1 - (box_center[1] * radius_y) as f32 - label_size.1 * 0.5,
      ));
    }
    return None;
  }

  let project = |point: [f64; 2], constraint: Constraint| match constraint {
    Constraint::HalfPlane { normal, minimum } => {
      let value = normal[0] * point[0] + normal[1] * point[1];
      if value >= minimum {
        point
      } else {
        let scale = (minimum - value) / (normal[0] * normal[0] + normal[1] * normal[1]);
        [point[0] + normal[0] * scale, point[1] + normal[1] * scale]
      }
    }
    Constraint::Circle { corner, radius } => {
      let corner_point = [point[0] + corner[0], point[1] + corner[1]];
      let length = corner_point[0].hypot(corner_point[1]);
      if length <= radius || length <= f64::EPSILON {
        point
      } else {
        let scale = radius / length;
        [
          corner_point[0] * scale - corner[0],
          corner_point[1] * scale - corner[1],
        ]
      }
    }
  };

  // Dykstra's cyclic projections return the nearest point in the convex
  // intersection instead of depending on an arbitrary constraint order.
  let mut box_center = initial_center;
  let mut corrections = vec![[0.0_f64; 2]; constraints.len()];
  for _ in 0..96 {
    let mut maximum_step = 0.0_f64;
    for (index, constraint) in constraints.iter().copied().enumerate() {
      let corrected = [
        box_center[0] + corrections[index][0],
        box_center[1] + corrections[index][1],
      ];
      let projected = project(corrected, constraint);
      corrections[index] = [corrected[0] - projected[0], corrected[1] - projected[1]];
      maximum_step =
        maximum_step.max((projected[0] - box_center[0]).hypot(projected[1] - box_center[1]));
      box_center = projected;
    }
    if maximum_step <= 1.0e-10 {
      break;
    }
  }
  let feasible = constraints.iter().all(|constraint| match *constraint {
    Constraint::HalfPlane { normal, minimum } => {
      normal[0] * box_center[0] + normal[1] * box_center[1] >= minimum - 1.0e-7
    }
    Constraint::Circle { corner, radius } => {
      (box_center[0] + corner[0]).hypot(box_center[1] + corner[1]) <= radius + 1.0e-7
    }
  });
  if !feasible {
    return None;
  }
  Some((
    center.0 + (box_center[0] * radius_x) as f32 - label_size.0 * 0.5,
    center.1 - (box_center[1] * radius_y) as f32 - label_size.1 * 0.5,
  ))
}

fn outside_radial_label_position(
  center: (f32, f32),
  radii: (f32, f32),
  clockwise_from_top: f32,
  label_size: (f32, f32),
) -> (f32, f32) {
  // LibreOffice PolarLabelPositionHelper anchors an outside label at the
  // slice's outer arc, adds the fixed 150 mm100 radial clearance used by
  // PieChart, then places the complete box on the outward side of that
  // anchor. Cardinal directions use centered alignment; diagonal quadrants
  // use the corresponding corner.
  const OUTSIDE_CLEARANCE_PT: f32 = 150.0 * 72.0 / 2540.0;
  let direction = (clockwise_from_top.sin(), -clockwise_from_top.cos());
  let anchor = (
    center.0 + direction.0 * (radii.0 + OUTSIDE_CLEARANCE_PT),
    center.1 + direction.1 * (radii.1 + OUTSIDE_CLEARANCE_PT),
  );
  let office_angle = (90.0 - clockwise_from_top.to_degrees()).rem_euclid(360.0);
  let (width, height) = label_size;
  if office_angle <= 5.0 || office_angle >= 355.0 {
    (anchor.0, anchor.1 - height * 0.5)
  } else if office_angle < 85.0 {
    (anchor.0, anchor.1 - height)
  } else if office_angle <= 95.0 {
    (anchor.0 - width * 0.5, anchor.1 - height)
  } else if office_angle < 175.0 {
    (anchor.0 - width, anchor.1 - height)
  } else if office_angle <= 185.0 {
    (anchor.0 - width, anchor.1 - height * 0.5)
  } else if office_angle < 265.0 {
    (anchor.0 - width, anchor.1)
  } else if office_angle <= 275.0 {
    (anchor.0 - width * 0.5, anchor.1)
  } else {
    (anchor.0, anchor.1)
  }
}

#[derive(Clone, Copy, Debug, Default)]
struct ChartTextBodyInsets {
  left: f32,
  top: f32,
  right: f32,
  bottom: f32,
}

#[derive(Clone, Copy, Debug, Default)]
struct ResolvedDataLabelTextFrame {
  outer_width: Option<f32>,
  outer_height: Option<f32>,
  insets: ChartTextBodyInsets,
}

impl ResolvedDataLabelTextFrame {
  fn is_fully_sized(self) -> bool {
    self.outer_width.is_some() && self.outer_height.is_some()
  }

  fn inner_width(self) -> Option<f32> {
    self
      .outer_width
      .map(|width| (width - self.insets.left - self.insets.right).max(0.0))
  }

  fn inner_height(self) -> Option<f32> {
    self
      .outer_height
      .map(|height| (height - self.insets.top - self.insets.bottom).max(0.0))
  }
}

fn resolved_data_label_text_frame(
  frame: ChartFrame,
  label: &crate::render::chart::ClusteredColumnDataLabel<'_>,
) -> ResolvedDataLabelTextFrame {
  use crate::render::chart::ChartLayoutMode;

  let Some(layout) = label.text_frame_layout else {
    return ResolvedDataLabelTextFrame::default();
  };
  // MS-OI29500 §21.2.3.20 says Office saves wMode/hMode as factor. Do not
  // reinterpret a noncanonical edge value without the final positioned edge;
  // the compatible label then keeps its automatically fitted dimension.
  let scaled = |value: Option<f32>, mode: ChartLayoutMode, extent: f32| {
    (mode == ChartLayoutMode::Factor)
      .then_some(value?)
      .filter(|value| value.is_finite() && *value > 0.0)
      .map(|value| value * extent)
  };
  let outer_width = scaled(layout.width, layout.width_mode, frame.width_pt);
  let outer_height = scaled(layout.height, layout.height_mode, frame.height_pt);
  let insets = if outer_width.is_some() || outer_height.is_some() {
    chart_text_body_insets(label.text_body_properties)
  } else {
    ChartTextBodyInsets::default()
  };
  ResolvedDataLabelTextFrame {
    outer_width,
    outer_height,
    insets,
  }
}

fn chart_text_body_insets(properties: Option<&a::BodyProperties>) -> ChartTextBodyInsets {
  let Some(properties) = properties else {
    return ChartTextBodyInsets::default();
  };
  // ECMA-376 Part 1 §20.1.7.1 defines the DrawingML text-body defaults as
  // 0.1 in horizontally and 0.05 in vertically. A present empty a:bodyPr is
  // therefore not a zero-inset box.
  let inset = |value: Option<ooxmlsdk::simple_type::Coordinate32Value>, default| {
    crate::units::emu_to_points(value.map_or(default, |value| value.to_emu()))
  };
  ChartTextBodyInsets {
    left: inset(
      properties.left_inset,
      DRAWINGML_DEFAULT_HORIZONTAL_TEXT_BODY_INSET_EMU,
    ),
    top: inset(
      properties.top_inset,
      DRAWINGML_DEFAULT_VERTICAL_TEXT_BODY_INSET_EMU,
    ),
    right: inset(
      properties.right_inset,
      DRAWINGML_DEFAULT_HORIZONTAL_TEXT_BODY_INSET_EMU,
    ),
    bottom: inset(
      properties.bottom_inset,
      DRAWINGML_DEFAULT_VERTICAL_TEXT_BODY_INSET_EMU,
    ),
  }
}

fn generated_chart_text_body_insets(properties: Option<&a::BodyProperties>) -> ChartTextBodyInsets {
  if let Some(properties) = properties {
    return chart_text_body_insets(Some(properties));
  }
  // Axis labels are application-generated text shapes even when neither the
  // axis nor chartSpace authors c:txPr. Their implicit DrawingML text body
  // still owns the same schema defaults as an empty a:bodyPr.
  ChartTextBodyInsets {
    left: crate::units::emu_to_points(DRAWINGML_DEFAULT_HORIZONTAL_TEXT_BODY_INSET_EMU),
    top: crate::units::emu_to_points(DRAWINGML_DEFAULT_VERTICAL_TEXT_BODY_INSET_EMU),
    right: crate::units::emu_to_points(DRAWINGML_DEFAULT_HORIZONTAL_TEXT_BODY_INSET_EMU),
    bottom: crate::units::emu_to_points(DRAWINGML_DEFAULT_VERTICAL_TEXT_BODY_INSET_EMU),
  }
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
      style.point_styles.get(index),
      None,
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
        style.point_styles.get(index),
        None,
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
      let point_style = style.point_styles.get(index);
      push_chart_shape_rect(
        items,
        secondary_center.0 - secondary_radius * 0.45,
        y,
        secondary_radius,
        height,
        point_style.map(|style| &style.fill),
        point_style.map(|style| &style.stroke),
        Some(style.point_colors[index % style.point_colors.len()]),
        1.0,
      );
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

#[derive(Clone, Copy)]
struct RadialPerspectiveProjection {
  conic_center: (f32, f32),
  radii: (f32, f32),
  strength: f32,
  plane_offset: (f32, f32),
}

impl RadialPerspectiveProjection {
  fn new(center: (f32, f32), radii: (f32, f32), view: crate::render::chart::Chart3DView) -> Self {
    // MS-OE376 5.7.2.137 records the field-of-view angle in half degrees.
    // LibreOffice's OOXML importer maps that value to its perspective
    // percentage by dividing by two. In the projected conic this is the
    // displacement of the projected circle centre relative to its vertical
    // radius: the default val=30 therefore produces a 15% displacement.
    let strength = if view.right_angle_axes {
      0.0
    } else {
      (view.perspective_half_degrees / 200.0).clamp(0.0, 0.9)
    };
    Self {
      conic_center: center,
      radii,
      strength,
      plane_offset: (0.0, 0.0),
    }
  }

  fn hub(self) -> (f32, f32) {
    self.project_plane_point(0.0, 0.0)
  }

  fn with_strength_scale(mut self, scale: f32) -> Self {
    // Explosion fitting changes the scene extent, not the camera distance.
    // Its projective displacement therefore contracts with the fitted scene.
    self.strength *= scale;
    self
  }

  fn with_plane_offset(mut self, offset: (f32, f32)) -> Self {
    self.plane_offset = offset;
    self
  }

  fn point(self, angle: f32) -> (f32, f32) {
    self.project_plane_point(angle.sin(), -angle.cos())
  }

  fn project_plane_point(self, horizontal: f32, vertical: f32) -> (f32, f32) {
    let horizontal = horizontal + self.plane_offset.0;
    let vertical = vertical + self.plane_offset.1;
    let denominator = (1.0 - self.strength * vertical).max(0.05);
    let one_minus_squared = 1.0 - self.strength * self.strength;
    let base_hub_y = self.conic_center.1 - self.radii.1 * self.strength;
    (
      self.conic_center.0 + self.radii.0 * one_minus_squared.sqrt() * horizontal / denominator,
      base_hub_y + self.radii.1 * one_minus_squared * vertical / denominator,
    )
  }

  fn horizontal_bounds(self) -> (f32, f32) {
    (0..=180).fold((f32::INFINITY, f32::NEG_INFINITY), |(left, right), step| {
      let point = self.point(step as f32 * std::f32::consts::TAU / 180.0);
      (left.min(point.0), right.max(point.0))
    })
  }
}

fn radial_segment_path(
  center: (f32, f32),
  radii: (f32, f32),
  hole_ratio: f32,
  angles: (f32, f32),
  paint: (RgbColor, f32),
  stroke_outline: bool,
  style: Option<&crate::common::ShapeStyle<'static>>,
  perspective: Option<RadialPerspectiveProjection>,
) -> PageItem {
  let (center_x, center_y) = center;
  let (radius_x, radius_y) = radii;
  let (start_angle, sweep) = angles;
  let (color, opacity) = paint;
  let segment_count = ((sweep.to_degrees().abs() / 2.0).ceil() as usize).max(2);
  let mut points = Vec::with_capacity(segment_count * 2 + 3);
  if hole_ratio <= f32::EPSILON {
    let hub = perspective.map_or((center_x, center_y), RadialPerspectiveProjection::hub);
    points.push(common_point(hub.0, hub.1));
  }
  for segment in 0..=segment_count {
    let angle = start_angle + sweep * segment as f32 / segment_count as f32;
    let point = perspective.map_or_else(
      || {
        (
          center_x + angle.sin() * radius_x,
          center_y - angle.cos() * radius_y,
        )
      },
      |projection| projection.point(angle),
    );
    points.push(common_point(point.0, point.1));
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
  // DrawingML binds a gradient to the bounding box of the painted shape,
  // not to the complete source ellipse from which a pie sector was cut.
  // This distinction is visible for every non-reflex sector: PowerPoint's
  // fixed output uses the sector's centre/radial/arc bounds independently
  // for each c:dPt.  Keep the display-list bounds aligned with the actual
  // polygon so gradient coordinates, effects, and pattern paint share that
  // authored shape box.
  let (left, top, right, bottom) = points.iter().fold(
    (
      f32::INFINITY,
      f32::INFINITY,
      f32::NEG_INFINITY,
      f32::NEG_INFINITY,
    ),
    |(left, top, right, bottom), point| {
      (
        left.min(point.x.0),
        top.min(point.y.0),
        right.max(point.x.0),
        bottom.max(point.y.0),
      )
    },
  );
  let bounds = common_rect(left, top, right - left, bottom - top);
  let fill = match style.map(|style| &style.fill) {
    Some(crate::common::ShapeStyleValue::Paint(fill)) => bind_chart_fill_to_bounds(fill, bounds),
    Some(crate::common::ShapeStyleValue::NoPaint) => crate::common::Fill::None,
    Some(crate::common::ShapeStyleValue::Unspecified) | None => {
      crate::common::Fill::Solid(common_rgb(color, opacity))
    }
  };
  let stroke = match style.map(|style| &style.stroke) {
    Some(crate::common::ShapeStyleValue::Paint(stroke)) => {
      Some(bind_chart_stroke_to_bounds(stroke, bounds, 1.0))
    }
    Some(crate::common::ShapeStyleValue::NoPaint) => None,
    Some(crate::common::ShapeStyleValue::Unspecified) | None => {
      stroke_outline.then(|| crate::common::Stroke {
        width: crate::common::Pt(0.75),
        color: common_rgb(
          RgbColor {
            r: 255,
            g: 255,
            b: 255,
          },
          opacity,
        ),
        ..Default::default()
      })
    }
  };
  PageItem::Path(crate::common::PathItem {
    bounds,
    points,
    commands: Vec::new(),
    closed: true,
    fill,
    stroke,
  })
}

fn radial_3d_outer_wall_paths(
  projection: RadialPerspectiveProjection,
  depth: f32,
  angles: (f32, f32),
  color: RgbColor,
  horizontal_light_bounds: (f32, f32),
) -> Vec<PageItem> {
  let (start_angle, sweep) = angles;
  let end_angle = start_angle + sweep;
  let mut front_start = std::f32::consts::FRAC_PI_2
    + ((start_angle - std::f32::consts::FRAC_PI_2) / std::f32::consts::TAU).floor()
      * std::f32::consts::TAU;
  let mut items = Vec::new();
  while front_start < end_angle {
    let front_end = front_start + std::f32::consts::PI;
    let visible_start = start_angle.max(front_start);
    let visible_end = end_angle.min(front_end);
    if visible_end > visible_start + f32::EPSILON {
      let visible_sweep = visible_end - visible_start;
      let segment_count = ((visible_sweep.to_degrees().abs() / 2.0).ceil() as usize).max(2);
      let mut points = Vec::with_capacity((segment_count + 1) * 2);
      for segment in 0..=segment_count {
        let angle = visible_start + visible_sweep * segment as f32 / segment_count as f32;
        let point = projection.point(angle);
        points.push(common_point(point.0, point.1));
      }
      for segment in (0..=segment_count).rev() {
        let angle = visible_start + visible_sweep * segment as f32 / segment_count as f32;
        let point = projection.point(angle);
        points.push(common_point(point.0, point.1 + depth));
      }
      let (left, top, right, bottom) = points.iter().fold(
        (
          f32::INFINITY,
          f32::INFINITY,
          f32::NEG_INFINITY,
          f32::NEG_INFINITY,
        ),
        |(left, top, right, bottom), point| {
          (
            left.min(point.x.0),
            top.min(point.y.0),
            right.max(point.x.0),
            bottom.max(point.y.0),
          )
        },
      );
      let gradient_y = projection.conic_center.1 + depth * 0.5;
      items.push(PageItem::Path(crate::common::PathItem {
        bounds: common_rect(left, top, right - left, bottom - top),
        points,
        commands: Vec::new(),
        closed: true,
        fill: crate::common::Fill::Gradient(crate::common::GradientFill {
          stops: vec![
            crate::common::GradientStop {
              position: 0.0,
              color: common_rgb(shade_chart_color(color, 0.86), 1.0),
              scheme: None,
            },
            crate::common::GradientStop {
              position: 0.2,
              color: common_rgb(shade_chart_color(color, 0.76), 1.0),
              scheme: None,
            },
            crate::common::GradientStop {
              position: 0.4,
              color: common_rgb(shade_chart_color(color, 0.63), 1.0),
              scheme: None,
            },
            crate::common::GradientStop {
              position: 0.7,
              color: common_rgb(shade_chart_color(color, 0.38), 1.0),
              scheme: None,
            },
            crate::common::GradientStop {
              position: 1.0,
              color: common_rgb(shade_chart_color(color, 0.38), 1.0),
              scheme: None,
            },
          ],
          angle_degrees: None,
          definition_bounds: None,
          line: Some((
            common_point(horizontal_light_bounds.0, gradient_y),
            common_point(horizontal_light_bounds.1, gradient_y),
          )),
          interpolation: crate::common::GradientInterpolation::LinearSrgb,
          scaled: false,
          rotate_with_shape: None,
          path: None,
        }),
        stroke: None,
      }));
    }
    front_start += std::f32::consts::TAU;
  }
  items
}

fn radial_3d_cut_face_path(
  projection: RadialPerspectiveProjection,
  depth: f32,
  angle: f32,
  color: RgbColor,
) -> PageItem {
  let hub = projection.hub();
  let outer = projection.point(angle);
  let points = vec![
    common_point(hub.0, hub.1),
    common_point(outer.0, outer.1),
    common_point(outer.0, outer.1 + depth),
    common_point(hub.0, hub.1 + depth),
  ];
  let left = hub.0.min(outer.0);
  let top = hub.1.min(outer.1);
  let right = hub.0.max(outer.0);
  let bottom = (hub.1 + depth).max(outer.1 + depth);
  // Office's fixed upper-left chart light leaves a 40% ambient face and
  // adds diffuse light only while the radial plane points toward the lower
  // left. This reconstructs the dark green/purple cuts and the brighter red
  // cut from the same surface-normal rule, without series-color exceptions.
  let shade = 0.4 + 0.33 * (-angle.cos()).max(0.0);
  PageItem::Path(crate::common::PathItem {
    bounds: common_rect(left, top, right - left, bottom - top),
    points,
    commands: Vec::new(),
    closed: true,
    fill: crate::common::Fill::Solid(common_rgb(shade_chart_color(color, shade), 1.0)),
    stroke: None,
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
  let marker = style.legend.font_size_pt * host_defaults.legend_marker_em;
  let gap = style.legend.font_size_pt * host_defaults.legend_marker_gap_em;
  if let Some(layout) = chart.legend_layout {
    let automatic = PlotRect {
      left: if position == ChartLegendPosition::Left {
        frame.x_pt
      } else {
        frame.x_pt + frame.width_pt * 0.8
      },
      top: frame.y_pt + frame.height_pt * 0.1,
      width: frame.width_pt * 0.2,
      height: frame.height_pt * 0.8,
    };
    let bounds = apply_manual_layout(frame, automatic, layout);
    if bounds.width <= 0.0 || bounds.height <= 0.0 {
      return;
    }

    if layout.width.is_some() && layout.height.is_some() {
      // LibreOffice VLegend::lcl_placeLegendEntries treats an authored-size
      // legend as a custom grid: the legend text owns a maximum frame width,
      // words wrap inside its DrawingML body insets, overflowing rows are
      // removed, and the remaining rows share the complete custom height.
      // Retaining those four stages is important: a narrow legend can wrap
      // its first entry and therefore expose fewer later entries even though
      // the unwrapped strings would fit vertically.
      let insets = chart_text_body_insets(chart.legend_text_body_properties);
      let millimeter_pt: f32 = 72.0 / 25.4;
      let marker_gap = millimeter_pt.max(style.legend.font_size_pt * 0.22);
      let maximum_text_width =
        (bounds.width - marker - marker_gap - insets.left - insets.right).max(0.0);
      let legend_line_height = line_height(&style.legend);
      let minimum_row_gap = millimeter_pt.max(style.legend.font_size_pt * 0.2);
      let mut entries = Vec::new();
      let mut occupied_height = 0.0_f32;
      for index in chart.visible_legend_indices.iter().copied() {
        let Some(text) = chart.categories.get(index) else {
          continue;
        };
        let lines = wrap_chart_label(text, maximum_text_width, &style.legend, metrics);
        let outer_height = lines.len() as f32 * legend_line_height + insets.top + insets.bottom;
        let next_count = entries.len() + 1;
        let required_height =
          occupied_height + outer_height + minimum_row_gap * (next_count + 1) as f32;
        // VLegend accepts a one-millimetre OOXML interop tolerance, but it
        // does not paint entries whose normal row spacing still exceeds it.
        if !entries.is_empty() && required_height > bounds.height + millimeter_pt {
          break;
        }
        occupied_height += outer_height;
        entries.push((index, lines));
      }
      if entries.is_empty() {
        return;
      }

      let maximum_line_width = entries
        .iter()
        .flat_map(|(_, lines)| lines)
        .map(|line| metrics.measure_text(line, &style.legend))
        .fold(0.0_f32, f32::max);
      let column_width = marker + marker_gap + maximum_line_width;
      let x = bounds.left + (bounds.width - column_width).max(0.0) * 0.5;
      let cell_height = bounds.height / entries.len() as f32;
      for (row, (index, lines)) in entries.into_iter().enumerate() {
        let y = bounds.top + row as f32 * cell_height;
        push_radial_legend_key(
          items,
          x,
          y + (legend_line_height - marker) * 0.5,
          marker,
          index,
          style,
        );
        for (line_index, line) in lines.into_iter().enumerate() {
          push_text(
            items,
            x + marker + marker_gap,
            y + line_index as f32 * legend_line_height,
            line,
            style.legend.clone(),
          );
        }
      }
      return;
    }

    let mut y = bounds.top;
    for index in chart.visible_legend_indices.iter().copied() {
      let Some(text) = chart.categories.get(index) else {
        continue;
      };
      push_radial_legend_key(items, bounds.left, y, marker, index, style);
      push_text(
        items,
        bounds.left + marker + gap,
        y - (line_height(&style.legend) - marker) * 0.5,
        text.clone(),
        style.legend.clone(),
      );
      y += line_height(&style.legend) * 1.25;
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
    let entry_gap = style.legend.font_size_pt * host_defaults.horizontal_legend_entry_gap_em;
    let widths = chart
      .visible_legend_indices
      .iter()
      .filter_map(|index| chart.categories.get(*index))
      .map(|text| marker + gap + metrics.measure_text(text, &style.legend))
      .collect::<Vec<_>>();
    let total = widths.iter().sum::<f32>() + entry_gap * widths.len().saturating_sub(1) as f32;
    let mut x = frame.x_pt + (frame.width_pt - total) * 0.5;
    x += style.legend.font_size_pt * host_defaults.horizontal_legend_center_offset_em;
    let legend_line_height = line_height(&style.legend);
    let legend_top = if position == ChartLegendPosition::Top {
      frame.y_pt + horizontal_height * 0.2
    } else if style.layout_profile == ChartLayoutProfile::Excel {
      let profiled_text_top = frame.y_pt + frame.height_pt
        - horizontal_height
          * if chart.title.is_some() {
            profiles::EXCEL_TITLED_BOTTOM_LEGEND_HEIGHT_SCALE
          } else {
            profiles::EXCEL_UNTITLED_BOTTOM_LEGEND_HEIGHT_SCALE
          };
      if chart.title.is_some() {
        // The established titled Excel profile was measured from a centered
        // legend text line. Normalize it back to the container top before
        // applying the authored DrawingML anchor below.
        profiled_text_top
          - single_line_vertical_anchor_offset(
            Some(a::TextAnchoringTypeValues::Center),
            horizontal_height,
            legend_line_height,
          )
      } else {
        profiled_text_top
      }
    } else {
      frame.y_pt + frame.height_pt - horizontal_height * 0.8
    };
    let y = legend_top
      + single_line_vertical_anchor_offset(
        chart.legend_vertical_anchor,
        horizontal_height,
        legend_line_height,
      );
    for ((index, text), width) in chart
      .visible_legend_indices
      .iter()
      .filter_map(|index| chart.categories.get(*index).map(|text| (*index, text)))
      .zip(widths)
    {
      push_radial_legend_key(
        items,
        x,
        y + (line_height(&style.legend) - marker) * 0.5,
        marker,
        index,
        style,
      );
      push_text(
        items,
        x + marker + gap,
        y,
        text.clone(),
        style.legend.clone(),
      );
      x += width + entry_gap;
    }
  } else {
    let side_inset = style.legend.font_size_pt * 0.4;
    let x = if position == ChartLegendPosition::Left {
      frame.x_pt + side_inset
    } else {
      frame.x_pt + frame.width_pt - side_width + side_inset
    };
    let entry_step = line_height(&style.legend) * host_defaults.side_legend_entry_step;
    let entry_count = chart.visible_legend_indices.len();
    let total_height =
      line_height(&style.legend) + entry_step * entry_count.saturating_sub(1) as f32;
    let center_y = frame.y_pt
      + frame.height_pt * 0.5
      + style.legend.font_size_pt * host_defaults.side_legend_center_offset_em;
    let mut y = center_y - total_height * 0.5;
    for index in chart.visible_legend_indices.iter().copied() {
      let Some(text) = chart.categories.get(index) else {
        continue;
      };
      push_radial_legend_key(
        items,
        x,
        y + (line_height(&style.legend) - marker) * 0.5,
        marker,
        index,
        style,
      );
      push_text(
        items,
        x + marker + gap,
        y,
        text.clone(),
        style.legend.clone(),
      );
      y += entry_step;
    }
  }
}

fn push_radial_legend_key(
  items: &mut Vec<PageItem>,
  x_pt: f32,
  y_pt: f32,
  size_pt: f32,
  point_index: usize,
  style: &RadialChartStyle,
) {
  let point_style = style.point_styles.get(point_index);
  push_chart_shape_rect(
    items,
    x_pt,
    y_pt,
    size_pt,
    size_pt,
    point_style.map(|style| &style.fill),
    point_style.map(|style| &style.stroke),
    Some(style.point_colors[point_index % style.point_colors.len()]),
    1.0,
  );
}

fn single_line_vertical_anchor_offset(
  anchor: Option<a::TextAnchoringTypeValues>,
  container_height: f32,
  line_height: f32,
) -> f32 {
  let available_height = (container_height - line_height).max(0.0);
  // ECMA-376 Part 1 §20.1.10.60 makes the one-line cases explicit:
  // distributed anchors in the middle, while justified anchors at the top.
  match anchor.unwrap_or(a::TextAnchoringTypeValues::Top) {
    a::TextAnchoringTypeValues::Center | a::TextAnchoringTypeValues::Distributed => {
      available_height * 0.5
    }
    a::TextAnchoringTypeValues::Bottom => available_height,
    a::TextAnchoringTypeValues::Top | a::TextAnchoringTypeValues::Justified => 0.0,
  }
}

#[derive(Clone, Copy, Debug)]
struct PlotRect {
  left: f32,
  top: f32,
  width: f32,
  height: f32,
}

#[derive(Clone, Copy)]
struct AxisTitleGeometry {
  frame: ChartFrame,
  plot: PlotRect,
  value_label_band_left: f32,
  category_band_top: f32,
  category_label_height: f32,
  data_table_height: f32,
  projection_3d: Option<Chart3DProjection>,
}

#[derive(Clone, Copy)]
struct HorizontalAxisGeometry {
  plot: PlotRect,
  scale: crate::render::chart::LinearAxisScale,
  projection_3d: Option<Chart3DProjection>,
  draw_gridlines: bool,
  draw_labels: bool,
}

#[derive(Clone, Copy, Debug)]
struct Chart3DProjection {
  input: PlotRect,
  rotate_x_rad: f32,
  rotate_y_rad: f32,
  right_angle_axes: bool,
  model_width: f32,
  model_height: f32,
  model_depth: f32,
  camera_distance: Option<f32>,
  raw_center_x: f32,
  raw_center_y: f32,
  scale: f32,
  screen_center_x: f32,
  screen_center_y: f32,
  screen_matrix: [f32; 4],
}

impl Chart3DProjection {
  fn project(self, x: f32, y: f32, depth_ratio: f32) -> (f32, f32) {
    let model_x = if self.input.width > f32::EPSILON {
      ((x - self.input.left) / self.input.width - 0.5) * self.model_width
    } else {
      0.0
    };
    let model_y = if self.input.height > f32::EPSILON {
      ((y - self.input.top) / self.input.height - 0.5) * self.model_height
    } else {
      0.0
    };
    let model_z = (depth_ratio - 0.5) * self.model_depth;
    let (raw_x, raw_y) = project_chart_model_point(
      self.rotate_x_rad,
      self.rotate_y_rad,
      self.right_angle_axes,
      self.camera_distance,
      model_x,
      model_y,
      model_z,
    );
    let screen_x = (raw_x - self.raw_center_x) * self.scale;
    let screen_y = (raw_y - self.raw_center_y) * self.scale;
    (
      self.screen_center_x + self.screen_matrix[0] * screen_x + self.screen_matrix[1] * screen_y,
      self.screen_center_y + self.screen_matrix[2] * screen_x + self.screen_matrix[3] * screen_y,
    )
  }

  fn x_for_visual_side(self, plot: PlotRect, visual_right: bool, y: f32, depth_ratio: f32) -> f32 {
    let left_screen_x = self.project(plot.left, y, depth_ratio).0;
    let right_screen_x = self.project(plot.left + plot.width, y, depth_ratio).0;
    if (right_screen_x >= left_screen_x) == visual_right {
      plot.left + plot.width
    } else {
      plot.left
    }
  }

  fn vertical_edge_for_visual_side(self, plot: PlotRect, visual_right: bool) -> (f32, f32) {
    let y = plot.top + plot.height * 0.5;
    let mut selected = (plot.left, 0.0_f32);
    let mut selected_x = self.project(selected.0, y, selected.1).0;
    for x in [plot.left, plot.left + plot.width] {
      for depth in [0.0_f32, 1.0] {
        let screen_x = self.project(x, y, depth).0;
        if (visual_right && screen_x > selected_x) || (!visual_right && screen_x < selected_x) {
          selected = (x, depth);
          selected_x = screen_x;
        }
      }
    }
    selected
  }

  fn vertical_axis_length(self, plot: PlotRect, visual_right: bool) -> f32 {
    let (x, depth) = self.vertical_edge_for_visual_side(plot, visual_right);
    let top = self.project(x, plot.top, depth);
    let bottom = self.project(x, plot.top + plot.height, depth);
    (bottom.0 - top.0).hypot(bottom.1 - top.1)
  }

  fn depth_vector(self) -> (f32, f32) {
    let x = self.input.left + self.input.width * 0.5;
    let y = self.input.top + self.input.height * 0.5;
    let front = self.project(x, y, 0.0);
    let back = self.project(x, y, 1.0);
    (back.0 - front.0, back.1 - front.1)
  }
}

#[derive(Clone, Copy)]
struct Marker3DContext<'context, 'chart, 'data> {
  geometry: &'context SeriesGeometryContext<'chart, 'data>,
  projection: Chart3DProjection,
  series_index: usize,
  category_index: usize,
  color: RgbColor,
}

#[derive(Clone, Copy)]
struct MarkerDepth {
  front: f32,
  back: f32,
}

#[derive(Clone, Copy)]
struct VerticalMarkerBounds {
  x: f32,
  width: f32,
  start_y: f32,
  end_y: f32,
}

#[derive(Clone, Copy)]
struct HorizontalMarkerBounds {
  start_x: f32,
  end_x: f32,
  y: f32,
  height: f32,
}

#[derive(Clone, Copy)]
struct VerticalTaperedBounds {
  center_x: f32,
  start_y: f32,
  end_y: f32,
  start_half_width: f32,
  end_half_width: f32,
}

#[derive(Clone, Copy)]
struct HorizontalTaperedBounds {
  start_x: f32,
  end_x: f32,
  center_y: f32,
  start_half_height: f32,
  end_half_height: f32,
}

fn cartesian_3d_preferred_model_aspect(
  chart: &ClusteredColumnChart<'_>,
  category_count: usize,
) -> (f32, bool) {
  let Some(series) = chart.series.iter().find(|series| {
    series.is_3d
      && matches!(
        series.kind,
        ChartSeriesKind::Column
          | ChartSeriesKind::Bar
          | ChartSeriesKind::Line
          | ChartSeriesKind::Area
      )
  }) else {
    return (1.0, false);
  };
  let peer_count = chart
    .series
    .iter()
    .filter(|peer| {
      peer.is_3d
        && peer.axis_set_index == series.axis_set_index
        && peer.kind == series.kind
        && peer.grouping == series.grouping
    })
    .count()
    .max(1);
  let preferred_depth = match series.kind {
    ChartSeriesKind::Column | ChartSeriesKind::Bar
      if series.grouping == ChartSeriesGrouping::Clustered =>
    {
      // LibreOffice BarChart::getPreferredDiagramAspectRatio derives scene
      // depth from the logical category span and the ordinary bar-slot
      // geometry. Clustered series occupy separate X slots.
      let category_span = category_count.max(1) as f32;
      let x_slot_count = peer_count;
      let outer_distance = (chart.gap_width_percent as f32 / 100.0).clamp(0.0, 6.0);
      // ChartView asks the plotter for its preferred aspect before
      // BarChart::createShapes updates BarPositionHelper's series count.
      // Consequently this particular slot-width query uses the helper's
      // one-series default; the separately counted X slots still widen the
      // denominator below. This ordering is observable in Office fixed
      // output and must not be replaced by the later marker-slot width.
      let slot_width = 1.0 / (1.0 + outer_distance).max(f32::EPSILON);
      1.0 / (category_span + category_span * x_slot_count.saturating_sub(1) as f32 * slot_width)
    }
    ChartSeriesKind::Column
    | ChartSeriesKind::Bar
    | ChartSeriesKind::Line
    | ChartSeriesKind::Area
    | ChartSeriesKind::Scatter
    | ChartSeriesKind::Bubble
    | ChartSeriesKind::Radar
    | ChartSeriesKind::Stock
    | ChartSeriesKind::Surface => 1.0,
  };
  (
    preferred_depth.clamp(0.05, 10.0),
    series.kind == ChartSeriesKind::Bar && series.grouping == ChartSeriesGrouping::Clustered,
  )
}

fn cartesian_3d_projection(
  view: Chart3DView,
  plot: PlotRect,
  layout_profile: ChartLayoutProfile,
  preferred_depth: f32,
  automatic_width: bool,
) -> Chart3DProjection {
  // ECMA-376 §21.2.2.41/§21.2.2.80 express scene depth and an authored
  // height as percentages of chart width.  An omitted hPercent is not 100%:
  // LibreOffice VDiagram::adjustAspectRatio3d solves the missing dimension
  // from the final available rectangle, then adjustPosAndSize_3d uniformly
  // fits and centers the rotated scene.
  let model_depth = if view.depth_percent_is_explicit {
    (view.depth_percent / 100.0).clamp(0.2, 20.0)
  } else {
    preferred_depth.clamp(0.05, 10.0)
  };
  let rotate_x_rad = view.rotate_x_deg.clamp(-90.0, 90.0).to_radians();
  let normalized_rotate_y_deg = {
    let positive = view.rotate_y_deg.rem_euclid(360.0);
    if positive > 180.0 {
      positive - 360.0
    } else {
      positive
    }
  };
  // LibreOffice's OOXML import first maps rotY from [0, 359] to
  // (-180, 180], then ThreeDHelper limits a right-angle view to +/-45
  // degrees before VDiagram constructs its oblique shear matrix.
  let rotate_y_rad = if view.right_angle_axes {
    normalized_rotate_y_deg.clamp(-45.0, 45.0).to_radians()
  } else {
    normalized_rotate_y_deg.to_radians()
  };
  let camera_distance = chart_3d_camera_distance(view);
  let authored_or_preferred_height = if view.height_percent_is_explicit {
    (view.height_percent / 100.0).clamp(0.05, 5.0)
  } else if automatic_width {
    1.0
  } else {
    automatic_chart_model_height(
      plot,
      rotate_x_rad,
      rotate_y_rad,
      view.right_angle_axes,
      camera_distance,
      1.0,
      model_depth,
    )
  };
  let model_width = if automatic_width {
    automatic_chart_model_width(
      plot,
      rotate_x_rad,
      rotate_y_rad,
      view.right_angle_axes,
      camera_distance,
      authored_or_preferred_height,
      model_depth,
    )
  } else {
    1.0
  };
  let model_height = authored_or_preferred_height;
  let bounds = projected_chart_model_bounds(
    rotate_x_rad,
    rotate_y_rad,
    view.right_angle_axes,
    camera_distance,
    model_width,
    model_height,
    model_depth,
  );
  let raw_width = (bounds.2 - bounds.0).max(f32::EPSILON);
  let raw_height = (bounds.3 - bounds.1).max(f32::EPSILON);
  let scale = (plot.width / raw_width).min(plot.height / raw_height);

  Chart3DProjection {
    input: plot,
    rotate_x_rad,
    rotate_y_rad,
    right_angle_axes: view.right_angle_axes,
    model_width,
    model_height,
    model_depth,
    camera_distance,
    raw_center_x: (bounds.0 + bounds.2) * 0.5,
    raw_center_y: (bounds.1 + bounds.3) * 0.5,
    scale,
    screen_center_x: plot.left + plot.width * 0.5,
    screen_center_y: plot.top + plot.height * 0.5,
    screen_matrix: if layout_profile == ChartLayoutProfile::PowerPoint && !view.right_angle_axes {
      profiles::POWERPOINT_CARTESIAN_3D_SCREEN_MATRIX
    } else {
      [1.0, 0.0, 0.0, 1.0]
    },
  }
}

fn chart_3d_camera_distance(view: Chart3DView) -> Option<f32> {
  if view.right_angle_axes {
    return None;
  }
  // MS-OE376 §21.2.2.152 defines c:perspective as twice the camera's
  // field-of-view angle. Office treats val=0 as 0.1 degrees rather than as a
  // parallel projection; c:rAngAx is the switch for parallel axes. Apply the
  // pinhole-camera relation in normalized chart-volume coordinates.
  let field_of_view_degrees = if view.perspective_half_degrees <= f32::EPSILON {
    0.1
  } else {
    view.perspective_half_degrees * 0.5
  }
  .clamp(0.1, 100.0);
  Some(
    profiles::OFFICE_CARTESIAN_3D_CAMERA_HALF_APERTURE
      / (field_of_view_degrees * 0.5).to_radians().tan(),
  )
}

fn cartesian_3d_series_axis_labels_visible(chart: &ClusteredColumnChart<'_>) -> bool {
  chart
    .axis_sets
    .iter()
    .enumerate()
    .any(|(axis_set_index, axes)| {
      axes.series_axis.is_some_and(|axis| {
        !axis
          .delete
          .as_ref()
          .is_some_and(|delete| delete.val.is_none_or(|value| value.as_bool()))
          && !axis
            .tick_label_position
            .as_ref()
            .is_some_and(|position| position.val == Some(c::TickLabelPositionValues::None))
          && chart
            .series
            .iter()
            .any(|series| series.axis_set_index == axis_set_index && series.is_3d)
      })
    })
}

fn cartesian_3d_series_axis_reservation(
  chart: &ClusteredColumnChart<'_>,
  plot: PlotRect,
  view: Chart3DView,
  preferred_model_aspect: (f32, bool),
  style: &ClusteredColumnStyle,
  metrics: &mut TextMetrics,
) -> Option<(bool, f32)> {
  let projection = cartesian_3d_projection(
    view,
    plot,
    style.layout_profile,
    preferred_model_aspect.0,
    preferred_model_aspect.1,
  );
  let mut reservation = 0.0_f32;
  let mut direction = 0.0_f32;
  for (axis_set_index, axes) in chart.axis_sets.iter().enumerate() {
    let Some(axis) = axes.series_axis else {
      continue;
    };
    if axis
      .delete
      .as_ref()
      .is_some_and(|delete| delete.val.is_none_or(|value| value.as_bool()))
      || axis
        .tick_label_position
        .as_ref()
        .is_some_and(|position| position.val == Some(c::TickLabelPositionValues::None))
    {
      continue;
    }
    let maximum_width = chart
      .series
      .iter()
      .filter(|series| series.axis_set_index == axis_set_index && series.is_3d)
      .map(|series| metrics.measure_text(&series.name, &style.series_label))
      .fold(0.0_f32, f32::max);
    if maximum_width <= f32::EPSILON {
      continue;
    }
    let base = match axis.axis_position.val {
      c::AxisPositionValues::Bottom | c::AxisPositionValues::Right => (
        projection.x_for_visual_side(plot, true, plot.top + plot.height, 0.0),
        plot.top + plot.height,
      ),
      c::AxisPositionValues::Top => (
        projection.x_for_visual_side(plot, true, plot.top, 0.0),
        plot.top,
      ),
      c::AxisPositionValues::Left => (
        projection.x_for_visual_side(plot, false, plot.top + plot.height, 0.0),
        plot.top + plot.height,
      ),
    };
    let start = projection.project(base.0, base.1, 0.0);
    let end = projection.project(base.0, base.1, 1.0);
    direction += (start.0 + end.0) * 0.5 - (plot.left + plot.width * 0.5);
    reservation = reservation.max(maximum_width);
  }
  (reservation > f32::EPSILON).then_some((direction >= 0.0, reservation))
}

fn automatic_chart_model_height(
  plot: PlotRect,
  rotate_x_rad: f32,
  rotate_y_rad: f32,
  right_angle_axes: bool,
  camera_distance: Option<f32>,
  model_width: f32,
  model_depth: f32,
) -> f32 {
  if right_angle_axes {
    // VDiagram::adjustAspectRatio3d intentionally solves the automatic
    // aspect with sines even though the later right-angle scene transform
    // uses the angles themselves as shear coefficients. The final uniform
    // scene fit preserves that small but visible distinction.
    let candidate = (plot.height / plot.width.max(f32::EPSILON))
      * (rotate_y_rad.sin().abs() * model_depth + model_width)
      - rotate_x_rad.sin().abs() * model_depth;
    return ensure_automatic_chart_model_scale(candidate);
  }
  let target_aspect = (plot.width / plot.height.max(f32::EPSILON)).max(f32::EPSILON);
  let aspect = |height| {
    let bounds = projected_chart_model_bounds(
      rotate_x_rad,
      rotate_y_rad,
      right_angle_axes,
      camera_distance,
      model_width,
      height,
      model_depth,
    );
    (bounds.2 - bounds.0) / (bounds.3 - bounds.1).max(f32::EPSILON)
  };
  let mut low = 0.2_f32;
  let mut high = 5.0_f32;
  if aspect(low) <= target_aspect {
    return low;
  }
  if aspect(high) >= target_aspect {
    return high;
  }
  for _ in 0..28 {
    let middle = (low + high) * 0.5;
    if aspect(middle) > target_aspect {
      low = middle;
    } else {
      high = middle;
    }
  }
  (low + high) * 0.5
}

fn automatic_chart_model_width(
  plot: PlotRect,
  rotate_x_rad: f32,
  rotate_y_rad: f32,
  right_angle_axes: bool,
  camera_distance: Option<f32>,
  model_height: f32,
  model_depth: f32,
) -> f32 {
  if right_angle_axes {
    let candidate = (plot.width / plot.height.max(f32::EPSILON))
      * (rotate_x_rad.sin().abs() * model_depth + model_height)
      - rotate_y_rad.sin().abs() * model_depth;
    return ensure_automatic_chart_model_scale(candidate);
  }
  let target_aspect = (plot.width / plot.height.max(f32::EPSILON)).max(f32::EPSILON);
  let aspect = |width| {
    let bounds = projected_chart_model_bounds(
      rotate_x_rad,
      rotate_y_rad,
      right_angle_axes,
      camera_distance,
      width,
      model_height,
      model_depth,
    );
    (bounds.2 - bounds.0) / (bounds.3 - bounds.1).max(f32::EPSILON)
  };
  let mut low = 0.2_f32;
  let mut high = 5.0_f32;
  if aspect(low) >= target_aspect {
    return low;
  }
  if aspect(high) <= target_aspect {
    return high;
  }
  for _ in 0..28 {
    let middle = (low + high) * 0.5;
    if aspect(middle) < target_aspect {
      low = middle;
    } else {
      high = middle;
    }
  }
  (low + high) * 0.5
}

fn ensure_automatic_chart_model_scale(candidate: f32) -> f32 {
  if candidate < 0.0 {
    1.0
  } else {
    candidate.clamp(0.2, 5.0)
  }
}

fn projected_chart_model_bounds(
  rotate_x_rad: f32,
  rotate_y_rad: f32,
  right_angle_axes: bool,
  camera_distance: Option<f32>,
  model_width: f32,
  model_height: f32,
  model_depth: f32,
) -> (f32, f32, f32, f32) {
  let mut minimum_x = f32::INFINITY;
  let mut minimum_y = f32::INFINITY;
  let mut maximum_x = f32::NEG_INFINITY;
  let mut maximum_y = f32::NEG_INFINITY;
  for x in [-model_width * 0.5, model_width * 0.5] {
    for y in [-model_height * 0.5, model_height * 0.5] {
      for z in [-model_depth * 0.5, model_depth * 0.5] {
        let (projected_x, projected_y) = project_chart_model_point(
          rotate_x_rad,
          rotate_y_rad,
          right_angle_axes,
          camera_distance,
          x,
          y,
          z,
        );
        minimum_x = minimum_x.min(projected_x);
        minimum_y = minimum_y.min(projected_y);
        maximum_x = maximum_x.max(projected_x);
        maximum_y = maximum_y.max(projected_y);
      }
    }
  }
  (minimum_x, minimum_y, maximum_x, maximum_y)
}

fn project_chart_model_point(
  rotate_x_rad: f32,
  rotate_y_rad: f32,
  right_angle_axes: bool,
  camera_distance: Option<f32>,
  model_x: f32,
  model_y_down: f32,
  model_z: f32,
) -> (f32, f32) {
  let (sin_y, cos_y) = rotate_y_rad.sin_cos();
  let (sin_x, cos_x) = rotate_x_rad.sin_cos();
  if right_angle_axes {
    // Right-angle axes use an oblique projection: the front XY plane stays
    // axis-aligned and the authored rotations control only the receding Z
    // vector. Office fixed output uses the orthographic direction cosines;
    // LibreOffice's VDiagram aspect equations independently use the same
    // sine terms, even though its final B3DHomMatrix shear stores radians.
    return (model_x + model_z * sin_y, model_y_down - model_z * sin_x);
  }
  let model_y_up = -model_y_down;
  let rotated_x = model_x * cos_y + model_z * sin_y;
  let yaw_depth = -model_x * sin_y + model_z * cos_y;
  // OOXML's positive rotX is an elevation of the camera, so it is the
  // inverse of rotating the chart volume around the X axis. This makes the
  // default back wall rise, while rotY values past 90 degrees correctly make
  // the same series-depth axis descend on screen.
  let rotated_y_up = model_y_up * cos_x + yaw_depth * sin_x;
  let camera_depth = -model_y_up * sin_x + yaw_depth * cos_x;
  let perspective_scale = camera_distance.map_or(1.0, |distance| {
    // Positive model depth is the far side of the chart volume.
    distance / (distance + camera_depth).max(distance * 0.15)
  });
  (
    rotated_x * perspective_scale,
    -rotated_y_up * perspective_scale,
  )
}

struct SeriesGeometryContext<'chart, 'data> {
  chart: &'chart ClusteredColumnChart<'data>,
  plot: PlotRect,
  scale: crate::render::chart::LinearAxisScale,
  zero_y: f32,
  category_count: usize,
  projection_3d: Option<Chart3DProjection>,
}

#[derive(Clone, Copy)]
struct ScatterGeometry {
  plot: PlotRect,
  scale: crate::render::chart::LinearAxisScale,
  x_scale: Option<crate::render::chart::LinearAxisScale>,
  bubble_maximum: f64,
}

#[derive(Clone, Copy, Debug)]
struct CartesianAxisScales {
  x: Option<crate::render::chart::LinearAxisScale>,
  y: crate::render::chart::LinearAxisScale,
}

fn axis_set_count(chart: &ClusteredColumnChart<'_>) -> usize {
  chart
    .series
    .iter()
    .map(|series| series.axis_set_index + 1)
    .chain(std::iter::once(chart.axis_sets.len()))
    .max()
    .unwrap_or(1)
    .max(1)
}

fn axis_set_value_axis<'chart, 'data>(
  chart: &'chart ClusteredColumnChart<'data>,
  axis_set_index: usize,
) -> Option<&'data c::ValueAxis> {
  chart
    .axis_sets
    .get(axis_set_index)
    .and_then(|set| set.vertical_value_axis)
    .or_else(|| (axis_set_index == 0).then_some(chart.value_axis).flatten())
}

fn axis_set_horizontal_value_axis<'chart, 'data>(
  chart: &'chart ClusteredColumnChart<'data>,
  axis_set_index: usize,
) -> Option<&'data c::ValueAxis> {
  chart
    .axis_sets
    .get(axis_set_index)
    .and_then(|set| set.horizontal_value_axis)
    .or_else(|| {
      (axis_set_index == 0)
        .then_some(chart.horizontal_value_axis)
        .flatten()
    })
}

fn axis_set_is_percent_stacked(chart: &ClusteredColumnChart<'_>, axis_set_index: usize) -> bool {
  let mut series = chart
    .series
    .iter()
    .filter(|series| series.axis_set_index == axis_set_index);
  series.next().is_some_and(|first| {
    first.grouping == ChartSeriesGrouping::PercentStacked
      && series.all(|series| series.grouping == ChartSeriesGrouping::PercentStacked)
  })
}

fn cartesian_axis_scales(
  chart: &ClusteredColumnChart<'_>,
  layout_profile: ChartLayoutProfile,
  category_count: usize,
  maximum_auto_value_increment_count: usize,
  maximum_auto_horizontal_increment_count: usize,
  bubble_plot_size: Option<(f32, f32)>,
) -> Vec<CartesianAxisScales> {
  let mut result = Vec::with_capacity(axis_set_count(chart));
  for axis_set_index in 0..axis_set_count(chart) {
    let value_axis = axis_set_value_axis(chart, axis_set_index);
    let y = if axis_set_is_percent_stacked(chart, axis_set_index) {
      chart_linear_axis_scale(
        vec![0.0, 1.0],
        value_axis,
        Some("0%"),
        maximum_auto_value_increment_count,
        LinearAxisScaleOptions {
          // ScaleAutomatism::setAutoScalingOptions disables the extra
          // close-to-border interval for a percent axis. Its semantic domain
          // remains 0..100%; only the automatic major unit is size-dependent.
          expand_if_values_close_to_border: false,
          minimum_automatic_major_unit: None,
        },
      )
    } else {
      let values = cartesian_scale_values(chart, category_count, axis_set_index);
      let options = LinearAxisScaleOptions {
        expand_if_values_close_to_border: !chart
          .series
          .iter()
          .any(|series| series.axis_set_index == axis_set_index && series.is_3d),
        minimum_automatic_major_unit: None,
      };
      bubble_aware_chart_linear_axis_scale(
        chart,
        axis_set_index,
        BubbleAxisDimension::Vertical,
        values,
        value_axis,
        vertical_axis_number_format_code(chart, axis_set_index),
        maximum_auto_value_increment_count,
        options,
        bubble_plot_size,
      )
    };
    let y = y.unwrap_or_else(|| crate::render::chart::LinearAxisScale {
      minimum: value_axis
        .and_then(|axis| axis.scaling.min_axis_value.as_ref())
        .map_or(0.0, |value| value.val),
      maximum: value_axis
        .and_then(|axis| axis.scaling.max_axis_value.as_ref())
        .map_or(1.0, |value| value.val),
      major_unit: value_axis
        .and_then(|axis| axis.major_unit.as_ref())
        .map_or(0.1, |value| value.val),
      logarithmic_base: None,
      reversed: value_axis
        .and_then(|axis| axis.scaling.orientation.as_ref())
        .and_then(|orientation| orientation.val)
        == Some(c::OrientationValues::MaxMin),
    });
    let has_numeric_x = chart.series.iter().any(|series| {
      series.axis_set_index == axis_set_index
        && matches!(
          series.kind,
          ChartSeriesKind::Scatter | ChartSeriesKind::Bubble
        )
    });
    let x = has_numeric_x
      .then(|| {
        let values = scatter_x_axis_values(chart, axis_set_index);
        let ordinal_x_axis = axis_set_scatter_uses_index_x_values(chart, axis_set_index);
        bubble_aware_chart_linear_axis_scale(
          chart,
          axis_set_index,
          BubbleAxisDimension::Horizontal,
          values,
          axis_set_horizontal_value_axis(chart, axis_set_index),
          horizontal_axis_number_format_code(chart, axis_set_index),
          maximum_auto_horizontal_increment_count,
          LinearAxisScaleOptions {
            // Word and PowerPoint serialize textual scatter X values as an
            // ordinal value axis with whole-number major units. Excel retains
            // an ordinary automatic numeric scale for the same cached string
            // sequence (`ser_labels.xlsx` uses half steps), so this host
            // policy must not leak into the workbook renderer.
            minimum_automatic_major_unit: (layout_profile != ChartLayoutProfile::Excel
              && ordinal_x_axis)
              .then_some(1.0),
            ..LinearAxisScaleOptions::default()
          },
          bubble_plot_size,
        )
      })
      .flatten();
    result.push(CartesianAxisScales { x, y });
  }
  result
}

#[derive(Clone, Copy)]
enum BubbleAxisDimension {
  Horizontal,
  Vertical,
}

fn bubble_aware_chart_linear_axis_scale(
  chart: &ClusteredColumnChart<'_>,
  axis_set_index: usize,
  dimension: BubbleAxisDimension,
  source_values: Vec<f64>,
  axis: Option<&c::ValueAxis>,
  format_code: Option<&str>,
  maximum_auto_increment_count: usize,
  options: LinearAxisScaleOptions,
  plot_size: Option<(f32, f32)>,
) -> Option<crate::render::chart::LinearAxisScale> {
  let mut scale = chart_linear_axis_scale(
    source_values.clone(),
    axis,
    format_code,
    maximum_auto_increment_count,
    options,
  )?;
  let Some(_) = plot_size.filter(|(width, height)| {
    width.is_finite() && height.is_finite() && *width > 0.0 && *height > 0.0
  }) else {
    return Some(scale);
  };
  let source_minimum = source_values.iter().copied().fold(f64::INFINITY, f64::min);
  let source_maximum = source_values
    .iter()
    .copied()
    .fold(f64::NEG_INFINITY, f64::max);
  let degenerate_source = source_minimum.is_finite()
    && source_maximum.is_finite()
    && (source_maximum - source_minimum).abs() <= f64::EPSILON;
  let group_maxima = bubble_group_maxima(chart);

  // Bubble radii are defined in final screen space while automatic axes are
  // defined in data space. Re-evaluate the discrete 1/2/5 scale after mapping
  // each painted circle edge back through the current axis transform. The
  // scale normally stabilizes after one pass; the bounded loop handles the
  // case where a newly selected major unit moves an edge to the next tick.
  for _ in 0..8 {
    let mut envelope_values = source_values.clone();
    let mut envelope_exceeds_scale = false;
    for series in chart.series.iter().filter(|series| {
      series.axis_set_index == axis_set_index && series.kind == ChartSeriesKind::Bubble
    }) {
      let Some(group_maximum) = series
        .bubble_group_index
        .and_then(|group| group_maxima.get(group).copied())
        .filter(|maximum| *maximum > 0.0)
      else {
        continue;
      };
      for point_index in 0..series.bubble_sizes.len() {
        let coordinate = match dimension {
          BubbleAxisDimension::Horizontal => series
            .x_values
            .get(point_index)
            .copied()
            .flatten()
            .unwrap_or(point_index as f64 + 1.0),
          BubbleAxisDimension::Vertical => {
            let Some(value) = series.values.get(point_index).copied().flatten() else {
              continue;
            };
            value
          }
        };
        let Some(radius_fraction) =
          bubble_marker_radius_fraction(series, point_index, group_maximum)
        else {
          continue;
        };
        let center = axis_value_ratio_unclamped(coordinate, scale);
        let lower = axis_value_at_ratio(center - radius_fraction, scale);
        let upper = axis_value_at_ratio(center + radius_fraction, scale);
        envelope_exceeds_scale |= lower < scale.minimum || upper > scale.maximum;
        envelope_values.push(lower);
        envelope_values.push(upper);
      }
    }
    // A constant-valued Office axis deliberately receives a broad default
    // domain (for example x=1 -> 0..1.2). Do not collapse that domain around
    // a small bubble envelope that already fits inside it.
    if degenerate_source && !envelope_exceeds_scale {
      break;
    }
    let next = chart_linear_axis_scale(
      envelope_values,
      axis,
      format_code,
      maximum_auto_increment_count,
      options,
    )?;
    if next == scale {
      break;
    }
    scale = next;
  }
  Some(scale)
}

fn axis_value_ratio_unclamped(value: f64, scale: crate::render::chart::LinearAxisScale) -> f64 {
  if let Some(base) = scale.logarithmic_base.filter(|base| {
    *base > 1.0 && value > 0.0 && scale.minimum > 0.0 && scale.maximum > scale.minimum
  }) {
    (value.log(base) - scale.minimum.log(base))
      / (scale.maximum.log(base) - scale.minimum.log(base))
  } else {
    (value - scale.minimum) / (scale.maximum - scale.minimum)
  }
}

fn axis_value_at_ratio(ratio: f64, scale: crate::render::chart::LinearAxisScale) -> f64 {
  if let Some(base) = scale
    .logarithmic_base
    .filter(|base| *base > 1.0 && scale.minimum > 0.0 && scale.maximum > scale.minimum)
  {
    base.powf(scale.minimum.log(base) + ratio * (scale.maximum.log(base) - scale.minimum.log(base)))
  } else {
    scale.minimum + ratio * (scale.maximum - scale.minimum)
  }
}

fn chart_linear_axis_scale(
  values: Vec<f64>,
  axis: Option<&c::ValueAxis>,
  format_code: Option<&str>,
  maximum_auto_increment_count: usize,
  options: LinearAxisScaleOptions,
) -> Option<crate::render::chart::LinearAxisScale> {
  let initial = linear_axis_scale_with_options(
    values.iter().copied(),
    axis,
    maximum_auto_increment_count,
    options,
  )?;
  let adjusted_count = automatic_increment_count_without_duplicate_labels(
    initial,
    axis,
    format_code,
    maximum_auto_increment_count,
  );
  if adjusted_count >= maximum_auto_increment_count {
    return Some(initial);
  }
  linear_axis_scale_with_options(values, axis, adjusted_count, options)
}

fn automatic_increment_count_without_duplicate_labels(
  scale: crate::render::chart::LinearAxisScale,
  axis: Option<&c::ValueAxis>,
  format_code: Option<&str>,
  maximum_auto_increment_count: usize,
) -> usize {
  let Some(axis) = axis else {
    return maximum_auto_increment_count;
  };
  if axis.major_unit.is_some() {
    return maximum_auto_increment_count;
  }
  let Some(format_code) = format_code.filter(|format| !format.eq_ignore_ascii_case("General"))
  else {
    return maximum_auto_increment_count;
  };
  let labels = scale_tick_labels(
    scale.minimum,
    scale.maximum,
    scale.major_unit,
    Some(format_code),
    scale.logarithmic_base,
    value_axis_display_unit(axis),
  );
  let mut previous = None;
  let mut same_label_count = 0usize;
  let mut maximum_same_label_count = 0usize;
  for (_, label) in &labels {
    if previous == Some(label.as_str()) {
      same_label_count += 1;
      maximum_same_label_count = maximum_same_label_count.max(same_label_count);
    } else {
      same_label_count = 0;
    }
    previous = Some(label.as_str());
  }
  if maximum_same_label_count == 0 {
    return maximum_auto_increment_count;
  }
  // LibreOffice VCartesianAxis feeds the longest formatted-label duplicate
  // run back into ScaleAutomatism. This is semantic, not overlap tuning: an
  // integer `#,##0` axis must not expose 0.5 intervals as repeated integers.
  maximum_auto_increment_count
    .min(labels.len() / (maximum_same_label_count + 1))
    .max(2)
}

fn cartesian_scale_values(
  chart: &ClusteredColumnChart<'_>,
  category_count: usize,
  axis_set_index: usize,
) -> Vec<f64> {
  let mut values = chart
    .series
    .iter()
    .filter(|series| {
      series.axis_set_index == axis_set_index
        && !matches!(
          series.grouping,
          ChartSeriesGrouping::Stacked | ChartSeriesGrouping::PercentStacked
        )
    })
    .flat_map(|series| series.values.iter().flatten().copied())
    .collect::<Vec<_>>();
  if chart.series.iter().any(|series| {
    series.axis_set_index == axis_set_index
      && series.grouping == ChartSeriesGrouping::PercentStacked
  }) {
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
      for series in chart.series.iter().filter(|series| {
        series.axis_set_index == axis_set_index
          && series.kind == kind
          && series.grouping == ChartSeriesGrouping::Stacked
      }) {
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
  // Error-bar endpoints participate in automatic value-axis scaling.  Keep
  // the authored axis bounds authoritative later in `linear_axis_scale`, but
  // do not clip automatic fixed/percentage/statistical/custom extents to the
  // raw data range.
  for (series_index, series) in chart
    .series
    .iter()
    .enumerate()
    .filter(|(_, series)| series.axis_set_index == axis_set_index)
  {
    for error_bars in series
      .error_bars
      .iter()
      .filter(|bars| bars.direction == c::ErrorBarDirectionValues::Y)
    {
      for (point_index, value) in series.values.iter().enumerate() {
        let Some(value) = value.as_ref().copied().filter(|value| value.is_finite()) else {
          continue;
        };
        let center = if matches!(
          series.grouping,
          ChartSeriesGrouping::Stacked | ChartSeriesGrouping::PercentStacked
        ) {
          stacked_value_bounds(chart, series_index, point_index, value).1
        } else {
          value
        };
        if error_bars.show_positive
          && let Some(delta) = chart_error_bar_delta(error_bars, series, point_index, true)
        {
          values.push(center + delta);
        }
        if error_bars.show_negative
          && let Some(delta) = chart_error_bar_delta(error_bars, series, point_index, false)
        {
          values.push(center - delta);
        }
      }
    }
  }
  values
}

fn chart_error_bar_delta(
  error_bars: &crate::render::chart::ChartErrorBars<'_>,
  series: &crate::render::chart::ClusteredColumnSeries<'_>,
  point_index: usize,
  positive: bool,
) -> Option<f64> {
  let data = match error_bars.direction {
    c::ErrorBarDirectionValues::X => &series.x_values,
    c::ErrorBarDirectionValues::Y => &series.values,
  };
  let delta = match &error_bars.values {
    ChartErrorBarValues::Custom {
      positive_values,
      negative_values,
      ..
    } => if positive {
      positive_values.get(point_index)
    } else {
      negative_values.get(point_index)
    }
    .copied()
    .flatten()?,
    ChartErrorBarValues::Fixed(value) => *value,
    ChartErrorBarValues::Percentage(percent) => {
      data.get(point_index).copied().flatten()?.abs() * percent / 100.0
    }
    ChartErrorBarValues::StandardDeviation(weight) => {
      chart_population_standard_deviation(data)? * weight
    }
    ChartErrorBarValues::StandardError => {
      let count = data
        .iter()
        .flatten()
        .filter(|value| value.is_finite())
        .count();
      if count == 0 {
        return None;
      }
      chart_population_standard_deviation(data)? / (count as f64).sqrt()
    }
  }
  .abs();
  (delta.is_finite()).then_some(delta)
}

fn chart_population_standard_deviation(values: &[Option<f64>]) -> Option<f64> {
  let mut count = 0usize;
  let mut sum = 0.0;
  let mut square_sum = 0.0;
  for value in values.iter().flatten().filter(|value| value.is_finite()) {
    count += 1;
    sum += *value;
    square_sum += *value * *value;
  }
  if count == 0 {
    return None;
  }
  let count = count as f64;
  let variance = ((square_sum - sum * sum / count) / count).max(0.0);
  Some(variance.sqrt())
}

fn visible_bubble_size(
  series: &crate::render::chart::ClusteredColumnSeries<'_>,
  point_index: usize,
) -> Option<f64> {
  let value = series.bubble_sizes.get(point_index).copied().flatten()?;
  if !value.is_finite() || value == 0.0 || (value < 0.0 && !series.show_negative_bubbles) {
    return None;
  }
  Some(value.abs())
}

fn bubble_group_maxima(chart: &ClusteredColumnChart<'_>) -> Vec<f64> {
  let group_count = chart
    .series
    .iter()
    .filter_map(|series| series.bubble_group_index)
    .max()
    .map_or(0, |index| index + 1);
  let mut maxima = vec![0.0_f64; group_count];
  for series in chart
    .series
    .iter()
    .filter(|series| series.kind == ChartSeriesKind::Bubble)
  {
    let Some(group_index) = series.bubble_group_index else {
      continue;
    };
    for point_index in 0..series.bubble_sizes.len() {
      if let Some(value) = visible_bubble_size(series, point_index) {
        maxima[group_index] = maxima[group_index].max(value);
      }
    }
  }
  maxima
}

fn bubble_marker_diameter(
  series: &crate::render::chart::ClusteredColumnSeries<'_>,
  point_index: usize,
  geometry: ScatterGeometry,
) -> Option<f32> {
  let relative_diameter = relative_bubble_diameter(series, point_index, geometry.bubble_maximum)?;
  let default_maximum =
    geometry.plot.width.min(geometry.plot.height) * MAXIMUM_BUBBLE_DIAMETER_RATIO;
  let diameter = default_maximum
    * (series.bubble_scale_percent.clamp(0.0, 300.0) / 100.0) as f32
    * relative_diameter;
  (diameter.is_finite() && diameter > 0.0).then_some(diameter)
}

fn relative_bubble_diameter(
  series: &crate::render::chart::ClusteredColumnSeries<'_>,
  point_index: usize,
  group_maximum: f64,
) -> Option<f32> {
  let value = visible_bubble_size(series, point_index)?;
  if group_maximum <= 0.0 {
    return None;
  }
  let relative_diameter = match series.bubble_size_represents {
    // ECMA-376 Part 1 §21.2.3.43: in area mode the painted area is
    // proportional to the value, so its diameter follows the square root.
    c::SizeRepresentsValues::Area => (value / group_maximum).sqrt(),
    // In width mode the bubble diameter itself is proportional to the value.
    c::SizeRepresentsValues::Width => value / group_maximum,
  };
  (relative_diameter.is_finite() && relative_diameter > 0.0).then_some(relative_diameter as f32)
}

fn bubble_marker_radius_fraction(
  series: &crate::render::chart::ClusteredColumnSeries<'_>,
  point_index: usize,
  group_maximum: f64,
) -> Option<f64> {
  let relative_diameter = relative_bubble_diameter(series, point_index, group_maximum)?;
  // Automatic scale is resolved in logical diagram coordinates before the
  // final rectangular plot is mapped to the output device. Reserve half of
  // the same 25% default bubble diameter on each logical value axis; final
  // marker painting later uses the smaller physical plot extent to keep the
  // bubble circular. This distinction is required by both wide and tall
  // fixed-output counterexamples.
  let fraction = MAXIMUM_BUBBLE_DIAMETER_RATIO
    * (series.bubble_scale_percent.clamp(0.0, 300.0) / 100.0) as f32
    * relative_diameter
    * 0.5;
  (fraction.is_finite() && fraction > 0.0).then_some(f64::from(fraction))
}

fn lower_series_geometry(
  items: &mut Vec<PageItem>,
  chart: &ClusteredColumnChart<'_>,
  style: &ClusteredColumnStyle,
  plot: PlotRect,
  axis_scales: &[CartesianAxisScales],
  category_count: usize,
  projection_3d: Option<Chart3DProjection>,
) {
  lower_surface_groups(
    items,
    chart,
    style,
    plot,
    axis_scales,
    category_count,
    projection_3d,
  );

  let bubble_group_maxima = bubble_group_maxima(chart);

  let mut series_indices = (0..chart.series.len()).collect::<Vec<_>>();
  if projection_3d.is_some() {
    // Paint far series first so nearer bars and markers occlude them. The
    // order is independent of legend/text order, which remains the authored
    // chart-group order.
    series_indices.sort_by(|left, right| {
      let left_depth = chart_3d_series_depth_slot(chart, *left).0;
      let right_depth = chart_3d_series_depth_slot(chart, *right).0;
      right_depth.total_cmp(&left_depth).then(left.cmp(right))
    });
  }
  for series_index in series_indices {
    let series = &chart.series[series_index];
    let Some(color) = style.series_colors.get(series_index).copied() else {
      continue;
    };
    let axes = axis_scales
      .get(series.axis_set_index)
      .unwrap_or(&axis_scales[0]);
    let context = SeriesGeometryContext {
      chart,
      plot,
      scale: axes.y,
      zero_y: value_y(
        0.0_f64.clamp(axes.y.minimum, axes.y.maximum),
        axes.y,
        plot.top,
        plot.height,
      ),
      category_count,
      projection_3d,
    };
    let scatter_geometry = ScatterGeometry {
      plot,
      scale: axes.y,
      x_scale: axes.x,
      bubble_maximum: series
        .bubble_group_index
        .and_then(|group| bubble_group_maxima.get(group).copied())
        .unwrap_or(0.0),
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
      ChartSeriesKind::Area => {
        lower_line_series(items, &context, series_index, color, true, style);
      }
      ChartSeriesKind::Surface => {}
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
    if !series.error_bars.is_empty() && !series.is_3d {
      lower_series_error_bars(items, &context, style, series_index, color, axes.x);
    }
    if !series.trendlines.is_empty() {
      lower_trendlines(
        items,
        chart,
        series,
        series_index,
        color,
        style,
        TrendlineGeometry {
          plot,
          scale: axes.y,
          category_count,
          x_scale: axes.x,
        },
      );
    }
  }
}

fn lower_series_error_bars(
  items: &mut Vec<PageItem>,
  context: &SeriesGeometryContext<'_, '_>,
  style: &ClusteredColumnStyle,
  series_index: usize,
  fallback_color: RgbColor,
  x_scale: Option<crate::render::chart::LinearAxisScale>,
) {
  let series = &context.chart.series[series_index];
  for (error_bar_index, error_bars) in series.error_bars.iter().enumerate() {
    let stroke = style
      .error_bar_styles
      .get(series_index)
      .and_then(|styles| styles.get(error_bar_index))
      .map(|shape| &shape.stroke);
    for point_index in 0..series.values.len().max(series.x_values.len()) {
      let Some(points) =
        chart_error_bar_points(context, series_index, point_index, error_bars, x_scale)
      else {
        continue;
      };
      let positive = error_bars
        .show_positive
        .then_some(points.positive)
        .flatten();
      let negative = error_bars
        .show_negative
        .then_some(points.negative)
        .flatten();
      let start = negative.unwrap_or(points.center);
      let end = positive.unwrap_or(points.center);
      if let Some((start, end)) = clip_chart_line_segment_to_plot(start, end, context.plot) {
        push_chart_styled_line(
          items,
          start,
          end,
          stroke,
          fallback_color,
          0.75 * style.stroke_scale,
          style.stroke_scale,
        );
      }
      if error_bars.no_end_cap {
        continue;
      }
      // LibreOffice's chart view uses a fixed 200 hundredths-of-a-millimetre
      // cap in scene coordinates. Keep the same 2 mm physical size rather
      // than scaling caps with the data interval or marker magnitude.
      let cap_half = (2.0 * 72.0 / 25.4) * 0.5;
      for endpoint in [negative, positive].into_iter().flatten() {
        if !plot_contains_point(context.plot, endpoint) {
          continue;
        }
        let (cap_start, cap_end) = if points.horizontal {
          (
            (endpoint.0, endpoint.1 - cap_half),
            (endpoint.0, endpoint.1 + cap_half),
          )
        } else {
          (
            (endpoint.0 - cap_half, endpoint.1),
            (endpoint.0 + cap_half, endpoint.1),
          )
        };
        push_chart_styled_line(
          items,
          cap_start,
          cap_end,
          stroke,
          fallback_color,
          0.75 * style.stroke_scale,
          style.stroke_scale,
        );
      }
    }
  }
}

#[derive(Clone, Copy)]
struct ChartErrorBarPoints {
  center: (f32, f32),
  positive: Option<(f32, f32)>,
  negative: Option<(f32, f32)>,
  horizontal: bool,
}

fn chart_error_bar_points(
  context: &SeriesGeometryContext<'_, '_>,
  series_index: usize,
  point_index: usize,
  error_bars: &crate::render::chart::ChartErrorBars<'_>,
  x_scale: Option<crate::render::chart::LinearAxisScale>,
) -> Option<ChartErrorBarPoints> {
  let chart = context.chart;
  let series = chart.series.get(series_index)?;
  let positive_delta = error_bars
    .show_positive
    .then(|| chart_error_bar_delta(error_bars, series, point_index, true))
    .flatten();
  let negative_delta = error_bars
    .show_negative
    .then(|| chart_error_bar_delta(error_bars, series, point_index, false))
    .flatten();
  if positive_delta.is_none() && negative_delta.is_none() {
    return None;
  }

  match series.kind {
    ChartSeriesKind::Scatter | ChartSeriesKind::Bubble => {
      let y_value = series.values.get(point_index).copied().flatten()?;
      let x_value = series
        .x_values
        .get(point_index)
        .copied()
        .flatten()
        .unwrap_or(point_index as f64 + 1.0);
      let center = (
        x_scale.map_or(context.plot.left + context.plot.width * 0.5, |scale| {
          value_x(x_value, scale, context.plot)
        }),
        value_y(
          y_value,
          context.scale,
          context.plot.top,
          context.plot.height,
        ),
      );
      match error_bars.direction {
        c::ErrorBarDirectionValues::X => {
          let scale = x_scale?;
          let positive =
            positive_delta.map(|delta| (value_x(x_value + delta, scale, context.plot), center.1));
          let negative =
            negative_delta.map(|delta| (value_x(x_value - delta, scale, context.plot), center.1));
          Some(ChartErrorBarPoints {
            center,
            positive,
            negative,
            horizontal: true,
          })
        }
        c::ErrorBarDirectionValues::Y => {
          let positive = positive_delta.map(|delta| {
            (
              center.0,
              value_y(
                y_value + delta,
                context.scale,
                context.plot.top,
                context.plot.height,
              ),
            )
          });
          let negative = negative_delta.map(|delta| {
            (
              center.0,
              value_y(
                y_value - delta,
                context.scale,
                context.plot.top,
                context.plot.height,
              ),
            )
          });
          Some(ChartErrorBarPoints {
            center,
            positive,
            negative,
            horizontal: false,
          })
        }
      }
    }
    ChartSeriesKind::Bar if error_bars.direction == c::ErrorBarDirectionValues::Y => {
      let value = series.values.get(point_index).copied().flatten()?;
      let (_, center_value) = stacked_value_bounds(chart, series_index, point_index, value);
      let center = (
        value_x(center_value, context.scale, context.plot),
        clustered_series_slot_center(context, series_index, point_index, true)?,
      );
      let positive = positive_delta.map(|delta| {
        (
          value_x(center_value + delta, context.scale, context.plot),
          center.1,
        )
      });
      let negative = negative_delta.map(|delta| {
        (
          value_x(center_value - delta, context.scale, context.plot),
          center.1,
        )
      });
      Some(ChartErrorBarPoints {
        center,
        positive,
        negative,
        horizontal: true,
      })
    }
    ChartSeriesKind::Column
    | ChartSeriesKind::Line
    | ChartSeriesKind::Area
    | ChartSeriesKind::Stock
      if error_bars.direction == c::ErrorBarDirectionValues::Y =>
    {
      let value = series.values.get(point_index).copied().flatten()?;
      let (_, center_value) = stacked_value_bounds(chart, series_index, point_index, value);
      let center_x = if series.kind == ChartSeriesKind::Column {
        clustered_series_slot_center(context, series_index, point_index, false)?
      } else {
        let display_index = category_display_index(chart, point_index, context.category_count);
        category_point_x(chart, display_index, context.category_count, context.plot)
      };
      let center = (
        center_x,
        value_y(
          center_value,
          context.scale,
          context.plot.top,
          context.plot.height,
        ),
      );
      let positive = positive_delta.map(|delta| {
        (
          center.0,
          value_y(
            center_value + delta,
            context.scale,
            context.plot.top,
            context.plot.height,
          ),
        )
      });
      let negative = negative_delta.map(|delta| {
        (
          center.0,
          value_y(
            center_value - delta,
            context.scale,
            context.plot.top,
            context.plot.height,
          ),
        )
      });
      Some(ChartErrorBarPoints {
        center,
        positive,
        negative,
        horizontal: false,
      })
    }
    ChartSeriesKind::Column
    | ChartSeriesKind::Bar
    | ChartSeriesKind::Line
    | ChartSeriesKind::Area
    | ChartSeriesKind::Radar
    | ChartSeriesKind::Stock
    | ChartSeriesKind::Surface => None,
  }
}

fn clustered_series_slot_center(
  context: &SeriesGeometryContext<'_, '_>,
  series_index: usize,
  category_index: usize,
  horizontal_bar: bool,
) -> Option<f32> {
  let series = context.chart.series.get(series_index)?;
  let peer_count = context
    .chart
    .series
    .iter()
    .filter(|peer| {
      peer.axis_set_index == series.axis_set_index
        && peer.kind == series.kind
        && peer.grouping == series.grouping
    })
    .count()
    .max(1);
  let peer_index = context.chart.series[..series_index]
    .iter()
    .filter(|peer| {
      peer.axis_set_index == series.axis_set_index
        && peer.kind == series.kind
        && peer.grouping == series.grouping
    })
    .count();
  let clustered = series.grouping == ChartSeriesGrouping::Clustered;
  let slot = clustered_column_slot(
    series_category_display_index(
      context.chart.category_axis_reversed,
      series.kind,
      category_index,
      context.category_count,
    ),
    if clustered { peer_index } else { 0 },
    context.category_count,
    if clustered { peer_count } else { 1 },
    context.chart.gap_width_percent,
    context.chart.overlap_percent,
  )?;
  Some(if horizontal_bar {
    context.plot.top + slot.center as f32 * context.plot.height
  } else {
    context.plot.left + slot.center as f32 * context.plot.width
  })
}

fn plot_contains_point(plot: PlotRect, point: (f32, f32)) -> bool {
  point.0 >= plot.left
    && point.0 <= plot.left + plot.width
    && point.1 >= plot.top
    && point.1 <= plot.top + plot.height
}

fn chart_3d_series_slot_context(
  chart: &ClusteredColumnChart<'_>,
  series_index: usize,
) -> Option<(bool, usize, usize, f32)> {
  let Some(series) = chart.series.get(series_index) else {
    return None;
  };
  if !series.is_3d {
    return None;
  }
  // Only the `standard` 3-D grouping is a deep chart: LibreOffice's
  // TypeGroupConverter maps it to Z_STACKING. Clustered bars share one depth
  // slab and use ordinary category-direction slots, while stacked families
  // share both the category and depth slots.
  let shares_one_depth_slot = series.grouping != ChartSeriesGrouping::Standard;
  let peers = chart
    .series
    .iter()
    .enumerate()
    .filter(|(_, peer)| {
      peer.is_3d
        && peer.axis_set_index == series.axis_set_index
        && peer.kind == series.kind
        && peer.grouping == series.grouping
    })
    .map(|(index, _)| index)
    .collect::<Vec<_>>();
  let slot_count = if shares_one_depth_slot {
    1
  } else {
    peers.len().max(1)
  };
  let slot_index = if shares_one_depth_slot {
    0
  } else {
    peers
      .iter()
      .position(|index| *index == series_index)
      .unwrap_or(0)
  };
  let outer_distance = (series.gap_depth_percent as f32 / 100.0).clamp(0.0, 6.0);
  Some((
    shares_one_depth_slot,
    slot_count,
    slot_index,
    outer_distance,
  ))
}

fn chart_3d_series_depth_slot(chart: &ClusteredColumnChart<'_>, series_index: usize) -> (f32, f32) {
  let Some((shares_one_depth_slot, slot_count, slot_index, gap_depth)) =
    chart_3d_series_slot_context(chart, series_index)
  else {
    return (0.0, 0.0);
  };
  let (marker_depth, front) = if shares_one_depth_slot || slot_count == 1 {
    // A single shared cluster retains the symmetric front/back clearance used
    // by LibreOffice's category-position model.
    let marker_depth = 1.0 / (slot_count as f32 + gap_depth);
    (marker_depth, gap_depth * marker_depth * 0.5)
  } else {
    // PowerPoint's GapDepth property is the distance *between data series* as
    // a percentage of marker depth. Therefore a standard/deep chart has N
    // markers and N-1 authored gaps; applying the ordinary category outer-gap
    // equation here makes every cuboid about twice as deep and changes the
    // painter occlusion order in Office's fixed output.
    let marker_depth = 1.0 / (slot_count as f32 + gap_depth * (slot_count - 1) as f32);
    (
      marker_depth,
      slot_index as f32 * marker_depth * (1.0 + gap_depth),
    )
  };
  (front, (front + marker_depth).min(1.0))
}

fn chart_3d_series_axis_label_depth(
  chart: &ClusteredColumnChart<'_>,
  series_index: usize,
  layout_profile: ChartLayoutProfile,
) -> f32 {
  let Some((_, slot_count, slot_index, gap_depth)) =
    chart_3d_series_slot_context(chart, series_index)
  else {
    return 0.0;
  };
  // The series axis is a shifted category axis, not the marker's front face.
  // LibreOffice explicitly constructs shifted label ticks separately from the
  // unshifted axis marks. Preserve that independent rhythm when GapDepth
  // narrows the actual cuboids.
  let tick_slot = 1.0 / (slot_count as f32 + gap_depth);
  let tick_front = gap_depth * tick_slot * 0.5 + slot_index as f32 * tick_slot;
  let slot_ratio = if layout_profile == ChartLayoutProfile::PowerPoint {
    profiles::POWERPOINT_CARTESIAN_3D_SERIES_AXIS_SLOT_RATIO
  } else {
    0.5
  };
  tick_front + tick_slot * slot_ratio
}

#[derive(Clone, Copy, Debug)]
struct SurfaceVertex {
  x: f32,
  value: f64,
  depth_ratio: f32,
}

fn lower_surface_groups(
  items: &mut Vec<PageItem>,
  chart: &ClusteredColumnChart<'_>,
  style: &ClusteredColumnStyle,
  plot: PlotRect,
  axis_scales: &[CartesianAxisScales],
  category_count: usize,
  projection_3d: Option<Chart3DProjection>,
) {
  for (group_index, group) in chart.surface_groups.iter().enumerate() {
    let Some(axes) = axis_scales
      .get(group.axis_set_index)
      .or_else(|| axis_scales.first())
    else {
      continue;
    };
    if group.wireframe {
      lower_surface_wireframe(
        items,
        chart,
        style,
        group,
        plot,
        axes.y,
        category_count,
        projection_3d,
      );
    } else {
      lower_filled_surface(
        items,
        chart,
        style,
        group_index,
        group,
        plot,
        axes.y,
        category_count,
        projection_3d,
      );
    }
  }
}

fn surface_series_depth_ratio(
  chart: &ClusteredColumnChart<'_>,
  group: &SurfaceChartGroup<'_>,
  row_index: usize,
) -> f32 {
  let mut ratio = if group.series_count <= 1 {
    0.5
  } else {
    row_index as f32 / (group.series_count - 1) as f32
  };
  let reversed = chart
    .axis_sets
    .get(group.axis_set_index)
    .and_then(|axes| axes.series_axis)
    .and_then(|axis| axis.scaling.orientation.as_ref())
    .and_then(|orientation| orientation.val)
    == Some(c::OrientationValues::MaxMin);
  if reversed {
    ratio = 1.0 - ratio;
  }
  ratio
}

fn surface_vertex_point(
  vertex: SurfaceVertex,
  group: &SurfaceChartGroup<'_>,
  plot: PlotRect,
  scale: crate::render::chart::LinearAxisScale,
  projection_3d: Option<Chart3DProjection>,
) -> (f32, f32) {
  if group.is_3d {
    let y = value_y(vertex.value, scale, plot.top, plot.height);
    projection_3d.map_or((vertex.x, y), |projection| {
      projection.project(vertex.x, y, vertex.depth_ratio)
    })
  } else {
    // ECMA-376 §21.2.2.204 defines `surfaceChart` as the 2-D contour
    // representation: category and series coordinates form the plane while
    // value bands provide the color.
    (
      vertex.x,
      plot.top + vertex.depth_ratio.clamp(0.0, 1.0) * plot.height,
    )
  }
}

fn lower_surface_wireframe(
  items: &mut Vec<PageItem>,
  chart: &ClusteredColumnChart<'_>,
  style: &ClusteredColumnStyle,
  group: &SurfaceChartGroup<'_>,
  plot: PlotRect,
  scale: crate::render::chart::LinearAxisScale,
  category_count: usize,
  projection_3d: Option<Chart3DProjection>,
) {
  let series_range =
    group.first_series_index..group.first_series_index.saturating_add(group.series_count);
  for (row_index, series_index) in series_range.clone().enumerate() {
    let Some(series) = chart.series.get(series_index) else {
      continue;
    };
    let depth_ratio = surface_series_depth_ratio(chart, group, row_index);
    let color = style
      .series_colors
      .get(series_index)
      .copied()
      .unwrap_or(style.gridline_color);
    let mut previous = None;
    for (category_index, value) in series.values.iter().enumerate() {
      let Some(value) = value.as_ref().copied().filter(|value| value.is_finite()) else {
        previous = None;
        continue;
      };
      let display_index = category_display_index(chart, category_index, category_count);
      let point = surface_vertex_point(
        SurfaceVertex {
          x: category_point_x(chart, display_index, category_count, plot),
          value,
          depth_ratio,
        },
        group,
        plot,
        scale,
        projection_3d,
      );
      if let Some(previous) = previous {
        lower_chart_line_segment(items, previous, point, color, 0.75 * style.stroke_scale);
      }
      previous = Some(point);
    }
  }

  for category_index in 0..category_count {
    let mut previous = None;
    for (row_index, series_index) in series_range.clone().enumerate() {
      let Some(value) = chart
        .series
        .get(series_index)
        .and_then(|series| series.values.get(category_index))
        .copied()
        .flatten()
        .filter(|value| value.is_finite())
      else {
        previous = None;
        continue;
      };
      let display_index = category_display_index(chart, category_index, category_count);
      let point = surface_vertex_point(
        SurfaceVertex {
          x: category_point_x(chart, display_index, category_count, plot),
          value,
          depth_ratio: surface_series_depth_ratio(chart, group, row_index),
        },
        group,
        plot,
        scale,
        projection_3d,
      );
      if let Some(previous) = previous {
        lower_chart_line_segment(
          items,
          previous,
          point,
          style.gridline_color,
          0.75 * style.stroke_scale,
        );
      }
      previous = Some(point);
    }
  }
}

fn lower_filled_surface(
  items: &mut Vec<PageItem>,
  chart: &ClusteredColumnChart<'_>,
  style: &ClusteredColumnStyle,
  group_index: usize,
  group: &SurfaceChartGroup<'_>,
  plot: PlotRect,
  scale: crate::render::chart::LinearAxisScale,
  category_count: usize,
  projection_3d: Option<Chart3DProjection>,
) {
  if group.series_count < 2 || category_count < 2 {
    return;
  }
  let mut cells = Vec::new();
  for row_index in 0..group.series_count - 1 {
    let Some(front_series) = chart.series.get(group.first_series_index + row_index) else {
      continue;
    };
    let Some(back_series) = chart.series.get(group.first_series_index + row_index + 1) else {
      continue;
    };
    let front_depth = surface_series_depth_ratio(chart, group, row_index);
    let back_depth = surface_series_depth_ratio(chart, group, row_index + 1);
    for category_index in 0..category_count - 1 {
      let Some(values) = [
        front_series.values.get(category_index).copied().flatten(),
        front_series
          .values
          .get(category_index + 1)
          .copied()
          .flatten(),
        back_series
          .values
          .get(category_index + 1)
          .copied()
          .flatten(),
        back_series.values.get(category_index).copied().flatten(),
      ]
      .into_iter()
      .collect::<Option<Vec<_>>>() else {
        continue;
      };
      if values.iter().any(|value| !value.is_finite()) {
        continue;
      }
      let first_display_index = category_display_index(chart, category_index, category_count);
      let second_display_index = category_display_index(chart, category_index + 1, category_count);
      cells.push([
        SurfaceVertex {
          x: category_point_x(chart, first_display_index, category_count, plot),
          value: values[0],
          depth_ratio: front_depth,
        },
        SurfaceVertex {
          x: category_point_x(chart, second_display_index, category_count, plot),
          value: values[1],
          depth_ratio: front_depth,
        },
        SurfaceVertex {
          x: category_point_x(chart, second_display_index, category_count, plot),
          value: values[2],
          depth_ratio: back_depth,
        },
        SurfaceVertex {
          x: category_point_x(chart, first_display_index, category_count, plot),
          value: values[3],
          depth_ratio: back_depth,
        },
      ]);
    }
  }
  if group.is_3d {
    cells.sort_by(|left, right| {
      let left_depth = left.iter().map(|vertex| vertex.depth_ratio).sum::<f32>();
      let right_depth = right.iter().map(|vertex| vertex.depth_ratio).sum::<f32>();
      right_depth.total_cmp(&left_depth)
    });
  }

  for cell in cells {
    let minimum_value = cell
      .iter()
      .map(|vertex| vertex.value)
      .fold(f64::INFINITY, f64::min);
    let maximum_value = cell
      .iter()
      .map(|vertex| vertex.value)
      .fold(f64::NEG_INFINITY, f64::max);
    let first_band = surface_band_index(minimum_value, scale);
    let last_band = surface_band_index(maximum_value, scale);
    for band_index in first_band..=last_band {
      let mut polygon = cell.to_vec();
      let lower = scale.minimum + band_index as f64 * scale.major_unit;
      let upper = (lower + scale.major_unit).min(scale.maximum);
      polygon = clip_surface_polygon(&polygon, lower, true);
      polygon = clip_surface_polygon(&polygon, upper, false);
      if polygon.len() < 3 {
        continue;
      }
      let points = polygon
        .into_iter()
        .map(|vertex| surface_vertex_point(vertex, group, plot, scale, projection_3d))
        .collect::<Vec<_>>();
      let color = surface_band_color(style, group_index, band_index as u32);
      push_chart_polygon(
        items,
        &points,
        color,
        Some((shade_chart_color(color, 0.72), 0.3 * style.stroke_scale)),
      );
    }
  }
}

fn surface_band_index(value: f64, scale: crate::render::chart::LinearAxisScale) -> usize {
  if !scale.major_unit.is_finite() || scale.major_unit <= f64::EPSILON {
    return 0;
  }
  let band_count = ((scale.maximum - scale.minimum) / scale.major_unit)
    .ceil()
    .max(1.0) as usize;
  (((value - scale.minimum) / scale.major_unit).floor() as isize)
    .clamp(0, band_count.saturating_sub(1) as isize) as usize
}

fn clip_surface_polygon(
  polygon: &[SurfaceVertex],
  threshold: f64,
  keep_above: bool,
) -> Vec<SurfaceVertex> {
  let Some(mut previous) = polygon.last().copied() else {
    return Vec::new();
  };
  let inside = |vertex: SurfaceVertex| {
    if keep_above {
      vertex.value >= threshold - f64::EPSILON * 16.0
    } else {
      vertex.value <= threshold + f64::EPSILON * 16.0
    }
  };
  let mut previous_inside = inside(previous);
  let mut result = Vec::new();
  for current in polygon.iter().copied() {
    let current_inside = inside(current);
    if current_inside != previous_inside {
      let value_span = current.value - previous.value;
      if value_span.abs() > f64::EPSILON {
        let ratio = ((threshold - previous.value) / value_span).clamp(0.0, 1.0) as f32;
        result.push(SurfaceVertex {
          x: previous.x + (current.x - previous.x) * ratio,
          value: threshold,
          depth_ratio: previous.depth_ratio + (current.depth_ratio - previous.depth_ratio) * ratio,
        });
      }
    }
    if current_inside {
      result.push(current);
    }
    previous = current;
    previous_inside = current_inside;
  }
  result
}

fn surface_band_color(
  style: &ClusteredColumnStyle,
  group_index: usize,
  band_index: u32,
) -> RgbColor {
  style
    .surface_band_colors
    .get(group_index)
    .and_then(|colors| {
      colors
        .iter()
        .find_map(|(index, color)| (*index == band_index).then_some(*color))
    })
    .or_else(|| {
      (!style.series_colors.is_empty())
        .then(|| style.series_colors[band_index as usize % style.series_colors.len()])
    })
    .unwrap_or(RgbColor {
      r: 68,
      g: 114,
      b: 196,
    })
}

fn lower_series_axes(
  items: &mut Vec<PageItem>,
  chart: &ClusteredColumnChart<'_>,
  plot: PlotRect,
  projection: Chart3DProjection,
  style: &ClusteredColumnStyle,
  metrics: &mut TextMetrics,
) {
  for (axis_set_index, axes) in chart.axis_sets.iter().enumerate() {
    let Some(axis) = axes.series_axis else {
      continue;
    };
    if axis
      .delete
      .as_ref()
      .is_some_and(|delete| delete.val.is_none_or(|value| value.as_bool()))
    {
      continue;
    }

    let surface_group = chart
      .surface_groups
      .iter()
      .find(|group| group.axis_set_index == axis_set_index);
    let mut labels = if let Some(group) = surface_group {
      (0..group.series_count)
        .filter_map(|row_index| {
          let series_index = group.first_series_index + row_index;
          chart.series.get(series_index).map(|series| {
            (
              series.name.as_str(),
              surface_series_depth_ratio(chart, group, row_index),
            )
          })
        })
        .collect::<Vec<_>>()
    } else {
      chart
        .series
        .iter()
        .enumerate()
        .filter(|(_, series)| series.axis_set_index == axis_set_index && series.is_3d)
        .map(|(series_index, series)| {
          (
            series.name.as_str(),
            chart_3d_series_axis_label_depth(chart, series_index, style.layout_profile),
          )
        })
        .collect::<Vec<_>>()
    };
    labels.dedup_by(|left, right| (left.1 - right.1).abs() <= f32::EPSILON);
    if labels.is_empty() {
      continue;
    }

    let axis_position = axis.axis_position.val;
    let base = match axis_position {
      c::AxisPositionValues::Bottom => (
        projection.x_for_visual_side(plot, true, plot.top + plot.height, 0.0),
        plot.top + plot.height,
      ),
      c::AxisPositionValues::Top => (
        projection.x_for_visual_side(plot, true, plot.top, 0.0),
        plot.top,
      ),
      c::AxisPositionValues::Left => (
        projection.x_for_visual_side(plot, false, plot.top + plot.height, 0.0),
        plot.top + plot.height,
      ),
      c::AxisPositionValues::Right => (
        projection.x_for_visual_side(plot, true, plot.top + plot.height, 0.0),
        plot.top + plot.height,
      ),
    };
    let start = projection.project(base.0, base.1, 0.0);
    let end = projection.project(base.0, base.1, 1.0);
    let axis_width = style.axis_line_width_pt.unwrap_or(0.75) * style.stroke_scale;
    items.push(PageItem::Line(LineItem {
      x1_pt: start.0,
      y1_pt: start.1,
      x2_pt: end.0,
      y2_pt: end.1,
      width_pt: axis_width,
      color: style.gridline_color,
      kind: LineItemKind::Stroke,
    }));

    let labels_visible = axis
      .tick_label_position
      .as_ref()
      .is_none_or(|position| position.val != Some(c::TickLabelPositionValues::None));
    let maximum_label_width = labels
      .iter()
      .map(|(text, _)| metrics.measure_text(text, &style.series_label))
      .fold(0.0_f32, f32::max);
    let explicit_rhythm = axis
      .tick_label_skip
      .as_ref()
      .map(|skip| skip.val.max(1) as usize);
    let label_points = labels
      .iter()
      .map(|(_, depth_ratio)| projection.project(base.0, base.1, *depth_ratio))
      .collect::<Vec<_>>();
    let rhythm = series_axis_label_rhythm(
      maximum_label_width,
      style.series_label.font_size_pt,
      &label_points,
      explicit_rhythm,
    );
    let center = (plot.left + plot.width * 0.5, plot.top + plot.height * 0.5);
    let midpoint = ((start.0 + end.0) * 0.5, (start.1 + end.1) * 0.5);
    let outward = {
      let vector = (midpoint.0 - center.0, midpoint.1 - center.1);
      let length = vector.0.hypot(vector.1);
      if length > f32::EPSILON {
        (vector.0 / length, vector.1 / length)
      } else {
        (0.0, 1.0)
      }
    };
    let tick_length = style.series_label.font_size_pt * 0.3;
    for (index, ((label, _), point)) in labels.into_iter().zip(label_points).enumerate() {
      items.push(PageItem::Line(LineItem {
        x1_pt: point.0,
        y1_pt: point.1,
        x2_pt: point.0 + outward.0 * tick_length,
        y2_pt: point.1 + outward.1 * tick_length,
        width_pt: axis_width,
        color: style.gridline_color,
        kind: LineItemKind::Stroke,
      }));
      if labels_visible && index % rhythm == 0 {
        let width = metrics.measure_text(label, &style.series_label);
        push_text(
          items,
          point.0 + outward.0 * tick_length * 1.8 - if outward.0 < 0.0 { width } else { 0.0 },
          point.1 + outward.1 * tick_length * 1.8
            - if outward.1 < 0.0 {
              line_height(&style.series_label)
            } else {
              0.0
            },
          label.to_string(),
          style.series_label.clone(),
        );
      }
    }
  }
}

fn series_axis_label_rhythm(
  maximum_label_width: f32,
  label_height: f32,
  label_points: &[(f32, f32)],
  explicit_rhythm: Option<usize>,
) -> usize {
  if let Some(rhythm) = explicit_rhythm {
    return rhythm.max(1);
  }
  let label_count = label_points.len();
  if label_count <= 1 || maximum_label_width <= 0.0 {
    return 1;
  }
  // LibreOffice VCartesianAxis increases rhythm only when the actual text
  // shapes overlap. A projected series axis is diagonal, so horizontal label
  // extents alone are insufficient: vertical separation can make adjacent
  // labels disjoint even when their x ranges intersect.
  for rhythm in 1..=label_count {
    let mut previous: Option<(f32, f32)> = None;
    let mut overlaps = false;
    for point in label_points.iter().step_by(rhythm) {
      if let Some(previous) = previous {
        let horizontal_overlap = (point.0 - previous.0).abs() < maximum_label_width * 1.05;
        let vertical_overlap = (point.1 - previous.1).abs() < label_height * 1.05;
        if horizontal_overlap && vertical_overlap {
          overlaps = true;
          break;
        }
      }
      previous = Some(*point);
    }
    if !overlaps {
      return rhythm;
    }
  }
  label_count
}

#[derive(Clone, Copy)]
struct TrendlineGeometry {
  plot: PlotRect,
  scale: crate::render::chart::LinearAxisScale,
  category_count: usize,
  x_scale: Option<crate::render::chart::LinearAxisScale>,
}

fn lower_trendlines(
  items: &mut Vec<PageItem>,
  chart: &ClusteredColumnChart<'_>,
  series: &crate::render::chart::ClusteredColumnSeries<'_>,
  series_index: usize,
  color: RgbColor,
  style: &ClusteredColumnStyle,
  geometry: TrendlineGeometry,
) {
  let TrendlineGeometry {
    plot,
    scale,
    category_count,
    x_scale,
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
  for (trendline_index, trendline) in series.trendlines.iter().enumerate() {
    let kind = trendline
      .trendline_type
      .val
      .unwrap_or(c::TrendlineValues::Linear);
    let mut regression = None;
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
    } else if kind == c::TrendlineValues::Polynomial {
      let degree = trendline
        .polynomial_order
        .as_ref()
        .map_or(2, |order| usize::from(order.val));
      let forced_intercept = trendline.intercept.as_ref().map(|intercept| intercept.val);
      let Some(coefficients) = polynomial_regression(&source, degree, forced_intercept) else {
        continue;
      };
      let (source_minimum, source_maximum) = source.iter().fold(
        (f64::INFINITY, f64::NEG_INFINITY),
        |(minimum, maximum), (x, _)| (minimum.min(*x), maximum.max(*x)),
      );
      let x_minimum = source_minimum
        - trendline
          .backward
          .as_ref()
          .map_or(0.0, |backward| backward.val);
      let x_maximum = source_maximum
        + trendline
          .forward
          .as_ref()
          .map_or(0.0, |forward| forward.val);
      // LibreOffice VSeriesPlotter requests 100 samples from its regression
      // calculator for the ordinary, non-extrapolated chart range. Retain
      // both endpoints so the fixed-output path reaches the authored data
      // extent before plot clipping.
      (0..100)
        .map(|step| x_minimum + (x_maximum - x_minimum) * step as f64 / 99.0)
        .map(|x| (x, polynomial_value(&coefficients, x)))
        .filter(|(_, y)| y.is_finite())
        .collect()
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
      let mean =
        transformed.iter().map(|(_, value)| *value).sum::<f64>() / transformed.len() as f64;
      let residual_sum = transformed
        .iter()
        .map(|(x, y)| {
          let residual = y - (slope * x + intercept);
          residual * residual
        })
        .sum::<f64>();
      let total_sum = transformed
        .iter()
        .map(|(_, y)| {
          let difference = y - mean;
          difference * difference
        })
        .sum::<f64>();
      let r_squared = if total_sum <= f64::EPSILON {
        1.0
      } else {
        (1.0 - residual_sum / total_sum).clamp(0.0, 1.0)
      };
      regression = Some((slope, intercept, r_squared));
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
    let points = points
      .into_iter()
      .map(|(x_value, y_value)| {
        let x = if matches!(
          series.kind,
          ChartSeriesKind::Scatter | ChartSeriesKind::Bubble
        ) {
          x_scale.map_or(plot.left + plot.width * 0.5, |scale| {
            value_x(x_value, scale, plot)
          })
        } else {
          category_value_x(chart, x_value as f32, category_count, plot)
        };
        (x, value_y(y_value, scale, plot.top, plot.height))
      })
      .collect::<Vec<_>>();
    let trendline_stroke = style
      .trendline_styles
      .get(series_index)
      .and_then(|styles| styles.get(trendline_index))
      .map(|shape| &shape.stroke);
    match trendline_stroke {
      Some(crate::common::ShapeStyleValue::NoPaint) => {}
      Some(crate::common::ShapeStyleValue::Paint(stroke)) if points.len() >= 2 => {
        let bounds = common_rect(plot.left, plot.top, plot.width, plot.height);
        items.push(PageItem::Path(crate::common::PathItem {
          bounds,
          points: points.iter().map(|(x, y)| common_point(*x, *y)).collect(),
          commands: Vec::new(),
          closed: false,
          fill: crate::common::Fill::None,
          stroke: Some(bind_chart_stroke_to_bounds(
            stroke,
            bounds,
            style.stroke_scale,
          )),
        }));
      }
      Some(crate::common::ShapeStyleValue::Paint(_))
      | Some(crate::common::ShapeStyleValue::Unspecified)
      | None => {
        for segment in points.windows(2) {
          items.push(PageItem::Line(LineItem {
            x1_pt: segment[0].0,
            y1_pt: segment[0].1,
            x2_pt: segment[1].0,
            y2_pt: segment[1].1,
            width_pt: 1.0,
            color,
            kind: LineItemKind::Stroke,
          }));
        }
      }
    }
    if trendline.trendline_label.is_some()
      && let Some((slope, intercept, r_squared)) = regression
    {
      let show_equation = trendline
        .display_equation
        .as_ref()
        .is_none_or(|value| value.val.is_none_or(|value| value.as_bool()));
      let show_r_squared = trendline
        .display_r_squared_value
        .as_ref()
        .is_none_or(|value| value.val.is_none_or(|value| value.as_bool()));
      let mut fields = Vec::new();
      if show_equation {
        fields.push(trendline_equation(kind, slope, intercept));
      }
      if show_r_squared {
        fields.push(format!(
          "R² = {}",
          crate::render::chart::format_chart_number(r_squared, None)
        ));
      }
      if !fields.is_empty() {
        push_text(
          items,
          plot.left + plot.width * 0.08,
          plot.top - line_height(&style.data_label),
          fields.join(" "),
          style.data_label.clone(),
        );
      }
    }
  }
}

fn trendline_equation(kind: c::TrendlineValues, slope: f64, intercept: f64) -> String {
  let coefficient = crate::render::chart::format_chart_number(slope.abs(), None);
  let intercept_value = crate::render::chart::format_chart_number(intercept.abs(), None);
  let signed_intercept = if intercept.abs() <= 1.0e-12 {
    String::new()
  } else if intercept < 0.0 {
    format!(" - {intercept_value}")
  } else {
    format!(" + {intercept_value}")
  };
  match kind {
    c::TrendlineValues::Exponential => {
      format!(
        "y = {}e^({}x)",
        crate::render::chart::format_chart_number(intercept.exp(), None),
        crate::render::chart::format_chart_number(slope, None)
      )
    }
    c::TrendlineValues::Logarithmic => {
      format!(
        "y = {}ln(x){signed_intercept}",
        crate::render::chart::format_chart_number(slope, None)
      )
    }
    c::TrendlineValues::Power => format!(
      "y = {}x^{}",
      crate::render::chart::format_chart_number(intercept.exp(), None),
      crate::render::chart::format_chart_number(slope, None)
    ),
    _ => {
      let slope = if (slope - 1.0).abs() <= 1.0e-12 {
        String::new()
      } else if (slope + 1.0).abs() <= 1.0e-12 {
        "-".to_string()
      } else {
        coefficient
      };
      format!("y = {slope}x{signed_intercept}")
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

fn polynomial_regression(
  values: &[(f64, f64)],
  degree: usize,
  forced_intercept: Option<f64>,
) -> Option<Vec<f64>> {
  if values.len() < 2 {
    return None;
  }
  let degree = degree.clamp(1, values.len().saturating_sub(1));
  let power_count = if forced_intercept.is_some() {
    degree
  } else {
    degree + 1
  };
  let row_count = values.len();
  let mut y = values
    .iter()
    .map(|(_, value)| *value - forced_intercept.unwrap_or(0.0))
    .collect::<Vec<_>>();
  let mut qr_transposed = vec![0.0; row_count * power_count];
  for column in 0..power_count {
    let power = if forced_intercept.is_some() {
      column + 1
    } else {
      column
    } as i32;
    for (row, (x, _)) in values.iter().enumerate() {
      qr_transposed[row + column * row_count] = x.powi(power);
    }
  }

  // Householder QR follows LibreOffice
  // PolynomialRegressionCurveCalculator::recalculateRegression. It avoids
  // the conditioning loss of normal equations for the OOXML-supported
  // degree range (2..=6).
  let minor_count = row_count.min(power_count);
  let mut diagonal = vec![0.0; minor_count];
  for minor in 0..minor_count {
    let norm_squared = (minor..row_count)
      .map(|row| {
        let value = qr_transposed[row + minor * row_count];
        value * value
      })
      .sum::<f64>();
    let leading = qr_transposed[minor + minor * row_count];
    let reflector = if leading > 0.0 {
      -norm_squared.sqrt()
    } else {
      norm_squared.sqrt()
    };
    if reflector.abs() <= f64::EPSILON {
      return None;
    }
    diagonal[minor] = reflector;
    qr_transposed[minor + minor * row_count] -= reflector;
    for column in minor + 1..power_count {
      let mut alpha = 0.0;
      for row in minor..row_count {
        alpha -= qr_transposed[row + column * row_count] * qr_transposed[row + minor * row_count];
      }
      let denominator = reflector * qr_transposed[minor + minor * row_count];
      if denominator.abs() <= f64::EPSILON {
        return None;
      }
      alpha /= denominator;
      for row in minor..row_count {
        qr_transposed[row + column * row_count] -= alpha * qr_transposed[row + minor * row_count];
      }
    }
  }

  for minor in 0..minor_count {
    let dot_product = (minor..row_count)
      .map(|row| y[row] * qr_transposed[row + minor * row_count])
      .sum::<f64>();
    let denominator = diagonal[minor] * qr_transposed[minor + minor * row_count];
    if denominator.abs() <= f64::EPSILON {
      return None;
    }
    let factor = dot_product / denominator;
    for row in minor..row_count {
      y[row] += factor * qr_transposed[row + minor * row_count];
    }
  }

  let mut coefficients = vec![0.0; power_count];
  for row in (0..minor_count).rev() {
    if diagonal[row].abs() <= f64::EPSILON {
      return None;
    }
    y[row] /= diagonal[row];
    coefficients[row] = y[row];
    for preceding in 0..row {
      y[preceding] -= y[row] * qr_transposed[preceding + row * row_count];
    }
  }
  if let Some(intercept) = forced_intercept {
    coefficients.insert(0, intercept);
  }
  Some(coefficients)
}

fn polynomial_value(coefficients: &[f64], x: f64) -> f64 {
  coefficients
    .iter()
    .rev()
    .fold(0.0, |value, coefficient| coefficient + x * value)
}

fn lower_chart_group_decorations(
  items: &mut Vec<PageItem>,
  chart: &ClusteredColumnChart<'_>,
  plot: PlotRect,
  axis_scales: &[CartesianAxisScales],
  projection_3d: Option<Chart3DProjection>,
  style: &ClusteredColumnStyle,
  category_count: usize,
) {
  for (group_index, group) in chart.group_decorations.iter().enumerate() {
    let Some(group_series) = chart
      .series
      .get(group.first_series_index..group.first_series_index.saturating_add(group.series_count))
    else {
      continue;
    };
    let axes = axis_scales
      .get(group.axis_set_index)
      .unwrap_or(&axis_scales[0]);
    let group_style = style.group_decoration_styles.get(group_index);

    if group.drop_lines.is_some() {
      let axis_value = 0.0_f64.clamp(axes.y.minimum, axes.y.maximum);
      let axis_y = value_y(axis_value, axes.y, plot.top, plot.height);
      for (relative_series_index, series) in group_series.iter().enumerate() {
        let series_index = group.first_series_index + relative_series_index;
        for (category_index, value) in series.values.iter().enumerate() {
          let Some(value) = value.as_ref().copied().filter(|value| value.is_finite()) else {
            continue;
          };
          let value = chart_group_series_end_value(
            chart,
            group.first_series_index,
            group.series_count,
            series_index,
            category_index,
            value,
          );
          let display_index = category_display_index(chart, category_index, category_count);
          let x = category_point_x(chart, display_index, category_count, plot);
          let data_y = value_y(value, axes.y, plot.top, plot.height);
          let (start, end) = if series.is_3d {
            let (front, back) = chart_3d_series_depth_slot(chart, series_index);
            let depth = (front + back) * 0.5;
            projection_3d.map_or(((x, data_y), (x, axis_y)), |projection| {
              (
                projection.project(x, data_y, depth),
                projection.project(x, axis_y, depth),
              )
            })
          } else {
            ((x, data_y), (x, axis_y))
          };
          push_chart_styled_line(
            items,
            start,
            end,
            group_style.map(|style| &style.drop_lines.stroke),
            style.gridline_color,
            0.75 * style.stroke_scale,
            style.stroke_scale,
          );
        }
      }
    }

    for category_index in 0..category_count {
      let display_index = category_display_index(chart, category_index, category_count);
      let x = category_point_x(chart, display_index, category_count, plot);
      if group.high_low_lines.is_some() {
        let values = group_series
          .iter()
          .enumerate()
          .filter_map(|(relative_series_index, series)| {
            let value = series.values.get(category_index).copied().flatten()?;
            value.is_finite().then(|| {
              chart_group_series_end_value(
                chart,
                group.first_series_index,
                group.series_count,
                group.first_series_index + relative_series_index,
                category_index,
                value,
              )
            })
          })
          .collect::<Vec<_>>();
        if values.len() >= 2 {
          let minimum = values.iter().copied().fold(f64::INFINITY, f64::min);
          let maximum = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
          push_chart_styled_line(
            items,
            (x, value_y(minimum, axes.y, plot.top, plot.height)),
            (x, value_y(maximum, axes.y, plot.top, plot.height)),
            group_style.map(|style| &style.high_low_lines.stroke),
            style.gridline_color,
            0.75 * style.stroke_scale,
            style.stroke_scale,
          );
        }
      }

      let Some(up_down_bars) = group.up_down_bars else {
        continue;
      };
      let Some((first_series, last_series)) = group_series.first().zip(group_series.last()) else {
        continue;
      };
      if group_series.len() < 2 {
        continue;
      }
      let Some(first) = first_series
        .values
        .get(category_index)
        .copied()
        .flatten()
        .filter(|value| value.is_finite())
      else {
        continue;
      };
      let Some(last) = last_series
        .values
        .get(category_index)
        .copied()
        .flatten()
        .filter(|value| value.is_finite())
      else {
        continue;
      };
      let first = chart_group_series_end_value(
        chart,
        group.first_series_index,
        group.series_count,
        group.first_series_index,
        category_index,
        first,
      );
      let last_series_index = group.first_series_index + group.series_count - 1;
      let last = chart_group_series_end_value(
        chart,
        group.first_series_index,
        group.series_count,
        last_series_index,
        category_index,
        last,
      );
      let top = value_y(first.max(last), axes.y, plot.top, plot.height);
      let bottom = value_y(first.min(last), axes.y, plot.top, plot.height);
      if (bottom - top).abs() <= f32::EPSILON {
        continue;
      }
      let category_step = if chart.category_axis_shifted {
        plot.width / category_count.max(1) as f32
      } else if category_count > 1 {
        plot.width / (category_count - 1) as f32
      } else {
        plot.width
      };
      let gap_width = up_down_bars
        .gap_width
        .as_ref()
        .and_then(|gap| gap.val)
        .map_or(150.0, f32::from)
        .clamp(0.0, 500.0);
      let bar_width = category_step / (1.0 + gap_width / 100.0);
      let is_up = last > first;
      let bar_style = if is_up {
        group_style.map(|style| &style.up_bars)
      } else {
        group_style.map(|style| &style.down_bars)
      };
      let fallback_fill = if is_up {
        RgbColor {
          r: 255,
          g: 255,
          b: 255,
        }
      } else {
        RgbColor { r: 0, g: 0, b: 0 }
      };
      let left = x - bar_width * 0.5;
      let right = x + bar_width * 0.5;
      let default_style = crate::common::ShapeStyle::default();
      push_chart_styled_polygon(
        items,
        &[(left, top), (right, top), (right, bottom), (left, bottom)],
        bar_style.unwrap_or(&default_style),
        Some(fallback_fill),
        Some((RgbColor { r: 0, g: 0, b: 0 }, 0.75 * style.stroke_scale)),
        style.stroke_scale,
      );
    }
  }
}

fn chart_group_series_end_value(
  chart: &ClusteredColumnChart<'_>,
  first_series_index: usize,
  series_count: usize,
  series_index: usize,
  category_index: usize,
  value: f64,
) -> f64 {
  let Some(series) = chart.series.get(series_index) else {
    return value;
  };
  if !matches!(
    series.grouping,
    ChartSeriesGrouping::Stacked | ChartSeriesGrouping::PercentStacked
  ) {
    return value;
  }
  let Some(group_series) = chart
    .series
    .get(first_series_index..first_series_index.saturating_add(series_count))
  else {
    return value;
  };
  let total = if series.grouping == ChartSeriesGrouping::PercentStacked {
    group_series
      .iter()
      .filter_map(|peer| peer.values.get(category_index).copied().flatten())
      .filter(|value| value.is_finite())
      .map(f64::abs)
      .sum::<f64>()
      .max(f64::EPSILON)
  } else {
    1.0
  };
  let start = chart.series[first_series_index..series_index]
    .iter()
    .filter_map(|peer| peer.values.get(category_index).copied().flatten())
    .filter(|previous| previous.is_finite() && previous.signum() == value.signum())
    .map(|previous| previous / total)
    .sum::<f64>();
  start + value / total
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
    .filter(|peer| {
      peer.axis_set_index == series.axis_set_index
        && peer.kind == ChartSeriesKind::Column
        && peer.grouping == series.grouping
    })
    .count()
    .max(1);
  let peer_index = chart.series[..series_index]
    .iter()
    .filter(|peer| {
      peer.axis_set_index == series.axis_set_index
        && peer.kind == ChartSeriesKind::Column
        && peer.grouping == series.grouping
    })
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
      series_category_display_index(
        chart.category_axis_reversed,
        series.kind,
        category_index,
        context.category_count,
      ),
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
    if series.is_3d
      && let Some(projection) = context.projection_3d
    {
      lower_3d_column_marker(
        items,
        Marker3DContext {
          geometry: context,
          projection,
          series_index,
          category_index,
          color: point_color,
        },
        VerticalMarkerBounds {
          x,
          width,
          start_y,
          end_y,
        },
        (start_value, end_value),
      );
      continue;
    }
    let left = word_fixed_chart_data_edge(x, style.layout_profile);
    let right = word_fixed_chart_data_edge(x + width, style.layout_profile);
    let start_y = word_fixed_chart_value_edge(start_y, start_value, style.layout_profile);
    let end_y = word_fixed_chart_value_edge(end_y, end_value, style.layout_profile);
    push_chart_data_rect(
      items,
      left,
      end_y.min(start_y),
      (right - left).abs(),
      (start_y - end_y).abs(),
      point_color,
      chart_series_fill_style(style, series_index, Some(category_index)),
      chart_series_stroke_style(style, series_index, Some(category_index)),
      style.stroke_scale,
    );
  }
}

fn lower_3d_column_marker(
  items: &mut Vec<PageItem>,
  marker: Marker3DContext<'_, '_, '_>,
  bounds: VerticalMarkerBounds,
  values: (f64, f64),
) {
  let series = &marker.geometry.chart.series[marker.series_index];
  let (front, back) = chart_3d_series_depth_slot(marker.geometry.chart, marker.series_index);
  let depth = MarkerDepth { front, back };
  match series.shape_3d {
    c::ShapeValues::Box => lower_3d_box(
      items,
      marker.projection,
      (bounds.x, bounds.start_y),
      (bounds.x + bounds.width, bounds.end_y),
      depth.front,
      depth.back,
      marker.color,
    ),
    c::ShapeValues::Cylinder => {
      lower_3d_vertical_cylinder(items, marker.projection, bounds, depth, marker.color)
    }
    c::ShapeValues::Cone
    | c::ShapeValues::ConeToMax
    | c::ShapeValues::Pyramid
    | c::ShapeValues::PyramidToMaximum => {
      let (start_ratio, end_ratio) = marker_taper_ratios(
        marker.geometry,
        marker.series_index,
        marker.category_index,
        values.0,
        values.1,
      );
      lower_3d_vertical_tapered_marker(
        items,
        marker.projection,
        bounds,
        (start_ratio, end_ratio),
        depth,
        marker.color,
        matches!(
          series.shape_3d,
          c::ShapeValues::Cone | c::ShapeValues::ConeToMax
        ),
      );
    }
  }
}

fn lower_3d_bar_marker(
  items: &mut Vec<PageItem>,
  marker: Marker3DContext<'_, '_, '_>,
  bounds: HorizontalMarkerBounds,
  values: (f64, f64),
) {
  let series = &marker.geometry.chart.series[marker.series_index];
  let (front, back) = chart_3d_series_depth_slot(marker.geometry.chart, marker.series_index);
  let depth = MarkerDepth { front, back };
  match series.shape_3d {
    c::ShapeValues::Box => lower_3d_box(
      items,
      marker.projection,
      (bounds.start_x, bounds.y),
      (bounds.end_x, bounds.y + bounds.height),
      depth.front,
      depth.back,
      marker.color,
    ),
    c::ShapeValues::Cylinder => {
      lower_3d_horizontal_cylinder(items, marker.projection, bounds, depth, marker.color)
    }
    c::ShapeValues::Cone
    | c::ShapeValues::ConeToMax
    | c::ShapeValues::Pyramid
    | c::ShapeValues::PyramidToMaximum => {
      let (start_ratio, end_ratio) = marker_taper_ratios(
        marker.geometry,
        marker.series_index,
        marker.category_index,
        values.0,
        values.1,
      );
      lower_3d_horizontal_tapered_marker(
        items,
        marker.projection,
        bounds,
        (start_ratio, end_ratio),
        depth,
        marker.color,
        matches!(
          series.shape_3d,
          c::ShapeValues::Cone | c::ShapeValues::ConeToMax
        ),
      );
    }
  }
}

fn marker_taper_ratios(
  context: &SeriesGeometryContext<'_, '_>,
  series_index: usize,
  category_index: usize,
  start_value: f64,
  end_value: f64,
) -> (f32, f32) {
  let series = &context.chart.series[series_index];
  let to_axis_maximum = matches!(
    series.shape_3d,
    c::ShapeValues::ConeToMax | c::ShapeValues::PyramidToMaximum
  );
  let apex_value = if to_axis_maximum {
    if end_value >= start_value {
      context.scale.maximum
    } else {
      context.scale.minimum
    }
  } else if matches!(
    series.grouping,
    ChartSeriesGrouping::Stacked | ChartSeriesGrouping::PercentStacked
  ) {
    let same_sign_positive = end_value >= 0.0;
    context
      .chart
      .series
      .iter()
      .filter(|peer| {
        peer.axis_set_index == series.axis_set_index
          && peer.kind == series.kind
          && peer.grouping == series.grouping
      })
      .filter_map(|peer| peer.values.get(category_index).copied().flatten())
      .filter(|value| (*value >= 0.0) == same_sign_positive)
      .sum()
  } else {
    end_value
  };
  let base_value = 0.0_f64.clamp(context.scale.minimum, context.scale.maximum);
  let range = apex_value - base_value;
  if range.abs() <= f64::EPSILON {
    return (1.0, 0.0);
  }
  let ratio = |value: f64| ((apex_value - value) / range).abs().clamp(0.0, 1.0) as f32;
  (ratio(start_value), ratio(end_value))
}

fn lower_3d_box(
  items: &mut Vec<PageItem>,
  projection: Chart3DProjection,
  corner1: (f32, f32),
  corner2: (f32, f32),
  front_depth: f32,
  back_depth: f32,
  color: RgbColor,
) {
  let left = corner1.0.min(corner2.0);
  let right = corner1.0.max(corner2.0);
  let top = corner1.1.min(corner2.1);
  let bottom = corner1.1.max(corner2.1);
  let front = [
    projection.project(left, top, front_depth),
    projection.project(right, top, front_depth),
    projection.project(right, bottom, front_depth),
    projection.project(left, bottom, front_depth),
  ];
  let back = [
    projection.project(left, top, back_depth),
    projection.project(right, top, back_depth),
    projection.project(right, bottom, back_depth),
    projection.project(left, bottom, back_depth),
  ];
  let stroke = Some((shade_chart_color(color, 0.68), 0.4));
  push_chart_polygon(items, &back, shade_chart_color(color, 0.72), stroke);
  let (depth_x, depth_y) = projection.depth_vector();
  let side_indices = if depth_x >= 0.0 { (1, 2) } else { (0, 3) };
  push_chart_polygon(
    items,
    &[
      front[side_indices.0],
      back[side_indices.0],
      back[side_indices.1],
      front[side_indices.1],
    ],
    shade_chart_color(color, profiles::OFFICE_CARTESIAN_3D_BOX_SIDE_SHADE),
    stroke,
  );
  let cap_indices = if depth_y <= 0.0 { (0, 1) } else { (3, 2) };
  push_chart_polygon(
    items,
    &[
      front[cap_indices.0],
      back[cap_indices.0],
      back[cap_indices.1],
      front[cap_indices.1],
    ],
    shade_chart_color(color, profiles::OFFICE_CARTESIAN_3D_BOX_TOP_SHADE),
    stroke,
  );
  push_chart_polygon(items, &front, color, stroke);
}

fn lower_3d_vertical_cylinder(
  items: &mut Vec<PageItem>,
  projection: Chart3DProjection,
  bounds: VerticalMarkerBounds,
  depth: MarkerDepth,
  color: RgbColor,
) {
  let center_depth = (depth.front + depth.back) * 0.5;
  let (left_start_x, start_center_y) = projection.project(bounds.x, bounds.start_y, center_depth);
  let (right_start_x, _) =
    projection.project(bounds.x + bounds.width, bounds.start_y, center_depth);
  let (left_end_x, end_center_y) = projection.project(bounds.x, bounds.end_y, center_depth);
  let (right_end_x, _) = projection.project(bounds.x + bounds.width, bounds.end_y, center_depth);
  let center_x = (left_start_x + right_start_x + left_end_x + right_end_x) * 0.25;
  let radius_x = ((right_start_x - left_start_x).abs() + (right_end_x - left_end_x).abs()) * 0.25;
  let front_center = projection.project(bounds.x + bounds.width * 0.5, bounds.start_y, depth.front);
  let back_center = projection.project(bounds.x + bounds.width * 0.5, bounds.start_y, depth.back);
  let radius_y = ((back_center.1 - front_center.1).abs() * 0.5)
    .max(bounds.width * 0.075)
    .min(bounds.width * 0.26);
  let top_y = start_center_y.min(end_center_y);
  let bottom_y = start_center_y.max(end_center_y);
  push_chart_ellipse(
    items,
    center_x,
    bottom_y,
    radius_x,
    radius_y,
    shade_chart_color(color, 0.75),
    None,
  );
  lower_rounded_marker_band(
    items,
    center_x - radius_x,
    top_y,
    radius_x * 2.0,
    bottom_y - top_y,
    true,
    color,
  );
  push_chart_ellipse(
    items,
    center_x,
    top_y,
    radius_x,
    radius_y,
    tint_chart_color(color, 0.24),
    Some((shade_chart_color(color, 0.66), 0.35)),
  );
}

fn lower_3d_horizontal_cylinder(
  items: &mut Vec<PageItem>,
  projection: Chart3DProjection,
  bounds: HorizontalMarkerBounds,
  depth: MarkerDepth,
  color: RgbColor,
) {
  let center_depth = (depth.front + depth.back) * 0.5;
  let start = projection.project(bounds.start_x, bounds.y + bounds.height * 0.5, center_depth);
  let end = projection.project(bounds.end_x, bounds.y + bounds.height * 0.5, center_depth);
  let (_, top_y) = projection.project(bounds.start_x, bounds.y, center_depth);
  let (_, bottom_y) = projection.project(bounds.start_x, bounds.y + bounds.height, center_depth);
  let radius_y = (bottom_y - top_y).abs() * 0.5;
  let front_center =
    projection.project(bounds.start_x, bounds.y + bounds.height * 0.5, depth.front);
  let back_center = projection.project(bounds.start_x, bounds.y + bounds.height * 0.5, depth.back);
  let radius_x = ((back_center.0 - front_center.0).abs() * 0.5)
    .max(bounds.height * 0.075)
    .min(bounds.height * 0.26);
  let left = start.0.min(end.0);
  let right = start.0.max(end.0);
  push_chart_ellipse(
    items,
    left,
    start.1,
    radius_x,
    radius_y,
    shade_chart_color(color, 0.75),
    None,
  );
  lower_rounded_marker_band(
    items,
    left,
    start.1 - radius_y,
    right - left,
    radius_y * 2.0,
    false,
    color,
  );
  push_chart_ellipse(
    items,
    right,
    end.1,
    radius_x,
    radius_y,
    tint_chart_color(color, 0.18),
    Some((shade_chart_color(color, 0.66), 0.35)),
  );
}

fn lower_3d_vertical_tapered_marker(
  items: &mut Vec<PageItem>,
  projection: Chart3DProjection,
  bounds: VerticalMarkerBounds,
  taper: (f32, f32),
  depth: MarkerDepth,
  color: RgbColor,
  rounded: bool,
) {
  let center_x = bounds.x + bounds.width * 0.5;
  let center_depth = (depth.front + depth.back) * 0.5;
  let start_half_width = bounds.width * taper.0 * 0.5;
  let end_half_width = bounds.width * taper.1 * 0.5;
  let tapered_bounds = VerticalTaperedBounds {
    center_x,
    start_y: bounds.start_y,
    end_y: bounds.end_y,
    start_half_width,
    end_half_width,
  };
  if rounded {
    lower_vertical_tapered_bands(items, projection, tapered_bounds, center_depth, color);
    return;
  }
  let front = [
    projection.project(center_x - start_half_width, bounds.start_y, depth.front),
    projection.project(center_x + start_half_width, bounds.start_y, depth.front),
    projection.project(center_x + end_half_width, bounds.end_y, depth.front),
    projection.project(center_x - end_half_width, bounds.end_y, depth.front),
  ];
  let back = [
    projection.project(center_x - start_half_width, bounds.start_y, depth.back),
    projection.project(center_x + start_half_width, bounds.start_y, depth.back),
    projection.project(center_x + end_half_width, bounds.end_y, depth.back),
    projection.project(center_x - end_half_width, bounds.end_y, depth.back),
  ];
  let stroke = Some((shade_chart_color(color, 0.65), 0.4));
  push_chart_polygon(items, &back, shade_chart_color(color, 0.72), stroke);
  push_chart_polygon(
    items,
    &[front[1], back[1], back[2], front[2]],
    shade_chart_color(color, 0.76),
    stroke,
  );
  push_chart_polygon(items, &front, color, stroke);
}

fn lower_3d_horizontal_tapered_marker(
  items: &mut Vec<PageItem>,
  projection: Chart3DProjection,
  bounds: HorizontalMarkerBounds,
  taper: (f32, f32),
  depth: MarkerDepth,
  color: RgbColor,
  rounded: bool,
) {
  let center_y = bounds.y + bounds.height * 0.5;
  let center_depth = (depth.front + depth.back) * 0.5;
  let start_half_height = bounds.height * taper.0 * 0.5;
  let end_half_height = bounds.height * taper.1 * 0.5;
  let tapered_bounds = HorizontalTaperedBounds {
    start_x: bounds.start_x,
    end_x: bounds.end_x,
    center_y,
    start_half_height,
    end_half_height,
  };
  if rounded {
    lower_horizontal_tapered_bands(items, projection, tapered_bounds, center_depth, color);
    return;
  }
  let front = [
    projection.project(bounds.start_x, center_y - start_half_height, depth.front),
    projection.project(bounds.end_x, center_y - end_half_height, depth.front),
    projection.project(bounds.end_x, center_y + end_half_height, depth.front),
    projection.project(bounds.start_x, center_y + start_half_height, depth.front),
  ];
  let back = [
    projection.project(bounds.start_x, center_y - start_half_height, depth.back),
    projection.project(bounds.end_x, center_y - end_half_height, depth.back),
    projection.project(bounds.end_x, center_y + end_half_height, depth.back),
    projection.project(bounds.start_x, center_y + start_half_height, depth.back),
  ];
  let stroke = Some((shade_chart_color(color, 0.65), 0.4));
  push_chart_polygon(items, &back, shade_chart_color(color, 0.72), stroke);
  push_chart_polygon(
    items,
    &[front[0], back[0], back[1], front[1]],
    tint_chart_color(color, 0.12),
    stroke,
  );
  push_chart_polygon(items, &front, color, stroke);
}

fn lower_vertical_tapered_bands(
  items: &mut Vec<PageItem>,
  projection: Chart3DProjection,
  bounds: VerticalTaperedBounds,
  depth: f32,
  color: RgbColor,
) {
  const BAND_EDGES: [f32; 6] = [-1.0, -0.6, -0.2, 0.2, 0.6, 1.0];
  const BAND_SHADE: [f32; 5] = [0.72, 0.88, 1.08, 0.96, 0.78];
  for band in 0..5 {
    let left = BAND_EDGES[band];
    let right = BAND_EDGES[band + 1];
    let points = [
      projection.project(
        bounds.center_x + bounds.start_half_width * left,
        bounds.start_y,
        depth,
      ),
      projection.project(
        bounds.center_x + bounds.start_half_width * right,
        bounds.start_y,
        depth,
      ),
      projection.project(
        bounds.center_x + bounds.end_half_width * right,
        bounds.end_y,
        depth,
      ),
      projection.project(
        bounds.center_x + bounds.end_half_width * left,
        bounds.end_y,
        depth,
      ),
    ];
    let band_color = if BAND_SHADE[band] <= 1.0 {
      shade_chart_color(color, BAND_SHADE[band])
    } else {
      tint_chart_color(color, BAND_SHADE[band] - 1.0)
    };
    push_chart_polygon(items, &points, band_color, None);
  }
}

fn lower_horizontal_tapered_bands(
  items: &mut Vec<PageItem>,
  projection: Chart3DProjection,
  bounds: HorizontalTaperedBounds,
  depth: f32,
  color: RgbColor,
) {
  const BAND_EDGES: [f32; 6] = [-1.0, -0.6, -0.2, 0.2, 0.6, 1.0];
  const BAND_SHADE: [f32; 5] = [0.72, 0.88, 1.08, 0.96, 0.78];
  for band in 0..5 {
    let top = BAND_EDGES[band];
    let bottom = BAND_EDGES[band + 1];
    let points = [
      projection.project(
        bounds.start_x,
        bounds.center_y + bounds.start_half_height * top,
        depth,
      ),
      projection.project(
        bounds.end_x,
        bounds.center_y + bounds.end_half_height * top,
        depth,
      ),
      projection.project(
        bounds.end_x,
        bounds.center_y + bounds.end_half_height * bottom,
        depth,
      ),
      projection.project(
        bounds.start_x,
        bounds.center_y + bounds.start_half_height * bottom,
        depth,
      ),
    ];
    let band_color = if BAND_SHADE[band] <= 1.0 {
      shade_chart_color(color, BAND_SHADE[band])
    } else {
      tint_chart_color(color, BAND_SHADE[band] - 1.0)
    };
    push_chart_polygon(items, &points, band_color, None);
  }
}

fn lower_rounded_marker_band(
  items: &mut Vec<PageItem>,
  x: f32,
  y: f32,
  width: f32,
  height: f32,
  vertical_gradient: bool,
  color: RgbColor,
) {
  const SHADES: [f32; 5] = [0.72, 0.88, 1.08, 0.96, 0.78];
  for (index, shade) in SHADES.into_iter().enumerate() {
    let ratio_start = index as f32 / SHADES.len() as f32;
    let ratio_end = (index + 1) as f32 / SHADES.len() as f32;
    let band_color = if shade <= 1.0 {
      shade_chart_color(color, shade)
    } else {
      tint_chart_color(color, shade - 1.0)
    };
    let (band_x, band_y, band_width, band_height) = if vertical_gradient {
      (
        x + width * ratio_start,
        y,
        width * (ratio_end - ratio_start),
        height,
      )
    } else {
      (
        x,
        y + height * ratio_start,
        width,
        height * (ratio_end - ratio_start),
      )
    };
    items.push(PageItem::Rect(RectItem {
      x_pt: band_x,
      y_pt: band_y,
      width_pt: band_width,
      height_pt: band_height,
      fill_color: Some(band_color),
      fill_opacity: 1.0,
      stroke: None,
      stroke_opacity: 1.0,
    }));
  }
}

fn push_chart_ellipse(
  items: &mut Vec<PageItem>,
  center_x: f32,
  center_y: f32,
  radius_x: f32,
  radius_y: f32,
  color: RgbColor,
  stroke: Option<(RgbColor, f32)>,
) {
  const SEGMENTS: usize = 32;
  let points = (0..SEGMENTS)
    .map(|index| {
      let angle = std::f32::consts::TAU * index as f32 / SEGMENTS as f32;
      (
        center_x + radius_x * angle.cos(),
        center_y + radius_y * angle.sin(),
      )
    })
    .collect::<Vec<_>>();
  push_chart_polygon(items, &points, color, stroke);
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
    .filter(|peer| {
      peer.axis_set_index == series.axis_set_index
        && peer.kind == ChartSeriesKind::Bar
        && peer.grouping == series.grouping
    })
    .count()
    .max(1);
  let peer_index = chart.series[..series_index]
    .iter()
    .filter(|peer| {
      peer.axis_set_index == series.axis_set_index
        && peer.kind == ChartSeriesKind::Bar
        && peer.grouping == series.grouping
    })
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
      series_category_display_index(
        chart.category_axis_reversed,
        series.kind,
        category_index,
        context.category_count,
      ),
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
    let y = context.plot.top + (slot.center - slot.width / 2.0) as f32 * context.plot.height;
    let height = slot.width as f32 * context.plot.height;
    if series.is_3d
      && let Some(projection) = context.projection_3d
    {
      lower_3d_bar_marker(
        items,
        Marker3DContext {
          geometry: context,
          projection,
          series_index,
          category_index,
          color: point_color,
        },
        HorizontalMarkerBounds {
          start_x,
          end_x,
          y,
          height,
        },
        (start_value, end_value),
      );
      continue;
    }
    let start_x = word_fixed_chart_value_edge(start_x, start_value, style.layout_profile);
    let end_x = word_fixed_chart_value_edge(end_x, end_value, style.layout_profile);
    let top = word_fixed_chart_data_edge(y, style.layout_profile);
    let bottom = word_fixed_chart_data_edge(y + height, style.layout_profile);
    push_chart_data_rect(
      items,
      start_x.min(end_x),
      top,
      (end_x - start_x).abs(),
      (bottom - top).abs(),
      point_color,
      chart_series_fill_style(style, series_index, Some(category_index)),
      chart_series_stroke_style(style, series_index, Some(category_index)),
      style.stroke_scale,
    );
  }
}

fn chart_series_fill_style(
  style: &ClusteredColumnStyle,
  series_index: usize,
  point_index: Option<usize>,
) -> Option<&crate::common::ShapeStyleValue<crate::common::Fill<'static>>> {
  let point = point_index.and_then(|point_index| {
    style
      .series_point_styles
      .get(series_index)
      .and_then(|points| points.get(point_index))
      .and_then(Option::as_ref)
      .map(|point| &point.fill)
  });
  if point.is_some_and(|fill| !matches!(fill, crate::common::ShapeStyleValue::Unspecified)) {
    return point;
  }
  style
    .series_styles
    .get(series_index)
    .map(|series| &series.fill)
    .filter(|fill| !matches!(fill, crate::common::ShapeStyleValue::Unspecified))
}

fn chart_series_stroke_style(
  style: &ClusteredColumnStyle,
  series_index: usize,
  point_index: Option<usize>,
) -> Option<&crate::common::ShapeStyleValue<crate::common::Stroke<'static>>> {
  let point = point_index.and_then(|point_index| {
    style
      .series_point_styles
      .get(series_index)
      .and_then(|points| points.get(point_index))
      .and_then(Option::as_ref)
      .map(|point| &point.stroke)
  });
  if point.is_some_and(|stroke| !matches!(stroke, crate::common::ShapeStyleValue::Unspecified)) {
    return point;
  }
  style
    .series_styles
    .get(series_index)
    .map(|series| &series.stroke)
    .filter(|stroke| !matches!(stroke, crate::common::ShapeStyleValue::Unspecified))
}

fn chart_series_area_fill(
  style: &ClusteredColumnStyle,
  series_index: usize,
  bounds: crate::common::Rect,
  fallback_color: RgbColor,
  fallback_opacity: f32,
) -> crate::common::Fill<'static> {
  match chart_series_fill_style(style, series_index, None) {
    Some(crate::common::ShapeStyleValue::Paint(fill)) => bind_chart_fill_to_bounds(fill, bounds),
    Some(crate::common::ShapeStyleValue::NoPaint) => crate::common::Fill::None,
    Some(crate::common::ShapeStyleValue::Unspecified) | None => {
      crate::common::Fill::Solid(common_rgb(fallback_color, fallback_opacity))
    }
  }
}

fn push_chart_data_rect(
  items: &mut Vec<PageItem>,
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  fallback_color: RgbColor,
  fill: Option<&crate::common::ShapeStyleValue<crate::common::Fill<'static>>>,
  stroke: Option<&crate::common::ShapeStyleValue<crate::common::Stroke<'static>>>,
  stroke_width_scale: f32,
) {
  push_chart_shape_rect(
    items,
    x_pt,
    y_pt,
    width_pt,
    height_pt,
    fill,
    stroke,
    Some(fallback_color),
    stroke_width_scale,
  );
}

fn word_fixed_chart_data_edge(value_pt: f32, layout_profile: ChartLayoutProfile) -> f32 {
  if layout_profile == ChartLayoutProfile::Word {
    (value_pt / WORD_FIXED_CHART_DATA_EDGE_GRID_PT).round() * WORD_FIXED_CHART_DATA_EDGE_GRID_PT
  } else {
    value_pt
  }
}

fn automatic_axis_line_width_pt(style: &ClusteredColumnStyle) -> f32 {
  // The legacy Word chart style emits a 9,525 EMU (0.75pt) automatic axis
  // outline. It is independent of the 0.14pt chart-space frame hairline.
  // Excel's current automatic profile retains the wider 1pt axis already
  // calibrated by the worksheet corpus.
  (match style.layout_profile {
    ChartLayoutProfile::Word | ChartLayoutProfile::PowerPoint => 0.75,
    ChartLayoutProfile::Excel => 1.0,
  }) * style.stroke_scale
}

fn word_fixed_chart_value_edge(
  value_pt: f32,
  data_value: f64,
  layout_profile: ChartLayoutProfile,
) -> f32 {
  if data_value.abs() <= f64::EPSILON {
    // The zero edge is the value-axis stroke itself. Office leaves that
    // automatic-layout coordinate unsnapped while quantizing nonzero marker
    // values on either side.
    value_pt
  } else {
    word_fixed_chart_data_edge(value_pt, layout_profile)
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
  if series.is_3d
    && let Some(projection) = context.projection_3d
  {
    lower_3d_line_or_area_series(
      items,
      context,
      series_index,
      color,
      fill_to_axis,
      style,
      projection,
    );
    return;
  }
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
        lower_cardinal_cubic_chart_line(
          items,
          run,
          chart_series_stroke_style(style, series_index, None),
          color,
          series
            .line_width_pt
            .unwrap_or(style.automatic_line_width_pt),
          style.stroke_scale,
        );
      }
    }
    if let Some(marker) = chart_marker_size(series) {
      for (x, y, index) in marker_points {
        lower_chart_marker(
          items,
          x,
          y,
          marker * style.stroke_scale,
          chart_point_color(style, series_index, index).unwrap_or(color),
          series,
          chart_marker_stroke_width(series, style.stroke_scale),
        );
      }
    }
    return;
  }
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
      let bounds = common_rect(plot.left, plot.top, plot.width, plot.height);
      let fill = chart_series_area_fill(style, series_index, bounds, color, 0.52);
      if fill != crate::common::Fill::None {
        items.push(PageItem::Path(crate::common::PathItem {
          bounds,
          points: upper,
          commands: Vec::new(),
          closed: true,
          fill,
          stroke: None,
        }));
      }
    }
  }
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
    );
    run.push(point);
    marker_points.push((point.0, point.1, index));
  }
  if run.len() >= 2 {
    runs.push(run);
  }
  if !series.line_hidden {
    for run in &runs {
      for clipped in clip_chart_polyline_to_plot(run, plot) {
        push_chart_styled_polyline(
          items,
          &clipped,
          chart_series_stroke_style(style, series_index, None),
          color,
          series
            .line_width_pt
            .unwrap_or(style.automatic_line_width_pt)
            * style.stroke_scale,
          style.stroke_scale,
        );
      }
    }
  }
  if let Some(marker) = chart_marker_size(series) {
    for (x, y, index) in marker_points {
      lower_chart_marker(
        items,
        x,
        y,
        marker * style.stroke_scale,
        chart_point_color(style, series_index, index).unwrap_or(color),
        series,
        chart_marker_stroke_width(series, style.stroke_scale),
      );
    }
  }
}

fn lower_3d_line_or_area_series(
  items: &mut Vec<PageItem>,
  context: &SeriesGeometryContext<'_, '_>,
  series_index: usize,
  color: RgbColor,
  fill_to_axis: bool,
  style: &ClusteredColumnStyle,
  projection: Chart3DProjection,
) {
  let chart = context.chart;
  let series = &chart.series[series_index];
  let (front_depth, back_depth) = chart_3d_series_depth_slot(chart, series_index);
  let stroke = (!series.line_hidden).then_some((
    shade_chart_color(color, 0.68),
    (series
      .line_width_pt
      .unwrap_or(style.automatic_line_width_pt)
      * style.stroke_scale
      * 0.3)
      .clamp(0.25, 1.5),
  ));

  if fill_to_axis {
    let mut upper = Vec::new();
    let mut lower = Vec::new();
    let mut runs = Vec::new();
    for (index, value) in series.values.iter().enumerate() {
      let Some(value) = value.as_ref().copied().filter(|value| value.is_finite()) else {
        if upper.len() >= 2 {
          runs.push((std::mem::take(&mut upper), std::mem::take(&mut lower)));
        } else {
          upper.clear();
          lower.clear();
        }
        continue;
      };
      let (stack_start, stack_end) = stacked_value_bounds(chart, series_index, index, value);
      let stacked = matches!(
        series.grouping,
        ChartSeriesGrouping::Stacked | ChartSeriesGrouping::PercentStacked
      );
      let display_index = category_display_index(chart, index, context.category_count);
      let x = category_point_x(chart, display_index, context.category_count, context.plot);
      upper.push((
        x,
        value_y(
          if stacked { stack_end } else { value },
          context.scale,
          context.plot.top,
          context.plot.height,
        ),
      ));
      lower.push((
        x,
        value_y(
          if stacked {
            stack_start
          } else {
            // LibreOffice AreaChart::impl_createArea grounds 3-D areas at
            // the logical minimum, not at the 2-D base value.
            context.scale.minimum
          },
          context.scale,
          context.plot.top,
          context.plot.height,
        ),
      ));
    }
    if upper.len() >= 2 {
      runs.push((upper, lower));
    }
    for (mut upper, lower) in runs {
      upper.extend(lower.into_iter().rev());
      lower_3d_extruded_polygon(
        items,
        &upper,
        projection,
        front_depth,
        back_depth,
        color,
        stroke,
      );
    }
    return;
  }

  if series.line_hidden {
    return;
  }
  let mut run = Vec::new();
  let mut runs = Vec::new();
  for (index, value) in series.values.iter().enumerate() {
    let Some(value) = value.as_ref().copied().filter(|value| value.is_finite()) else {
      if run.len() >= 2 {
        runs.push(std::mem::take(&mut run));
      } else {
        run.clear();
      }
      continue;
    };
    let (_, stack_end) = stacked_value_bounds(chart, series_index, index, value);
    let point_value = if matches!(
      series.grouping,
      ChartSeriesGrouping::Stacked | ChartSeriesGrouping::PercentStacked
    ) {
      stack_end
    } else {
      value
    };
    let display_index = category_display_index(chart, index, context.category_count);
    run.push((
      category_point_x(chart, display_index, context.category_count, context.plot),
      value_y(
        point_value,
        context.scale,
        context.plot.top,
        context.plot.height,
      ),
      index,
    ));
  }
  if run.len() >= 2 {
    runs.push(run);
  }
  for run in runs {
    let points = if series.smooth == Some(true) {
      sample_cardinal_chart_line(&run, 12)
    } else {
      run.iter().map(|point| (point.0, point.1)).collect()
    };
    lower_3d_line_stripes(
      items,
      &points,
      projection,
      front_depth,
      back_depth,
      color,
      stroke,
    );
  }
}

fn lower_3d_line_stripes(
  items: &mut Vec<PageItem>,
  points: &[(f32, f32)],
  projection: Chart3DProjection,
  front_depth: f32,
  back_depth: f32,
  color: RgbColor,
  stroke: Option<(RgbColor, f32)>,
) {
  for segment in points.windows(2) {
    let point1_front = projection.project(segment[0].0, segment[0].1, front_depth);
    let point2_front = projection.project(segment[1].0, segment[1].1, front_depth);
    let point2_back = projection.project(segment[1].0, segment[1].1, back_depth);
    let point1_back = projection.project(segment[0].0, segment[0].1, back_depth);
    // LibreOffice AreaChart::impl_createLine and Stripe.cxx define a 3-D
    // line segment as p1,p2,p2+depth,p1+depth with filled-series
    // properties. Preserve that face instead of drawing a projected 2-D
    // stroke.
    push_chart_polygon(
      items,
      &[point1_front, point2_front, point2_back, point1_back],
      color,
      stroke,
    );
  }
}

fn lower_3d_extruded_polygon(
  items: &mut Vec<PageItem>,
  polygon: &[(f32, f32)],
  projection: Chart3DProjection,
  front_depth: f32,
  back_depth: f32,
  color: RgbColor,
  stroke: Option<(RgbColor, f32)>,
) {
  if polygon.len() < 3 {
    return;
  }
  let front = polygon
    .iter()
    .map(|point| projection.project(point.0, point.1, front_depth))
    .collect::<Vec<_>>();
  let back = polygon
    .iter()
    .map(|point| projection.project(point.0, point.1, back_depth))
    .collect::<Vec<_>>();
  push_chart_polygon(items, &back, shade_chart_color(color, 0.72), stroke);
  for index in 0..polygon.len() {
    let next = (index + 1) % polygon.len();
    let side_color =
      if (polygon[next].1 - polygon[index].1).abs() > (polygon[next].0 - polygon[index].0).abs() {
        shade_chart_color(color, 0.72)
      } else {
        tint_chart_color(color, 0.12)
      };
    push_chart_polygon(
      items,
      &[front[index], front[next], back[next], back[index]],
      side_color,
      stroke,
    );
  }
  push_chart_polygon(items, &front, color, stroke);
}

fn sample_cardinal_chart_line(
  points: &[(f32, f32, usize)],
  resolution_per_segment: usize,
) -> Vec<(f32, f32)> {
  if points.len() < 2 {
    return points.iter().map(|point| (point.0, point.1)).collect();
  }
  let resolution = resolution_per_segment.max(1);
  let mut result = vec![(points[0].0, points[0].1)];
  for index in 0..points.len() - 1 {
    let start = (points[index].0, points[index].1);
    let end = (points[index + 1].0, points[index + 1].1);
    let (control1, control2) = cardinal_cubic_controls(points, index);
    for step in 1..=resolution {
      let t = step as f32 / resolution as f32;
      result.push(cubic_bezier_point(start, control1, control2, end, t));
    }
  }
  result
}

fn cubic_bezier_point(
  start: (f32, f32),
  control1: (f32, f32),
  control2: (f32, f32),
  end: (f32, f32),
  t: f32,
) -> (f32, f32) {
  let inverse = 1.0 - t;
  let start_weight = inverse * inverse * inverse;
  let control1_weight = 3.0 * inverse * inverse * t;
  let control2_weight = 3.0 * inverse * t * t;
  let end_weight = t * t * t;
  (
    start.0 * start_weight
      + control1.0 * control1_weight
      + control2.0 * control2_weight
      + end.0 * end_weight,
    start.1 * start_weight
      + control1.1 * control1_weight
      + control2.1 * control2_weight
      + end.1 * end_weight,
  )
}

fn lower_cardinal_cubic_chart_line(
  items: &mut Vec<PageItem>,
  points: &[(f32, f32, usize)],
  stroke: Option<&crate::common::ShapeStyleValue<crate::common::Stroke<'static>>>,
  color: RgbColor,
  width_pt: f32,
  stroke_width_scale: f32,
) {
  if points.len() < 2 {
    return;
  }

  // Office fixed output lowers c:smooth through a uniform cardinal spline.
  // For each segment, the adjacent data points determine the endpoint
  // tangents; extrapolated endpoint neighbors preserve the one-sided slope.
  // The resulting Catmull-Rom controls reproduce Office's emitted Bézier
  // coordinates exactly (one sixth of the neighbor-to-neighbor vector).
  let count = points.len();
  let mut path = BezPath::with_capacity(count);
  path.move_to((f64::from(points[0].0), f64::from(points[0].1)));
  for index in 0..count - 1 {
    let (control1, control2) = cardinal_cubic_controls(points, index);
    let end = points[index + 1];
    path.curve_to(
      (f64::from(control1.0), f64::from(control1.1)),
      (f64::from(control2.0), f64::from(control2.1)),
      (f64::from(end.0), f64::from(end.1)),
    );
  }
  let bounds = path.control_box();
  let commands = bez_path_commands(path);
  let bounds = common_rect(
    bounds.x0 as f32,
    bounds.y0 as f32,
    bounds.width() as f32,
    bounds.height() as f32,
  );
  let stroke = match stroke {
    Some(crate::common::ShapeStyleValue::NoPaint) => return,
    Some(crate::common::ShapeStyleValue::Paint(stroke)) => {
      bind_chart_stroke_to_bounds(stroke, bounds, stroke_width_scale)
    }
    Some(crate::common::ShapeStyleValue::Unspecified) | None => crate::common::Stroke {
      width: crate::common::Pt(width_pt * stroke_width_scale),
      color: common_rgb(color, 1.0),
      ..Default::default()
    },
  };
  items.push(PageItem::Path(crate::common::PathItem {
    bounds,
    points: Vec::new(),
    commands,
    closed: false,
    fill: crate::common::Fill::None,
    stroke: Some(stroke),
  }));
}

fn cardinal_cubic_controls(points: &[(f32, f32, usize)], index: usize) -> ((f32, f32), (f32, f32)) {
  let start = points[index];
  let end = points[index + 1];
  let previous = points.get(index.wrapping_sub(1)).copied().unwrap_or((
    2.0 * start.0 - end.0,
    2.0 * start.1 - end.1,
    0,
  ));
  let following =
    points
      .get(index + 2)
      .copied()
      .unwrap_or((2.0 * end.0 - start.0, 2.0 * end.1 - start.1, 0));
  (
    (
      start.0 + (end.0 - previous.0) / 6.0,
      start.1 + (end.1 - previous.1) / 6.0,
    ),
    (
      end.0 - (following.0 - start.0) / 6.0,
      end.1 - (following.1 - start.1) / 6.0,
    ),
  )
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
  let point_for = |index: usize, value: f64| {
    let x_value = series
      .x_values
      .get(index)
      .copied()
      .flatten()
      .unwrap_or(index as f64 + 1.0);
    let x = geometry
      .x_scale
      .map_or(geometry.plot.left + geometry.plot.width * 0.5, |scale| {
        value_x(x_value, scale, geometry.plot)
      });
    (
      x,
      value_y(
        value,
        geometry.scale,
        geometry.plot.top,
        geometry.plot.height,
      ),
    )
  };

  if !bubbles && series.smooth == Some(true) {
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
      let (x, y) = point_for(index, *value);
      let point = (x, y, index);
      marker_points.push(point);
      run.push(point);
    }
    if run.len() >= 2 {
      runs.push(run);
    }
    if !series.line_hidden {
      for run in &runs {
        lower_cardinal_cubic_chart_line(
          items,
          run,
          chart_series_stroke_style(style, series_index, None),
          color,
          series
            .line_width_pt
            .unwrap_or(style.automatic_line_width_pt),
          style.stroke_scale,
        );
      }
    }
    if let Some(size) = chart_marker_size(series) {
      for (x, y, index) in marker_points {
        lower_chart_marker(
          items,
          x,
          y,
          size * style.stroke_scale,
          chart_point_color(style, series_index, index).unwrap_or(color),
          series,
          chart_marker_stroke_width(series, style.stroke_scale),
        );
      }
    }
    return;
  }

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
    let (x, y) = point_for(index, *value);
    if !bubbles {
      run.push((x, y));
    }
    marker_points.push((x, y, index));
  }
  if run.len() >= 2 {
    runs.push(run);
  }
  if !bubbles && !series.line_hidden {
    for run in &runs {
      for clipped in clip_chart_polyline_to_plot(run, geometry.plot) {
        push_chart_styled_polyline(
          items,
          &clipped,
          chart_series_stroke_style(style, series_index, None),
          color,
          series
            .line_width_pt
            .unwrap_or(style.automatic_line_width_pt)
            * style.stroke_scale,
          style.stroke_scale,
        );
      }
    }
  }
  for (x, y, index) in marker_points {
    let size = if bubbles {
      let Some(size) = bubble_marker_diameter(series, index, geometry) else {
        continue;
      };
      size
    } else {
      let Some(size) = chart_marker_size(series) else {
        continue;
      };
      size * style.stroke_scale
    };
    let point_color = chart_point_color(style, series_index, index).unwrap_or(color);
    if bubbles {
      push_chart_shape_ellipse(
        items,
        x,
        y,
        size,
        chart_series_fill_style(style, series_index, Some(index)),
        chart_series_stroke_style(style, series_index, Some(index)),
        Some(point_color),
        style.stroke_scale,
      );
    } else {
      lower_chart_marker(
        items,
        x,
        y,
        size,
        point_color,
        series,
        chart_marker_stroke_width(series, style.stroke_scale),
      );
    }
  }
}

fn lower_chart_line_segment(
  items: &mut Vec<PageItem>,
  start: (f32, f32),
  end: (f32, f32),
  color: RgbColor,
  width_pt: f32,
) {
  items.push(PageItem::Line(LineItem {
    x1_pt: start.0,
    y1_pt: start.1,
    x2_pt: end.0,
    y2_pt: end.1,
    width_pt,
    color,
    kind: LineItemKind::Stroke,
  }));
}

fn chart_marker_size(series: &crate::render::chart::ClusteredColumnSeries<'_>) -> Option<f32> {
  if let Some(marker) = series.marker {
    if marker
      .symbol
      .as_ref()
      .is_some_and(|symbol| symbol.val == c::MarkerStyleValues::None)
    {
      return None;
    }
    return Some(
      marker
        .size
        .as_ref()
        .and_then(|size| size.val)
        // ECMA-376 CT_MarkerSize declares 5pt as the attribute default.
        .map_or(5.0, f32::from),
    );
  }
  // Office's built-in modern chart styles supply a 6pt marker when the
  // marker-bearing chart group omits `c:marker`. This is distinct from the
  // 5pt CT_MarkerSize default used by an explicit `c:marker`.
  series.automatic_marker_symbol.map(|_| 6.0)
}

fn chart_marker_stroke_width(
  series: &crate::render::chart::ClusteredColumnSeries<'_>,
  stroke_scale: f32,
) -> Option<f32> {
  let width = if let Some(marker) = series.marker {
    if marker
      .chart_shape_properties
      .as_deref()
      .is_some_and(crate::render::chart::chart_shape_properties_has_no_outline)
    {
      return None;
    }
    marker
      .chart_shape_properties
      .as_deref()
      .and_then(|properties| properties.outline.as_deref())
      .and_then(|outline| outline.width)
      .map(|width| crate::units::emu_to_points(i64::from(width)))
      // Office's explicit marker outline default is 0.5pt. The line width of
      // the owning series is not inherited when `c:marker` is present.
      .unwrap_or(0.5)
  } else {
    series.line_width_pt.unwrap_or(1.5)
  };
  Some(width * stroke_scale)
}

fn lower_chart_marker(
  items: &mut Vec<PageItem>,
  x: f32,
  y: f32,
  size: f32,
  color: RgbColor,
  series: &crate::render::chart::ClusteredColumnSeries<'_>,
  stroke_width_pt: Option<f32>,
) {
  let symbol = series
    .marker
    .and_then(|marker| marker.symbol.as_ref())
    .map(|symbol| symbol.val)
    .map(|symbol| {
      if symbol == c::MarkerStyleValues::Auto {
        series
          .automatic_marker_symbol
          .unwrap_or(c::MarkerStyleValues::Auto)
      } else {
        symbol
      }
    })
    .or(series.automatic_marker_symbol)
    .unwrap_or(c::MarkerStyleValues::Circle);
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
        stroke: stroke_width_pt.map(|width_pt| BorderStyle {
          width_pt,
          color,
          ..Default::default()
        }),
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
        stroke_width_pt,
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
        stroke_width_pt,
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
      push_marker_polygon(items, points, x, y, size, color, stroke_width_pt);
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
  stroke_width_pt: Option<f32>,
) {
  items.push(PageItem::Path(crate::common::PathItem {
    bounds: common_rect(x - size * 0.6, y - size * 0.6, size * 1.2, size * 1.2),
    points,
    commands: Vec::new(),
    closed: true,
    fill: crate::common::Fill::Solid(common_rgb(color, 1.0)),
    stroke: stroke_width_pt.map(|width| crate::common::Stroke {
      width: crate::common::Pt(width),
      color: common_rgb(color, 1.0),
      ..Default::default()
    }),
  }));
}

fn chart_point_color(
  style: &ClusteredColumnStyle,
  series_index: usize,
  point_index: usize,
) -> Option<RgbColor> {
  style
    .series_point_styles
    .get(series_index)
    .and_then(|points| points.get(point_index))
    .and_then(Option::as_ref)
    .and_then(|point| match &point.fill {
      crate::common::ShapeStyleValue::Paint(fill) => chart_fill_fallback_color(fill),
      crate::common::ShapeStyleValue::Unspecified | crate::common::ShapeStyleValue::NoPaint => None,
    })
    .or_else(|| {
      style
        .series_point_colors
        .get(series_index)
        .and_then(|points| points.get(point_index))
        .copied()
        .flatten()
    })
}

fn radar_plot_geometry(
  chart: &ClusteredColumnChart<'_>,
  style: &ClusteredColumnStyle,
  plot: PlotRect,
) -> ((f32, f32), f32, f32) {
  let mut center = (plot.left + plot.width * 0.5, plot.top + plot.height * 0.5);
  let radius = plot.width.min(plot.height) * 0.46;
  if powerpoint_derived_single_series_title_layout(chart, style) {
    // PowerPoint fits the radar net to a square independently of the wider
    // category-label envelope. Immutable fixed output for the POI radar
    // fixture places all four outer vertices 108.9pt from the center; the
    // normalized factors retain that geometry when the chart frame scales.
    center.0 -= radius * 0.007_22;
    center.1 -= radius * 0.005_76;
    let radius = radius * 0.944_8;
    (center, radius, radius)
  } else {
    (center, radius, radius)
  }
}

fn radar_category_label_geometry(
  chart: &ClusteredColumnChart<'_>,
  style: &ClusteredColumnStyle,
  plot: PlotRect,
) -> ((f32, f32), f32, f32) {
  if powerpoint_derived_single_series_title_layout(chart, style) {
    // The legacy PowerPoint category text is laid out in a separate envelope
    // from the square radar net. Keeping the two rectangles independent is
    // necessary for the left/right labels, which sit beyond the net by a
    // larger distance than the top/bottom labels.
    let radius = plot.width.min(plot.height) * 0.46;
    (
      (
        plot.left + plot.width * 0.5,
        plot.top + plot.height * 0.5 - style.label.font_size_pt * 0.094,
      ),
      radius + style.label.font_size_pt * 0.56,
      (radius - style.label.font_size_pt * 0.294).max(1.0),
    )
  } else {
    radar_plot_geometry(chart, style, plot)
  }
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
  let (center, radius_x, radius_y) = radar_plot_geometry(context.chart, style, plot);
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
      center.0 + angle.sin() * radius_x * ratio as f32,
      center.1 - angle.cos() * radius_y * ratio as f32,
    );
    polygon_points.push(common_point(point.0, point.1));
    if let Some((previous_x, previous_y)) = previous {
      push_chart_styled_line(
        items,
        (previous_x, previous_y),
        point,
        chart_series_stroke_style(style, series_index, None),
        color,
        series
          .line_width_pt
          .unwrap_or(style.automatic_line_width_pt)
          * style.stroke_scale,
        style.stroke_scale,
      );
    } else {
      first = Some(point);
    }
    previous = Some(point);
    if let Some(size) = chart_marker_size(series) {
      lower_chart_marker(
        items,
        point.0,
        point.1,
        size * style.stroke_scale,
        chart_point_color(style, series_index, index).unwrap_or(color),
        series,
        chart_marker_stroke_width(series, style.stroke_scale),
      );
    }
  }
  if let (Some(first), Some(last)) = (first, previous) {
    push_chart_styled_line(
      items,
      last,
      first,
      chart_series_stroke_style(style, series_index, None),
      color,
      series
        .line_width_pt
        .unwrap_or(style.automatic_line_width_pt)
        * style.stroke_scale,
      style.stroke_scale,
    );
  }
  if series.filled_area && polygon_points.len() >= 3 {
    let bounds = common_rect(plot.left, plot.top, plot.width, plot.height);
    let fill = chart_series_area_fill(style, series_index, bounds, color, 0.42);
    if fill != crate::common::Fill::None {
      items.insert(
        geometry_start,
        PageItem::Path(crate::common::PathItem {
          bounds,
          points: polygon_points,
          commands: Vec::new(),
          closed: true,
          fill,
          stroke: None,
        }),
      );
    }
  }
}

fn lower_radar_axes(
  items: &mut Vec<PageItem>,
  chart: &ClusteredColumnChart<'_>,
  plot: PlotRect,
  scale: crate::render::chart::LinearAxisScale,
  style: &ClusteredColumnStyle,
  metrics: &mut TextMetrics,
  category_labels_first: bool,
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
  let (center, radius_x, radius_y) = radar_plot_geometry(chart, style, plot);
  let (label_center, label_radius_x, label_radius_y) =
    radar_category_label_geometry(chart, style, plot);
  let rings = ((scale.maximum - scale.minimum) / scale.major_unit)
    .round()
    .clamp(1.0, 10.0) as usize;
  for ring in 1..=rings {
    let ring_radius_x = radius_x * ring as f32 / rings as f32;
    let ring_radius_y = radius_y * ring as f32 / rings as f32;
    let mut previous = None;
    let mut first = None;
    for index in 0..count {
      let angle = std::f32::consts::TAU * index as f32 / count as f32;
      let point = (
        center.0 + angle.sin() * ring_radius_x,
        center.1 - angle.cos() * ring_radius_y,
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
  let value_labels_visible = chart.value_axis.is_none_or(|axis| {
    value_axis_is_visible(axis)
      && axis
        .tick_label_position
        .as_ref()
        .is_none_or(|position| position.val != Some(c::TickLabelPositionValues::None))
  });
  let mut value_label_items = Vec::new();
  if value_labels_visible {
    let format_code = vertical_axis_number_format_code(chart, 0);
    let display_unit = chart.value_axis.map_or(1.0, value_axis_display_unit);
    for (value, label) in scale_tick_labels(
      scale.minimum,
      scale.maximum,
      scale.major_unit,
      format_code,
      scale.logarithmic_base,
      display_unit,
    ) {
      let ratio = ((value - scale.minimum) / (scale.maximum - scale.minimum)).clamp(0.0, 1.0);
      let width = metrics.measure_text(&label, &style.value_label);
      let legacy_title_layout = powerpoint_derived_single_series_title_layout(chart, style);
      push_text(
        &mut value_label_items,
        center.0
          - width
          - style.value_label.font_size_pt * if legacy_title_layout { 1.06 } else { 0.22 },
        center.1 - radius_y * ratio as f32 - line_height(&style.value_label) * 0.5,
        label,
        style.value_label.clone(),
      );
    }
  }
  let mut category_label_items = Vec::new();
  for index in 0..count {
    let display_index = category_display_index(chart, index, count);
    let angle = std::f32::consts::TAU * display_index as f32 / count as f32;
    let outer = (
      center.0 + angle.sin() * radius_x,
      center.1 - angle.cos() * radius_y,
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
      let label_radius_x = label_radius_x + style.label.font_size_pt * 0.85;
      let label_radius_y = label_radius_y + style.label.font_size_pt * 0.85;
      push_text(
        &mut category_label_items,
        label_center.0 + angle.sin() * label_radius_x - width * 0.5,
        label_center.1 - angle.cos() * label_radius_y - line_height(&style.label) * 0.5,
        category.clone(),
        style.label.clone(),
      );
    }
  }
  if category_labels_first {
    items.append(&mut category_label_items);
    items.append(&mut value_label_items);
  } else {
    items.append(&mut value_label_items);
    items.append(&mut category_label_items);
  }
}

fn lower_horizontal_bar_axes(
  items: &mut Vec<PageItem>,
  chart: &ClusteredColumnChart<'_>,
  tick_labels: &[(f64, String)],
  geometry: HorizontalAxisGeometry,
  style: &ClusteredColumnStyle,
  metrics: &mut TextMetrics,
  category_labels_first: bool,
) {
  let HorizontalAxisGeometry {
    plot,
    scale,
    projection_3d,
    draw_gridlines,
    draw_labels,
  } = geometry;
  let value_axis_visible = chart.value_axis.is_none_or(value_axis_is_visible);
  let value_labels_visible = value_axis_visible
    && chart.value_axis.is_none_or(|axis| {
      axis
        .tick_label_position
        .as_ref()
        .is_none_or(|position| position.val != Some(c::TickLabelPositionValues::None))
    });
  let value_gridlines_visible = value_axis_visible
    && chart
      .value_axis
      .is_none_or(|axis| axis.major_gridlines.is_some());
  let mut value_label_items = Vec::new();
  for (value, label) in tick_labels {
    let x = value_x(*value, scale, plot);
    if draw_gridlines && value_gridlines_visible {
      let ((x1, y1), (x2, y2)) =
        projection_3d.map_or(((x, plot.top), (x, plot.top + plot.height)), |projection| {
          (
            projection.project(x, plot.top, 1.0),
            projection.project(x, plot.top + plot.height, 1.0),
          )
        });
      items.push(PageItem::Line(LineItem {
        x1_pt: x1,
        y1_pt: y1,
        x2_pt: x2,
        y2_pt: y2,
        width_pt: 0.75,
        color: style.gridline_color,
        kind: LineItemKind::Stroke,
      }));
    }
    if draw_labels && value_labels_visible {
      let (axis_x, axis_y) = projection_3d.map_or((x, plot.top + plot.height), |projection| {
        projection.project(x, plot.top + plot.height, 1.0)
      });
      let width = metrics.measure_text(label, &style.value_label);
      push_text(
        &mut value_label_items,
        axis_x - width * 0.5,
        axis_y + style.value_label.font_size_pt * if category_labels_first { 0.70 } else { 0.25 },
        label.clone(),
        style.value_label.clone(),
      );
    }
  }
  if !draw_labels {
    return;
  }
  let category_labels_visible = chart
    .category_axis
    .map(category_axis_is_visible)
    .or_else(|| chart.date_axis.map(date_axis_is_visible))
    .unwrap_or(true)
    && chart
      .category_axis
      .map(|axis| {
        axis
          .tick_label_position
          .as_ref()
          .is_none_or(|position| position.val != Some(c::TickLabelPositionValues::None))
      })
      .or_else(|| {
        chart.date_axis.map(|axis| {
          axis
            .tick_label_position
            .as_ref()
            .is_none_or(|position| position.val != Some(c::TickLabelPositionValues::None))
        })
      })
      .unwrap_or(true);
  if !category_labels_visible {
    items.append(&mut value_label_items);
    return;
  }
  let count = chart.categories.len().max(1);
  let mut category_label_items = Vec::new();
  for (index, category) in chart.categories.iter().enumerate() {
    let width = metrics.measure_text(category, &style.category_label);
    let display_index = horizontal_bar_category_display_index(chart, index, count);
    let y = plot.top + (display_index as f32 + 0.5) / count as f32 * plot.height
      - line_height(&style.category_label) * 0.5;
    let (axis_x, axis_y) = projection_3d.map_or((plot.left, y), |projection| {
      projection.project(plot.left, y, 1.0)
    });
    push_text(
      &mut category_label_items,
      axis_x
        - width
        - style.category_label.font_size_pt * if category_labels_first { 0.925 } else { 0.45 },
      axis_y,
      category.clone(),
      style.category_label.clone(),
    );
  }
  if category_labels_first {
    items.append(&mut category_label_items);
    items.append(&mut value_label_items);
  } else {
    items.append(&mut value_label_items);
    items.append(&mut category_label_items);
  }
}

fn lower_scatter_x_axis(
  items: &mut Vec<PageItem>,
  chart: &ClusteredColumnChart<'_>,
  plot: PlotRect,
  scale: crate::render::chart::LinearAxisScale,
  style: &ClusteredColumnStyle,
  metrics: &mut TextMetrics,
  draw_gridlines: bool,
  draw_labels: bool,
) {
  let axis_visible = chart
    .horizontal_value_axis
    .is_none_or(value_axis_is_visible);
  let labels_visible = axis_visible
    && chart.horizontal_value_axis.is_none_or(|axis| {
      axis
        .tick_label_position
        .as_ref()
        .is_none_or(|position| position.val != Some(c::TickLabelPositionValues::None))
    });
  let gridlines_visible = axis_visible
    && chart
      .horizontal_value_axis
      .is_none_or(|axis| axis.major_gridlines.is_some());
  let format_code = horizontal_axis_number_format_code(chart, 0);
  let display_unit = chart
    .horizontal_value_axis
    .map_or(1.0, value_axis_display_unit);
  for (value, label) in scale_tick_labels(
    scale.minimum,
    scale.maximum,
    scale.major_unit,
    format_code,
    scale.logarithmic_base,
    display_unit,
  ) {
    let x = value_x(value, scale, plot);
    if draw_gridlines && gridlines_visible {
      items.push(PageItem::Line(LineItem {
        x1_pt: x,
        y1_pt: plot.top,
        x2_pt: x,
        y2_pt: plot.top + plot.height,
        width_pt: 0.75,
        color: style.gridline_color,
        kind: LineItemKind::Stroke,
      }));
    }
    if draw_labels && labels_visible {
      let width = metrics.measure_text(&label, &style.category_label);
      let has_modern_single_series_scatter_title_layout = style.layout_profile
        == ChartLayoutProfile::Excel
        && style.modern_excel_profile
        && !style.has_explicit_title
        && matches!(chart.title.as_ref(), Some(ChartTitleText::Explicit(_)))
        && chart.series.len() == 1
        && chart.legend_position.is_none();
      push_text(
        items,
        x - width * 0.5,
        plot.top
          + plot.height
          + style.category_label.font_size_pt
            * if (style.layout_profile == ChartLayoutProfile::Excel
              && (scatter_uses_index_x_values(chart)
                || has_modern_single_series_scatter_title_layout))
              || powerpoint_derived_single_series_title_layout(chart, style)
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
}

fn scatter_uses_index_x_values(chart: &ClusteredColumnChart<'_>) -> bool {
  (0..axis_set_count(chart))
    .any(|axis_set_index| axis_set_scatter_uses_index_x_values(chart, axis_set_index))
}

fn axis_set_scatter_uses_index_x_values(
  chart: &ClusteredColumnChart<'_>,
  axis_set_index: usize,
) -> bool {
  let scatter_series = chart.series.iter().filter(|series| {
    series.axis_set_index == axis_set_index
      && matches!(
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

fn scatter_x_axis_values(chart: &ClusteredColumnChart<'_>, axis_set_index: usize) -> Vec<f64> {
  let mut values = chart
    .series
    .iter()
    .filter(|series| {
      series.axis_set_index == axis_set_index
        && matches!(
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
    .collect::<Vec<_>>();
  for series in chart.series.iter().filter(|series| {
    series.axis_set_index == axis_set_index
      && matches!(
        series.kind,
        ChartSeriesKind::Scatter | ChartSeriesKind::Bubble
      )
  }) {
    for error_bars in series
      .error_bars
      .iter()
      .filter(|bars| bars.direction == c::ErrorBarDirectionValues::X)
    {
      let point_count = series.x_values.len().max(series.values.len());
      for point_index in 0..point_count {
        let center = series
          .x_values
          .get(point_index)
          .copied()
          .flatten()
          .unwrap_or(point_index as f64 + 1.0);
        if !center.is_finite() {
          continue;
        }
        if error_bars.show_positive
          && let Some(delta) = chart_error_bar_delta(error_bars, series, point_index, true)
        {
          values.push(center + delta);
        }
        if error_bars.show_negative
          && let Some(delta) = chart_error_bar_delta(error_bars, series, point_index, false)
        {
          values.push(center - delta);
        }
      }
    }
  }
  values
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
      .filter(|peer| {
        peer.axis_set_index == series.axis_set_index
          && peer.kind == series.kind
          && peer.grouping == series.grouping
      })
      .filter_map(|peer| peer.values.get(category_index).copied().flatten())
      .map(f64::abs)
      .sum::<f64>()
      .max(f64::EPSILON)
  } else {
    1.0
  };
  let normalized = value / total;
  let mut start = 0.0;
  for peer in chart.series[..series_index].iter().filter(|peer| {
    peer.axis_set_index == series.axis_set_index
      && peer.kind == series.kind
      && peer.grouping == series.grouping
  }) {
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
  axes: CartesianAxisScales,
  zero_y: f32,
  category_count: usize,
) -> Option<ChartPointAnchor> {
  let series = chart.series.get(series_index)?;
  let value = series.values.get(point_index).copied().flatten()?;
  let scale = axes.y;
  match series.kind {
    ChartSeriesKind::Column => {
      let peers = chart
        .series
        .iter()
        .filter(|peer| {
          peer.axis_set_index == series.axis_set_index
            && peer.kind == series.kind
            && peer.grouping == series.grouping
        })
        .count()
        .max(1);
      let peer_index = chart.series[..series_index]
        .iter()
        .filter(|peer| {
          peer.axis_set_index == series.axis_set_index
            && peer.kind == series.kind
            && peer.grouping == series.grouping
        })
        .count();
      let clustered = series.grouping == ChartSeriesGrouping::Clustered;
      let slot = clustered_column_slot(
        series_category_display_index(
          chart.category_axis_reversed,
          series.kind,
          point_index,
          category_count,
        ),
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
        .filter(|peer| {
          peer.axis_set_index == series.axis_set_index
            && peer.kind == series.kind
            && peer.grouping == series.grouping
        })
        .count()
        .max(1);
      let peer_index = chart.series[..series_index]
        .iter()
        .filter(|peer| {
          peer.axis_set_index == series.axis_set_index
            && peer.kind == series.kind
            && peer.grouping == series.grouping
        })
        .count();
      let clustered = series.grouping == ChartSeriesGrouping::Clustered;
      let slot = clustered_column_slot(
        series_category_display_index(
          chart.category_axis_reversed,
          series.kind,
          point_index,
          category_count,
        ),
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
      let x_value = series
        .x_values
        .get(point_index)
        .copied()
        .flatten()
        .unwrap_or(point_index as f64 + 1.0);
      let x = axes.x.map_or(plot.left + plot.width * 0.5, |scale| {
        value_x(x_value, scale, plot)
      });
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
  series_category_display_index(
    chart.category_axis_reversed,
    ChartSeriesKind::Column,
    source_index,
    category_count,
  )
}

fn series_category_display_index(
  category_axis_reversed: bool,
  kind: ChartSeriesKind,
  source_index: usize,
  category_count: usize,
) -> usize {
  // A horizontal bar swaps the category/value axes. Screen Y grows downward,
  // so OOXML minMax places the first source category at the bottom; all other
  // cartesian series place the first minMax category at the left.
  let reverse = if kind == ChartSeriesKind::Bar {
    !category_axis_reversed
  } else {
    category_axis_reversed
  };
  if reverse && source_index < category_count {
    category_count - 1 - source_index
  } else {
    source_index
  }
}

fn horizontal_bar_category_display_index(
  chart: &ClusteredColumnChart<'_>,
  source_index: usize,
  category_count: usize,
) -> usize {
  // BarChartTypeTemplate swaps X/Y. PlottingPositionHelper then applies the
  // category orientation to the physical Y scale, whose screen direction is
  // inverted. Consequently the OOXML default minMax order paints the first
  // source category at the bottom; explicit maxMin restores source order.
  series_category_display_index(
    chart.category_axis_reversed,
    ChartSeriesKind::Bar,
    source_index,
    category_count,
  )
}

fn project_3d_data_label_anchor(
  chart: &ClusteredColumnChart<'_>,
  series_index: usize,
  mut anchor: ChartPointAnchor,
  projection: Option<Chart3DProjection>,
) -> ChartPointAnchor {
  let Some(projection) = projection else {
    return anchor;
  };
  let Some(series) = chart.series.get(series_index).filter(|series| series.is_3d) else {
    return anchor;
  };
  let (front, _) = chart_3d_series_depth_slot(chart, series_index);
  match series.kind {
    ChartSeriesKind::Column | ChartSeriesKind::Bar => {
      // BarChart::getLabelScreenPositionAndAlignment transforms the complete
      // 3-D anchor to screen coordinates first; createDataLabel then applies
      // the separate 260 mm100 clearance. Projecting only the category
      // coordinate detaches outside labels from their visible marker face.
      let end = projection.project(anchor.x, anchor.y, front);
      let base = projection.project(anchor.base_x, anchor.base_y, front);
      anchor.x = end.0;
      anchor.y = end.1;
      anchor.base_x = base.0;
      anchor.base_y = base.1;
    }
    ChartSeriesKind::Line
    | ChartSeriesKind::Area
    | ChartSeriesKind::Scatter
    | ChartSeriesKind::Bubble
    | ChartSeriesKind::Radar
    | ChartSeriesKind::Stock
    | ChartSeriesKind::Surface => {}
  }
  anchor
}

fn category_point_x(
  chart: &ClusteredColumnChart<'_>,
  display_index: usize,
  category_count: usize,
  plot: PlotRect,
) -> f32 {
  if chart.date_axis.is_some() {
    let source_index = if chart.category_axis_reversed && display_index < category_count {
      category_count - 1 - display_index
    } else {
      display_index
    };
    if let Some(value) = chart
      .category_axis_values
      .get(source_index)
      .copied()
      .flatten()
    {
      if let Some(ratio) = date_axis_data_position(chart, value) {
        let ratio = if chart.category_axis_reversed {
          1.0 - ratio
        } else {
          ratio
        };
        return plot.left + ratio as f32 * plot.width;
      }
    }
  }
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

fn value_axis_is_on_right(axis: &c::ValueAxis) -> bool {
  axis.axis_position.val == c::AxisPositionValues::Right
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

fn powerpoint_major_tick_offsets(
  tick_mark: c::TickMarkValues,
  outward_sign: f32,
) -> Option<(f32, f32)> {
  let length = profiles::POWERPOINT_AUTOMATIC_MAJOR_TICK_LENGTH_PT;
  match tick_mark {
    c::TickMarkValues::None => None,
    c::TickMarkValues::Outside => Some((0.0, outward_sign * length)),
    c::TickMarkValues::Inside => Some((0.0, -outward_sign * length)),
    c::TickMarkValues::Cross => Some((-outward_sign * length, outward_sign * length)),
  }
}

#[derive(Clone, Copy)]
struct PowerPointHorizontalAxisTickContext<'a> {
  plot: PlotRect,
  axis_y: f32,
  x_scale: Option<crate::render::chart::LinearAxisScale>,
  date_ticks: Option<&'a [ChartCategoryTick]>,
  category_count: usize,
  width_pt: f32,
  color: RgbColor,
}

fn lower_powerpoint_horizontal_axis_major_ticks(
  items: &mut Vec<PageItem>,
  chart: &ClusteredColumnChart<'_>,
  context: PowerPointHorizontalAxisTickContext<'_>,
) {
  let numeric_axis = context.x_scale.zip(chart.horizontal_value_axis);
  let (tick_mark, axis_on_top) = if let Some((_, axis)) = numeric_axis {
    (
      axis
        .major_tick_mark
        .as_ref()
        .map(|tick| tick.val.unwrap_or_default()),
      axis.axis_position.val == c::AxisPositionValues::Top,
    )
  } else if let Some(axis) = chart.category_axis {
    (
      axis
        .major_tick_mark
        .as_ref()
        .map(|tick| tick.val.unwrap_or_default()),
      axis.axis_position.val == c::AxisPositionValues::Top,
    )
  } else if let Some(axis) = chart.date_axis {
    (
      axis
        .major_tick_mark
        .as_ref()
        .map(|tick| tick.val.unwrap_or_default()),
      axis.axis_position.val == c::AxisPositionValues::Top,
    )
  } else {
    (None, false)
  };
  let Some((start_offset, end_offset)) = tick_mark.and_then(|tick_mark| {
    powerpoint_major_tick_offsets(tick_mark, if axis_on_top { -1.0 } else { 1.0 })
  }) else {
    return;
  };

  let positions = if let Some((scale, _)) = numeric_axis {
    scale_tick_labels(
      scale.minimum,
      scale.maximum,
      scale.major_unit,
      None,
      scale.logarithmic_base,
      1.0,
    )
    .into_iter()
    .map(|(value, _)| value_x(value, scale, context.plot))
    .collect::<Vec<_>>()
  } else if let Some(ticks) = context.date_ticks {
    ticks
      .iter()
      .map(|tick| {
        let position = if chart.category_axis_reversed {
          1.0 - tick.position
        } else {
          tick.position
        };
        context.plot.left + position as f32 * context.plot.width
      })
      .collect()
  } else if chart.category_axis_shifted {
    (0..=context.category_count)
      .map(|index| {
        let position = index as f32 / context.category_count.max(1) as f32;
        let position = if chart.category_axis_reversed {
          1.0 - position
        } else {
          position
        };
        context.plot.left + position * context.plot.width
      })
      .collect()
  } else {
    let denominator = context.category_count.saturating_sub(1).max(1) as f32;
    (0..context.category_count.max(1))
      .map(|index| {
        let position = index as f32 / denominator;
        let position = if chart.category_axis_reversed {
          1.0 - position
        } else {
          position
        };
        context.plot.left + position * context.plot.width
      })
      .collect()
  };
  for x in positions {
    items.push(PageItem::Line(LineItem {
      x1_pt: x,
      y1_pt: context.axis_y + start_offset,
      x2_pt: x,
      y2_pt: context.axis_y + end_offset,
      width_pt: context.width_pt,
      color: context.color,
      kind: LineItemKind::Stroke,
    }));
  }
}

#[derive(Clone, Copy)]
struct PowerPointVerticalAxisTickContext<'a> {
  axis: Option<&'a c::ValueAxis>,
  axis_x: f32,
  axis_on_right: bool,
  scale: crate::render::chart::LinearAxisScale,
  plot_top: f32,
  plot_height: f32,
  width_pt: f32,
  color: RgbColor,
}

fn lower_powerpoint_vertical_axis_major_ticks(
  items: &mut Vec<PageItem>,
  tick_labels: &[(f64, String)],
  context: PowerPointVerticalAxisTickContext<'_>,
) {
  let Some(tick_mark) = context
    .axis
    .and_then(|axis| axis.major_tick_mark.as_ref())
    .map(|tick| tick.val.unwrap_or_default())
  else {
    return;
  };
  let Some((start_offset, end_offset)) =
    powerpoint_major_tick_offsets(tick_mark, if context.axis_on_right { 1.0 } else { -1.0 })
  else {
    return;
  };
  for (value, _) in tick_labels {
    let y = value_y(*value, context.scale, context.plot_top, context.plot_height);
    items.push(PageItem::Line(LineItem {
      x1_pt: context.axis_x + start_offset,
      y1_pt: y,
      x2_pt: context.axis_x + end_offset,
      y2_pt: y,
      width_pt: context.width_pt,
      color: context.color,
      kind: LineItemKind::Stroke,
    }));
  }
}

fn category_axis_text_rotation_is_supported(
  properties: Option<&c::TextProperties>,
  category_count: usize,
) -> bool {
  properties.is_none_or(|properties| {
    properties
      .body_properties
      .rotation
      .is_none_or(|_| category_axis_text_rotation_degrees(Some(properties)).abs() <= 90.0)
      || category_count <= 6
  })
}

fn category_axis_text_rotation_degrees(properties: Option<&c::TextProperties>) -> f32 {
  let Some(rotation) = properties.and_then(|properties| properties.body_properties.rotation) else {
    return 0.0;
  };
  // DrawingML stores 1/60000 degrees, but chart text is more restrictive
  // than an ordinary DrawingML shape. LibreOffice
  // ObjectFormatter::convertTextRotation and PowerPoint's immutable PDFs map
  // values outside the chart UI's -90..90 range to zero; do not modulo-wrap
  // -1000 degrees into an unrelated 80-degree rotation.
  if !(-5_400_000..=5_400_000).contains(&rotation) {
    return 0.0;
  }
  rotation as f32 / 60_000.0
}

fn category_axis_text_rotation_degrees_for_layout(
  profile: ChartLayoutProfile,
  properties: Option<&c::TextProperties>,
  available_axis_length_pt: f32,
  label_count: usize,
  maximum_label_width_pt: f32,
  font_size_pt: f32,
) -> f32 {
  let authored = category_axis_text_rotation_degrees(properties);
  let Some(rotation) = properties.and_then(|properties| properties.body_properties.rotation) else {
    return authored;
  };
  if profile != ChartLayoutProfile::Excel
    || (-5_400_000..=5_400_000).contains(&rotation)
    || label_count == 0
  {
    return authored;
  }

  // LibreOffice-produced XLSX files use -1000 degrees as an out-of-range
  // automatic-layout sentinel. Excel leaves short labels horizontal, but
  // rotates a crowded axis to the nearest vertical bound. The Office PDFs
  // provide both sides of this rule: tdf132076/tdf134118 stay horizontal,
  // while the 22 long monthly labels in tdf165503 become -90 degrees.
  let available_per_label = available_axis_length_pt / label_count as f32;
  let horizontal_extent = maximum_label_width_pt + font_size_pt * 0.7;
  if horizontal_extent > available_per_label.max(1.0) {
    if rotation.is_negative() { -90.0 } else { 90.0 }
  } else {
    0.0
  }
}

fn estimated_date_axis_maximum_auto_main_increment_count(
  profile: ChartLayoutProfile,
  chart: &ClusteredColumnChart<'_>,
  available_axis_length_pt: f32,
  ticks: &[ChartCategoryTick],
  style: &TextStyle,
  metrics: &mut TextMetrics,
) -> usize {
  if chart
    .date_axis
    .and_then(|axis| axis.major_unit.as_ref())
    .is_some()
  {
    return 500;
  }
  let maximum_width = ticks
    .iter()
    .map(|tick| metrics.measure_text(&tick.text, style))
    .fold(0.0f32, f32::max);
  if maximum_width <= f32::EPSILON {
    return 500;
  }
  let text_properties = chart
    .date_axis
    .and_then(|axis| axis.text_properties.as_deref());
  let rotation = category_axis_text_rotation_degrees_for_layout(
    profile,
    text_properties,
    available_axis_length_pt,
    ticks.len(),
    maximum_width,
    style.font_size_pt,
  )
  .to_radians();
  let insets = generated_chart_text_body_insets(
    text_properties
      .map(|properties| properties.body_properties.as_ref())
      .or(chart.default_text_body_properties),
  );
  // Date/category labels are complete DrawingML text bodies whose rotated
  // frame is used for collision spacing. Unlike the generated numeric value
  // labels measured above, both opposing body insets remain part of that
  // frame. The PowerPoint date-categories fixture is a useful counterexample:
  // at -90 degrees a one-sided inset admits daily labels, while the complete
  // body selects the observed two-day interval without overlap.
  let shape_width = maximum_width + insets.left + insets.right;
  let shape_height = metrics
    .inline_text_box_height(style)
    .max(line_height(style))
    + insets.top
    + insets.bottom;
  let projected_width = shape_width * rotation.cos().abs() + shape_height * rotation.sin().abs();
  (available_axis_length_pt / projected_width.max(1.0))
    .floor()
    .clamp(2.0, 500.0) as usize
}

fn chart_3d_category_label_rotation(
  chart: &ClusteredColumnChart<'_>,
  label_lines: &[Vec<String>],
  plot: PlotRect,
  axis_y: f32,
  projection: Chart3DProjection,
  style: &TextStyle,
  metrics: &mut TextMetrics,
) -> f32 {
  let authored = chart
    .category_axis
    .and_then(|axis| axis.text_properties.as_deref())
    .or_else(|| {
      chart
        .date_axis
        .and_then(|axis| axis.text_properties.as_deref())
    })
    .and_then(|properties| properties.body_properties.rotation)
    .map(|rotation| {
      let normalized = rotation.rem_euclid(21_600_000);
      let normalized = if normalized > 10_800_000 {
        normalized - 21_600_000
      } else {
        normalized
      };
      normalized as f32 / 60_000.0
    })
    .filter(|rotation| rotation.abs() <= 90.0);
  if let Some(rotation) = authored {
    return rotation;
  }
  if label_lines.len() <= 1 {
    return 0.0;
  }

  // LibreOffice VCartesianAxis first creates the real label shapes, checks
  // their overlap on the projected axis, then retries at 45 degrees before
  // increasing the label rhythm. Use the same final-axis geometry here.
  let maximum_width = label_lines
    .iter()
    .flatten()
    .map(|line| metrics.measure_text(line, style))
    .fold(0.0_f32, f32::max);
  let first_x = category_point_x(
    chart,
    category_display_index(chart, 0, label_lines.len()),
    label_lines.len(),
    plot,
  );
  let second_x = category_point_x(
    chart,
    category_display_index(chart, 1, label_lines.len()),
    label_lines.len(),
    plot,
  );
  let first = projection.project(first_x, axis_y, 0.0);
  let second = projection.project(second_x, axis_y, 0.0);
  let spacing = (second.0 - first.0).hypot(second.1 - first.1);
  if maximum_width > spacing * 0.9 {
    -45.0
  } else {
    0.0
  }
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
  use crate::render::chart::ChartLayoutMode;

  let left = layout
    .x
    .map_or(automatic.left, |value| match layout.x_mode {
      ChartLayoutMode::Factor => automatic.left + value * frame.width_pt,
      ChartLayoutMode::Edge => frame.x_pt + value * frame.width_pt,
    });
  let top = layout.y.map_or(automatic.top, |value| match layout.y_mode {
    ChartLayoutMode::Factor => automatic.top + value * frame.height_pt,
    ChartLayoutMode::Edge => frame.y_pt + value * frame.height_pt,
  });
  let width = layout
    .width
    .map_or(automatic.width, |value| match layout.width_mode {
      ChartLayoutMode::Factor => value * frame.width_pt,
      ChartLayoutMode::Edge => frame.x_pt + value * frame.width_pt - left,
    });
  let height = layout
    .height
    .map_or(automatic.height, |value| match layout.height_mode {
      ChartLayoutMode::Factor => value * frame.height_pt,
      ChartLayoutMode::Edge => frame.y_pt + value * frame.height_pt - top,
    });
  let width = width.max(0.0);
  let height = height.max(0.0);
  PlotRect {
    left,
    top,
    width: width.min(frame.x_pt + frame.width_pt - left),
    height: height.min(frame.y_pt + frame.height_pt - top),
  }
}

fn apply_manual_text_layout(
  frame: ChartFrame,
  automatic: PlotRect,
  layout: crate::render::chart::ChartManualLayout,
) -> PlotRect {
  use crate::render::chart::ChartLayoutMode;

  let left = layout
    .x
    .map_or(automatic.left, |value| match layout.x_mode {
      ChartLayoutMode::Factor => automatic.left + value * frame.width_pt,
      ChartLayoutMode::Edge => frame.x_pt + value * frame.width_pt,
    });
  let top = layout.y.map_or(automatic.top, |value| match layout.y_mode {
    ChartLayoutMode::Factor => automatic.top + value * frame.height_pt,
    ChartLayoutMode::Edge => frame.y_pt + value * frame.height_pt,
  });
  // MS-OI29500 §21.2.2.232/235 keeps the fitted text box inside the chart
  // while retaining its size. Width and height are ignored for titles and
  // individual data labels, so clamp only the resolved position.
  let maximum_left = (frame.x_pt + frame.width_pt - automatic.width).max(frame.x_pt);
  let maximum_top = (frame.y_pt + frame.height_pt - automatic.height).max(frame.y_pt);
  PlotRect {
    left: left.clamp(frame.x_pt, maximum_left),
    top: top.clamp(frame.y_pt, maximum_top),
    width: automatic.width,
    height: automatic.height,
  }
}

fn lower_axis_titles(
  items: &mut Vec<PageItem>,
  geometry: AxisTitleGeometry,
  chart: &ClusteredColumnChart<'_>,
  style: &ClusteredColumnStyle,
  metrics: &mut TextMetrics,
) {
  let AxisTitleGeometry {
    frame,
    plot,
    value_label_band_left,
    category_band_top,
    category_label_height,
    data_table_height,
    projection_3d,
  } = geometry;
  // Axis-title rich text is independent from tick labels and legends. Host
  // import has already resolved its explicit run properties and bodyPr
  // rotation on top of the automatic `spAxisTitleTexts` role.
  let value_title_style = &style.value_axis_title;
  let category_title_style = &style.category_axis_title;
  let horizontal_bar = chart
    .series
    .iter()
    .all(|series| series.kind == ChartSeriesKind::Bar);
  if let Some(title) = chart.value_axis_title.as_deref() {
    if horizontal_bar {
      let width = metrics.measure_text(title, value_title_style);
      let title_height = line_height(value_title_style);
      let (box_left, box_top, box_right, box_bottom) =
        rotated_text_box_offsets(width, title_height, value_title_style.rotation_deg);
      let projected_bottom = projection_3d.map_or(plot.top + plot.height, |projection| {
        projection.project(plot.left, plot.top + plot.height, 1.0).1
      });
      let content_bottom = if chart.data_table.is_some() {
        category_band_top + data_table_height
      } else {
        projected_bottom
      };
      lower_fitted_axis_title(
        items,
        frame,
        PlotRect {
          left: plot.left + (plot.width - (box_right - box_left)) * 0.5,
          top: content_bottom + title_height * 0.8,
          width: box_right - box_left,
          height: box_bottom - box_top,
        },
        (box_left, box_top),
        chart.value_axis_title_layout,
        title,
        value_title_style,
      );
    } else {
      let width = metrics.measure_text(title, value_title_style);
      let line_height = line_height(value_title_style);
      let (box_left, box_top, box_right, box_bottom) =
        rotated_text_box_offsets(width, line_height, value_title_style.rotation_deg);
      let x = if chart.value_axis.is_some_and(value_axis_is_on_right) {
        plot.left + plot.width + line_height * 0.1 - box_left
      } else {
        value_label_band_left - line_height * 0.1 - box_right
      };
      let y = plot.top + plot.height * 0.5 - (box_top + box_bottom) * 0.5;
      lower_fitted_axis_title(
        items,
        frame,
        PlotRect {
          left: x + box_left,
          top: y + box_top,
          width: box_right - box_left,
          height: box_bottom - box_top,
        },
        (box_left, box_top),
        chart.value_axis_title_layout,
        title,
        value_title_style,
      );
    }
  }
  if let Some(title) = chart.category_axis_title.as_deref() {
    if horizontal_bar {
      let width = metrics.measure_text(title, category_title_style);
      let maximum_category_width = chart
        .categories
        .iter()
        .map(|category| metrics.measure_text(category, &style.category_label))
        .fold(0.0_f32, f32::max);
      let (axis_x, axis_y) = projection_3d
        .map_or((plot.left, plot.top + plot.height * 0.5), |projection| {
          projection.project(plot.left, plot.top + plot.height * 0.5, 1.0)
        });
      let line_height = line_height(category_title_style);
      let (box_left, box_top, box_right, box_bottom) =
        rotated_text_box_offsets(width, line_height, category_title_style.rotation_deg);
      let category_labels_left =
        axis_x - maximum_category_width - style.category_label.font_size_pt * 0.45;
      let x = category_labels_left - line_height * 0.15 - box_right;
      let y = axis_y - (box_top + box_bottom) * 0.5;
      lower_fitted_axis_title(
        items,
        frame,
        PlotRect {
          left: x + box_left,
          top: y + box_top,
          width: box_right - box_left,
          height: box_bottom - box_top,
        },
        (box_left, box_top),
        chart.category_axis_title_layout,
        title,
        category_title_style,
      );
    } else {
      let width = metrics.measure_text(title, category_title_style);
      let title_height = line_height(category_title_style);
      let (box_left, box_top, box_right, box_bottom) =
        rotated_text_box_offsets(width, title_height, category_title_style.rotation_deg);
      let automatic_top = if chart.data_table.is_some() {
        category_band_top + title_height * 0.05
      } else {
        // `category_band_top` is the top of the complete tick-label box.
        // Reserve that box before placing the title; using a fixed single
        // line offset overlaps rotated or wrapped labels. For an ordinary
        // one-line horizontal axis this reduces to the previous 1.25-line
        // position.
        category_band_top + category_label_height + title_height * 0.25
      };
      lower_fitted_axis_title(
        items,
        frame,
        PlotRect {
          left: plot.left + (plot.width - (box_right - box_left)) * 0.5,
          top: automatic_top,
          width: box_right - box_left,
          height: box_bottom - box_top,
        },
        (box_left, box_top),
        chart.category_axis_title_layout,
        title,
        category_title_style,
      );
    }
  }
  for (index, title) in chart.additional_axis_titles.iter().enumerate() {
    let additional_title_style = style
      .additional_axis_titles
      .get(index)
      .cloned()
      .unwrap_or_else(|| {
        let mut fallback = style.label.clone();
        fallback.bold = true;
        fallback.rotation_deg = title.automatic_rotation_deg;
        fallback
      });
    let width = metrics.measure_text(&title.text, &additional_title_style);
    let title_height = line_height(&additional_title_style);
    let (box_left, box_top, box_right, box_bottom) =
      rotated_text_box_offsets(width, title_height, additional_title_style.rotation_deg);
    let side_offset = additional_title_style.font_size_pt * (0.4 + index as f32 * 1.25);
    let (x, y) = match title.position {
      c::AxisPositionValues::Right => (
        frame.x_pt + frame.width_pt - side_offset,
        plot.top + plot.height * 0.5 - (box_top + box_bottom) * 0.5,
      ),
      c::AxisPositionValues::Left => (
        frame.x_pt + side_offset - box_right,
        plot.top + plot.height * 0.5 - (box_top + box_bottom) * 0.5,
      ),
      c::AxisPositionValues::Top => (
        plot.left + plot.width * 0.5 - (box_left + box_right) * 0.5,
        frame.y_pt + side_offset - box_top,
      ),
      c::AxisPositionValues::Bottom => (
        plot.left + plot.width * 0.5 - (box_left + box_right) * 0.5,
        frame.y_pt + frame.height_pt - side_offset - box_bottom,
      ),
    };
    lower_fitted_axis_title(
      items,
      frame,
      PlotRect {
        left: x + box_left,
        top: y + box_top,
        width: box_right - box_left,
        height: box_bottom - box_top,
      },
      (box_left, box_top),
      title.layout,
      &title.text,
      &additional_title_style,
    );
  }
}

fn lower_fitted_axis_title(
  items: &mut Vec<PageItem>,
  frame: ChartFrame,
  automatic_bounds: PlotRect,
  box_origin_offset: (f32, f32),
  layout: Option<crate::render::chart::ChartManualLayout>,
  title: &str,
  style: &TextStyle,
) {
  // c:title/c:layout positions the final fitted title box. In particular,
  // its x/y coordinates describe the axis-aligned bounds after bodyPr
  // rotation, not the unrotated text origin. Convert back to the origin used
  // by TextItem only after applying the shared title-layout semantics.
  let bounds = layout.map_or(automatic_bounds, |layout| {
    apply_manual_text_layout(frame, automatic_bounds, layout)
  });
  let x = bounds.left - box_origin_offset.0;
  let y = bounds.top - box_origin_offset.1;
  push_text_with_rotation_center(items, x, y, title.to_string(), style.clone(), Some((x, y)));
}

fn rotated_text_box_offsets(width: f32, height: f32, rotation_deg: f32) -> (f32, f32, f32, f32) {
  let (sin, cos) = rotation_deg.to_radians().sin_cos();
  let mut left = f32::INFINITY;
  let mut top = f32::INFINITY;
  let mut right = f32::NEG_INFINITY;
  let mut bottom = f32::NEG_INFINITY;
  for (x, y) in [(0.0, 0.0), (width, 0.0), (width, height), (0.0, height)] {
    let rotated_x = x * cos - y * sin;
    let rotated_y = x * sin + y * cos;
    left = left.min(rotated_x);
    top = top.min(rotated_y);
    right = right.max(rotated_x);
    bottom = bottom.max(rotated_y);
  }
  (left, top, right, bottom)
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
    push_data_table_text(
      items,
      bounds.left + display_column as f32 * column_width + (column_width - width) * 0.5,
      bounds.top + (row_height - line_height(&style.label)) * 0.5,
      category.clone(),
      style.label.clone(),
    );
  }
  let mut series_indices = (0..chart.series.len()).collect::<Vec<_>>();
  if chart
    .series
    .iter()
    .all(|series| series.kind == ChartSeriesKind::Bar)
  {
    // LibreOffice VSeriesPlotter::createLegendEntries reverses a swapped
    // X/Y chart, and its data table consumes the same BarChart slot order.
    // This keeps table rows aligned with the visible top-to-bottom bars.
    series_indices.reverse();
  }
  for (row, series_index) in series_indices.into_iter().enumerate() {
    let series = &chart.series[series_index];
    let y = bounds.top + (row + 1) as f32 * row_height;
    if show_keys {
      let key_size = style.label.font_size_pt * 0.45;
      push_cartesian_legend_key(
        items,
        bounds.left - key_size * 1.5,
        y + (row_height - key_size) * 0.5,
        key_size,
        false,
        &CartesianLegendEntry {
          label: Cow::Borrowed(&series.name),
          color: style.series_colors.get(series_index).copied(),
          kind: series.kind,
          series_index: Some(series_index),
          point_index: None,
          trendline_index: None,
        },
        style,
      );
    }
    let legend_style = style.label.clone();
    let name_width = metrics.measure_text(&series.name, &legend_style);
    push_data_table_text(
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
      push_data_table_text(
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
  scale: crate::render::chart::LinearAxisScale,
) {
  let bounds = apply_manual_layout(
    frame,
    PlotRect {
      left: frame.x_pt + frame.width_pt * 0.8,
      top: frame.y_pt + frame.height_pt * 0.1,
      width: frame.width_pt * 0.2,
      height: frame.height_pt * 0.8,
    },
    layout,
  );
  let marker_size = style.legend.font_size_pt * 0.55;
  let marker_gap = style.legend.font_size_pt * 0.26;
  let entries = cartesian_legend_entries(chart, style, scale);
  if entries.is_empty() || bounds.width <= 0.0 || bounds.height <= 0.0 {
    return;
  }
  let mut metrics = TextMetrics::new();
  let entry_widths = entries
    .iter()
    .map(|entry| {
      marker_size + marker_gap + metrics.measure_text(entry.label.as_ref(), &style.legend)
    })
    .collect::<Vec<_>>();
  let maximum_entry_width = entry_widths.iter().copied().fold(0.0_f32, f32::max);
  let column_count =
    ((bounds.width / maximum_entry_width.max(1.0)).floor() as usize).clamp(1, entries.len());
  let row_count = entries.len().div_ceil(column_count);
  let cell_width = bounds.width / column_count as f32;
  let cell_height = bounds.height / row_count as f32;
  let label_line_height = line_height(&style.legend);
  for (index, (entry, entry_width)) in entries.into_iter().zip(entry_widths).enumerate() {
    let column = index % column_count;
    let row = index / column_count;
    let x = bounds.left + column as f32 * cell_width + (cell_width - entry_width).max(0.0) * 0.5;
    let y =
      bounds.top + row as f32 * cell_height + (cell_height - label_line_height).max(0.0) * 0.5;
    push_cartesian_legend_key(
      items,
      x,
      y + (line_height(&style.legend) - marker_size) * 0.5,
      marker_size,
      cartesian_legend_entry_uses_line_key(chart, &entry),
      &entry,
      style,
    );
    push_text(
      items,
      x + marker_size + marker_gap,
      y,
      entry.label.into_owned(),
      style.legend.clone(),
    );
  }
}

fn lower_horizontal_legend(
  items: &mut Vec<PageItem>,
  frame: ChartFrame,
  available_left: f32,
  y: f32,
  chart: &ClusteredColumnChart<'_>,
  style: &ClusteredColumnStyle,
  scale: crate::render::chart::LinearAxisScale,
  metrics: &mut TextMetrics,
) {
  let titled_indexed_scatter = style.layout_profile == ChartLayoutProfile::Excel
    && !chart.title_overlay
    && matches!(chart.title.as_ref(), Some(ChartTitleText::Automatic))
    && scatter_uses_index_x_values(chart);
  let explicit_bottom_column = excel_explicit_bottom_column_layout(chart, style);
  let untitled_bottom_column = excel_untitled_bottom_column_layout(chart, style);
  let untitled_bottom_line_no_marker = excel_untitled_bottom_line_no_marker_layout(chart, style);
  let word_automatic_title_bottom_column = word_automatic_titled_bottom_layout(chart, style)
    && !style.has_explicit_title
    && chart
      .series
      .iter()
      .all(|series| matches!(series.kind, ChartSeriesKind::Column | ChartSeriesKind::Bar));
  let legend_profile = if word_automatic_title_bottom_column {
    profiles::WORD_AUTOMATIC_TITLE_BOTTOM_COLUMN_LEGEND
  } else if word_automatic_titled_bottom_layout(chart, style) {
    profiles::WORD_TITLED_BOTTOM_CARTESIAN_LEGEND
  } else if titled_indexed_scatter {
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
  let marker_gap = style.legend.font_size_pt * legend_profile.marker_gap_em;
  let entries = cartesian_legend_entries(chart, style, scale);
  let base_entry_gap =
    style.legend.font_size_pt * profiles::DEFAULT_HORIZONTAL_CARTESIAN_LEGEND.entry_gap_em;
  let entry_gap = style.legend.font_size_pt * legend_profile.entry_gap_em;
  let widths: Vec<f32> = entries
    .iter()
    .map(|entry| {
      let legend_style = style.legend.clone();
      horizontal_legend_key_width(
        cartesian_legend_entry_uses_line_key(chart, entry),
        style,
        legend_profile.line_key_width_em,
      ) + marker_gap
        + metrics.measure_text(entry.label.as_ref(), &legend_style)
    })
    .collect();
  let total_width = widths.iter().sum::<f32>() + entry_gap * entries.len().saturating_sub(1) as f32;
  // The value-axis label band has already consumed the leading side of the
  // diagram. Center a bottom legend in the remaining horizontal region, as
  // Office does, rather than recentering it over the full graphic frame.
  let available_right = frame.x_pt + frame.width_pt;
  let mut x = available_left + (available_right - available_left - total_width) / 2.0;
  if titled_indexed_scatter {
    x += (entry_gap - base_entry_gap) * entries.len().saturating_sub(1) as f32 / 2.0;
  }
  x += frame.height_pt * legend_profile.x_offset_height_ratio;
  let y = y + frame.height_pt * legend_profile.y_offset_height_ratio;
  for (entry, width) in entries.into_iter().zip(widths) {
    let line_key = cartesian_legend_entry_uses_line_key(chart, &entry);
    let key_width = horizontal_legend_key_width(line_key, style, legend_profile.line_key_width_em);
    push_cartesian_legend_key(
      items,
      x,
      y + (line_height(&style.legend) - key_width) / 2.0,
      key_width,
      line_key,
      &entry,
      style,
    );
    let legend_style = style.legend.clone();
    push_text(
      items,
      x + key_width + marker_gap,
      y,
      entry.label.into_owned(),
      legend_style,
    );
    x += width + entry_gap;
  }
}

fn horizontal_legend_key_width(
  line_key: bool,
  style: &ClusteredColumnStyle,
  line_key_width_em: f32,
) -> f32 {
  style.legend.font_size_pt * if line_key { line_key_width_em } else { 0.55 }
}

fn vertical_legend_width(
  chart: &ClusteredColumnChart<'_>,
  style: &ClusteredColumnStyle,
  scale: crate::render::chart::LinearAxisScale,
  metrics: &mut TextMetrics,
) -> f32 {
  let marker_size = style.legend.font_size_pt * 0.55;
  let marker_gap = style.legend.font_size_pt * 0.26;
  cartesian_legend_entries(chart, style, scale)
    .iter()
    .map(|entry| {
      let legend_style = style.legend.clone();
      marker_size + marker_gap + metrics.measure_text(entry.label.as_ref(), &legend_style)
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
  scale: crate::render::chart::LinearAxisScale,
) {
  let marker_size = style.legend.font_size_pt
    * if excel_vary_colors_data_table_layout(chart, style) {
      profiles::EXCEL_VARY_COLORS_DATA_TABLE_LEGEND_MARKER_EM
    } else {
      0.55
    };
  let marker_gap = style.legend.font_size_pt * 0.26;
  let entry_gap = style.legend.font_size_pt
    * if excel_vary_colors_data_table_layout(chart, style) {
      profiles::EXCEL_VARY_COLORS_DATA_TABLE_LEGEND_ENTRY_GAP_EM
    } else if matches!(
      style.layout_profile,
      ChartLayoutProfile::Excel | ChartLayoutProfile::Word
    ) {
      profiles::OFFICE_VERTICAL_LEGEND_ENTRY_GAP_EM
    } else {
      profiles::POWERPOINT_VERTICAL_LEGEND_ENTRY_GAP_EM
    };
  let line_height = line_height(&style.legend);
  let entries = cartesian_legend_entries(chart, style, scale);
  let total_height =
    line_height * entries.len() as f32 + entry_gap * entries.len().saturating_sub(1) as f32;
  let mut y = if align_top {
    frame.y_pt + frame.height_pt * 0.04
  } else {
    frame.y_pt + (frame.height_pt - total_height) / 2.0
  };
  if powerpoint_derived_single_series_title_layout(chart, style) && !align_top {
    y += frame.height_pt * profiles::POWERPOINT_DERIVED_SERIES_TITLE_LEGEND_Y_RATIO;
  } else if style.layout_profile == ChartLayoutProfile::Excel
    && style.has_explicit_title
    && !align_top
  {
    y += frame.height_pt
      * if excel_explicit_single_series_side_title_layout(chart, style)
        || excel_legacy_default_single_series_side_title_layout(chart, style)
      {
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
  if excel_vary_colors_data_table_layout(chart, style) && !align_top {
    y += frame.height_pt * profiles::EXCEL_VARY_COLORS_DATA_TABLE_LEGEND_Y_RATIO;
  }
  for entry in entries {
    let line_key = cartesian_legend_entry_uses_line_key(chart, &entry);
    push_cartesian_legend_key(
      items,
      x,
      y + (line_height - marker_size) / 2.0,
      marker_size,
      line_key,
      &entry,
      style,
    );
    let legend_style = style.legend.clone();
    push_text(
      items,
      x + marker_size + marker_gap,
      y,
      entry.label.into_owned(),
      legend_style,
    );
    y += line_height + entry_gap;
  }
}

#[derive(Clone)]
struct CartesianLegendEntry<'a> {
  label: Cow<'a, str>,
  color: Option<RgbColor>,
  kind: ChartSeriesKind,
  series_index: Option<usize>,
  point_index: Option<usize>,
  trendline_index: Option<usize>,
}

fn cartesian_legend_entry_uses_line_key(
  chart: &ClusteredColumnChart<'_>,
  entry: &CartesianLegendEntry<'_>,
) -> bool {
  if entry.trendline_index.is_some() {
    return true;
  }
  if !matches!(
    entry.kind,
    ChartSeriesKind::Line
      | ChartSeriesKind::Scatter
      | ChartSeriesKind::Radar
      | ChartSeriesKind::Stock
  ) {
    return false;
  }
  entry
    .series_index
    .and_then(|index| chart.series.get(index))
    .is_none_or(|series| !series.line_hidden && !series.filled_area)
}

fn push_cartesian_legend_key(
  items: &mut Vec<PageItem>,
  x_pt: f32,
  y_pt: f32,
  size_pt: f32,
  line_key: bool,
  entry: &CartesianLegendEntry<'_>,
  style: &ClusteredColumnStyle,
) {
  let fill = entry
    .trendline_index
    .is_none()
    .then(|| {
      entry
        .series_index
        .and_then(|series_index| chart_series_fill_style(style, series_index, entry.point_index))
    })
    .flatten();
  let stroke = match (entry.series_index, entry.trendline_index) {
    (Some(series_index), Some(trendline_index)) => style
      .trendline_styles
      .get(series_index)
      .and_then(|styles| styles.get(trendline_index))
      .map(|shape| &shape.stroke),
    (Some(series_index), None) => chart_series_stroke_style(style, series_index, entry.point_index),
    (None, _) => None,
  };
  if line_key {
    let y = y_pt + size_pt * 0.5;
    push_chart_styled_line(
      items,
      (x_pt, y),
      (x_pt + size_pt, y),
      stroke,
      entry.color.unwrap_or(RgbColor { r: 0, g: 0, b: 0 }),
      style.automatic_line_width_pt,
      style.stroke_scale,
    );
  } else if entry.kind == ChartSeriesKind::Bubble {
    // Bubble legend keys use the same circular data-point geometry; a square
    // key incorrectly describes a filled bar/area series.
    push_chart_shape_ellipse(
      items,
      x_pt + size_pt * 0.5,
      y_pt + size_pt * 0.5,
      size_pt,
      fill,
      stroke,
      entry.color,
      style.stroke_scale,
    );
  } else {
    push_chart_shape_rect(
      items,
      x_pt,
      y_pt,
      size_pt,
      size_pt,
      fill,
      stroke,
      entry.color,
      style.stroke_scale,
    );
  }
}

fn cartesian_legend_entries<'a>(
  chart: &'a ClusteredColumnChart<'_>,
  style: &ClusteredColumnStyle,
  scale: crate::render::chart::LinearAxisScale,
) -> Vec<CartesianLegendEntry<'a>> {
  if chart.surface_groups.len() == 1
    && chart
      .series
      .iter()
      .all(|series| series.kind == ChartSeriesKind::Surface)
  {
    let band_count = ((scale.maximum - scale.minimum) / scale.major_unit)
      .ceil()
      .max(1.0) as usize;
    return (0..band_count)
      .filter_map(|band_index| {
        let lower = scale.minimum + band_index as f64 * scale.major_unit;
        let upper = (lower + scale.major_unit).min(scale.maximum);
        (upper > lower).then(|| CartesianLegendEntry {
          label: Cow::Owned(format!(
            "{}-{}",
            format_axis_value(lower, scale.major_unit),
            format_axis_value(upper, scale.major_unit)
          )),
          color: Some(surface_band_color(style, 0, band_index as u32)),
          kind: ChartSeriesKind::Surface,
          series_index: None,
          point_index: None,
          trendline_index: None,
        })
      })
      .enumerate()
      .filter_map(|(entry_index, entry)| {
        chart
          .deleted_legend_entry_indices
          .binary_search(&entry_index)
          .is_err()
          .then_some(entry)
      })
      .collect();
  }
  if chart.vary_colors_by_point {
    let Some(series) = chart.series.first() else {
      return Vec::new();
    };
    let mut entries = Vec::new();
    let mut entry_index = 0;
    for (point_index, category) in chart.categories.iter().enumerate() {
      if chart
        .deleted_legend_entry_indices
        .binary_search(&entry_index)
        .is_err()
      {
        entries.push(CartesianLegendEntry {
          label: Cow::Borrowed(category),
          color: chart_point_color(style, 0, point_index)
            .or_else(|| style.series_colors.first().copied()),
          kind: series.kind,
          series_index: Some(0),
          point_index: Some(point_index),
          trendline_index: None,
        });
      }
      entry_index += 1;
    }
    for (trendline_index, trendline) in series.trendlines.iter().enumerate() {
      if chart
        .deleted_legend_entry_indices
        .binary_search(&entry_index)
        .is_err()
      {
        entries.push(CartesianLegendEntry {
          label: trendline_legend_title(trendline, &series.name, chart.ui_language.as_deref()),
          color: style.series_colors.first().copied(),
          kind: series.kind,
          series_index: Some(0),
          point_index: None,
          trendline_index: Some(trendline_index),
        });
      }
      entry_index += 1;
    }
    return entries;
  }
  let mut entry_index = 0;
  let mut groups = Vec::with_capacity(chart.series.len());
  for (series_index, series) in chart.series.iter().enumerate() {
    let mut group = Vec::with_capacity(1 + series.trendlines.len());
    if chart
      .deleted_legend_entry_indices
      .binary_search(&entry_index)
      .is_err()
    {
      group.push(CartesianLegendEntry {
        label: Cow::Borrowed(&series.name),
        color: style.series_colors.get(series_index).copied(),
        kind: series.kind,
        series_index: Some(series_index),
        point_index: None,
        trendline_index: None,
      });
    }
    entry_index += 1;
    for (trendline_index, trendline) in series.trendlines.iter().enumerate() {
      if chart
        .deleted_legend_entry_indices
        .binary_search(&entry_index)
        .is_err()
      {
        group.push(CartesianLegendEntry {
          label: trendline_legend_title(trendline, &series.name, chart.ui_language.as_deref()),
          color: style.series_colors.get(series_index).copied(),
          kind: series.kind,
          series_index: Some(series_index),
          point_index: None,
          trendline_index: Some(trendline_index),
        });
      }
      entry_index += 1;
    }
    groups.push(group);
  }
  if cartesian_legend_reverses_series(chart) {
    groups.reverse();
  }
  groups.into_iter().flatten().collect()
}

fn cartesian_legend_reverses_series(chart: &ClusteredColumnChart<'_>) -> bool {
  let horizontal_bar = chart
    .series
    .iter()
    .all(|series| series.kind == ChartSeriesKind::Bar);
  let stacked = chart.series.iter().all(|series| {
    matches!(
      series.grouping,
      ChartSeriesGrouping::Stacked | ChartSeriesGrouping::PercentStacked
    )
  });
  if horizontal_bar {
    // LibreOffice VSeriesPlotter reverses swapped-axis legend entries unless
    // the series stack in the Y direction.
    !stacked
  } else {
    // Side legends follow the visible top-to-bottom stack, which is the
    // reverse of the authored series order. Top/bottom legends retain the
    // authored left-to-right sequence.
    stacked
      && matches!(
        chart.legend_position,
        Some(
          ChartLegendPosition::Left | ChartLegendPosition::Right | ChartLegendPosition::TopRight
        )
      )
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
  display_unit: f64,
) -> Vec<(f64, String)> {
  let display_unit = if display_unit.is_finite() && display_unit > 0.0 {
    display_unit
  } else {
    1.0
  };
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
            || crate::render::chart::format_chart_number(value / display_unit, None),
            |format| crate::render::chart::format_chart_number(value / display_unit, Some(format)),
          ),
        )
      })
      .collect();
  }
  let count = axis_interval_count(minimum, maximum, unit, 10_000);
  (0..=count)
    .map(|index| {
      let value = minimum + index as f64 * unit;
      (
        value,
        format_code.map_or_else(
          || format_axis_value(value / display_unit, unit / display_unit),
          |format| crate::render::chart::format_chart_number(value / display_unit, Some(format)),
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
  rich_text_styles: &[TextStyle],
  text_frame: ResolvedDataLabelTextFrame,
) {
  if !label.rich_text_runs.is_empty() {
    let line_dimensions =
      data_label_rich_text_line_dimensions(metrics, label, style, rich_text_styles);
    let content_width = line_dimensions
      .iter()
      .map(|(width, _)| *width)
      .fold(0.0_f32, f32::max);
    let content_height = line_dimensions
      .iter()
      .map(|(_, height)| *height)
      .sum::<f32>();
    let available_width = text_frame.inner_width().unwrap_or(content_width);
    let available_height = text_frame.inner_height().unwrap_or(content_height);
    let mut line_y = y
      + text_frame
        .outer_height
        .map_or(0.0, |_| text_frame.insets.top)
      + single_line_vertical_anchor_offset(
        label
          .text_body_properties
          .and_then(|properties| properties.anchor),
        available_height,
        content_height,
      );
    let text_left = x
      + text_frame
        .outer_width
        .map_or(0.0, |_| text_frame.insets.left);
    for (line_index, (line_width, line_height_pt)) in line_dimensions.iter().copied().enumerate() {
      let mut run_x = text_left + (available_width - line_width).max(0.0) * 0.5;
      for (run_index, run) in label.rich_text_runs.iter().enumerate() {
        if run.line_index != line_index {
          continue;
        }
        let run_style = rich_text_styles.get(run_index).unwrap_or(style);
        let run_height = line_height(run_style);
        push_text_with_segmentation(
          items,
          run_x,
          line_y + (line_height_pt - run_height).max(0.0),
          run.text.clone(),
          run_style.clone(),
          data_label_pdf_text_segmentation(&run.text),
        );
        run_x += metrics.measure_text(&run.text, run_style);
      }
      line_y += line_height_pt;
    }
    return;
  }

  let lines = plain_data_label_lines(metrics, label, style, text_frame.inner_width());
  let content_width = lines
    .iter()
    .map(|line| metrics.measure_text(line, style))
    .fold(0.0_f32, f32::max);
  let content_height = line_height(style) * lines.len().max(1) as f32;
  let available_width = text_frame.inner_width().unwrap_or(content_width);
  let available_height = text_frame.inner_height().unwrap_or(content_height);
  let text_left = x
    + text_frame
      .outer_width
      .map_or(0.0, |_| text_frame.insets.left);
  let text_top = y
    + text_frame
      .outer_height
      .map_or(0.0, |_| text_frame.insets.top)
    + single_line_vertical_anchor_offset(
      label
        .text_body_properties
        .and_then(|properties| properties.anchor),
      available_height,
      content_height,
    );

  if lines.len() > 1 {
    for (index, line) in lines.iter().enumerate() {
      let line_width = metrics.measure_text(line, style);
      push_text_with_segmentation(
        items,
        text_left + (available_width - line_width).max(0.0) * 0.5,
        text_top + index as f32 * line_height(style),
        line.clone(),
        style.clone(),
        data_label_pdf_text_segmentation(line),
      );
    }
    return;
  }

  let one_line_x = text_left + (available_width - content_width).max(0.0) * 0.5;
  if label.text_components.len() <= 1 {
    push_text_with_segmentation(
      items,
      one_line_x,
      text_top,
      label.text.clone(),
      style.clone(),
      data_label_pdf_text_segmentation(&label.text),
    );
    return;
  }

  let mut component_x = one_line_x;
  let painted_separator = label.separator.trim_end();
  for (index, component) in label.text_components.iter().enumerate() {
    let is_last = index + 1 == label.text_components.len();
    let painted_text = if is_last {
      component.clone()
    } else {
      format!("{component}{painted_separator}")
    };
    let segmentation = data_label_pdf_text_segmentation(&painted_text);
    push_text_with_segmentation(
      items,
      component_x,
      text_top,
      painted_text,
      style.clone(),
      segmentation,
    );
    if !is_last {
      component_x += metrics.measure_text(&format!("{component}{}", label.separator), style);
    }
  }
}

fn data_label_text_dimensions(
  metrics: &mut TextMetrics,
  label: &crate::render::chart::ClusteredColumnDataLabel<'_>,
  style: &TextStyle,
  rich_text_styles: &[TextStyle],
  text_frame: ResolvedDataLabelTextFrame,
) -> (f32, f32) {
  if !label.rich_text_runs.is_empty() {
    let dimensions = data_label_rich_text_line_dimensions(metrics, label, style, rich_text_styles);
    let content_width = dimensions
      .iter()
      .map(|(width, _)| *width)
      .fold(0.0_f32, f32::max);
    let content_height = dimensions.iter().map(|(_, height)| *height).sum();
    return (
      text_frame.outer_width.unwrap_or(content_width),
      text_frame.outer_height.unwrap_or(content_height),
    );
  }
  let lines = plain_data_label_lines(metrics, label, style, text_frame.inner_width());
  let content_width = lines
    .iter()
    .map(|line| metrics.measure_text(line, style))
    .fold(0.0_f32, f32::max);
  let content_height = line_height(style) * lines.len().max(1) as f32;
  (
    text_frame.outer_width.unwrap_or(content_width),
    text_frame.outer_height.unwrap_or(content_height),
  )
}

fn plain_data_label_lines(
  metrics: &mut TextMetrics,
  label: &crate::render::chart::ClusteredColumnDataLabel<'_>,
  style: &TextStyle,
  maximum_width: Option<f32>,
) -> Vec<String> {
  let mut lines = if label.text_components.len() > 1
    && (label.separator.contains('\r') || label.separator.contains('\n'))
  {
    label.text_components.clone()
  } else if label.text_components.len() > 1 {
    let pieces = label
      .text_components
      .iter()
      .enumerate()
      .map(|(index, component)| {
        if index + 1 == label.text_components.len() {
          component.clone()
        } else {
          format!("{component}{}", label.separator)
        }
      })
      .collect::<Vec<_>>();
    if let Some(maximum_width) = maximum_width.filter(|width| *width > 0.0) {
      let mut packed = Vec::new();
      let mut current = String::new();
      for piece in pieces {
        let combined = format!("{current}{piece}");
        if !current.is_empty() && metrics.measure_text(&combined, style) > maximum_width {
          packed.push(current);
          current = piece;
        } else {
          current = combined;
        }
      }
      if !current.is_empty() {
        packed.push(current);
      }
      packed
    } else {
      vec![label.text.clone()]
    }
  } else {
    vec![label.text.clone()]
  };

  let Some(maximum_width) = maximum_width.filter(|width| *width > 0.0) else {
    return lines;
  };
  let mut wrapped = Vec::new();
  for line in lines.drain(..) {
    if metrics.measure_text(&line, style) <= maximum_width || !line.contains(char::is_whitespace) {
      wrapped.push(line);
      continue;
    }
    let mut current = String::new();
    for word in line.split_whitespace() {
      let combined = if current.is_empty() {
        word.to_string()
      } else {
        format!("{current} {word}")
      };
      if !current.is_empty() && metrics.measure_text(&combined, style) > maximum_width {
        wrapped.push(current);
        current = word.to_string();
      } else {
        current = combined;
      }
    }
    if !current.is_empty() {
      wrapped.push(current);
    }
  }
  if wrapped.is_empty() {
    wrapped.push(String::new());
  }
  wrapped
}

fn data_label_rich_text_line_dimensions(
  metrics: &mut TextMetrics,
  label: &crate::render::chart::ClusteredColumnDataLabel<'_>,
  style: &TextStyle,
  rich_text_styles: &[TextStyle],
) -> Vec<(f32, f32)> {
  let line_count = label
    .rich_text_runs
    .iter()
    .map(|run| run.line_index)
    .max()
    .map_or(1, |index| index + 1);
  let mut dimensions = vec![(0.0_f32, line_height(style)); line_count];
  for (run_index, run) in label.rich_text_runs.iter().enumerate() {
    let run_style = rich_text_styles.get(run_index).unwrap_or(style);
    dimensions[run.line_index].0 += metrics.measure_text(&run.text, run_style);
    dimensions[run.line_index].1 = dimensions[run.line_index].1.max(line_height(run_style));
  }
  dimensions
}

fn push_text(items: &mut Vec<PageItem>, x: f32, y: f32, text: String, style: TextStyle) {
  push_text_with_rotation_center(items, x, y, text, style, None);
}

fn push_text_with_rotation_center(
  items: &mut Vec<PageItem>,
  x: f32,
  y: f32,
  text: String,
  style: TextStyle,
  rotation_center_pt: Option<(f32, f32)>,
) {
  push_text_with_segmentation_and_rotation_center(
    items,
    x,
    y,
    text,
    style,
    PdfTextSegmentation::Line,
    rotation_center_pt,
  );
}

fn push_text_with_segmentation(
  items: &mut Vec<PageItem>,
  x: f32,
  y: f32,
  text: String,
  style: TextStyle,
  pdf_text_segmentation: PdfTextSegmentation,
) {
  push_text_with_segmentation_and_rotation_center(
    items,
    x,
    y,
    text,
    style,
    pdf_text_segmentation,
    None,
  );
}

fn push_text_with_segmentation_and_rotation_center(
  items: &mut Vec<PageItem>,
  x: f32,
  y: f32,
  text: String,
  style: TextStyle,
  pdf_text_segmentation: PdfTextSegmentation,
  rotation_center_pt: Option<(f32, f32)>,
) {
  items.push(PageItem::Text(TextItem {
    x_pt: x,
    y_pt: y,
    line_height_pt: line_height(&style),
    paint_clip: None,
    discard_if_horizontally_clipped: false,
    text,
    style,
    rotation_center_pt,
    hyperlink_url: None,
    form_widget_id: None,
    paragraph_bidi: false,
    preserve_text_portion: true,
    pdf_text_segmentation,
    source_path: Vec::new(),
  }));
}

fn data_label_pdf_text_segmentation(text: &str) -> PdfTextSegmentation {
  if text.as_bytes().windows(3).any(|window| {
    matches!(window[0], b'E' | b'e') && window[1] == b'-' && window[2].is_ascii_digit()
  }) {
    PdfTextSegmentation::WordLine
  } else {
    PdfTextSegmentation::Line
  }
}

fn push_data_table_text(items: &mut Vec<PageItem>, x: f32, y: f32, text: String, style: TextStyle) {
  items.push(PageItem::Text(TextItem {
    x_pt: x,
    y_pt: y,
    line_height_pt: line_height(&style),
    paint_clip: None,
    discard_if_horizontally_clipped: true,
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
  use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_chart as c;
  use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
  use ooxmlsdk::sdk::SdkType;

  use super::{
    Chart3DView, ChartLayoutProfile, PlotRect, SurfaceVertex, bind_chart_gradient_to_bounds,
    cardinal_cubic_controls, cartesian_3d_projection, cartesian_legend_reverses_series,
    category_axis_text_rotation_degrees, category_axis_text_rotation_degrees_for_layout,
    category_axis_text_rotation_is_supported, clip_surface_polygon,
    data_label_pdf_text_segmentation, format_axis_value, lower_3d_extruded_polygon,
    lower_3d_line_stripes, maximum_auto_main_increment_count, push_chart_data_rect,
    sample_cardinal_chart_line, series_axis_label_rhythm, series_category_display_index,
    single_line_vertical_anchor_offset, word_fixed_chart_data_edge, word_fixed_chart_value_edge,
  };
  use crate::model::{PageItem, PdfTextSegmentation, RgbColor, common_rect};
  use crate::render::chart::ChartSeriesKind;

  #[test]
  fn axis_values_do_not_expose_binary_float_artifacts() {
    let value_with_binary_artifact = f64::from_bits(4.4_f64.to_bits() + 1);
    assert_eq!(format_axis_value(value_with_binary_artifact, 0.2), "4.4");
    assert_eq!(format_axis_value(6.0, 1.0), "6");
  }

  #[test]
  fn column_and_horizontal_bar_categories_follow_their_physical_axes() {
    assert_eq!(
      series_category_display_index(false, ChartSeriesKind::Column, 0, 4),
      0
    );
    assert_eq!(
      series_category_display_index(false, ChartSeriesKind::Bar, 0, 4),
      3
    );
    assert_eq!(
      series_category_display_index(true, ChartSeriesKind::Column, 0, 4),
      3
    );
    assert_eq!(
      series_category_display_index(true, ChartSeriesKind::Bar, 0, 4),
      0
    );
  }

  #[test]
  fn word_fixed_output_quantizes_data_edges_but_preserves_the_zero_axis() {
    assert!(
      (word_fixed_chart_data_edge(108.746_23, ChartLayoutProfile::Word) - 108.72).abs() < 0.001
    );
    assert!(
      (word_fixed_chart_data_edge(144.257_37, ChartLayoutProfile::Word) - 144.24).abs() < 0.001
    );
    assert_eq!(
      word_fixed_chart_data_edge(108.746_23, ChartLayoutProfile::PowerPoint),
      108.746_23
    );
    assert_eq!(
      word_fixed_chart_value_edge(298.949_92, 0.0, ChartLayoutProfile::Word),
      298.949_92
    );
  }

  #[test]
  fn chart_data_rectangle_binds_gradient_and_keeps_authored_dash() {
    let fill = crate::common::ShapeStyleValue::Paint(crate::common::Fill::Gradient(
      crate::common::GradientFill {
        path: Some(crate::common::GradientPath {
          kind: crate::common::GradientPathKind::Rectangle,
          fill_to: crate::common::RelativeRect::default(),
          transform: crate::common::Transform::default(),
          mirror_tile: false,
        }),
        ..Default::default()
      },
    ));
    let stroke = crate::common::ShapeStyleValue::Paint(crate::common::Stroke {
      width: crate::common::Pt(6.0),
      color: crate::common::Color {
        r: 0xC0,
        g: 0,
        b: 0,
        a: 0xFF,
      },
      preset_dash: Some(crate::common::StrokeDashPreset::SystemDash),
      ..Default::default()
    });
    let mut items = Vec::new();

    push_chart_data_rect(
      &mut items,
      10.0,
      20.0,
      30.0,
      40.0,
      RgbColor::default(),
      Some(&fill),
      Some(&stroke),
      1.0,
    );

    let [PageItem::Path(path)] = items.as_slice() else {
      panic!("expected one styled chart path");
    };
    let crate::common::Fill::Gradient(gradient) = &path.fill else {
      panic!("expected gradient fill");
    };
    assert_eq!(gradient.definition_bounds, Some(path.bounds));
    let gradient_path = gradient.path.expect("bound path gradient");
    assert_eq!(gradient_path.transform.m11, 30.0);
    assert_eq!(gradient_path.transform.m22, 40.0);
    assert_eq!(gradient_path.transform.dx.0, 10.0);
    assert_eq!(gradient_path.transform.dy.0, 20.0);
    let stroke = path.stroke.as_ref().expect("series outline");
    assert_eq!(stroke.width.0, 6.0);
    assert_eq!(
      stroke.preset_dash,
      Some(crate::common::StrokeDashPreset::SystemDash)
    );
  }

  #[test]
  fn chart_circle_gradient_keeps_tile_geometry_when_bound_to_plot() {
    let gradient = crate::common::GradientFill {
      path: Some(crate::common::GradientPath {
        kind: crate::common::GradientPathKind::Circle,
        fill_to: crate::common::RelativeRect {
          left: 0.5,
          top: 0.5,
          right: 0.5,
          bottom: 0.5,
        },
        transform: crate::common::Transform {
          m11: 2.0,
          m22: 2.0,
          dx: crate::common::Pt(-1.0),
          dy: crate::common::Pt(-1.0),
          ..Default::default()
        },
        mirror_tile: false,
      }),
      ..Default::default()
    };
    let bounds = common_rect(10.0, 20.0, 30.0, 40.0);

    let bound = bind_chart_gradient_to_bounds(&gradient, bounds);
    let path = bound.path.expect("bound path gradient");

    assert_eq!(path.transform.m11, 100.0);
    assert_eq!(path.transform.m22, 100.0);
    assert_eq!(path.transform.dx.0, -40.0);
    assert_eq!(path.transform.dy.0, -30.0);
    let focus_x = path.transform.dx.0 + path.transform.m11 * path.fill_to.left;
    let focus_y = path.transform.dy.0 + path.transform.m22 * path.fill_to.top;
    assert_eq!(focus_x, bounds.origin.x.0);
    assert_eq!(focus_y, bounds.origin.y.0);
  }

  #[test]
  fn scientific_data_labels_isolate_negative_exponents_but_axis_text_does_not() {
    assert_eq!(
      data_label_pdf_text_segmentation("4.3E-09"),
      PdfTextSegmentation::WordLine
    );
    assert_eq!(
      data_label_pdf_text_segmentation("4.3E+09"),
      PdfTextSegmentation::Line
    );
    assert_eq!(
      data_label_pdf_text_segmentation("Series-1"),
      PdfTextSegmentation::Line
    );
  }

  #[test]
  fn axis_budget_uses_the_generated_label_shape_extent() {
    let generated_label_extent = 10.0 * super::TEXT_LINE_HEIGHT_SCALE;
    assert_eq!(
      maximum_auto_main_increment_count(152.25, generated_label_extent,),
      10
    );
    assert_eq!(
      maximum_auto_main_increment_count(90.0, generated_label_extent,),
      6
    );
    assert_eq!(
      maximum_auto_main_increment_count(90.0, generated_label_extent,),
      6
    );
  }

  #[test]
  fn side_legend_reverses_a_vertical_stack_but_bottom_legend_does_not() {
    let chart_space = c::ChartSpace::from_bytes(
      br#"<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"><c:chart><c:plotArea><c:lineChart><c:grouping val="stacked"/><c:ser><c:idx val="0"/><c:order val="0"/><c:tx><c:v>Series 1</c:v></c:tx><c:val><c:numLit><c:pt idx="0"><c:v>1</c:v></c:pt></c:numLit></c:val></c:ser><c:ser><c:idx val="1"/><c:order val="1"/><c:tx><c:v>Series 2</c:v></c:tx><c:val><c:numLit><c:pt idx="0"><c:v>2</c:v></c:pt></c:numLit></c:val></c:ser><c:axId val="1"/><c:axId val="2"/></c:lineChart></c:plotArea><c:legend><c:legendPos val="r"/></c:legend></c:chart></c:chartSpace>"#,
    )
    .expect("chart space");
    let mut chart =
      crate::render::chart::cartesian_chart_for_ui_language(&chart_space, None).expect("chart");

    assert!(cartesian_legend_reverses_series(&chart));
    chart.legend_position = Some(crate::render::chart::ChartLegendPosition::Bottom);
    assert!(!cartesian_legend_reverses_series(&chart));
  }

  #[test]
  fn smooth_chart_line_uses_office_cardinal_controls() {
    let points = [
      (29.4, 495.12, 0),
      (69.96, 425.28, 1),
      (110.52, 425.28, 2),
      (151.08, 442.74, 3),
    ];

    let (control1, control2) = cardinal_cubic_controls(&points, 1);

    assert!((control1.0 - 83.48).abs() < 0.001);
    assert!((control1.1 - 413.64).abs() < 0.001);
    assert!((control2.0 - 97.0).abs() < 0.001);
    assert!((control2.1 - 422.37).abs() < 0.001);
  }

  #[test]
  fn three_dimensional_projection_rotates_and_fits_the_complete_scene() {
    let plot = PlotRect {
      left: 10.0,
      top: 20.0,
      width: 240.0,
      height: 120.0,
    };
    let default = cartesian_3d_projection(
      Chart3DView {
        right_angle_axes: true,
        ..Chart3DView::default()
      },
      plot,
      ChartLayoutProfile::Word,
      1.0,
      false,
    );
    let front = default.project(130.0, 80.0, 0.0);
    let back = default.project(130.0, 80.0, 1.0);
    assert!(back.0 > front.0);
    assert!(back.1 < front.1);
    let left = default.project(plot.left, plot.top + plot.height, 0.0);
    let right = default.project(plot.left + plot.width, plot.top + plot.height, 0.0);
    assert!(right.0 > left.0);
    assert!((right.1 - left.1).abs() < 0.001);

    let reversed = cartesian_3d_projection(
      Chart3DView {
        rotate_x_deg: 20.0,
        rotate_y_deg: 170.0,
        perspective_half_degrees: 0.0,
        ..Chart3DView::default()
      },
      plot,
      ChartLayoutProfile::Word,
      1.0,
      false,
    );
    let front = reversed.project(130.0, 80.0, 0.0);
    let back = reversed.project(130.0, 80.0, 1.0);
    assert!(back.0 > front.0);
    assert!(back.1 > front.1);
    assert!(reversed.vertical_axis_length(plot, true) < plot.height);
  }

  #[test]
  fn three_dimensional_line_and_area_geometry_are_filled_depth_faces() {
    let projection = cartesian_3d_projection(
      Chart3DView {
        right_angle_axes: true,
        ..Chart3DView::default()
      },
      PlotRect {
        left: 0.0,
        top: 0.0,
        width: 40.0,
        height: 30.0,
      },
      ChartLayoutProfile::Word,
      1.0,
      false,
    );
    let color = RgbColor {
      r: 68,
      g: 114,
      b: 196,
    };
    let mut line_items = Vec::new();
    lower_3d_line_stripes(
      &mut line_items,
      &[(10.0, 20.0), (30.0, 15.0)],
      projection,
      0.25,
      0.75,
      color,
      None,
    );
    assert!(matches!(
      line_items.as_slice(),
      [PageItem::Path(path)] if path.closed && path.points.len() == 4
    ));

    let mut area_items = Vec::new();
    lower_3d_extruded_polygon(
      &mut area_items,
      &[(10.0, 20.0), (30.0, 15.0), (30.0, 40.0), (10.0, 40.0)],
      projection,
      0.25,
      0.75,
      color,
      None,
    );
    // Back + one side for every polygon edge + front.
    assert_eq!(area_items.len(), 6);
    assert!(
      area_items
        .iter()
        .all(|item| matches!(item, PageItem::Path(path) if path.closed && path.points.len() == 4))
    );
  }

  #[test]
  fn smooth_three_dimensional_lines_sample_the_same_cardinal_curve() {
    let points = [(0.0, 0.0, 0), (10.0, 10.0, 1), (20.0, 0.0, 2)];
    let sampled = sample_cardinal_chart_line(&points, 4);

    assert_eq!(sampled.len(), 9);
    assert_eq!(sampled.first().copied(), Some((0.0, 0.0)));
    assert_eq!(sampled.last().copied(), Some((20.0, 0.0)));
    assert!((sampled[2].1 - 6.25).abs() < 0.001);
  }

  #[test]
  fn surface_cells_are_clipped_at_value_band_boundaries() {
    let cell = [
      SurfaceVertex {
        x: 0.0,
        value: 0.0,
        depth_ratio: 0.0,
      },
      SurfaceVertex {
        x: 10.0,
        value: 3.0,
        depth_ratio: 0.0,
      },
      SurfaceVertex {
        x: 10.0,
        value: 4.0,
        depth_ratio: 1.0,
      },
      SurfaceVertex {
        x: 0.0,
        value: 1.0,
        depth_ratio: 1.0,
      },
    ];
    let above = clip_surface_polygon(&cell, 1.0, true);
    let band = clip_surface_polygon(&above, 2.0, false);

    assert!(band.len() >= 3);
    assert!(
      band
        .iter()
        .all(|point| point.value >= 1.0 && point.value <= 2.0)
    );
  }

  #[test]
  fn series_axis_increases_label_rhythm_when_depth_ticks_overlap() {
    assert_eq!(
      series_axis_label_rhythm(25.0, 10.0, &[(0.0, 0.0), (16.0, 0.0), (32.0, 0.0)], None),
      2
    );
    assert_eq!(
      series_axis_label_rhythm(25.0, 10.0, &[(0.0, 0.0), (32.0, 0.0), (64.0, 0.0)], None),
      1
    );
    assert_eq!(
      series_axis_label_rhythm(25.0, 10.0, &[(0.0, 0.0), (16.0, 12.0), (32.0, 24.0)], None),
      1
    );
    assert_eq!(
      series_axis_label_rhythm(25.0, 10.0, &[(0.0, 0.0), (16.0, 0.0), (32.0, 0.0)], Some(3)),
      3
    );
  }

  #[test]
  fn category_axis_rotation_recovers_out_of_range_values_per_host() {
    let properties = a::BodyProperties {
      rotation: Some(-60_000_000),
      ..Default::default()
    };
    let text_properties =
      ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_chart::TextProperties {
        body_properties: Box::new(properties),
        ..Default::default()
      };

    assert!(category_axis_text_rotation_is_supported(
      Some(&text_properties),
      8
    ));
    assert_eq!(
      category_axis_text_rotation_degrees(Some(&text_properties)),
      0.0
    );
    assert_eq!(
      category_axis_text_rotation_degrees_for_layout(
        ChartLayoutProfile::Excel,
        Some(&text_properties),
        100.0,
        10,
        20.0,
        10.0,
      ),
      -90.0
    );
    assert_eq!(
      category_axis_text_rotation_degrees_for_layout(
        ChartLayoutProfile::Excel,
        Some(&text_properties),
        100.0,
        2,
        20.0,
        10.0,
      ),
      0.0
    );
  }

  #[test]
  fn linear_trendline_equation_uses_office_terms() {
    assert_eq!(
      super::trendline_equation(c::TrendlineValues::Linear, 1.0, 1.0),
      "y = x + 1"
    );
  }

  #[test]
  fn single_line_chart_text_honors_drawingml_vertical_anchor() {
    let container_height = 19.44;
    let line_height = 10.8;

    assert_eq!(
      single_line_vertical_anchor_offset(
        Some(a::TextAnchoringTypeValues::Top),
        container_height,
        line_height,
      ),
      0.0
    );
    assert!(
      (single_line_vertical_anchor_offset(
        Some(a::TextAnchoringTypeValues::Center),
        container_height,
        line_height,
      ) - 4.32)
        .abs()
        < 0.001
    );
    assert!(
      (single_line_vertical_anchor_offset(
        Some(a::TextAnchoringTypeValues::Bottom),
        container_height,
        line_height,
      ) - 8.64)
        .abs()
        < 0.001
    );
    assert!(
      (single_line_vertical_anchor_offset(
        Some(a::TextAnchoringTypeValues::Distributed),
        container_height,
        line_height,
      ) - 4.32)
        .abs()
        < 0.001
    );
    assert_eq!(
      single_line_vertical_anchor_offset(
        Some(a::TextAnchoringTypeValues::Justified),
        container_height,
        line_height,
      ),
      0.0
    );
  }
}
