use crate::model::{
  BorderStyle, LineItem, LineItemKind, PageItem, PdfTextSegmentation, RectItem, RgbColor, TextItem,
  TextStyle, common_point, common_rect, common_rgb,
};
use crate::render::chart::{
  Chart3DView, ChartLegendPosition, ChartSeriesGrouping, ChartSeriesKind, ChartTitleText,
  ClusteredColumnChart, LinearAxisScaleOptions, PieChartModel, RadialChartKind, SurfaceChartGroup,
  clustered_column_slot, date_axis_minor_tick_positions, date_axis_ticks, linear_axis_scale,
  linear_axis_scale_with_options, value_axis_display_unit, value_axis_display_unit_label_text,
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
// LibreOffice chart2 `AreaChart.cxx` passes a fixed 100 mm100 offset to
// `createDataLabel` after moving a top label above the marker.
const CARTESIAN_DATA_LABEL_OFFSET_PT: f32 = 72.0 / 25.4;
// `BarChart.cxx` raises the corresponding offset to 260 mm100 for every
// non-centered 3-D bar/column label so it clears the projected marker face.
const BAR_3D_DATA_LABEL_OFFSET_PT: f32 = 2.6 * 72.0 / 25.4;
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
  pub modern_excel_profile: bool,
  pub stroke_scale: f32,
  pub has_explicit_title: bool,
  pub title: TextStyle,
  pub title_fill_color: Option<RgbColor>,
  pub label: TextStyle,
  pub category_label: TextStyle,
  pub value_label: TextStyle,
  pub series_label: TextStyle,
  pub data_label: TextStyle,
  pub data_label_styles: Vec<Vec<Option<TextStyle>>>,
  pub gridline_color: RgbColor,
  pub value_gridline_width_pt: Option<f32>,
  pub axis_line_width_pt: Option<f32>,
  pub category_major_gridline: Option<(RgbColor, f32)>,
  pub category_minor_gridline: Option<(RgbColor, f32)>,
  pub series_colors: Vec<RgbColor>,
  pub series_styles: Vec<crate::common::ShapeStyle<'static>>,
  pub series_point_styles: Vec<Vec<Option<crate::common::ShapeStyle<'static>>>>,
  /// Surface-chart value-band colors keyed by `c:bandFmt/c:idx`, one vector
  /// per surface plot-area group.
  pub surface_band_colors: Vec<Vec<(u32, RgbColor)>>,
  pub data_label_fill_colors: Vec<Vec<Option<RgbColor>>>,
  pub chart_area_style: crate::common::ShapeStyle<'static>,
  pub plot_area_style: crate::common::ShapeStyle<'static>,
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
  profile: ChartLayoutProfile,
  available_axis_length_pt: f32,
  label_font_size_pt: f32,
  has_bubble_series: bool,
  is_3d: bool,
) -> usize {
  // LibreOffice VCartesianAxis::estimateMaximumAutoMainIncrementCount divides
  // the final axis length by the maximum recorded label shape extent, then
  // ScaleAutomatism clamps that result to 2..10. DrawingML chart label shapes
  // include their internal leading/margins; their ordinary vertical extent is
  // about 1.2 em. Bubble plots reserve an additional marker envelope.
  match profile {
    ChartLayoutProfile::Word => {
      if is_3d {
        // Word resolves ordinary 3-D value axes before projected label
        // extents are stable. Keep the unmeasured ten-increment ceiling;
        // family-specific scale rules can still lower it below.
        return 10;
      }
      let label_extent = label_font_size_pt.max(1.0) * if has_bubble_series { 3.0 } else { 1.2 };
      (available_axis_length_pt / label_extent)
        .floor()
        .clamp(2.0, 10.0) as usize
    }
    ChartLayoutProfile::PowerPoint | ChartLayoutProfile::Excel => 10,
  }
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
  pub label: TextStyle,
  pub data_label: TextStyle,
  pub point_colors: Vec<RgbColor>,
  pub point_styles: Vec<crate::common::ShapeStyle<'static>>,
  pub data_label_fill_colors: Vec<Option<RgbColor>>,
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

pub(crate) fn solid_chart_point_styles(
  colors: Vec<Vec<Option<RgbColor>>>,
) -> Vec<Vec<Option<crate::common::ShapeStyle<'static>>>> {
  colors
    .into_iter()
    .map(|points| {
      points
        .into_iter()
        .map(|color| color.map(|color| solid_chart_shape_style(Some(color), None)))
        .collect()
    })
    .collect()
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
  let axis_scales = cartesian_axis_scales(chart, category_count, 10);
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
  let title_line_height = line_height(&style.title);
  let category_label_line_height = line_height(&style.category_label);
  let value_label_line_height = line_height(&style.value_label);
  let label_line_height = category_label_line_height
    .max(value_label_line_height)
    .max(line_height(&style.label));
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
  let has_derived_single_series_side_title_layout =
    excel_derived_single_series_side_title_layout(chart, style);
  let has_explicit_single_series_side_title_layout =
    excel_explicit_single_series_side_title_layout(chart, style);
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
  let date_ticks = category_tick_labels_visible
    .then(|| date_axis_ticks(chart))
    .flatten();
  let category_label_lines: Vec<Vec<String>> = if category_tick_labels_visible {
    let slot_width = frame.width_pt / category_count as f32 * 0.9;
    if let Some(ticks) = date_ticks.as_ref() {
      ticks
        .iter()
        .map(|tick| wrap_chart_label(&tick.text, slot_width, &style.category_label, &mut metrics))
        .collect()
    } else {
      chart
        .categories
        .iter()
        .map(|category| wrap_chart_label(category, slot_width, &style.category_label, &mut metrics))
        .collect()
    }
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
  } else if style.layout_profile == ChartLayoutProfile::Excel
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
    + frame.height_pt * word_no_legend_adjustment.category_top_ratio
    + frame.height_pt * word_titled_bottom_adjustment.category_top_ratio
    + frame.height_pt * excel_side_adjustment.category_top_ratio
    + frame.height_pt * excel_title_only_adjustment.category_top_ratio
    + frame.height_pt * excel_untitled_side_adjustment.category_top_ratio
    + frame.height_pt * excel_vary_colors_data_table_adjustment.category_top_ratio;
  if has_bottom_legend
    && !horizontal_bar_only
    && chart.category_axis_title.is_some()
    && chart.data_table.is_none()
  {
    // Office's automatic bottom stack is category labels, category-axis
    // title, then legend. The title line and its two inter-band half-leading
    // gaps must be removed from the plot/category budget; otherwise all three
    // text objects overlap and PDF extraction merges independent lines.
    category_top -= line_height(&style.label) * 2.25;
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
    + frame.height_pt * word_no_legend_adjustment.plot_top_ratio
    + frame.height_pt * word_titled_bottom_adjustment.plot_top_ratio
    + frame.height_pt * excel_side_adjustment.plot_top_ratio
    + frame.height_pt * excel_title_only_adjustment.plot_top_ratio
    + frame.height_pt * excel_untitled_side_adjustment.plot_top_ratio
    + frame.height_pt * excel_vary_colors_data_table_adjustment.plot_top_ratio;
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
  plot_bottom += frame.height_pt
    * (word_side_adjustment.plot_bottom_ratio
      + word_no_legend_adjustment.plot_bottom_ratio
      + word_titled_bottom_adjustment.plot_bottom_ratio);
  plot_bottom += frame.height_pt * excel_vary_colors_data_table_adjustment.plot_bottom_ratio;
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
      let format_code = axis
        .numbering_format
        .as_ref()
        .map(|format| format.format_code.as_str())
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
  let mut tick_left = tick_left
    + frame.height_pt * word_titled_bottom_adjustment.tick_left_ratio
    + if has_indexed_scatter_automatic_layout {
      frame.height_pt * profiles::EXCEL_AUTOMATIC_INDEXED_SCATTER.tick_left_ratio
    } else if has_legacy_indexed_scatter_layout {
      frame.height_pt * profiles::EXCEL_LEGACY_INDEXED_SCATTER.tick_left_ratio
    } else {
      0.0
    }
    + frame.height_pt * excel_vary_colors_data_table_adjustment.tick_left_ratio;
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
    - right_value_axis_band_width;
  plot_left += frame.height_pt
    * (word_side_adjustment.plot_left_ratio
      + word_no_legend_adjustment.plot_left_ratio
      + word_titled_bottom_adjustment.plot_left_ratio);
  plot_right += frame.height_pt
    * (word_side_adjustment.plot_right_ratio
      + word_no_legend_adjustment.plot_right_ratio
      + word_titled_bottom_adjustment.plot_right_ratio);
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
      let horizontal_inset = style.value_label.font_size_pt * 1.2;
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
    plot_left = scene_plot.left;
    plot_top = scene_plot.top;
    plot_right = scene_plot.left + scene_plot.width;
    plot_bottom = scene_plot.top + scene_plot.height;
    projection_3d = Some(cartesian_3d_projection(view, scene_plot));
  }
  if plot_right <= plot_left || plot_bottom <= plot_top {
    return Vec::new();
  }
  let plot_width = plot_right - plot_left;
  let plot_height = plot_bottom - plot_top;
  let axis_text_projection_3d =
    projection_3d.filter(|_| chart.view_3d.is_none_or(|view| !view.right_angle_axes));
  let (primary_value_axis_x, primary_value_axis_depth) = projection_3d.map_or(
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
  let (primary_value_label_axis_x, primary_value_label_axis_depth) = axis_text_projection_3d
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
  let available_value_axis_length = if radar_only {
    // A radar value axis runs from the center to the outer polygon. Its label
    // budget is therefore the radius, not the full plot height used by a
    // cartesian value axis.
    plot_width.min(plot_height) * 0.46
  } else {
    projection_3d.map_or(plot_height, |projection| {
      projection.vertical_axis_length(
        PlotRect {
          left: plot_left,
          top: plot_top,
          width: plot_width,
          height: plot_height,
        },
        primary_value_axis_on_right,
      )
    })
  };
  let maximum_auto_increment_count = maximum_auto_main_increment_count(
    style.layout_profile,
    available_value_axis_length,
    style.value_label.font_size_pt,
    chart
      .series
      .iter()
      .any(|series| series.kind == ChartSeriesKind::Bubble),
    projection_3d.is_some(),
  );
  let axis_scales = cartesian_axis_scales(chart, category_count, maximum_auto_increment_count);
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
      let format_code = axis
        .numbering_format
        .as_ref()
        .map(|format| format.format_code.as_str())
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
      if style.layout_profile == ChartLayoutProfile::Word && value.abs() < f64::EPSILON {
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
      for tick in ticks
        .iter()
        .filter(|tick| tick.position > f64::EPSILON && tick.position < 1.0 + f64::EPSILON)
      {
        let x = plot_left + tick.position as f32 * plot_width;
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
      && let Some(positions) = date_axis_minor_tick_positions(chart)
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
      true,
      false,
      maximum_auto_increment_count,
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
      let anchor = project_3d_data_label_category_coordinate(
        chart,
        series_index,
        label.position,
        anchor,
        projection_3d,
      );
      let width = metrics.measure_text(&label.text, data_label_style);
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
          anchor.x - width - data_label_style.font_size_pt * 0.2,
          anchor.y - data_label_line_height * 0.5,
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
          anchor.y - data_label_line_height * 0.5,
        ),
        c::DataLabelPositionValues::OutsideEnd | c::DataLabelPositionValues::BestFit => (
          anchor.x - width * 0.5,
          anchor.y
            - data_label_line_height
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
          height_pt: data_label_line_height + vertical_padding * 2.0,
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
      push_data_label_text_components(target, &mut metrics, x, y, label, data_label_style);
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
    );
  }

  // Ordinary Office chart streams emit value ticks before categories and
  // legends. A chart data table is emitted before those ticks, matching the
  // tagged reading order in Excel fixed output.
  if value_tick_labels_visible && chart.data_table.is_none() {
    lower_cartesian_value_tick_labels(
      &mut items,
      &tick_labels,
      ValueTickLabelContext {
        axis_x: primary_value_label_axis_x,
        labels_on_right: primary_value_axis_on_right,
        label_gap: tick_gap
          + if primary_value_axis_on_right {
            frame.height_pt * 0.012_59
          } else {
            0.0
          },
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
  if scatter_only {
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
      style,
      &mut metrics,
      false,
      true,
      maximum_auto_increment_count,
    );
  }
  let mut painted_category_label_style = style.category_label.clone();
  if category_tick_labels_visible && let Some(projection) = axis_text_projection_3d {
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
    if painted_category_label_style.rotation_deg.abs() > f32::EPSILON {
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
        let (x, y) = axis_text_projection_3d.map_or(
          (
            center - width / 2.0,
            category_top + line_index as f32 * category_label_line_height,
          ),
          |projection| {
            let point = projection.project(center, zero_y, 0.0);
            let label_gap = painted_category_label_style.font_size_pt * 0.45;
            if painted_category_label_style.rotation_deg.abs() > f32::EPSILON {
              (
                point.0 - painted_category_label_style.font_size_pt * 0.15,
                point.1 + label_gap + line_index as f32 * category_label_line_height,
              )
            } else {
              (
                point.0 - width / 2.0,
                point.1 + label_gap + line_index as f32 * category_label_line_height,
              )
            }
          },
        );
        push_text(
          &mut items,
          x,
          y,
          line.clone(),
          painted_category_label_style.clone(),
        );
      }
    }
  }
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
        category_band_top: painted_category_top,
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
          label_gap: tick_gap
            + if primary_value_axis_on_right {
              frame.height_pt * 0.012_59
            } else {
              0.0
            },
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
        category_band_top: painted_category_top,
        data_table_height,
        projection_3d,
      },
      chart,
      style,
      &mut metrics,
    );
  }
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
      axis_y - context.value_label_line_height / 2.0 + context.tick_top_offset,
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

  let back_color = chart_style_fill_fallback_color(&style.plot_area_style).unwrap_or(RgbColor {
    r: 250,
    g: 250,
    b: 247,
  });
  let (outline_color, outline_width) = chart_style_stroke_fallback(&style.plot_area_style)
    .unwrap_or((style.gridline_color, 0.75 * style.stroke_scale));
  push_chart_polygon(
    items,
    &[
      back_top_left,
      back_top_right,
      back_bottom_right,
      back_bottom_left,
    ],
    back_color,
    Some((outline_color, outline_width)),
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
  push_chart_polygon(
    items,
    &[
      side_front_top,
      side_back_top,
      side_back_bottom,
      side_front_bottom,
    ],
    shade_chart_color(back_color, 0.72),
    Some((outline_color, outline_width)),
  );
  push_chart_polygon(
    items,
    &[
      front_bottom_left,
      back_bottom_left,
      back_bottom_right,
      front_bottom_right,
    ],
    shade_chart_color(back_color, 0.88),
    Some((outline_color, outline_width)),
  );
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
          None,
        ));
      }
      items.push(radial_segment_path(
        (center_x + offset_x, center_y + offset_y),
        (radius_x, radius_y),
        hole_ratio,
        (start_angle, sweep),
        (color, 1.0),
        true,
        style.point_styles.get(index),
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
      style.point_styles.get(index),
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

fn radial_segment_path(
  center: (f32, f32),
  radii: (f32, f32),
  hole_ratio: f32,
  angles: (f32, f32),
  paint: (RgbColor, f32),
  stroke_outline: bool,
  style: Option<&crate::common::ShapeStyle<'static>>,
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
  let bounds = common_rect(
    center_x - radius_x,
    center_y - radius_y,
    radius_x * 2.0,
    radius_y * 2.0,
  );
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
      push_radial_legend_key(items, x, y, marker, index, style);
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
    let legend_line_height = line_height(&style.label);
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
        y + (line_height(&style.label) - marker) * 0.5,
        marker,
        index,
        style,
      );
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
      push_radial_legend_key(
        items,
        x,
        y + (line_height(&style.label) - marker) * 0.5,
        marker,
        index,
        style,
      );
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
  category_band_top: f32,
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
  model_height: f32,
  model_depth: f32,
  camera_distance: Option<f32>,
  raw_center_x: f32,
  raw_center_y: f32,
  scale: f32,
  screen_center_x: f32,
  screen_center_y: f32,
}

impl Chart3DProjection {
  fn project(self, x: f32, y: f32, depth_ratio: f32) -> (f32, f32) {
    let model_x = if self.input.width > f32::EPSILON {
      (x - self.input.left) / self.input.width - 0.5
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
      self.camera_distance,
      model_x,
      model_y,
      model_z,
    );
    (
      self.screen_center_x + (raw_x - self.raw_center_x) * self.scale,
      self.screen_center_y + (raw_y - self.raw_center_y) * self.scale,
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

fn cartesian_3d_projection(view: Chart3DView, plot: PlotRect) -> Chart3DProjection {
  // ECMA-376 §21.2.2.41/§21.2.2.80 express scene depth and an authored
  // height as percentages of chart width.  An omitted hPercent is not 100%:
  // LibreOffice VDiagram::adjustAspectRatio3d solves the missing dimension
  // from the final available rectangle, then adjustPosAndSize_3d uniformly
  // fits and centers the rotated scene.
  let model_depth = (view.depth_percent / 100.0).clamp(0.2, 20.0);
  let rotate_x_rad = view.rotate_x_deg.clamp(-90.0, 90.0).to_radians();
  let rotate_y_rad = view.rotate_y_deg.rem_euclid(360.0).to_radians();
  let camera_distance = chart_3d_camera_distance(view);
  let model_height = if view.height_percent_is_explicit {
    (view.height_percent / 100.0).clamp(0.05, 5.0)
  } else {
    automatic_chart_model_height(
      plot,
      rotate_x_rad,
      rotate_y_rad,
      camera_distance,
      model_depth,
    )
  };
  let bounds = projected_chart_model_bounds(
    rotate_x_rad,
    rotate_y_rad,
    camera_distance,
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
    model_height,
    model_depth,
    camera_distance,
    raw_center_x: (bounds.0 + bounds.2) * 0.5,
    raw_center_y: (bounds.1 + bounds.3) * 0.5,
    scale,
    screen_center_x: plot.left + plot.width * 0.5,
    screen_center_y: plot.top + plot.height * 0.5,
  }
}

fn chart_3d_camera_distance(view: Chart3DView) -> Option<f32> {
  if view.right_angle_axes || view.perspective_half_degrees <= f32::EPSILON {
    return None;
  }
  // LibreOffice View3DConverter maps OOXML's half-degree field to its
  // 0..100 perspective scale by dividing by two. ThreeDHelper then maps that
  // scale hyperbolically to a camera distance of 0.75..20 chart volumes.
  let perspective = (view.perspective_half_degrees / 2.0).clamp(0.0, 100.0);
  let minimum_distance = 0.75_f32;
  let maximum_distance = 20.0_f32;
  let numerator =
    100.0 * maximum_distance * minimum_distance / (maximum_distance - minimum_distance);
  let offset = -numerator / maximum_distance;
  Some(numerator / (perspective - offset))
}

fn automatic_chart_model_height(
  plot: PlotRect,
  rotate_x_rad: f32,
  rotate_y_rad: f32,
  camera_distance: Option<f32>,
  model_depth: f32,
) -> f32 {
  let target_aspect = (plot.width / plot.height.max(f32::EPSILON)).max(f32::EPSILON);
  let aspect = |height| {
    let bounds = projected_chart_model_bounds(
      rotate_x_rad,
      rotate_y_rad,
      camera_distance,
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

fn projected_chart_model_bounds(
  rotate_x_rad: f32,
  rotate_y_rad: f32,
  camera_distance: Option<f32>,
  model_height: f32,
  model_depth: f32,
) -> (f32, f32, f32, f32) {
  let mut minimum_x = f32::INFINITY;
  let mut minimum_y = f32::INFINITY;
  let mut maximum_x = f32::NEG_INFINITY;
  let mut maximum_y = f32::NEG_INFINITY;
  for x in [-0.5_f32, 0.5] {
    for y in [-model_height * 0.5, model_height * 0.5] {
      for z in [-model_depth * 0.5, model_depth * 0.5] {
        let (projected_x, projected_y) =
          project_chart_model_point(rotate_x_rad, rotate_y_rad, camera_distance, x, y, z);
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
  camera_distance: Option<f32>,
  model_x: f32,
  model_y_down: f32,
  model_z: f32,
) -> (f32, f32) {
  let (sin_y, cos_y) = rotate_y_rad.sin_cos();
  let (sin_x, cos_x) = rotate_x_rad.sin_cos();
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
  category_count: usize,
  maximum_auto_increment_count: usize,
) -> Vec<CartesianAxisScales> {
  let mut result = Vec::with_capacity(axis_set_count(chart));
  for axis_set_index in 0..axis_set_count(chart) {
    let value_axis = axis_set_value_axis(chart, axis_set_index);
    let horizontal_bar_axis = chart
      .series
      .iter()
      .filter(|series| series.axis_set_index == axis_set_index)
      .all(|series| series.kind == ChartSeriesKind::Bar);
    let y = if axis_set_is_percent_stacked(chart, axis_set_index) {
      Some(crate::render::chart::LinearAxisScale {
        minimum: value_axis
          .and_then(|axis| axis.scaling.min_axis_value.as_ref())
          .map_or(0.0, |value| value.val),
        maximum: value_axis
          .and_then(|axis| axis.scaling.max_axis_value.as_ref())
          .map_or(1.0, |value| value.val),
        major_unit: value_axis
          .and_then(|axis| axis.major_unit.as_ref())
          .map_or(if horizontal_bar_axis { 0.2 } else { 0.1 }, |value| {
            value.val
          }),
        logarithmic_base: None,
        reversed: value_axis
          .and_then(|axis| axis.scaling.orientation.as_ref())
          .and_then(|orientation| orientation.val)
          == Some(c::OrientationValues::MaxMin),
      })
    } else {
      let maximum_auto_increment_count = if chart.series.iter().any(|series| {
        series.axis_set_index == axis_set_index
          && series.kind == ChartSeriesKind::Area
          && series.grouping == ChartSeriesGrouping::Stacked
      }) {
        maximum_auto_increment_count.min(5)
      } else {
        maximum_auto_increment_count
      };
      linear_axis_scale_with_options(
        bubble_padded_axis_values(
          cartesian_scale_values(chart, category_count, axis_set_index),
          chart
            .series
            .iter()
            .filter(|series| {
              series.axis_set_index == axis_set_index && series.kind == ChartSeriesKind::Bubble
            })
            .flat_map(|series| series.values.iter().flatten().copied()),
        ),
        value_axis,
        if horizontal_bar_axis {
          // A bar chart's value axis runs across the plot width.  The shared
          // budget above is measured from the vertical axis length, so use
          // LibreOffice's unmeasured-label default here instead of imposing a
          // five-increment cap from the shorter category-axis dimension.
          10
        } else {
          maximum_auto_increment_count
        },
        LinearAxisScaleOptions {
          expand_if_values_close_to_border: !chart
            .series
            .iter()
            .any(|series| series.axis_set_index == axis_set_index && series.is_3d),
        },
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
        linear_axis_scale(
          bubble_padded_axis_values(
            scatter_x_axis_values(chart, axis_set_index),
            chart
              .series
              .iter()
              .filter(|series| {
                series.axis_set_index == axis_set_index && series.kind == ChartSeriesKind::Bubble
              })
              .flat_map(|series| series.x_values.iter().flatten().copied()),
          ),
          axis_set_horizontal_value_axis(chart, axis_set_index),
          maximum_auto_increment_count,
        )
      })
      .flatten();
    result.push(CartesianAxisScales { x, y });
  }
  result
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
  values
}

fn bubble_padded_axis_values(
  mut axis_values: Vec<f64>,
  bubble_values: impl IntoIterator<Item = f64>,
) -> Vec<f64> {
  // LibreOffice BubbleChart.cxx sizes the largest bubble's diameter to 25% of
  // the diagram extent. Reserve its 12.5% radius in logical axis space so a
  // point center cannot sit directly on the plot boundary and clip half of
  // the marker. Solving `extent = value + extent / 8` gives the 8/7 factor.
  for value in bubble_values.into_iter().filter(|value| value.is_finite()) {
    axis_values.push(value * (8.0 / 7.0));
  }
  axis_values
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

  let bubble_maximum = chart
    .series
    .iter()
    .flat_map(|series| series.bubble_sizes.iter().flatten().copied())
    .fold(0.0_f64, f64::max);

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
      bubble_maximum,
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
    if !series.trendlines.is_empty() {
      lower_trendlines(
        items,
        chart,
        series,
        color,
        &style.data_label,
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

fn chart_3d_series_depth_slot(chart: &ClusteredColumnChart<'_>, series_index: usize) -> (f32, f32) {
  let Some(series) = chart.series.get(series_index) else {
    return (0.0, 0.0);
  };
  if !series.is_3d {
    return (0.0, 0.0);
  }
  let shares_one_depth_slot = matches!(
    series.grouping,
    ChartSeriesGrouping::Stacked | ChartSeriesGrouping::PercentStacked
  );
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
  // This is the same slot equation used by LibreOffice's
  // CategoryPositionHelper: the authored outer distance is split equally
  // before and after the series collection. In particular gapDepth=150 with
  // one series yields a centered marker spanning 0.3..0.7, not 0.0..0.4.
  let marker_depth = 1.0 / (slot_count as f32 + outer_distance);
  let front = outer_distance * marker_depth * 0.5 + slot_index as f32 * marker_depth;
  (front, (front + marker_depth).min(1.0))
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
          let (front, back) = chart_3d_series_depth_slot(chart, series_index);
          (series.name.as_str(), (front + back) * 0.5)
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
        projection.x_for_visual_side(plot, false, plot.top + plot.height, 0.0),
        plot.top + plot.height,
      ),
      c::AxisPositionValues::Top => (
        projection.x_for_visual_side(plot, false, plot.top, 0.0),
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
    let axis_length = (end.0 - start.0).hypot(end.1 - start.1);
    let explicit_rhythm = axis
      .tick_label_skip
      .as_ref()
      .map(|skip| skip.val.max(1) as usize);
    let rhythm = series_axis_label_rhythm(
      maximum_label_width,
      axis_length,
      labels.len(),
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
    for (index, (label, depth_ratio)) in labels.into_iter().enumerate() {
      let point = projection.project(base.0, base.1, depth_ratio);
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
          point.0 + outward.0 * tick_length * 1.8 - width * 0.5,
          point.1 + outward.1 * tick_length * 1.8 - line_height(&style.series_label) * 0.5,
          label.to_string(),
          style.series_label.clone(),
        );
      }
    }
  }
}

fn series_axis_label_rhythm(
  maximum_label_width: f32,
  axis_length: f32,
  label_count: usize,
  explicit_rhythm: Option<usize>,
) -> usize {
  if let Some(rhythm) = explicit_rhythm {
    return rhythm.max(1);
  }
  if label_count <= 1 || maximum_label_width <= 0.0 {
    return 1;
  }
  if axis_length <= f32::EPSILON {
    return label_count;
  }
  // Axis labels occupy centered shapes, so the two endpoint half-widths are
  // part of the available-axis budget as well. This is the same whole-axis
  // extent calculation used by VCartesianAxis for automatic label rhythm.
  let maximum_visible =
    ((axis_length / (maximum_label_width * 1.05)).floor() as usize).clamp(2, label_count);
  (label_count - 1).div_ceil(maximum_visible - 1).max(1)
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
  color: RgbColor,
  label_style: &TextStyle,
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
  for trendline in series.trendlines {
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
    let mut previous = None;
    for (x_value, y_value) in points {
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
          plot.top - line_height(label_style),
          fields.join(" "),
          label_style.clone(),
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
    let slot_series_count = if series.is_3d {
      1
    } else if series.grouping == ChartSeriesGrouping::Clustered {
      peer_count
    } else {
      1
    };
    let slot_series_index = if series.is_3d {
      0
    } else if series.grouping == ChartSeriesGrouping::Clustered {
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
    shade_chart_color(color, 0.76),
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
    tint_chart_color(color, 0.18),
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
    let slot_series_count = if series.is_3d {
      1
    } else if series.grouping == ChartSeriesGrouping::Clustered {
      peer_count
    } else {
      1
    };
    let slot_series_index = if series.is_3d {
      0
    } else if series.grouping == ChartSeriesGrouping::Clustered {
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
          series.line_width_pt.unwrap_or(1.5),
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
      push_chart_styled_line(
        items,
        (previous_x, previous_y),
        point,
        chart_series_stroke_style(style, series_index, None),
        color,
        series.line_width_pt.unwrap_or(1.5) * style.stroke_scale,
        style.stroke_scale,
      );
    }
    if let Some(marker) = chart_marker_size(series) {
      lower_chart_marker(
        items,
        point.0,
        point.1,
        marker * style.stroke_scale,
        chart_point_color(style, series_index, index).unwrap_or(color),
        series,
        chart_marker_stroke_width(series, style.stroke_scale),
      );
    }
    previous = Some(point);
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
    (series.line_width_pt.unwrap_or(1.5) * style.stroke_scale * 0.3).clamp(0.25, 1.5),
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
          series.line_width_pt.unwrap_or(1.5),
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

  let mut previous = None;
  for (index, value) in series.values.iter().enumerate() {
    let Some(value) = value else {
      previous = None;
      continue;
    };
    let (x, y) = point_for(index, *value);
    if !bubbles
      && !series.line_hidden
      && let Some((previous_x, previous_y)) = previous
    {
      push_chart_styled_line(
        items,
        (previous_x, previous_y),
        (x, y),
        chart_series_stroke_style(style, series_index, None),
        color,
        series.line_width_pt.unwrap_or(1.5) * style.stroke_scale,
        style.stroke_scale,
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
      let Some(size) = chart_marker_size(series) else {
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
      series,
      chart_marker_stroke_width(series, style.stroke_scale),
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
      push_chart_styled_line(
        items,
        (previous_x, previous_y),
        point,
        chart_series_stroke_style(style, series_index, None),
        color,
        1.25 * style.stroke_scale,
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
      1.25 * style.stroke_scale,
      style.stroke_scale,
    );
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
  let value_labels_visible = chart.value_axis.is_none_or(|axis| {
    value_axis_is_visible(axis)
      && axis
        .tick_label_position
        .as_ref()
        .is_none_or(|position| position.val != Some(c::TickLabelPositionValues::None))
  });
  if value_labels_visible {
    let format_code = chart
      .value_axis
      .and_then(|axis| axis.numbering_format.as_ref())
      .map(|format| format.format_code.as_str());
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
      push_text(
        items,
        center.0 - width - style.value_label.font_size_pt * 0.22,
        center.1 - radius * ratio as f32 - line_height(&style.value_label) * 0.5,
        label,
        style.value_label.clone(),
      );
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
  tick_labels: &[(f64, String)],
  geometry: HorizontalAxisGeometry,
  style: &ClusteredColumnStyle,
  metrics: &mut TextMetrics,
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
        items,
        axis_x - width * 0.5,
        axis_y + style.value_label.font_size_pt * 0.25,
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
    return;
  }
  let count = chart.categories.len().max(1);
  for (index, category) in chart.categories.iter().enumerate() {
    let width = metrics.measure_text(category, &style.category_label);
    let display_index = horizontal_bar_category_display_index(chart, index, count);
    let y = plot.top + (display_index as f32 + 0.5) / count as f32 * plot.height
      - line_height(&style.category_label) * 0.5;
    let (axis_x, axis_y) = projection_3d.map_or((plot.left, y), |projection| {
      projection.project(plot.left, y, 1.0)
    });
    push_text(
      items,
      axis_x - width - style.category_label.font_size_pt * 0.45,
      axis_y,
      category.clone(),
      style.category_label.clone(),
    );
  }
}

fn lower_scatter_x_axis(
  items: &mut Vec<PageItem>,
  chart: &ClusteredColumnChart<'_>,
  plot: PlotRect,
  style: &ClusteredColumnStyle,
  metrics: &mut TextMetrics,
  draw_gridlines: bool,
  draw_labels: bool,
  maximum_auto_increment_count: usize,
) {
  let values = bubble_padded_axis_values(
    scatter_x_axis_values(chart, 0),
    chart
      .series
      .iter()
      .filter(|series| series.kind == ChartSeriesKind::Bubble)
      .flat_map(|series| series.x_values.iter().flatten().copied()),
  );
  let Some(scale) = linear_axis_scale(
    values,
    chart.horizontal_value_axis,
    maximum_auto_increment_count,
  ) else {
    return;
  };
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
  let format_code = chart
    .horizontal_value_axis
    .and_then(|axis| axis.numbering_format.as_ref())
    .map(|format| format.format_code.as_str());
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

fn scatter_x_axis_values(chart: &ClusteredColumnChart<'_>, axis_set_index: usize) -> Vec<f64> {
  chart
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

fn project_3d_data_label_category_coordinate(
  chart: &ClusteredColumnChart<'_>,
  series_index: usize,
  position: c::DataLabelPositionValues,
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
    ChartSeriesKind::Column => {
      // The category coordinate of a 3-D column follows the visible depth
      // face. Its value coordinate remains in the 2-D label overlay because
      // BarChart::createDataLabel applies the separate 260 mm100 clearance.
      anchor.x = projection.project(anchor.x, anchor.y, front).0;
      anchor.base_x = projection.project(anchor.base_x, anchor.base_y, front).0;
      if position == c::DataLabelPositionValues::Center {
        anchor.y = projection.project(anchor.x, anchor.y, front).1;
        anchor.base_y = projection.project(anchor.base_x, anchor.base_y, front).1;
      }
    }
    ChartSeriesKind::Bar => {
      anchor.y = projection.project(anchor.x, anchor.y, front).1;
      anchor.base_y = projection.project(anchor.base_x, anchor.base_y, front).1;
      if position == c::DataLabelPositionValues::Center {
        anchor.x = projection.project(anchor.x, anchor.y, front).0;
        anchor.base_x = projection.project(anchor.base_x, anchor.base_y, front).0;
      }
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
  if let Some(axis) = chart.date_axis {
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
      let source_minimum = chart
        .category_axis_values
        .iter()
        .flatten()
        .copied()
        .filter(|value| value.is_finite())
        .fold(f64::INFINITY, f64::min);
      let source_maximum = chart
        .category_axis_values
        .iter()
        .flatten()
        .copied()
        .filter(|value| value.is_finite())
        .fold(f64::NEG_INFINITY, f64::max);
      let minimum = axis
        .scaling
        .min_axis_value
        .as_ref()
        .map_or(source_minimum, |value| value.val);
      let maximum = axis
        .scaling
        .max_axis_value
        .as_ref()
        .map_or(source_maximum, |value| value.val);
      if minimum.is_finite() && maximum.is_finite() && maximum > minimum {
        let mut ratio = (value - minimum) / (maximum - minimum);
        if chart.category_axis_shifted {
          // ECMA-376 §21.2.2.32: crossBetween="between" makes date values
          // occupy category slots just like a textual category axis. The
          // first/last dates therefore sit half a slot inside the plot rather
          // than directly on its borders (notably Word stock charts).
          ratio =
            (ratio * category_count.saturating_sub(1) as f64 + 0.5) / category_count.max(1) as f64;
        }
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

fn category_axis_text_rotation_is_supported(
  properties: Option<&c::TextProperties>,
  category_count: usize,
) -> bool {
  properties
    .and_then(|properties| properties.body_properties.rotation)
    // DrawingML chart tick-label rotation is limited to -90..90 degrees.
    // Normalize legacy values by full revolutions before applying that range:
    // Office documents in the wild can store, for example, -1000 degrees for
    // the equivalent visible 80-degree orientation.
    .is_none_or(|rotation| {
      let normalized = rotation.rem_euclid(21_600_000);
      let normalized = if normalized > 10_800_000 {
        normalized - 21_600_000
      } else {
        normalized
      };
      (-5_400_000..=5_400_000).contains(&normalized) || category_count <= 6
    })
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
    category_band_top,
    data_table_height,
    projection_3d,
  } = geometry;
  // OOXML chart styles assign axis titles (and display-unit labels) the
  // source-backed `spAxisTitleTexts` role, which is bold by default. Keep the
  // role separate from ordinary axis tick labels and legends.
  let mut horizontal_title_style = style.label.clone();
  horizontal_title_style.bold = true;
  horizontal_title_style.rotation_deg = 0.0;
  let mut vertical_title_style = horizontal_title_style.clone();
  vertical_title_style.rotation_deg = -90.0;
  let horizontal_bar = chart
    .series
    .iter()
    .all(|series| series.kind == ChartSeriesKind::Bar);
  if let Some(title) = chart.value_axis_title.as_deref() {
    if horizontal_bar {
      let width = metrics.measure_text(title, &horizontal_title_style);
      let projected_bottom = projection_3d.map_or(plot.top + plot.height, |projection| {
        projection.project(plot.left, plot.top + plot.height, 1.0).1
      });
      let content_bottom = if chart.data_table.is_some() {
        category_band_top + data_table_height
      } else {
        projected_bottom
      };
      push_text(
        items,
        plot.left + (plot.width - width) * 0.5,
        content_bottom + line_height(&horizontal_title_style) * 0.8,
        title.to_string(),
        horizontal_title_style.clone(),
      );
    } else {
      let width = metrics.measure_text(title, &vertical_title_style);
      let x = frame.x_pt + vertical_title_style.font_size_pt * 0.2;
      let y = plot.top + (plot.height + width) * 0.5;
      items.push(PageItem::Text(TextItem {
        x_pt: x,
        y_pt: y,
        line_height_pt: line_height(&vertical_title_style),
        paint_clip: None,
        discard_if_horizontally_clipped: false,
        text: title.to_string(),
        style: vertical_title_style.clone(),
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
  if let Some(title) = chart.category_axis_title.as_deref() {
    if horizontal_bar {
      let width = metrics.measure_text(title, &vertical_title_style);
      let maximum_category_width = chart
        .categories
        .iter()
        .map(|category| metrics.measure_text(category, &style.category_label))
        .fold(0.0_f32, f32::max);
      let (axis_x, axis_y) = projection_3d
        .map_or((plot.left, plot.top + plot.height * 0.5), |projection| {
          projection.project(plot.left, plot.top + plot.height * 0.5, 1.0)
        });
      let x = axis_x
        - maximum_category_width
        - style.category_label.font_size_pt * 0.45
        - line_height(&vertical_title_style) * 1.15;
      let y = axis_y + width * 0.5;
      items.push(PageItem::Text(TextItem {
        x_pt: x,
        y_pt: y,
        line_height_pt: line_height(&vertical_title_style),
        paint_clip: None,
        discard_if_horizontally_clipped: false,
        text: title.to_string(),
        style: vertical_title_style.clone(),
        rotation_center_pt: Some((x, y)),
        hyperlink_url: None,
        form_widget_id: None,
        paragraph_bidi: false,
        preserve_text_portion: true,
        pdf_text_segmentation: PdfTextSegmentation::Line,
        source_path: Vec::new(),
      }));
    } else {
      let width = metrics.measure_text(title, &horizontal_title_style);
      push_text(
        items,
        plot.left + (plot.width - width) * 0.5,
        if chart.data_table.is_some() {
          category_band_top + line_height(&horizontal_title_style) * 0.05
        } else {
          category_band_top + line_height(&horizontal_title_style) * 1.25
        },
        title.to_string(),
        horizontal_title_style.clone(),
      );
    }
  }
  for (index, title) in chart.additional_axis_titles.iter().enumerate() {
    let width = metrics.measure_text(title, &vertical_title_style);
    let x =
      frame.x_pt + frame.width_pt - vertical_title_style.font_size_pt * (0.4 + index as f32 * 1.25);
    let y = plot.top + (plot.height + width) * 0.5;
    items.push(PageItem::Text(TextItem {
      x_pt: x,
      y_pt: y,
      line_height_pt: line_height(&vertical_title_style),
      paint_clip: None,
      discard_if_horizontally_clipped: false,
      text: title.clone(),
      style: vertical_title_style.clone(),
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
  let marker_size = style.label.font_size_pt * 0.55;
  let marker_gap = style.label.font_size_pt * 0.26;
  let entries = cartesian_legend_entries(chart, style, scale);
  if entries.is_empty() || bounds.width <= 0.0 || bounds.height <= 0.0 {
    return;
  }
  let mut metrics = TextMetrics::new();
  let entry_widths = entries
    .iter()
    .map(|entry| {
      marker_size + marker_gap + metrics.measure_text(entry.label.as_ref(), &style.label)
    })
    .collect::<Vec<_>>();
  let maximum_entry_width = entry_widths.iter().copied().fold(0.0_f32, f32::max);
  let column_count =
    ((bounds.width / maximum_entry_width.max(1.0)).floor() as usize).clamp(1, entries.len());
  let row_count = entries.len().div_ceil(column_count);
  let cell_width = bounds.width / column_count as f32;
  let cell_height = bounds.height / row_count as f32;
  let label_line_height = line_height(&style.label);
  for (index, (entry, entry_width)) in entries.into_iter().zip(entry_widths).enumerate() {
    let column = index % column_count;
    let row = index / column_count;
    let x = bounds.left + column as f32 * cell_width + (cell_width - entry_width).max(0.0) * 0.5;
    let y =
      bounds.top + row as f32 * cell_height + (cell_height - label_line_height).max(0.0) * 0.5;
    push_cartesian_legend_key(
      items,
      x,
      y + (line_height(&style.label) - marker_size) * 0.5,
      marker_size,
      false,
      &entry,
      style,
    );
    push_text(
      items,
      x + marker_size + marker_gap,
      y,
      entry.label.into_owned(),
      style.label.clone(),
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
  let marker_gap = style.label.font_size_pt * legend_profile.marker_gap_em;
  let entries = cartesian_legend_entries(chart, style, scale);
  let base_entry_gap =
    style.label.font_size_pt * profiles::DEFAULT_HORIZONTAL_CARTESIAN_LEGEND.entry_gap_em;
  let entry_gap = style.label.font_size_pt * legend_profile.entry_gap_em;
  let widths: Vec<f32> = entries
    .iter()
    .map(|entry| {
      let legend_style = style.label.clone();
      horizontal_legend_key_width(entry.kind, style, legend_profile.line_key_width_em)
        + marker_gap
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
    let key_width =
      horizontal_legend_key_width(entry.kind, style, legend_profile.line_key_width_em);
    push_cartesian_legend_key(
      items,
      x,
      y + (line_height(&style.label) - key_width) / 2.0,
      key_width,
      entry.kind == ChartSeriesKind::Line,
      &entry,
      style,
    );
    let legend_style = style.label.clone();
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
  kind: ChartSeriesKind,
  style: &ClusteredColumnStyle,
  line_key_width_em: f32,
) -> f32 {
  style.label.font_size_pt
    * if kind == ChartSeriesKind::Line {
      line_key_width_em
    } else {
      0.55
    }
}

fn vertical_legend_width(
  chart: &ClusteredColumnChart<'_>,
  style: &ClusteredColumnStyle,
  scale: crate::render::chart::LinearAxisScale,
  metrics: &mut TextMetrics,
) -> f32 {
  let marker_size = style.label.font_size_pt * 0.55;
  let marker_gap = style.label.font_size_pt * 0.26;
  cartesian_legend_entries(chart, style, scale)
    .iter()
    .map(|entry| {
      let legend_style = style.label.clone();
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
  let marker_size = style.label.font_size_pt
    * if excel_vary_colors_data_table_layout(chart, style) {
      profiles::EXCEL_VARY_COLORS_DATA_TABLE_LEGEND_MARKER_EM
    } else {
      0.55
    };
  let marker_gap = style.label.font_size_pt * 0.26;
  let entry_gap = style.label.font_size_pt
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
  let line_height = line_height(&style.label);
  let entries = cartesian_legend_entries(chart, style, scale);
  let total_height =
    line_height * entries.len() as f32 + entry_gap * entries.len().saturating_sub(1) as f32;
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
  if excel_vary_colors_data_table_layout(chart, style) && !align_top {
    y += frame.height_pt * profiles::EXCEL_VARY_COLORS_DATA_TABLE_LEGEND_Y_RATIO;
  }
  for entry in entries {
    push_cartesian_legend_key(
      items,
      x,
      y + (line_height - marker_size) / 2.0,
      marker_size,
      false,
      &entry,
      style,
    );
    let legend_style = style.label.clone();
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
    .series_index
    .and_then(|series_index| chart_series_fill_style(style, series_index, entry.point_index));
  let stroke = entry
    .series_index
    .and_then(|series_index| chart_series_stroke_style(style, series_index, entry.point_index));
  if line_key {
    let y = y_pt + size_pt * 0.5;
    push_chart_styled_line(
      items,
      (x_pt, y),
      (x_pt + size_pt, y),
      stroke,
      entry.color.unwrap_or(RgbColor { r: 0, g: 0, b: 0 }),
      1.5,
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
        })
      })
      .collect();
  }
  if chart.vary_colors_by_point {
    let Some(series) = chart.series.first() else {
      return Vec::new();
    };
    return chart
      .visible_legend_indices
      .iter()
      .filter_map(|index| {
        chart
          .categories
          .get(*index)
          .map(|category| CartesianLegendEntry {
            label: Cow::Borrowed(category),
            color: chart_point_color(style, 0, *index)
              .or_else(|| style.series_colors.first().copied()),
            kind: series.kind,
            series_index: Some(0),
            point_index: Some(*index),
          })
      })
      .collect();
  }
  let mut entries = chart
    .visible_legend_indices
    .iter()
    .filter_map(|index| {
      chart.series.get(*index).map(|series| CartesianLegendEntry {
        label: Cow::Borrowed(&series.name),
        color: style.series_colors.get(*index).copied(),
        kind: series.kind,
        series_index: Some(*index),
        point_index: None,
      })
    })
    .collect::<Vec<_>>();
  if cartesian_legend_reverses_series(chart) {
    entries.reverse();
  }
  entries
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
  let count = ((maximum - minimum) / unit).floor().max(0.0) as usize;
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
) {
  if label.text_components.len() <= 1
    || label.separator.contains('\r')
    || label.separator.contains('\n')
  {
    push_text_with_segmentation(
      items,
      x,
      y,
      label.text.clone(),
      style.clone(),
      data_label_pdf_text_segmentation(&label.text),
    );
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
    let segmentation = data_label_pdf_text_segmentation(&painted_text);
    push_text_with_segmentation(
      items,
      component_x,
      y,
      painted_text,
      style.clone(),
      segmentation,
    );
    if !is_last {
      component_x += metrics.measure_text(&format!("{component}{}", label.separator), style);
    }
  }
}

fn push_text(items: &mut Vec<PageItem>, x: f32, y: f32, text: String, style: TextStyle) {
  push_text_with_segmentation(items, x, y, text, style, PdfTextSegmentation::Line);
}

fn push_text_with_segmentation(
  items: &mut Vec<PageItem>,
  x: f32,
  y: f32,
  text: String,
  style: TextStyle,
  pdf_text_segmentation: PdfTextSegmentation,
) {
  items.push(PageItem::Text(TextItem {
    x_pt: x,
    y_pt: y,
    line_height_pt: line_height(&style),
    paint_clip: None,
    discard_if_horizontally_clipped: false,
    text,
    style,
    rotation_center_pt: None,
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
    bubble_padded_axis_values, cardinal_cubic_controls, cartesian_3d_projection,
    cartesian_legend_reverses_series, category_axis_text_rotation_is_supported,
    clip_surface_polygon, data_label_pdf_text_segmentation, format_axis_value,
    lower_3d_extruded_polygon, lower_3d_line_stripes, maximum_auto_main_increment_count,
    push_chart_data_rect, sample_cardinal_chart_line, series_axis_label_rhythm,
    series_category_display_index, single_line_vertical_anchor_offset, word_fixed_chart_data_edge,
    word_fixed_chart_value_edge,
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
  fn word_axes_use_final_plot_length_and_bubble_marker_envelope() {
    assert_eq!(
      maximum_auto_main_increment_count(ChartLayoutProfile::Word, 152.25, 10.0, false, false),
      10
    );
    assert_eq!(
      maximum_auto_main_increment_count(ChartLayoutProfile::Word, 200.0, 10.0, false, false),
      10
    );
    assert_eq!(
      maximum_auto_main_increment_count(ChartLayoutProfile::Word, 90.0, 10.0, false, true),
      10
    );
    let small_budget =
      maximum_auto_main_increment_count(ChartLayoutProfile::Word, 152.25, 10.0, true, false);
    let large_budget =
      maximum_auto_main_increment_count(ChartLayoutProfile::Word, 252.0, 10.0, true, false);
    assert_eq!(small_budget, 5);
    assert_eq!(large_budget, 8);

    let y_values = bubble_padded_axis_values(vec![0.8, 2.7, 3.2], [0.8, 2.7, 3.2]);
    let small_y = crate::render::chart::linear_axis_scale(y_values.clone(), None, small_budget)
      .expect("small bubble y axis");
    let large_y = crate::render::chart::linear_axis_scale(y_values, None, large_budget)
      .expect("large bubble y axis");
    assert_eq!((small_y.maximum, small_y.major_unit), (4.0, 1.0));
    assert_eq!((large_y.maximum, large_y.major_unit), (4.0, 0.5));

    let x_values = bubble_padded_axis_values(vec![0.7, 1.8, 2.6], [0.7, 1.8, 2.6]);
    let small_x = crate::render::chart::linear_axis_scale(x_values.clone(), None, small_budget)
      .expect("small bubble x axis");
    let large_x = crate::render::chart::linear_axis_scale(x_values, None, large_budget)
      .expect("large bubble x axis");
    assert_eq!((small_x.maximum, small_x.major_unit), (4.0, 1.0));
    assert_eq!((large_x.maximum, large_x.major_unit), (3.5, 0.5));
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
    );
    let front = default.project(130.0, 80.0, 0.0);
    let back = default.project(130.0, 80.0, 1.0);
    assert!(back.0 > front.0);
    assert!(back.1 < front.1);
    let left = default.project(plot.left, plot.top + plot.height, 0.0);
    let right = default.project(plot.left + plot.width, plot.top + plot.height, 0.0);
    assert!(right.0 > left.0);
    assert!(right.1 > left.1);

    let reversed = cartesian_3d_projection(
      Chart3DView {
        rotate_x_deg: 20.0,
        rotate_y_deg: 170.0,
        perspective_half_degrees: 0.0,
        ..Chart3DView::default()
      },
      plot,
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
    assert_eq!(series_axis_label_rhythm(25.0, 32.0, 3, None), 2);
    assert_eq!(series_axis_label_rhythm(25.0, 64.0, 3, None), 2);
    assert_eq!(series_axis_label_rhythm(25.0, 80.0, 3, None), 1);
    assert_eq!(series_axis_label_rhythm(25.0, 64.0, 3, Some(3)), 3);
  }

  #[test]
  fn category_axis_rotation_normalizes_full_revolutions() {
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
