use ooxmlsdk::schemas::schemas_microsoft_com_office_drawing_2012_chart as c15;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_chart as c;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use std::borrow::Cow;

use crate::common::color_math;
use crate::field_datetime;
use crate::localization::{ChartDisplayUnit, ChartTrendlineKind, OfficeStringCatalog};
use crate::model::RgbColor;
use crate::options::FieldUpdateDateTime;
use crate::{render::math::text_math_text, units};

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChartHostApplication {
  Wordprocessing,
  Spreadsheet,
  Presentation,
}

#[derive(Clone, Copy)]
pub struct ChartSeriesRef<'a> {
  /// `c:ser/c:idx`, used by the classic chart-style formatting algorithm.
  /// This is independent from display order and may contain gaps in combined
  /// charts, so reducing it to the vector position changes fade colors.
  pub formatting_index: usize,
  pub series_text: Option<&'a c::SeriesText>,
  pub category_axis_data: Option<&'a c::CategoryAxisData>,
  pub values: Option<&'a c::Values>,
  pub y_values: Option<&'a c::YValues>,
  pub x_values: Option<&'a c::XValues>,
  pub bubble_size: Option<&'a c::BubbleSize>,
  pub data_labels: Option<&'a c::DataLabels>,
  /// Office 2013 "Value From Cells" source attached to the series extension.
  /// The point-level `c15:showDataLabelsRange` switch lives on `c:dLbls` or
  /// `c:dLbl`; keeping the source and the switch separate preserves their
  /// normal chart-group/series/point override semantics.
  pub data_labels_range: Option<&'a c15::DataLabelsRange>,
  pub chart_shape_properties: Option<&'a c::ChartShapeProperties>,
  pub data_points: &'a [c::DataPoint],
  pub marker: Option<&'a c::Marker>,
  pub smooth: Option<&'a c::Smooth>,
  pub trendlines: &'a [c::Trendline],
  /// Classic `c:errBars` records. Scatter, bubble, and area series may own
  /// independent X and Y records, while line and bar series own at most one.
  /// A fixed two-slot view keeps this lightweight source adapter `Copy`
  /// without dropping the second direction.
  pub error_bars: [Option<&'a c::ErrorBars>; 2],
}

#[derive(Clone, Copy, Debug)]
pub struct ChartDataPointFill<'a> {
  pub index: u32,
  pub fill: &'a a::SolidFill,
}

#[derive(Clone, Debug)]
pub struct SurfaceChartGroup<'a> {
  /// Index of the first series owned by this plot-area chart group.
  pub first_series_index: usize,
  pub series_count: usize,
  pub axis_set_index: usize,
  /// `surfaceChart` is the two-dimensional contour form; `surface3DChart`
  /// preserves the value axis as height in the projected 3-D scene.
  pub is_3d: bool,
  /// ECMA-376 Part 1 §21.2.2.230: an omitted element means a filled
  /// surface, while a present element with an omitted `val` means true.
  pub wireframe: bool,
  /// Optional per-value-band fills, keyed by `c:bandFmt/c:idx`.
  pub band_fills: Vec<ChartDataPointFill<'a>>,
}

/// Lines and bars authored on one cartesian chart group rather than on an
/// individual series. Keeping the exact series span is important for combo
/// charts: high-low/up-down geometry must not accidentally mix a primary
/// line group with a secondary line or stock group that happens to share the
/// same plot area.
#[derive(Clone, Debug)]
pub struct CartesianChartGroupDecorations<'a> {
  pub first_series_index: usize,
  pub series_count: usize,
  pub axis_set_index: usize,
  pub drop_lines: Option<&'a c::DropLines>,
  pub high_low_lines: Option<&'a c::HighLowLines>,
  pub up_down_bars: Option<&'a c::UpDownBars>,
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Chart3DView {
  pub rotate_x_deg: f32,
  pub rotate_y_deg: f32,
  pub height_percent: f32,
  /// Whether `c:hPercent` was authored.  An omitted height is automatic and
  /// must be fitted from the rotated scene, rather than treated as 100%.
  pub height_percent_is_explicit: bool,
  pub depth_percent: f32,
  /// Whether `c:depthPercent` was authored. When it is omitted, Office uses
  /// the chart type's preferred 3-D aspect ratio instead of a one-chart-width
  /// depth.
  pub depth_percent_is_explicit: bool,
  pub right_angle_axes: bool,
  /// OOXML stores the field-of-view angle in half degrees.
  pub perspective_half_degrees: f32,
}

impl Default for Chart3DView {
  fn default() -> Self {
    Self {
      rotate_x_deg: 15.0,
      rotate_y_deg: 20.0,
      height_percent: 100.0,
      height_percent_is_explicit: false,
      depth_percent: 100.0,
      depth_percent_is_explicit: false,
      right_angle_axes: false,
      perspective_half_degrees: 30.0,
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChartSeriesGrouping {
  Clustered,
  Standard,
  Stacked,
  PercentStacked,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ChartLayoutMode {
  /// Offset or size relative to the element's automatic rectangle.
  #[default]
  Factor,
  /// Coordinate or far edge relative to the complete chart frame.
  Edge,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ChartManualLayout {
  /// `c:layoutTarget="inner"` positions the rectangle bounded by the axes;
  /// axis labels and a data table remain outside that rectangle.
  pub targets_inner_plot: bool,
  pub x: Option<f32>,
  pub y: Option<f32>,
  pub width: Option<f32>,
  pub height: Option<f32>,
  pub x_mode: ChartLayoutMode,
  pub y_mode: ChartLayoutMode,
  pub width_mode: ChartLayoutMode,
  pub height_mode: ChartLayoutMode,
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
  /// `c:ser/c:idx`, retained for ECMA-376 §21.2.3.46 Table 5 automatic
  /// series formatting. It is not interchangeable with display order.
  pub formatting_index: usize,
  pub name: String,
  pub has_explicit_name: bool,
  /// Whether c:tx carries a non-empty cached or literal series title.
  ///
  /// A present but empty c:tx still affects host layout, so it cannot replace
  /// `has_explicit_name`. It must not, however, promote a generated Row/Column
  /// fallback into an automatic chart title.
  pub has_nonempty_explicit_name: bool,
  /// Authored series shape properties retained for host color resolution.
  ///
  /// Keeping the typed `c:spPr` here preserves DrawingML line semantics such
  /// as preset dashes, caps, and joins; reducing it to a solid color and width
  /// during shared import loses visible chart geometry.
  pub shape_properties: Option<&'a c::ChartShapeProperties>,
  /// Authored point overrides retained in index form.  A `c:dPt` may carry
  /// any DrawingML fill/outline, including an explicit `a:noFill`; reducing
  /// this list to solid RGB values loses both inheritance and line paint.
  pub data_points: &'a [c::DataPoint],
  /// Worksheet reference backing the series name, when `c:tx` uses a
  /// `c:strRef`. Spreadsheet hosts use it when the embedded string cache is
  /// absent rather than substituting an application-generated name.
  pub name_formula: Option<&'a str>,
  /// Worksheet reference backing category coordinates or labels.
  pub category_formula: Option<&'a str>,
  /// Formula backing the numeric values, when the chart uses a worksheet
  /// reference. Renderers normally use the embedded cache, but spreadsheet
  /// hosts need the reference when `plotVisOnly=0` asks for hidden cells that
  /// Office deliberately omits from that cache.
  pub value_formula: Option<&'a str>,
  pub values: Vec<Option<f64>>,
  pub number_format_code: Option<&'a str>,
  /// Worksheet reference backing scatter/bubble X values.
  pub x_value_formula: Option<&'a str>,
  pub x_values: Vec<Option<f64>>,
  /// Cached number format for a scatter/bubble X-value sequence.  Numeric X
  /// and Y axes can both be `c:valAx`, but `sourceLinked` resolves them from
  /// different data roles.
  pub x_number_format_code: Option<&'a str>,
  /// Worksheet reference backing bubble sizes.
  pub bubble_size_formula: Option<&'a str>,
  pub bubble_sizes: Vec<Option<f64>>,
  /// Bubble-chart group that owns this series. Keeping group identity avoids
  /// combining maxima and scaling rules from independent bubble groups in a
  /// mixed chart.
  pub bubble_group_index: Option<usize>,
  /// `c:bubbleScale`, as a percentage of the application's default bubble
  /// diameter. ECMA-376 constrains the value to 0..=300 and gives an omitted
  /// value the default 100.
  pub bubble_scale_percent: f64,
  /// Whether a bubble value controls its painted area or its diameter.
  pub bubble_size_represents: c::SizeRepresentsValues,
  /// `c:showNegBubbles`: negative sizes are hidden unless the owning group
  /// explicitly requests their absolute magnitudes.
  pub show_negative_bubbles: bool,
  /// Effective group/series `c:bubble3D` state. A point-level `c:dPt`
  /// override remains available through `data_points` and is resolved by the
  /// renderer for that point.
  pub bubble_3d: bool,
  pub solid_fill: Option<&'a a::SolidFill>,
  pub data_point_fills: Vec<ChartDataPointFill<'a>>,
  pub data_labels: Vec<ClusteredColumnDataLabel<'a>>,
  /// Coordinate-system set selected by the chart group's ordered c:axId
  /// vector. Series from a combined chart may use a secondary X/Y pair.
  pub axis_set_index: usize,
  pub kind: ChartSeriesKind,
  pub grouping: ChartSeriesGrouping,
  pub is_3d: bool,
  /// Geometry selected by `c:bar3DChart/c:shape`. The OOXML default for an
  /// omitted marker shape is a box.
  pub shape_3d: c::ShapeValues,
  /// Space behind the series cluster as a percentage of marker depth.
  pub gap_depth_percent: f64,
  /// Per-series line smoothing. `None` preserves the OOXML omission so hosts
  /// can apply the document-version default instead of collapsing it to
  /// `false` during shared chart import.
  pub smooth: Option<bool>,
  pub marker: Option<&'a c::Marker>,
  /// Application-defined marker used when the chart-group style requires
  /// markers but the series omits `c:marker`. An explicit series marker still
  /// overrides this value, including `c:symbol="none"`.
  pub automatic_marker_symbol: Option<c::MarkerStyleValues>,
  /// Whether the series explicitly suppresses its connecting line with
  /// `c:spPr/a:ln/a:noFill`. An omitted outline keeps the chart-style default.
  pub line_hidden: bool,
  pub line_width_pt: Option<f32>,
  pub filled_area: bool,
  pub trendlines: &'a [c::Trendline],
  pub error_bars: Vec<ChartErrorBars<'a>>,
}

#[derive(Clone, Debug)]
pub struct ChartErrorBars<'a> {
  /// `errDir` is optional in OOXML. Excel's object model defines Y as the
  /// default and permits X only for scatter-family charts.
  pub direction: c::ErrorBarDirectionValues,
  pub show_positive: bool,
  pub show_negative: bool,
  pub no_end_cap: bool,
  pub values: ChartErrorBarValues<'a>,
  pub shape_properties: Option<&'a c::ChartShapeProperties>,
}

#[derive(Clone, Debug)]
pub enum ChartErrorBarValues<'a> {
  Custom {
    positive_formula: Option<&'a str>,
    positive_values: Vec<Option<f64>>,
    negative_formula: Option<&'a str>,
    negative_values: Vec<Option<f64>>,
  },
  Fixed(f64),
  Percentage(f64),
  StandardDeviation(f64),
  StandardError,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClusteredColumnDataLabel<'a> {
  pub point_index: usize,
  pub text: String,
  /// Logical data-label fields in Office's authored order. Office paints
  /// these as separate text runs, which matters when a worksheet page
  /// boundary falls between the fields.
  pub text_components: Vec<String>,
  /// Index of the automatically composed value component. Custom c:tx
  /// labels intentionally keep this unset because their field semantics are
  /// already resolved by the producer.
  pub value_component_index: Option<usize>,
  /// Resolved DrawingML runs for an individual `c:dLbl/c:tx/c:rich` label.
  /// Paragraph and explicit break boundaries are retained as line indices;
  /// direct and paragraph-default properties remain separate so each host can
  /// resolve its own theme fonts and colors without flattening the shared
  /// chart model.
  pub rich_text_runs: Vec<ChartDataLabelTextRun<'a>>,
  pub value_format_code: Option<&'a str>,
  pub separator: &'a str,
  pub position: c::DataLabelPositionValues,
  /// Individual `c:dLbl/c:layout/c:manualLayout` position. Group and series
  /// `c:dLbls` do not own this rectangle.
  pub layout: Option<ChartManualLayout>,
  /// Office 2013 `c15:layout` state. Unlike the compatibility `c:layout`, its
  /// manual layout can retain only `w`/`h`; PowerPoint uses those dimensions
  /// as the data-label text frame even when position remains automatic.
  pub text_frame_layout: Option<ChartManualLayout>,
  /// Effective c:dLbls/c:dLbl text properties after applying Office's
  /// chart-group < series < point override hierarchy.
  pub text_properties: Option<&'a c::TextProperties>,
  /// Effective DrawingML text-body properties. An individual rich `c:tx`
  /// owns its text body and therefore overrides the inherited `c:txPr`
  /// body. Retaining this separately lets host layout include the complete
  /// label box (including schema-default insets) instead of measuring only
  /// the painted glyphs.
  pub text_body_properties: Option<&'a a::BodyProperties>,
  /// Resolved c:dLbls/c:dLbl shape properties after applying Office's
  /// chart-group < series < point override hierarchy.
  pub shape_properties: Option<&'a c::ChartShapeProperties>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChartDataLabelTextRun<'a> {
  pub text: String,
  pub line_index: usize,
  pub paragraph_default_run_properties: Option<&'a a::DefaultRunProperties>,
  pub run_properties: Option<&'a a::RunProperties>,
}

#[derive(Clone, Debug)]
pub struct CartesianAxisSet<'a> {
  /// Ordered chart-group axis identifiers. ECMA-376 §21.2.2.9 defines these
  /// as the coordinate space of the chart group; the first identifier is X,
  /// the second is Y, and a third identifier (when present) is Z.
  pub axis_ids: Vec<i32>,
  pub category_axis: Option<&'a c::CategoryAxis>,
  pub date_axis: Option<&'a c::DateAxis>,
  pub horizontal_value_axis: Option<&'a c::ValueAxis>,
  pub vertical_value_axis: Option<&'a c::ValueAxis>,
  pub series_axis: Option<&'a c::SeriesAxis>,
}

#[derive(Clone, Debug)]
pub struct AdditionalAxisTitle<'a> {
  pub text: String,
  pub source: &'a c::Title,
  pub position: c::AxisPositionValues,
  pub automatic_rotation_deg: f32,
  pub layout: Option<ChartManualLayout>,
}

#[derive(Clone, Debug)]
pub struct ClusteredColumnChart<'a> {
  /// Output UI language used for application-generated chart text such as
  /// automatic titles, series names, and built-in display-unit labels.
  pub ui_language: Option<String>,
  /// Output format locale used for locale-dependent numeric and date labels.
  /// This remains independent from UI resources such as automatic titles.
  pub format_locale: Option<String>,
  /// Chart-wide DrawingML text-body geometry from `c:chartSpace/c:txPr`.
  ///
  /// Axis-local `c:txPr` overrides this body. Keeping the chart default in
  /// the shared model is important even when `a:bodyPr` is empty: ECMA-376
  /// assigns non-zero schema defaults to all four text-box insets, and Office
  /// includes the resulting generated label shape in automatic axis scaling.
  pub default_text_body_properties: Option<&'a a::BodyProperties>,
  pub title: Option<ChartTitleText>,
  pub title_overlay: bool,
  pub title_layout: Option<ChartManualLayout>,
  /// Whether `c:title/c:layout` was authored, even when it contains no
  /// `c:manualLayout`. Office distinguishes that automatic-layout marker
  /// from a title with no layout element at all.
  pub title_layout_container_present: bool,
  /// Independent DrawingML text-body rotation in degrees.
  pub title_rotation_deg: f32,
  /// Vertical anchoring authored on a rich chart title's `a:bodyPr`.
  ///
  /// Keeping this separate from the extracted title string lets host
  /// renderers reproduce Office's automatic title-slot placement.
  pub title_vertical_anchor: Option<a::TextAnchoringTypeValues>,
  pub has_automatic_title_marker: bool,
  /// Number of category slots represented by the embedded chart cache before
  /// a spreadsheet host resolves live worksheet references.
  pub cached_category_count: usize,
  pub has_explicit_categories: bool,
  pub category_axis_title: Option<String>,
  /// Manual position authored on the primary horizontal-axis title. Title
  /// layouts use fitted text-box semantics rather than plot-rectangle size.
  pub category_axis_title_layout: Option<ChartManualLayout>,
  pub value_axis_title: Option<String>,
  /// Manual position authored on the primary vertical-axis title.
  pub value_axis_title_layout: Option<ChartManualLayout>,
  /// Titles owned by non-primary category, value, or series axes. Retaining
  /// the source and axis position prevents secondary titles from being
  /// flattened to a generic right-side, -90-degree label.
  pub additional_axis_titles: Vec<AdditionalAxisTitle<'a>>,
  pub categories: Vec<String>,
  /// Numeric category coordinates retained for a date axis. String
  /// formatting alone loses the spacing and major-unit rhythm Office uses.
  pub category_axis_values: Vec<Option<f64>>,
  /// Number format carried by the numeric category sequence. A date axis
  /// whose c:numFmt is source-linked resolves through this format rather than
  /// treating the axis' persisted fallback code as authoritative.
  pub category_number_format_code: Option<String>,
  pub date_1904: bool,
  pub series: Vec<ClusteredColumnSeries<'a>>,
  pub surface_groups: Vec<SurfaceChartGroup<'a>>,
  pub group_decorations: Vec<CartesianChartGroupDecorations<'a>>,
  pub gap_width_percent: f64,
  pub overlap_percent: f64,
  pub category_axis: Option<&'a c::CategoryAxis>,
  pub date_axis: Option<&'a c::DateAxis>,
  /// Numeric X axis for scatter and bubble charts. It is distinct from the
  /// vertical value axis even though both are represented by c:valAx.
  pub horizontal_value_axis: Option<&'a c::ValueAxis>,
  pub category_axis_reversed: bool,
  /// Whether category markers occupy the centers of slots whose boundaries
  /// meet the plot edges. An explicit `c:crossBetween="midCat"` instead puts
  /// the first and last markers on the plot edges.
  pub category_axis_shifted: bool,
  pub value_axis: Option<&'a c::ValueAxis>,
  pub axis_sets: Vec<CartesianAxisSet<'a>>,
  pub view_3d: Option<Chart3DView>,
  pub legend_position: Option<ChartLegendPosition>,
  pub legend_overlay: bool,
  /// DrawingML text-body geometry and overflow policy authored for the
  /// legend. Cartesian legends are lowered as generated entry text shapes,
  /// so retaining the source `a:bodyPr` is the only way to preserve wrapping,
  /// clipping, ellipsis, insets, and anchoring through that generated layer.
  pub legend_text_body_properties: Option<&'a a::BodyProperties>,
  /// A single cartesian series with c:varyColors enabled exposes one legend
  /// entry per data point. Office ignores varyColors when multiple series are
  /// present (MS-OI29500 §21.2.2.227).
  pub vary_colors_by_point: bool,
  pub visible_legend_indices: Vec<usize>,
  /// Zero-based indices of application legend entries suppressed by
  /// `c:legendEntry/c:delete`. For an ordinary cartesian chart the logical
  /// entry stream is each series followed by that series' trendlines; a
  /// vary-colors series contributes its point entries before its trendlines.
  pub deleted_legend_entry_indices: Vec<usize>,
  pub legend_layout: Option<ChartManualLayout>,
  pub plot_layout: Option<ChartManualLayout>,
  pub data_table: Option<&'a c::DataTable>,
  pub data_label_text_properties: Option<&'a c::TextProperties>,
}

#[derive(Clone, Debug)]
pub struct PieChartModel<'a> {
  pub kind: RadialChartKind,
  /// Camera and scene parameters for `c:pie3DChart`.
  ///
  /// A 3-D pie is not the 2-D ellipse plus an arbitrary shadow: `c:view3D`
  /// controls its projected top face and visible extrusion. Retain the view
  /// here so every host can lower the same authored scene semantics.
  pub view_3d: Option<Chart3DView>,
  pub title: Option<ChartTitleText>,
  pub title_layout: Option<ChartManualLayout>,
  pub title_rotation_deg: f32,
  /// Formatting index of the displayed first series and the highest index in
  /// the radial chart group. When `varyColors` is false, classic chart-style
  /// fade colors are selected from these values rather than point position.
  pub series_formatting_index: usize,
  pub maximum_series_formatting_index: usize,
  pub series_name: String,
  /// Whether the displayed first series owns a non-empty authored c:tx.
  ///
  /// Office may use that caption as an automatic chart title, but must not
  /// promote an application-generated Row/Column fallback in its place.
  pub has_nonempty_explicit_series_name: bool,
  pub categories: Vec<String>,
  pub values: Vec<Option<f64>>,
  pub series_shape_properties: Option<&'a c::ChartShapeProperties>,
  pub data_points: &'a [c::DataPoint],
  pub series_solid_fill: Option<&'a a::SolidFill>,
  pub data_point_fills: Vec<ChartDataPointFill<'a>>,
  pub first_slice_angle_deg: f64,
  pub hole_size_percent: f64,
  pub series_explosion_percent: f64,
  /// Point-level `c:dPt/c:explosion` overrides. `None` is distinct from an
  /// authored zero: an absent point override inherits the series explosion.
  pub point_explosion_percent: Vec<Option<f64>>,
  pub secondary_indices: Vec<usize>,
  pub secondary_size_percent: f64,
  pub vary_colors: bool,
  pub legend_position: Option<ChartLegendPosition>,
  pub legend_overlay: bool,
  /// Vertical anchoring authored on the legend's `c:txPr/a:bodyPr`.
  ///
  /// Automatic legend geometry still belongs to the host layout profile, but
  /// the text must be positioned inside that geometry according to DrawingML.
  pub legend_vertical_anchor: Option<a::TextAnchoringTypeValues>,
  /// DrawingML text-body geometry authored for the legend. Manual legends
  /// need these insets when wrapping entries and determining row capacity.
  pub legend_text_body_properties: Option<&'a a::BodyProperties>,
  pub visible_legend_indices: Vec<usize>,
  pub legend_layout: Option<ChartManualLayout>,
  pub plot_layout: Option<ChartManualLayout>,
  pub data_labels: Vec<ClusteredColumnDataLabel<'a>>,
  pub data_label_text_properties: Option<&'a c::TextProperties>,
  pub show_leader_lines: bool,
  /// Authored paint for radial-chart leader lines.  This is independent of
  /// both the label text color and the series outline.
  pub leader_line_shape_properties: Option<&'a c::ChartShapeProperties>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LinearAxisScale {
  pub minimum: f64,
  pub maximum: f64,
  pub major_unit: f64,
  pub logarithmic_base: Option<f64>,
  pub reversed: bool,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct LinearAxisScaleOptions {
  /// LibreOffice `VSeriesPlotter::isExpandIfValuesCloseToBorder` disables the
  /// extra 1/21 border interval for every 3-D plotter. Two-dimensional
  /// plotters retain it so markers and line strokes do not sit on the frame.
  pub expand_if_values_close_to_border: bool,
  /// Smallest automatically selected major unit for an ordinal numeric axis.
  ///
  /// Scatter charts whose `xVal` sequence contains text are repaired by
  /// Office to one-based ordinal positions.  Those positions may be thinned
  /// when the plot is crowded, but fractional positions do not exist and an
  /// automatic 0.5-unit scale would expose synthetic half-categories.
  pub minimum_automatic_major_unit: Option<f64>,
}

impl Default for LinearAxisScaleOptions {
  fn default() -> Self {
    Self {
      expand_if_values_close_to_border: true,
      minimum_automatic_major_unit: None,
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClusteredColumnSlot {
  pub center: f64,
  pub width: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChartCategoryTick {
  /// Normalized position in the plot band, before axis reversal.
  pub position: f64,
  /// Unshifted major-tick/gridline position. Date-axis labels use the center
  /// of a shifted time interval, while Office keeps tick marks and gridlines
  /// on the interval boundary.
  pub gridline_position: f64,
  pub text: String,
}

pub fn automatic_chart_title(ui_language: Option<&str>) -> &'static str {
  OfficeStringCatalog::for_ui_language(ui_language).chart_title()
}

pub fn automatic_series_title(ui_language: Option<&str>, series_index: usize) -> String {
  OfficeStringCatalog::for_ui_language(ui_language).chart_series_title(series_index)
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
  let category_axis_values = chart_numeric_category_values(chart_space);
  let category_number_format_code = chart_numeric_category_format_code(chart_space);
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
    let explicit_name = series_ref
      .series_text
      .map(series_text_value)
      .filter(|value| !value.is_empty());
    let name = explicit_name
      .clone()
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
      DataLabelSeriesData {
        series_name: &name,
        categories: &label_categories,
        values: &values,
        bubble_sizes: None,
        data_labels_range: &data_labels_range_values(series_ref.data_labels_range),
      },
      DataLabelDefaults {
        value_format_code: series_number_format_code(series_ref),
        position: c::DataLabelPositionValues::OutsideEnd,
        supports_percent: false,
        separator: ", ",
      },
    );
    series.push(ClusteredColumnSeries {
      formatting_index: series_ref.formatting_index,
      name,
      has_explicit_name: series_ref.series_text.is_some(),
      has_nonempty_explicit_name: explicit_name.is_some(),
      shape_properties: series_ref.chart_shape_properties,
      data_points: &source.data_point,
      name_formula: series_name_formula(series_ref),
      category_formula: series_category_formula(series_ref),
      value_formula: series_value_formula(series_ref),
      values,
      number_format_code: series_number_format_code(series_ref),
      x_value_formula: series_x_value_formula(series_ref),
      x_values: Vec::new(),
      x_number_format_code: None,
      bubble_size_formula: series_bubble_size_formula(series_ref),
      bubble_sizes: Vec::new(),
      bubble_group_index: None,
      bubble_scale_percent: 100.0,
      bubble_size_represents: c::SizeRepresentsValues::Area,
      show_negative_bubbles: false,
      bubble_3d: false,
      solid_fill,
      data_point_fills,
      data_labels,
      axis_set_index: 0,
      kind: ChartSeriesKind::Column,
      grouping: ChartSeriesGrouping::Clustered,
      is_3d: false,
      shape_3d: c::ShapeValues::Box,
      gap_depth_percent: 150.0,
      smooth: None,
      marker: None,
      automatic_marker_symbol: None,
      line_hidden: false,
      line_width_pt: series_ref
        .chart_shape_properties
        .and_then(|properties| properties.outline.as_deref())
        .and_then(|outline| outline.width)
        .map(|width| units::emu_to_points(i64::from(width))),
      filled_area: false,
      trendlines: &[],
      error_bars: resolved_error_bars(series_ref.error_bars),
    });
  }

  let has_explicit_categories = !categories.is_empty();
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
  let axis_sets = cartesian_axis_sets(
    chart_space,
    &[bar_chart
      .axis_id
      .iter()
      .map(|axis| axis.val)
      .collect::<Vec<_>>()],
  );
  let primary_axis_set = axis_sets.first();
  let category_axis_set = axis_sets
    .get(visible_category_axis_set_index(&axis_sets))
    .or(primary_axis_set);
  let value_axis = primary_axis_set
    .and_then(|set| set.vertical_value_axis)
    .or_else(|| value_axes.first().copied());
  let horizontal_value_axis = primary_axis_set.and_then(|set| set.horizontal_value_axis);
  let category_axis = category_axis_set.and_then(|set| set.category_axis);
  let date_axis = category_axis_set.and_then(|set| set.date_axis);
  let category_crossing_value_axis = category_axis_set
    .and_then(|set| set.vertical_value_axis)
    .or(value_axis);
  apply_axis_display_units_to_data_labels(&mut series, &axis_sets, value_axis);
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
  let cached_category_count = series
    .iter()
    .map(|series| series.values.len())
    .chain(std::iter::once(categories.len()))
    .max()
    .unwrap_or(0);
  let vary_colors_by_point = bar_chart.bar_chart_series.len() == 1
    && bar_chart
      .vary_colors
      .as_ref()
      .is_some_and(|vary| vary.val.is_none_or(|value| value.as_bool()));
  let legend_entry_count = if vary_colors_by_point {
    categories.len()
  } else {
    bar_chart.bar_chart_series.len()
  };
  let category_axis_title_source = category_axis
    .and_then(|axis| axis.title.as_deref())
    .or_else(|| date_axis.and_then(|axis| axis.title.as_deref()))
    .or_else(|| horizontal_value_axis.and_then(|axis| axis.title.as_deref()));
  let value_axis_title_source = value_axis.and_then(|axis| axis.title.as_deref());

  Some(ClusteredColumnChart {
    ui_language: ui_language.map(ToOwned::to_owned),
    format_locale: ui_language.map(ToOwned::to_owned),
    default_text_body_properties: chart_space
      .text_properties
      .as_deref()
      .map(|properties| properties.body_properties.as_ref()),
    title,
    title_overlay: chart_space
      .chart
      .title
      .as_deref()
      .and_then(|title| title.overlay.as_ref())
      .is_some_and(|overlay| overlay.val.is_none_or(|value| value.as_bool())),
    title_layout: chart_title_layout(&chart_space.chart),
    title_layout_container_present: chart_space
      .chart
      .title
      .as_deref()
      .is_some_and(|title| title.layout.is_some()),
    title_rotation_deg: chart_title_rotation_degrees(&chart_space.chart),
    title_vertical_anchor: chart_title_vertical_anchor(&chart_space.chart),
    has_automatic_title_marker: chart_space.chart.auto_title_deleted.is_some(),
    cached_category_count,
    has_explicit_categories,
    category_axis_title: category_axis_title_source.and_then(|title| {
      title_text_or_automatic(title, ChartHostApplication::Spreadsheet, ui_language)
    }),
    category_axis_title_layout: category_axis_title_source
      .and_then(|title| chart_text_layout(title.layout.as_deref())),
    value_axis_title: value_axis_title_source.and_then(|title| {
      title_text_or_automatic(title, ChartHostApplication::Spreadsheet, ui_language)
    }),
    value_axis_title_layout: value_axis_title_source
      .and_then(|title| chart_text_layout(title.layout.as_deref())),
    additional_axis_titles: Vec::new(),
    categories,
    category_axis_values,
    category_number_format_code,
    date_1904: chart_uses_1904_date_system(chart_space),
    series,
    surface_groups: Vec::new(),
    group_decorations: Vec::new(),
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
    horizontal_value_axis,
    category_axis_reversed: category_axis
      .and_then(|axis| axis.scaling.orientation.as_ref())
      .or_else(|| date_axis.and_then(|axis| axis.scaling.orientation.as_ref()))
      .and_then(|orientation| orientation.val)
      == Some(c::OrientationValues::MaxMin),
    category_axis_shifted: category_crossing_value_axis
      .and_then(|axis| axis.cross_between.as_ref())
      .is_none_or(|cross_between| cross_between.val == c::CrossBetweenValues::Between),
    value_axis,
    axis_sets,
    view_3d: None,
    legend_position,
    legend_overlay: chart_space
      .chart
      .legend
      .as_deref()
      .and_then(|legend| legend.overlay.as_ref())
      .is_some_and(|overlay| overlay.val.is_none_or(|value| value.as_bool())),
    legend_text_body_properties: chart_space
      .chart
      .legend
      .as_deref()
      .and_then(|legend| legend.text_properties.as_deref())
      .map(|properties| properties.body_properties.as_ref()),
    vary_colors_by_point,
    visible_legend_indices: visible_series_legend_indices(
      chart_space.chart.legend.as_deref(),
      legend_entry_count,
    ),
    deleted_legend_entry_indices: deleted_legend_entry_indices(chart_space.chart.legend.as_deref()),
    legend_layout: chart_space
      .chart
      .legend
      .as_deref()
      .and_then(|legend| chart_layout(legend.layout.as_deref())),
    plot_layout: chart_layout(chart_space.chart.plot_area.layout.as_deref()),
    data_table: chart_space.chart.plot_area.data_table.as_deref(),
    data_label_text_properties: chart_data_label_text_properties(chart_space),
  })
}

/// Extracts every cartesian/polar series group that can share the common
/// Office chart frame. Pie-family groups use [`pie_chart_model`] because their
/// category legends and radial geometry have different semantics.
fn plot_area_choice_axis_ids(choice: &c::PlotAreaChoice) -> Option<&[c::AxisId]> {
  match choice {
    c::PlotAreaChoice::AreaChart(chart) => Some(&chart.axis_id),
    c::PlotAreaChoice::Area3DChart(chart) => Some(&chart.axis_id),
    c::PlotAreaChoice::LineChart(chart) => Some(&chart.axis_id),
    c::PlotAreaChoice::Line3DChart(chart) => Some(&chart.axis_id),
    c::PlotAreaChoice::StockChart(chart) => Some(&chart.axis_id),
    c::PlotAreaChoice::RadarChart(chart) => Some(&chart.axis_id),
    c::PlotAreaChoice::ScatterChart(chart) => Some(&chart.axis_id),
    c::PlotAreaChoice::BarChart(chart) => Some(&chart.axis_id),
    c::PlotAreaChoice::Bar3DChart(chart) => Some(&chart.axis_id),
    c::PlotAreaChoice::SurfaceChart(chart) => Some(&chart.axis_id),
    c::PlotAreaChoice::Surface3DChart(chart) => Some(&chart.axis_id),
    c::PlotAreaChoice::BubbleChart(chart) => Some(&chart.axis_id),
    c::PlotAreaChoice::PieChart(_)
    | c::PlotAreaChoice::Pie3DChart(_)
    | c::PlotAreaChoice::DoughnutChart(_)
    | c::PlotAreaChoice::OfPieChart(_) => None,
  }
}

fn cartesian_axis_sets<'a>(
  chart_space: &'a c::ChartSpace,
  axis_id_sets: &[Vec<i32>],
) -> Vec<CartesianAxisSet<'a>> {
  let axes = &chart_space.chart.plot_area.plot_area_choice2;
  let sets = if axis_id_sets.is_empty() {
    &[Vec::new()][..]
  } else {
    axis_id_sets
  };
  sets
    .iter()
    .map(|axis_ids| {
      let x_id = axis_ids.first().copied();
      let y_id = axis_ids.get(1).copied();
      let z_id = axis_ids.get(2).copied();
      let category_axis = x_id
        .and_then(|axis_id| {
          axes.iter().find_map(|choice| match choice {
            c::PlotAreaChoice2::CategoryAxis(axis) if axis.axis_id.val == axis_id => {
              Some(axis.as_ref())
            }
            _ => None,
          })
        })
        .or_else(|| {
          x_id.is_none().then(|| {
            axes.iter().find_map(|choice| match choice {
              c::PlotAreaChoice2::CategoryAxis(axis) => Some(axis.as_ref()),
              _ => None,
            })
          })?
        });
      let date_axis = x_id
        .and_then(|axis_id| {
          axes.iter().find_map(|choice| match choice {
            c::PlotAreaChoice2::DateAxis(axis) if axis.axis_id.val == axis_id => {
              Some(axis.as_ref())
            }
            _ => None,
          })
        })
        .or_else(|| {
          x_id.is_none().then(|| {
            axes.iter().find_map(|choice| match choice {
              c::PlotAreaChoice2::DateAxis(axis) => Some(axis.as_ref()),
              _ => None,
            })
          })?
        });
      let horizontal_value_axis = x_id.and_then(|axis_id| {
        axes.iter().find_map(|choice| match choice {
          c::PlotAreaChoice2::ValueAxis(axis) if axis.axis_id.val == axis_id => Some(axis.as_ref()),
          _ => None,
        })
      });
      let vertical_value_axis = y_id
        .and_then(|axis_id| {
          axes.iter().find_map(|choice| match choice {
            c::PlotAreaChoice2::ValueAxis(axis) if axis.axis_id.val == axis_id => {
              Some(axis.as_ref())
            }
            _ => None,
          })
        })
        .or_else(|| {
          axes.iter().find_map(|choice| match choice {
            c::PlotAreaChoice2::ValueAxis(axis)
              if horizontal_value_axis
                .is_none_or(|horizontal| horizontal.axis_id.val != axis.axis_id.val) =>
            {
              Some(axis.as_ref())
            }
            _ => None,
          })
        });
      let series_axis = z_id
        .and_then(|axis_id| {
          axes.iter().find_map(|choice| match choice {
            c::PlotAreaChoice2::SeriesAxis(axis) if axis.axis_id.val == axis_id => {
              Some(axis.as_ref())
            }
            _ => None,
          })
        })
        .or_else(|| {
          z_id.is_none().then(|| {
            axes.iter().find_map(|choice| match choice {
              c::PlotAreaChoice2::SeriesAxis(axis) => Some(axis.as_ref()),
              _ => None,
            })
          })?
        });
      CartesianAxisSet {
        axis_ids: axis_ids.clone(),
        category_axis,
        date_axis,
        horizontal_value_axis,
        vertical_value_axis,
        series_axis,
      }
    })
    .collect()
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

/// Selects the category coordinate space Office exposes to the fixed-output
/// text layer. Combined charts may put a deleted category axis on the first
/// chart group and a visible category axis on a later group. The group order
/// still controls series and value-axis reading order, but it must not hide
/// the later category labels.
fn visible_category_axis_set_index(axis_sets: &[CartesianAxisSet<'_>]) -> usize {
  axis_sets
    .iter()
    .position(|set| {
      set.category_axis.is_some_and(category_axis_is_visible)
        || set.date_axis.is_some_and(date_axis_is_visible)
    })
    .or_else(|| {
      axis_sets
        .iter()
        .position(|set| set.category_axis.is_some() || set.date_axis.is_some())
    })
    .unwrap_or(0)
}

fn apply_axis_display_units_to_data_labels(
  series: &mut [ClusteredColumnSeries<'_>],
  axis_sets: &[CartesianAxisSet<'_>],
  fallback_value_axis: Option<&c::ValueAxis>,
) {
  for series in series {
    let axis = axis_sets
      .get(series.axis_set_index)
      .and_then(|set| set.vertical_value_axis)
      .or(fallback_value_axis);
    let display_unit = axis.map_or(1.0, value_axis_display_unit);
    if !display_unit.is_finite()
      || display_unit <= 0.0
      || (display_unit - 1.0).abs() <= f64::EPSILON
    {
      continue;
    }
    for label in &mut series.data_labels {
      let Some(component_index) = label.value_component_index else {
        continue;
      };
      let Some(value) = series.values.get(label.point_index).copied().flatten() else {
        continue;
      };
      let Some(component) = label.text_components.get_mut(component_index) else {
        continue;
      };
      *component = format_chart_number(value / display_unit, label.value_format_code);
      label.text = label.text_components.join(label.separator);
    }
  }
}

pub fn cartesian_chart_for_ui_language<'a>(
  chart_space: &'a c::ChartSpace,
  ui_language: Option<&str>,
) -> Option<ClusteredColumnChart<'a>> {
  cartesian_chart_for_locales(chart_space, ui_language, ui_language)
}

pub fn cartesian_chart_for_locales<'a>(
  chart_space: &'a c::ChartSpace,
  ui_language: Option<&str>,
  format_locale: Option<&str>,
) -> Option<ClusteredColumnChart<'a>> {
  cartesian_chart_for_host_locales(
    chart_space,
    ChartHostApplication::Spreadsheet,
    ui_language,
    format_locale,
  )
}

pub fn cartesian_chart_for_host_locales<'a>(
  chart_space: &'a c::ChartSpace,
  host: ChartHostApplication,
  ui_language: Option<&str>,
  format_locale: Option<&str>,
) -> Option<ClusteredColumnChart<'a>> {
  let category_axis_values = chart_numeric_category_values(chart_space);
  let category_number_format_code = chart_numeric_category_format_code(chart_space);
  let mut series = Vec::new();
  let mut categories = Vec::new();
  let mut gap_width_percent = 150.0;
  let mut overlap_percent = 0.0;
  let mut vary_colors_requested = false;
  let mut axis_id_sets = Vec::<Vec<i32>>::new();
  let mut surface_groups = Vec::new();
  let mut group_decorations = Vec::new();
  let mut bubble_group_count = 0usize;

  for choice in &chart_space.chart.plot_area.plot_area_choice1 {
    let axis_set_index = plot_area_choice_axis_ids(choice)
      .map(|axes| axes.iter().map(|axis| axis.val).collect::<Vec<_>>())
      .map(|axis_ids| {
        axis_id_sets
          .iter()
          .position(|candidate| *candidate == axis_ids)
          .unwrap_or_else(|| {
            axis_id_sets.push(axis_ids);
            axis_id_sets.len() - 1
          })
      })
      .unwrap_or_default();
    match choice {
      c::PlotAreaChoice::AreaChart(chart) => {
        let first_series_index = series.len();
        append_cartesian_series(
          &mut series,
          &mut categories,
          chart.area_chart_series.iter().map(area_series_ref),
          chart.data_labels.as_deref(),
          (
            ChartSeriesKind::Area,
            grouping(chart.grouping.as_ref()),
            false,
          ),
          axis_set_index,
          ui_language,
        );
        push_cartesian_group_decorations(
          &mut group_decorations,
          first_series_index,
          series.len(),
          axis_set_index,
          chart.drop_lines.as_deref(),
          None,
          None,
        );
      }
      c::PlotAreaChoice::Area3DChart(chart) => {
        let first_series_index = series.len();
        append_cartesian_series(
          &mut series,
          &mut categories,
          chart.area_chart_series.iter().map(area_series_ref),
          chart.data_labels.as_deref(),
          (
            ChartSeriesKind::Area,
            grouping(chart.grouping.as_ref()),
            true,
          ),
          axis_set_index,
          ui_language,
        );
        let gap_depth_percent = chart
          .gap_depth
          .as_ref()
          .and_then(|gap| gap.val)
          .map_or(150.0, f64::from);
        for series in &mut series[first_series_index..] {
          series.gap_depth_percent = gap_depth_percent;
        }
        push_cartesian_group_decorations(
          &mut group_decorations,
          first_series_index,
          series.len(),
          axis_set_index,
          chart.drop_lines.as_deref(),
          None,
          None,
        );
      }
      c::PlotAreaChoice::LineChart(chart) => {
        let first_series_index = series.len();
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
          axis_set_index,
          ui_language,
        );
        push_cartesian_group_decorations(
          &mut group_decorations,
          first_series_index,
          series.len(),
          axis_set_index,
          chart.drop_lines.as_deref(),
          chart.high_low_lines.as_deref(),
          chart.up_down_bars.as_deref(),
        );
      }
      c::PlotAreaChoice::Line3DChart(chart) => {
        let first_series_index = series.len();
        append_cartesian_series(
          &mut series,
          &mut categories,
          chart.line_chart_series.iter().map(line_series_ref),
          chart.data_labels.as_deref(),
          (ChartSeriesKind::Line, grouping(Some(&chart.grouping)), true),
          axis_set_index,
          ui_language,
        );
        let gap_depth_percent = chart
          .gap_depth
          .as_ref()
          .and_then(|gap| gap.val)
          .map_or(150.0, f64::from);
        for series in &mut series[first_series_index..] {
          series.gap_depth_percent = gap_depth_percent;
        }
        push_cartesian_group_decorations(
          &mut group_decorations,
          first_series_index,
          series.len(),
          axis_set_index,
          chart.drop_lines.as_deref(),
          None,
          None,
        );
      }
      c::PlotAreaChoice::StockChart(chart) => {
        let first_series_index = series.len();
        append_cartesian_series(
          &mut series,
          &mut categories,
          chart.line_chart_series.iter().map(line_series_ref),
          chart.data_labels.as_deref(),
          (ChartSeriesKind::Stock, ChartSeriesGrouping::Standard, false),
          axis_set_index,
          ui_language,
        );
        push_cartesian_group_decorations(
          &mut group_decorations,
          first_series_index,
          series.len(),
          axis_set_index,
          chart.drop_lines.as_deref(),
          chart.high_low_lines.as_deref(),
          chart.up_down_bars.as_deref(),
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
          axis_set_index,
          ui_language,
        );
        if chart.radar_style.val == c::RadarStyleValues::Filled {
          for series in &mut series[first..] {
            series.filled_area = true;
          }
        }
      }
      c::PlotAreaChoice::ScatterChart(chart) => {
        let first = series.len();
        append_cartesian_series(
          &mut series,
          &mut categories,
          chart.scatter_chart_series.iter().map(scatter_series_ref),
          chart.data_labels.as_deref(),
          (
            ChartSeriesKind::Scatter,
            ChartSeriesGrouping::Standard,
            false,
          ),
          axis_set_index,
          ui_language,
        );
        let style = chart
          .scatter_style
          .val
          .unwrap_or(c::ScatterStyleValues::Marker);
        if matches!(
          style,
          c::ScatterStyleValues::LineMarker
            | c::ScatterStyleValues::Marker
            | c::ScatterStyleValues::SmoothMarker
        ) {
          // ECMA-376 requires markers for these scatter styles. Marker `auto`
          // is application-defined; Office golden output for built-in chart
          // style 2 establishes this series-order sequence.
          const OFFICE_STYLE_2_MARKERS: [c::MarkerStyleValues; 3] = [
            c::MarkerStyleValues::Circle,
            c::MarkerStyleValues::Square,
            c::MarkerStyleValues::Triangle,
          ];
          for (index, series) in series[first..].iter_mut().enumerate() {
            series.automatic_marker_symbol = Some(if chart_style_id(chart_space) == Some(2) {
              OFFICE_STYLE_2_MARKERS
                .get(index)
                .copied()
                .unwrap_or(c::MarkerStyleValues::Auto)
            } else {
              c::MarkerStyleValues::Auto
            });
          }
        }
      }
      c::PlotAreaChoice::BarChart(chart) => {
        vary_colors_requested |= chart
          .vary_colors
          .as_ref()
          .is_some_and(|vary| vary.val.is_none_or(|value| value.as_bool()));
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
          axis_set_index,
          ui_language,
        );
      }
      c::PlotAreaChoice::Bar3DChart(chart) => {
        vary_colors_requested |= chart
          .vary_colors
          .as_ref()
          .is_some_and(|vary| vary.val.is_none_or(|value| value.as_bool()));
        gap_width_percent = f64::from(
          chart
            .gap_width
            .as_ref()
            .and_then(|gap| gap.val)
            .unwrap_or(150),
        );
        let first_series = series.len();
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
          axis_set_index,
          ui_language,
        );
        let shape = chart
          .shape
          .as_ref()
          .and_then(|shape| shape.val)
          .unwrap_or(c::ShapeValues::Box);
        let gap_depth_percent = f64::from(
          chart
            .gap_depth
            .as_ref()
            .and_then(|gap| gap.val)
            .unwrap_or(150),
        );
        for series in &mut series[first_series..] {
          series.shape_3d = shape;
          series.gap_depth_percent = gap_depth_percent;
        }
      }
      c::PlotAreaChoice::SurfaceChart(chart) => {
        let first_series_index = series.len();
        append_cartesian_series(
          &mut series,
          &mut categories,
          chart.surface_chart_series.iter().map(surface_series_ref),
          None,
          (
            ChartSeriesKind::Surface,
            ChartSeriesGrouping::Standard,
            false,
          ),
          axis_set_index,
          ui_language,
        );
        surface_groups.push(SurfaceChartGroup {
          first_series_index,
          series_count: series.len() - first_series_index,
          axis_set_index,
          is_3d: false,
          wireframe: chart
            .wireframe
            .as_ref()
            .is_some_and(|wireframe| wireframe.val.is_none_or(|value| value.as_bool())),
          band_fills: surface_band_fills(chart.band_formats.as_ref()),
        });
      }
      c::PlotAreaChoice::Surface3DChart(chart) => {
        let first_series_index = series.len();
        append_cartesian_series(
          &mut series,
          &mut categories,
          chart.surface_chart_series.iter().map(surface_series_ref),
          None,
          (
            ChartSeriesKind::Surface,
            ChartSeriesGrouping::Standard,
            true,
          ),
          axis_set_index,
          ui_language,
        );
        surface_groups.push(SurfaceChartGroup {
          first_series_index,
          series_count: series.len() - first_series_index,
          axis_set_index,
          is_3d: true,
          wireframe: chart
            .wireframe
            .as_ref()
            .is_some_and(|wireframe| wireframe.val.is_none_or(|value| value.as_bool())),
          band_fills: surface_band_fills(chart.band_formats.as_ref()),
        });
      }
      c::PlotAreaChoice::BubbleChart(chart) => {
        let first_series_index = series.len();
        append_cartesian_series(
          &mut series,
          &mut categories,
          chart.bubble_chart_series.iter().map(bubble_series_ref),
          chart.data_labels.as_deref(),
          (
            ChartSeriesKind::Bubble,
            ChartSeriesGrouping::Standard,
            false,
          ),
          axis_set_index,
          ui_language,
        );
        let group_bubble_3d = chart
          .bubble3_d
          .as_ref()
          .is_some_and(|bubble| bubble.val.is_none_or(|value| value.as_bool()));
        let bubble_scale_percent = chart
          .bubble_scale
          .as_ref()
          .and_then(|scale| scale.val)
          .map_or(100.0, f64::from);
        let show_negative_bubbles = chart
          .show_negative_bubbles
          .as_ref()
          .is_some_and(|show| show.val.is_none_or(|value| value.as_bool()));
        let size_represents = chart
          .size_represents
          .as_ref()
          .and_then(|size| size.val)
          .unwrap_or(c::SizeRepresentsValues::Area);
        for (target, source) in series[first_series_index..]
          .iter_mut()
          .zip(&chart.bubble_chart_series)
        {
          target.bubble_group_index = Some(bubble_group_count);
          target.bubble_scale_percent = bubble_scale_percent;
          target.bubble_size_represents = size_represents;
          target.show_negative_bubbles = show_negative_bubbles;
          target.bubble_3d = source.bubble3_d.as_ref().map_or(group_bubble_3d, |bubble| {
            bubble.val.is_none_or(|value| value.as_bool())
          });
        }
        bubble_group_count += 1;
      }
      c::PlotAreaChoice::PieChart(_)
      | c::PlotAreaChoice::Pie3DChart(_)
      | c::PlotAreaChoice::DoughnutChart(_)
      | c::PlotAreaChoice::OfPieChart(_) => {}
    }
  }

  if series.is_empty() {
    return None;
  }
  let has_explicit_categories = !categories.is_empty();
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
  let axis_sets = cartesian_axis_sets(chart_space, &axis_id_sets);
  let primary_axis_set = axis_sets.first();
  let category_axis_set_index = visible_category_axis_set_index(&axis_sets);
  let category_axis_set = axis_sets.get(category_axis_set_index).or(primary_axis_set);
  let category_axis = category_axis_set.and_then(|set| set.category_axis);
  let date_axis = category_axis_set.and_then(|set| set.date_axis);
  let horizontal_value_axis = primary_axis_set.and_then(|set| set.horizontal_value_axis);
  let value_axis = primary_axis_set
    .and_then(|set| set.vertical_value_axis)
    .or_else(|| value_axes.first().copied());
  let category_crossing_value_axis = category_axis_set
    .and_then(|set| set.vertical_value_axis)
    .or(value_axis);
  apply_axis_display_units_to_data_labels(&mut series, &axis_sets, value_axis);
  let category_axis_title_source = category_axis
    .and_then(|axis| axis.title.as_deref())
    .or_else(|| date_axis.and_then(|axis| axis.title.as_deref()))
    .or_else(|| horizontal_value_axis.and_then(|axis| axis.title.as_deref()));
  let category_axis_title =
    category_axis_title_source.and_then(|title| title_text_or_automatic(title, host, ui_language));
  let value_axis_title_source = value_axis.and_then(|axis| axis.title.as_deref());
  let value_axis_title =
    value_axis_title_source.and_then(|title| title_text_or_automatic(title, host, ui_language));
  let mut additional_axis_titles = Vec::new();
  for (axis_set_index, set) in axis_sets.iter().enumerate() {
    let horizontal_bar = axis_set_uses_horizontal_bars(&series, axis_set_index);
    if axis_set_index != category_axis_set_index {
      additional_axis_titles.extend(
        set
          .category_axis
          .and_then(|axis| {
            axis
              .title
              .as_deref()
              .map(|title| (title, axis.axis_position.val))
          })
          .or_else(|| {
            set.date_axis.and_then(|axis| {
              axis
                .title
                .as_deref()
                .map(|title| (title, axis.axis_position.val))
            })
          })
          .or_else(|| {
            set.horizontal_value_axis.and_then(|axis| {
              axis
                .title
                .as_deref()
                .map(|title| (title, axis.axis_position.val))
            })
          })
          .and_then(|(title, position)| {
            additional_axis_title(
              title,
              position,
              automatic_axis_title_rotation_for_dimension(
                position,
                ChartAxisDimension::X,
                horizontal_bar,
              ),
              host,
              ui_language,
            )
          }),
      );
    }
    if axis_set_index != 0 {
      additional_axis_titles.extend(
        set
          .vertical_value_axis
          .and_then(|axis| {
            axis
              .title
              .as_deref()
              .map(|title| (title, axis.axis_position.val))
          })
          .and_then(|(title, position)| {
            additional_axis_title(
              title,
              position,
              automatic_axis_title_rotation_for_dimension(
                position,
                ChartAxisDimension::Y,
                horizontal_bar,
              ),
              host,
              ui_language,
            )
          }),
      );
    }
    additional_axis_titles.extend(
      set
        .series_axis
        .and_then(|axis| {
          axis
            .title
            .as_deref()
            .map(|title| (title, axis.axis_position.val))
        })
        .and_then(|(title, position)| {
          additional_axis_title(
            title,
            position,
            automatic_axis_title_rotation_degrees(position),
            host,
            ui_language,
          )
        }),
    );
  }
  let series_count = series.len();
  let vary_colors_by_point = vary_colors_requested && series_count == 1;
  let legend_entry_count = if vary_colors_by_point {
    categories.len()
  } else {
    series_count
  };
  let cached_category_count = series
    .iter()
    .map(|series| series.values.len())
    .chain(std::iter::once(categories.len()))
    .max()
    .unwrap_or(0);
  let view_3d = series
    .iter()
    .any(|series| series.is_3d)
    .then(|| chart_3d_view(&chart_space.chart));

  Some(ClusteredColumnChart {
    ui_language: ui_language.map(ToOwned::to_owned),
    format_locale: format_locale.or(ui_language).map(ToOwned::to_owned),
    default_text_body_properties: chart_space
      .text_properties
      .as_deref()
      .map(|properties| properties.body_properties.as_ref()),
    title: chart_title_text(&chart_space.chart),
    title_overlay: chart_space
      .chart
      .title
      .as_deref()
      .and_then(|title| title.overlay.as_ref())
      .is_some_and(|overlay| overlay.val.is_none_or(|value| value.as_bool())),
    title_layout: chart_title_layout(&chart_space.chart),
    title_layout_container_present: chart_space
      .chart
      .title
      .as_deref()
      .is_some_and(|title| title.layout.is_some()),
    title_rotation_deg: chart_title_rotation_degrees(&chart_space.chart),
    title_vertical_anchor: chart_title_vertical_anchor(&chart_space.chart),
    has_automatic_title_marker: chart_space.chart.auto_title_deleted.is_some(),
    cached_category_count,
    has_explicit_categories,
    category_axis_title,
    category_axis_title_layout: category_axis_title_source
      .and_then(|title| chart_text_layout(title.layout.as_deref())),
    value_axis_title,
    value_axis_title_layout: value_axis_title_source
      .and_then(|title| chart_text_layout(title.layout.as_deref())),
    additional_axis_titles,
    categories,
    category_axis_values,
    category_number_format_code,
    date_1904: chart_uses_1904_date_system(chart_space),
    series,
    surface_groups,
    group_decorations,
    gap_width_percent,
    overlap_percent,
    category_axis,
    date_axis,
    horizontal_value_axis,
    category_axis_reversed: category_axis
      .and_then(|axis| axis.scaling.orientation.as_ref())
      .or_else(|| date_axis.and_then(|axis| axis.scaling.orientation.as_ref()))
      .and_then(|orientation| orientation.val)
      == Some(c::OrientationValues::MaxMin),
    category_axis_shifted: category_crossing_value_axis
      .and_then(|axis| axis.cross_between.as_ref())
      .is_none_or(|cross_between| cross_between.val == c::CrossBetweenValues::Between),
    value_axis,
    axis_sets,
    view_3d,
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
    legend_text_body_properties: chart_space
      .chart
      .legend
      .as_deref()
      .and_then(|legend| legend.text_properties.as_deref())
      .map(|properties| properties.body_properties.as_ref()),
    vary_colors_by_point,
    visible_legend_indices: visible_series_legend_indices(
      chart_space.chart.legend.as_deref(),
      legend_entry_count,
    ),
    deleted_legend_entry_indices: deleted_legend_entry_indices(chart_space.chart.legend.as_deref()),
    legend_layout: chart_space
      .chart
      .legend
      .as_deref()
      .and_then(|legend| chart_layout(legend.layout.as_deref())),
    plot_layout: chart_layout(chart_space.chart.plot_area.layout.as_deref()),
    data_table: chart_space.chart.plot_area.data_table.as_deref(),
    data_label_text_properties: chart_data_label_text_properties(chart_space),
  })
}

fn additional_axis_title<'a>(
  title: &'a c::Title,
  position: c::AxisPositionValues,
  automatic_rotation_deg: f32,
  host: ChartHostApplication,
  ui_language: Option<&str>,
) -> Option<AdditionalAxisTitle<'a>> {
  Some(AdditionalAxisTitle {
    text: title_text_or_automatic(title, host, ui_language)?,
    source: title,
    position,
    automatic_rotation_deg,
    layout: chart_text_layout(title.layout.as_deref()),
  })
}

pub(crate) fn chart_style_id(chart_space: &c::ChartSpace) -> Option<u8> {
  match chart_space.chart_space_choice.as_ref()? {
    c::ChartSpaceChoice::C14Style(style) => {
      let style = u16::from(style.val);
      (101..=148).contains(&style).then_some((style - 100) as u8)
    }
    c::ChartSpaceChoice::CStyle(style) => Some(style.val.unwrap_or(2)),
    c::ChartSpaceChoice::AlternateContent(_) => None,
  }
}

#[derive(Clone, Copy)]
enum AutomaticChartColorTransform {
  /// DrawingML `a:tint/@val`: the retained fraction of the input color.
  Tint(i32),
  /// DrawingML `a:shade/@val`: the retained fraction of the input color.
  Shade(i32),
}

/// Resolves the automatic series/data-point color defined by the classic
/// chart styles 1..=48.
///
/// ECMA-376 Part 1 §21.2.3.46 Tables 5 and 6 define two six-color patterns,
/// two neutral `dk1` tint patterns, and six single-accent fade styles in each
/// eight-style family. `[MS-OI29500]` clarifies that `accent1-6` means one
/// accent per successive style. LibreOffice
/// `drawingml::chart::DetailFormatterBase::getPhColor()` supplies the
/// interoperable cycle-fade algorithm: the leading cycle is shaded, the
/// trailing cycle is tinted, and the exclusive endpoints are -70% and +70%.
///
/// `formatting_index` is `c:ser/c:idx` for a series or the point index for a
/// vary-colors single series. It must not be replaced by display order.
pub(crate) fn automatic_chart_series_color(
  chart_style_id: u8,
  formatting_index: usize,
  maximum_formatting_index: usize,
  mut resolve_scheme: impl FnMut(a::SchemeColorValues) -> Option<RgbColor>,
) -> Option<RgbColor> {
  const PATTERN_1_TINTS: [i32; 6] = [88_500, 55_000, 78_000, 92_500, 70_000, 30_000];
  const PATTERN_4_TINTS: [i32; 6] = [5_000, 55_000, 78_000, 15_000, 70_000, 30_000];

  let style = if (1..=48).contains(&chart_style_id) {
    chart_style_id
  } else {
    2
  };
  let family_slot = (style - 1) % 8;
  let (scheme, base_transform, pattern_length) = match family_slot {
    0 => {
      let pattern_index = formatting_index % PATTERN_1_TINTS.len();
      let tint = if style == 41 {
        PATTERN_4_TINTS[pattern_index]
      } else {
        PATTERN_1_TINTS[pattern_index]
      };
      (
        a::SchemeColorValues::Dark1,
        Some(AutomaticChartColorTransform::Tint(tint)),
        PATTERN_1_TINTS.len(),
      )
    }
    // Styles 2/10/18/26/34/42 use Pattern 2. In particular, ECMA Table 5
    // resolves style 42 as Pattern 2; this avoids inheriting LibreOffice's
    // overlapping 41..=42 Pattern 4 lookup range.
    1 => (chart_accent_scheme_color(formatting_index % 6), None, 6),
    // The remaining six styles in each family select one accent and fade it
    // across the formatting-index stream.
    2..=7 => (
      chart_accent_scheme_color(usize::from(family_slot - 2)),
      None,
      1,
    ),
    _ => unreachable!(),
  };

  let mut color = resolve_scheme(scheme)?;
  if let Some(transform) = base_transform {
    color = apply_automatic_chart_color_transform(color, transform);
  }

  let cycle_index = formatting_index / pattern_length;
  let maximum_cycle_index = maximum_formatting_index.max(formatting_index) / pattern_length;
  let shade_or_tint = (cycle_index + 1) as f64 / (maximum_cycle_index + 2) as f64 * 1.4 - 0.7;
  let transform = if shade_or_tint < -f64::EPSILON {
    Some(AutomaticChartColorTransform::Shade(
      ((1.0 + shade_or_tint) * 100_000.0).round() as i32,
    ))
  } else if shade_or_tint > f64::EPSILON {
    Some(AutomaticChartColorTransform::Tint(
      ((1.0 - shade_or_tint) * 100_000.0).round() as i32,
    ))
  } else {
    None
  };
  Some(transform.map_or(color, |transform| {
    apply_automatic_chart_color_transform(color, transform)
  }))
}

fn chart_accent_scheme_color(index: usize) -> a::SchemeColorValues {
  [
    a::SchemeColorValues::Accent1,
    a::SchemeColorValues::Accent2,
    a::SchemeColorValues::Accent3,
    a::SchemeColorValues::Accent4,
    a::SchemeColorValues::Accent5,
    a::SchemeColorValues::Accent6,
  ][index % 6]
}

fn apply_automatic_chart_color_transform(
  color: RgbColor,
  transform: AutomaticChartColorTransform,
) -> RgbColor {
  let rgb = [color.r, color.g, color.b];
  let [r, g, b] = match transform {
    AutomaticChartColorTransform::Tint(retention) => {
      color_math::drawingml_tint_srgb8(rgb, retention)
    }
    AutomaticChartColorTransform::Shade(retention) => {
      color_math::drawingml_shade_srgb8(rgb, retention)
    }
  };
  RgbColor { r, g, b }
}

pub(crate) fn automatic_linear_series_line_width_scale(chart_space: &c::ChartSpace) -> f32 {
  // LibreOffice ObjectFormatter::spLinearSeriesLines maps the classic 1..48
  // chart styles to a percentage of the theme's subtle line. c14 styles
  // 101..148 are the corresponding modern aliases and chart_style_id()
  // normalizes them back to that range.
  match chart_style_id(chart_space).unwrap_or(2) {
    1..=8 => 3.0,
    25..=32 => 7.0,
    9..=24 | 33..=48 => 5.0,
    _ => 3.0,
  }
}

fn append_cartesian_series<'a>(
  target: &mut Vec<ClusteredColumnSeries<'a>>,
  categories: &mut Vec<String>,
  sources: impl Iterator<Item = ChartSeriesRef<'a>>,
  chart_group_labels: Option<&'a c::DataLabels>,
  series_spec: (ChartSeriesKind, ChartSeriesGrouping, bool),
  axis_set_index: usize,
  ui_language: Option<&str>,
) {
  let (kind, grouping, is_3d) = series_spec;
  for source in sources {
    let series_index = target.len() + 1;
    let explicit_name = source
      .series_text
      .map(series_text_value)
      .filter(|value| !value.is_empty());
    let name = explicit_name
      .clone()
      .unwrap_or_else(|| default_series_label(source, series_index, ui_language));
    let source_categories = source
      .category_axis_data
      .map(indexed_category_axis_text_values)
      .unwrap_or_default();
    if categories.is_empty() && !source_categories.is_empty() {
      categories.clone_from(&source_categories);
    }
    let values = chart_series_numeric_values(source);
    let x_values = chart_series_x_numeric_values(source);
    let error_bars = resolved_error_bars(source.error_bars);
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
      formatting_index: source.formatting_index,
      name: name.clone(),
      has_explicit_name: source.series_text.is_some(),
      has_nonempty_explicit_name: explicit_name.is_some(),
      shape_properties: source.chart_shape_properties,
      data_points: source.data_points,
      name_formula: series_name_formula(source),
      category_formula: series_category_formula(source),
      value_formula: series_value_formula(source),
      x_value_formula: series_x_value_formula(source),
      x_values,
      x_number_format_code: series_x_number_format_code(source),
      bubble_size_formula: series_bubble_size_formula(source),
      bubble_sizes: bubble_sizes.clone(),
      bubble_group_index: None,
      bubble_scale_percent: 100.0,
      bubble_size_represents: c::SizeRepresentsValues::Area,
      show_negative_bubbles: false,
      bubble_3d: false,
      solid_fill: source.chart_shape_properties.and_then(|properties| {
        chart_shape_solid_fill(properties).or_else(|| chart_shape_outline_solid_fill(properties))
      }),
      data_point_fills,
      data_labels: resolved_data_labels(
        source.data_labels,
        chart_group_labels,
        DataLabelSeriesData {
          series_name: &name,
          categories: &label_categories,
          values: &values,
          bubble_sizes: (!bubble_sizes.is_empty()).then_some(bubble_sizes.as_slice()),
          data_labels_range: &data_labels_range_values(source.data_labels_range),
        },
        DataLabelDefaults {
          value_format_code: series_number_format_code(source),
          position: default_data_label_position(kind, grouping),
          supports_percent: false,
          separator: ", ",
        },
      ),
      axis_set_index,
      values,
      number_format_code: series_number_format_code(source),
      kind,
      grouping,
      is_3d,
      shape_3d: c::ShapeValues::Box,
      gap_depth_percent: 150.0,
      smooth: source
        .smooth
        .map(|smooth| smooth.val.is_none_or(|value| value.as_bool())),
      marker: source.marker,
      automatic_marker_symbol: None,
      line_hidden: source
        .chart_shape_properties
        .is_some_and(chart_shape_properties_has_no_outline),
      line_width_pt: source
        .chart_shape_properties
        .and_then(|properties| properties.outline.as_deref())
        .and_then(|outline| outline.width)
        .map(|width| units::emu_to_points(i64::from(width))),
      filled_area: false,
      trendlines: source.trendlines,
      error_bars,
    });
  }
}

fn push_cartesian_group_decorations<'a>(
  target: &mut Vec<CartesianChartGroupDecorations<'a>>,
  first_series_index: usize,
  next_series_index: usize,
  axis_set_index: usize,
  drop_lines: Option<&'a c::DropLines>,
  high_low_lines: Option<&'a c::HighLowLines>,
  up_down_bars: Option<&'a c::UpDownBars>,
) {
  let series_count = next_series_index.saturating_sub(first_series_index);
  if series_count == 0
    || (drop_lines.is_none() && high_low_lines.is_none() && up_down_bars.is_none())
  {
    return;
  }
  target.push(CartesianChartGroupDecorations {
    first_series_index,
    series_count,
    axis_set_index,
    drop_lines,
    high_low_lines,
    up_down_bars,
  });
}

fn resolved_error_bars<'a>(sources: [Option<&'a c::ErrorBars>; 2]) -> Vec<ChartErrorBars<'a>> {
  sources
    .into_iter()
    .flatten()
    .map(|source| {
      let (show_positive, show_negative) = match source.error_bar_type.val {
        c::ErrorBarValues::Both => (true, true),
        c::ErrorBarValues::Minus => (false, true),
        c::ErrorBarValues::Plus => (true, false),
      };
      let scalar = source
        .error_bar_value
        .as_ref()
        .map_or(0.0, |value| value.val);
      let values = match source.error_bar_value_type.val {
        c::ErrorValues::Custom => {
          let (positive_formula, positive_values) = source
            .plus
            .as_deref()
            .map(error_bar_positive_source)
            .unwrap_or_default();
          let (negative_formula, negative_values) = source
            .minus
            .as_deref()
            .map(error_bar_negative_source)
            .unwrap_or_default();
          ChartErrorBarValues::Custom {
            positive_formula,
            positive_values,
            negative_formula,
            negative_values,
          }
        }
        c::ErrorValues::FixedValue => ChartErrorBarValues::Fixed(scalar),
        c::ErrorValues::Percentage => ChartErrorBarValues::Percentage(scalar),
        c::ErrorValues::StandardDeviation => ChartErrorBarValues::StandardDeviation(scalar),
        c::ErrorValues::StandardError => ChartErrorBarValues::StandardError,
      };
      ChartErrorBars {
        direction: source
          .error_direction
          .as_ref()
          .map_or(c::ErrorBarDirectionValues::Y, |direction| direction.val),
        show_positive,
        show_negative,
        no_end_cap: source
          .no_end_cap
          .as_ref()
          .is_some_and(|value| value.val.is_none_or(|value| value.as_bool())),
        values,
        shape_properties: source.chart_shape_properties.as_deref(),
      }
    })
    .collect()
}

fn error_bar_positive_source(source: &c::Plus) -> (Option<&str>, Vec<Option<f64>>) {
  match source.plus_choice.as_ref() {
    Some(c::PlusChoice::NumberReference(reference)) => (
      reference.formula.xml_content.as_deref(),
      reference
        .numbering_cache
        .as_deref()
        .map(|cache| indexed_numeric_values(&cache.numeric_point))
        .unwrap_or_default(),
    ),
    Some(c::PlusChoice::NumberLiteral(literal)) => {
      (None, indexed_numeric_values(&literal.numeric_point))
    }
    None => (None, Vec::new()),
  }
}

fn error_bar_negative_source(source: &c::Minus) -> (Option<&str>, Vec<Option<f64>>) {
  match source.minus_choice.as_ref() {
    Some(c::MinusChoice::NumberReference(reference)) => (
      reference.formula.xml_content.as_deref(),
      reference
        .numbering_cache
        .as_deref()
        .map(|cache| indexed_numeric_values(&cache.numeric_point))
        .unwrap_or_default(),
    ),
    Some(c::MinusChoice::NumberLiteral(literal)) => {
      (None, indexed_numeric_values(&literal.numeric_point))
    }
    None => (None, Vec::new()),
  }
}

fn surface_band_fills(band_formats: Option<&c::BandFormats>) -> Vec<ChartDataPointFill<'_>> {
  let mut fills = band_formats
    .into_iter()
    .flat_map(|formats| &formats.band_format)
    .filter_map(|format| {
      let fill = format
        .chart_shape_properties
        .as_deref()
        .and_then(chart_shape_solid_fill)?;
      Some(ChartDataPointFill {
        index: format.index.val,
        fill,
      })
    })
    .collect::<Vec<_>>();
  fills.sort_by_key(|fill| fill.index);
  fills
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

fn chart_3d_view(chart: &c::Chart) -> Chart3DView {
  let Some(view) = chart.view3_d.as_deref() else {
    return Chart3DView::default();
  };
  let right_angle_axes = view
    .right_angle_axes
    .as_ref()
    .is_some_and(|right_angle| right_angle.val.is_none_or(|value| value.as_bool()));
  Chart3DView {
    rotate_x_deg: f32::from(
      view
        .rotate_x
        .as_ref()
        .and_then(|rotation| rotation.val)
        .unwrap_or(15),
    ),
    rotate_y_deg: f32::from(
      view
        .rotate_y
        .as_ref()
        .and_then(|rotation| rotation.val)
        // LibreOffice's OOXML converter applies Office's type-dependent
        // default to 3-D bar/area/line charts even with right-angle axes.
        .unwrap_or(20),
    ),
    height_percent: f32::from(
      view
        .height_percent
        .as_ref()
        .and_then(|height| height.val)
        .unwrap_or(100),
    ),
    height_percent_is_explicit: view.height_percent.is_some(),
    depth_percent: f32::from(
      view
        .depth_percent
        .as_ref()
        .and_then(|depth| depth.val)
        .unwrap_or(100),
    ),
    depth_percent_is_explicit: view.depth_percent.is_some(),
    right_angle_axes,
    perspective_half_degrees: f32::from(
      view
        .perspective
        .as_ref()
        .and_then(|perspective| perspective.val)
        .unwrap_or(30),
    ),
  }
}

fn default_data_label_position(
  kind: ChartSeriesKind,
  grouping: ChartSeriesGrouping,
) -> c::DataLabelPositionValues {
  match kind {
    ChartSeriesKind::Column | ChartSeriesKind::Bar
      if matches!(
        grouping,
        ChartSeriesGrouping::Stacked | ChartSeriesGrouping::PercentStacked
      ) =>
    {
      c::DataLabelPositionValues::Center
    }
    ChartSeriesKind::Column | ChartSeriesKind::Bar => c::DataLabelPositionValues::OutsideEnd,
    ChartSeriesKind::Area | ChartSeriesKind::Surface => c::DataLabelPositionValues::Center,
    ChartSeriesKind::Line
    | ChartSeriesKind::Scatter
    | ChartSeriesKind::Bubble
    | ChartSeriesKind::Radar
    | ChartSeriesKind::Stock => c::DataLabelPositionValues::Right,
  }
}

fn deleted_legend_entry_indices(legend: Option<&c::Legend>) -> Vec<usize> {
  let mut indices = legend
    .into_iter()
    .flat_map(|legend| &legend.legend_entry)
    .filter_map(|entry| {
      matches!(
        entry.legend_entry_choice.as_ref(),
        Some(c::LegendEntryChoice::Delete(delete))
          if delete.val.is_none_or(|value| value.as_bool())
      )
      .then_some(entry.index.val as usize)
    })
    .collect::<Vec<_>>();
  indices.sort_unstable();
  indices.dedup();
  indices
}

fn visible_series_legend_indices(legend: Option<&c::Legend>, count: usize) -> Vec<usize> {
  let deleted = deleted_legend_entry_indices(legend);
  (0..count)
    .filter(|index| deleted.binary_search(index).is_err())
    .collect()
}

pub(crate) fn trendline_legend_title<'a>(
  trendline: &'a c::Trendline,
  series_name: &str,
  ui_language: Option<&str>,
) -> Cow<'a, str> {
  if let Some(name) = trendline
    .trendline_name
    .as_deref()
    .filter(|name| !name.is_empty())
  {
    return Cow::Borrowed(name);
  }
  let kind = match trendline
    .trendline_type
    .val
    .unwrap_or(c::TrendlineValues::Linear)
  {
    c::TrendlineValues::Linear => ChartTrendlineKind::Linear,
    c::TrendlineValues::Logarithmic => ChartTrendlineKind::Logarithmic,
    c::TrendlineValues::Exponential => ChartTrendlineKind::Exponential,
    c::TrendlineValues::Power => ChartTrendlineKind::Power,
    c::TrendlineValues::Polynomial => ChartTrendlineKind::Polynomial,
    c::TrendlineValues::MovingAverage => ChartTrendlineKind::MovingAverage,
  };
  Cow::Owned(
    OfficeStringCatalog::for_ui_language(ui_language)
      .chart_trendline_legend_title(kind, series_name),
  )
}

fn chart_layout(layout: Option<&c::Layout>) -> Option<ChartManualLayout> {
  chart_manual_layout(layout?.manual_layout.as_deref()?)
}

fn chart_manual_layout(manual: &c::ManualLayout) -> Option<ChartManualLayout> {
  let mode = |value: Option<c::LayoutModeValues>| match value {
    Some(c::LayoutModeValues::Edge) => ChartLayoutMode::Edge,
    Some(c::LayoutModeValues::Factor) | None => ChartLayoutMode::Factor,
  };
  Some(ChartManualLayout {
    targets_inner_plot: manual.layout_target.as_ref().and_then(|target| target.val)
      == Some(c::LayoutTargetValues::Inner),
    x: manual.left.as_ref().map(|value| value.val as f32),
    y: manual.top.as_ref().map(|value| value.val as f32),
    width: manual.width.as_ref().map(|value| value.val as f32),
    height: manual.height.as_ref().map(|value| value.val as f32),
    x_mode: mode(manual.left_mode.as_ref().and_then(|value| value.val)),
    y_mode: mode(manual.top_mode.as_ref().and_then(|value| value.val)),
    width_mode: mode(manual.width_mode.as_ref().and_then(|value| value.val)),
    height_mode: mode(manual.height_mode.as_ref().and_then(|value| value.val)),
  })
}

/// Office treats title and individual data-label manual layouts as positions,
/// not resizable plot rectangles. MS-OI29500 §21.2.2.78/229 ignores `h` and
/// `w` for those owners and §21.2.2.104 requires `x` and `y` together.
fn chart_text_layout(layout: Option<&c::Layout>) -> Option<ChartManualLayout> {
  chart_manual_text_layout(layout?.manual_layout.as_deref()?)
}

fn chart_manual_text_layout(manual: &c::ManualLayout) -> Option<ChartManualLayout> {
  let mut layout = chart_manual_layout(manual)?;
  let x = layout.x?;
  let y = layout.y?;
  let valid_coordinate = |value: f32, mode: ChartLayoutMode| {
    value.is_finite()
      && match mode {
        ChartLayoutMode::Edge => (0.0..=1.0).contains(&value),
        ChartLayoutMode::Factor => (-1.0..=1.0).contains(&value),
      }
  };
  if !valid_coordinate(x, layout.x_mode) || !valid_coordinate(y, layout.y_mode) {
    return None;
  }
  layout.targets_inner_plot = false;
  layout.width = None;
  layout.height = None;
  Some(layout)
}

fn data_label_text_layout(
  label: &c::DataLabel,
  legacy_layout: Option<&c::Layout>,
) -> Option<ChartManualLayout> {
  // Office 2013 introduced c15:layout specifically for a data label or its
  // parent dLbls object. Its presence is the current layout state; an empty
  // extension therefore means automatic placement and supersedes a legacy
  // c:layout retained for older consumers. PowerPoint emits both forms in
  // percentage-number-formats.pptx, and its fixed output follows the empty
  // c15 form rather than applying the stale legacy offset.
  if let Some(layout) = data_label_extension_layout(label) {
    let manual = layout.manual_layout.as_deref()?;
    if manual.left.is_some() || manual.top.is_some() {
      return chart_manual_text_layout(manual);
    }
    if manual.width.is_some() || manual.height.is_some() {
      // PowerPoint 2013+ splits a manually sized label across the two
      // compatibility representations: c15:layout carries the current w/h,
      // while c:layout retains the x/y factor offset from the automatic
      // position. tdf146487 is a fixed-output counterexample to treating the
      // legacy coordinates as stale when the modern layout is non-empty.
      return chart_text_layout(legacy_layout);
    }
    return None;
  }
  chart_text_layout(legacy_layout)
}

fn data_label_extension_layout(label: &c::DataLabel) -> Option<&c15::Layout> {
  label
    .d_lbl_extension_list
    .as_ref()
    .into_iter()
    .flat_map(|list| &list.d_lbl_extension)
    .flat_map(|extension| &extension.d_lbl_extension_choice)
    .find_map(|choice| match choice {
      c::DLblExtensionChoice::Layout(layout) => Some(layout.as_ref()),
      _ => None,
    })
}

fn data_labels_extension_layout(labels: &c::DataLabels) -> Option<&c15::Layout> {
  labels
    .d_lbls_extension_list
    .as_ref()
    .into_iter()
    .flat_map(|list| &list.d_lbls_extension)
    .flat_map(|extension| &extension.d_lbls_extension_choice)
    .find_map(|choice| match choice {
      c::DLblsExtensionChoice::Layout(layout) => Some(layout.as_ref()),
      _ => None,
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
  let explicit_series_name = series_ref
    .series_text
    .map(series_text_value)
    .filter(|value| !value.is_empty());
  let series_name = explicit_series_name
    .clone()
    .unwrap_or_else(|| default_series_label(series_ref, 1, None));
  let title = match chart_title_text(&chart_space.chart) {
    None
      if chart_space
        .chart
        .title
        .as_deref()
        .is_some_and(|title| explicit_title_text(title).is_none())
        && chart_automatic_title_is_visible(&chart_space.chart)
        && explicit_series_name.is_some() =>
    {
      // LibreOffice ChartSpaceConverter::convertFromModel derives the
      // automatic chart title from an authored series title. Pie-family
      // charts paint only their first series, so later cached series do not
      // prevent that first title from becoming the automatic title. Office
      // does not promote the generated Row/Column fallback to a chart title.
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
    DataLabelSeriesData {
      series_name: &series_name,
      categories: &categories,
      values: &values,
      bubble_sizes: None,
      data_labels_range: &data_labels_range_values(series_ref.data_labels_range),
    },
    DataLabelDefaults {
      value_format_code: series_number_format_code(series_ref),
      position: if radial_kind == RadialChartKind::Doughnut {
        c::DataLabelPositionValues::Center
      } else {
        c::DataLabelPositionValues::BestFit
      },
      supports_percent: true,
      separator: ", ",
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

  let mut point_explosion_percent = vec![None; values.len()];
  for point in &series.data_point {
    if let (Ok(index), Some(explosion)) =
      (usize::try_from(point.index.val), point.explosion.as_ref())
      && let Some(target) = point_explosion_percent.get_mut(index)
    {
      *target = Some(f64::from(explosion.val));
    }
  }
  let secondary_indices = of_pie
    .map(|chart| of_pie_secondary_indices(chart, &values))
    .unwrap_or_default();
  let maximum_series_formatting_index = pie_series
    .iter()
    .map(|series| series.index.as_ref().map_or(0, |index| index.val as usize))
    .max()
    .unwrap_or(0);

  Some(PieChartModel {
    kind: radial_kind,
    view_3d: (radial_kind == RadialChartKind::Pie3D).then(|| chart_3d_view(&chart_space.chart)),
    title,
    title_layout: chart_title_layout(&chart_space.chart),
    title_rotation_deg: chart_title_rotation_degrees(&chart_space.chart),
    series_formatting_index: series_ref.formatting_index,
    maximum_series_formatting_index,
    series_name,
    has_nonempty_explicit_series_name: explicit_series_name.is_some(),
    categories,
    values,
    series_shape_properties: series.chart_shape_properties.as_deref(),
    data_points: &series.data_point,
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
    legend_vertical_anchor: legend
      .and_then(|legend| legend.text_properties.as_deref())
      .and_then(|properties| properties.body_properties.anchor),
    legend_text_body_properties: legend
      .and_then(|legend| legend.text_properties.as_deref())
      .map(|properties| properties.body_properties.as_ref()),
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
    leader_line_shape_properties: series
      .data_labels
      .as_deref()
      .and_then(data_labels_leader_line_shape_properties)
      .or_else(|| chart_group_labels.and_then(data_labels_leader_line_shape_properties)),
  })
}

fn data_labels_show_leader_lines(labels: &c::DataLabels) -> Option<bool> {
  // Office 2013 writes the effective state into c15:showLeaderLines while
  // retaining a stale classic value for down-level consumers.  LibreOffice's
  // DataLabelsContext feeds both through the same model in document order, so
  // the extension wins when present.
  labels
    .d_lbls_extension_list
    .as_ref()
    .into_iter()
    .flat_map(|list| &list.d_lbls_extension)
    .flat_map(|extension| &extension.d_lbls_extension_choice)
    .find_map(|choice| match choice {
      c::DLblsExtensionChoice::ShowLeaderLines(show) => {
        Some(show.val.is_none_or(|value| value.as_bool()))
      }
      _ => None,
    })
    .or_else(|| {
      let c::DataLabelsChoice::Sequence(sequence) = labels.data_labels_choice.as_ref()? else {
        return None;
      };
      sequence
        .show_leader_lines
        .as_ref()
        .map(|show| show.val.is_none_or(|value| value.as_bool()))
    })
}

fn data_labels_leader_line_shape_properties(
  labels: &c::DataLabels,
) -> Option<&c::ChartShapeProperties> {
  labels
    .d_lbls_extension_list
    .as_ref()
    .into_iter()
    .flat_map(|list| &list.d_lbls_extension)
    .flat_map(|extension| &extension.d_lbls_extension_choice)
    .find_map(|choice| match choice {
      c::DLblsExtensionChoice::LeaderLines(lines) => lines.chart_shape_properties.as_deref(),
      _ => None,
    })
    .or_else(|| {
      let c::DataLabelsChoice::Sequence(sequence) = labels.data_labels_choice.as_ref()? else {
        return None;
      };
      sequence
        .leader_lines
        .as_deref()?
        .chart_shape_properties
        .as_deref()
    })
}

fn data_labels_show_range(labels: &c::DataLabels) -> Option<bool> {
  labels
    .d_lbls_extension_list
    .as_ref()
    .into_iter()
    .flat_map(|list| &list.d_lbls_extension)
    .flat_map(|extension| &extension.d_lbls_extension_choice)
    .find_map(|choice| match choice {
      c::DLblsExtensionChoice::ShowDataLabelsRange(show) => {
        Some(show.val.is_none_or(|value| value.as_bool()))
      }
      _ => None,
    })
}

fn data_label_show_range(label: &c::DataLabel) -> Option<bool> {
  label
    .d_lbl_extension_list
    .as_ref()
    .into_iter()
    .flat_map(|list| &list.d_lbl_extension)
    .flat_map(|extension| &extension.d_lbl_extension_choice)
    .find_map(|choice| match choice {
      c::DLblExtensionChoice::ShowDataLabelsRange(show) => {
        Some(show.val.is_none_or(|value| value.as_bool()))
      }
      _ => None,
    })
}

fn of_pie_secondary_indices(chart: &c::OfPieChart, values: &[Option<f64>]) -> Vec<usize> {
  if values.len() <= 1 {
    return Vec::new();
  }
  let automatic_split = chart.split_type.is_none();
  let split_type = chart
    .split_type
    .as_ref()
    .map_or(c::SplitValues::Position, |split| split.val);
  let split_position = chart.split_position.as_ref().map_or(
    if automatic_split {
      values.len().div_ceil(3) as f64
    } else {
      2.0
    },
    |position| position.val,
  );
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

struct DataLabelSeriesData<'a> {
  series_name: &'a str,
  categories: &'a [String],
  values: &'a [Option<f64>],
  bubble_sizes: Option<&'a [Option<f64>]>,
  data_labels_range: &'a [String],
}

fn resolved_data_labels<'a>(
  series_labels: Option<&'a c::DataLabels>,
  chart_group_labels: Option<&'a c::DataLabels>,
  data: DataLabelSeriesData<'_>,
  defaults: DataLabelDefaults<'a>,
) -> Vec<ClusteredColumnDataLabel<'a>> {
  let DataLabelSeriesData {
    series_name,
    categories,
    values,
    bubble_sizes,
    data_labels_range,
  } = data;
  // ECMA-376 Part 1 §21.2.2.49 defines c:dLbls as the settings for an
  // entire series or chart. MS-OI29500 §21.2.2.49 adds the Office override
  // hierarchy: chart-group dLbls < series dLbls < individual dLbl. Expand the
  // resolved series settings across every point, then apply point overrides.
  let mut settings = ClusteredColumnDataLabelSettings {
    separator: defaults.separator,
    position: defaults.position,
    value_format_code: defaults.value_format_code,
    use_pie_separator_default: defaults.supports_percent,
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
  let percentage_values_are_valid = values
    .iter()
    .flatten()
    .all(|value| value.is_finite() && *value >= 0.0);
  let whole_percentages =
    (defaults.supports_percent && percentage_values_are_valid && percentage_total > f64::EPSILON)
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
      let mut point_layout = None;
      let mut text_frame_layout = settings.text_frame_layout;
      let mut text_properties = series_labels
        .and_then(data_labels_text_properties)
        .or_else(|| chart_group_labels.and_then(data_labels_text_properties));
      let mut custom_chart_text = None;
      if let Some(label) = point_labels[point_index] {
        if let Some(layout) = data_label_extension_layout(label) {
          text_frame_layout = layout
            .manual_layout
            .as_deref()
            .and_then(chart_manual_layout);
        }
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
          point_layout = data_label_text_layout(label, sequence.layout.as_deref());
          if sequence.text_properties.is_some() {
            text_properties = sequence.text_properties.as_deref();
          }
          if let Some(chart_text) = sequence.chart_text.as_deref() {
            // MS-OI29500 §21.2.2.47: when c:tx is present Office ignores
            // the component-selection fields on the same individual label.
            // Position, number format, and shape properties remain the
            // presentation state of that custom text.
            custom_chart_text = Some(chart_text);
            apply_data_label_sequence_presentation_settings(&mut point_settings, sequence);
          } else {
            apply_data_label_sequence_settings(&mut point_settings, sequence);
          }
        }
        if let Some(show) = data_label_show_range(label) {
          point_settings.show_data_labels_range = show;
        }
      }

      if point_settings.deleted {
        return None;
      }
      let text_body_properties = custom_chart_text
        .and_then(|chart_text| match chart_text.chart_text_choice.as_ref() {
          Some(c::ChartTextChoice::RichText(rich)) => Some(rich.body_properties.as_ref()),
          Some(c::ChartTextChoice::StringReference(_))
          | Some(c::ChartTextChoice::StringLiteral(_))
          | None => None,
        })
        .or_else(|| text_properties.map(|properties| properties.body_properties.as_ref()));
      let percentage_text = (point_settings.show_percent
        && percentage_total.abs() > f64::EPSILON)
        .then(|| {
          point_settings.percentage_format_code.map_or_else(
            || {
              whole_percentages.as_ref().map_or_else(
                || format_chart_number(value / percentage_total, Some("0%")),
                |percentages| format!("{}%", percentages[point_index] as i32),
              )
            },
            |format| {
              if percentage_format_has_fractional_placeholder(format) {
                format_chart_number(value / percentage_total, Some(format))
              } else {
                whole_percentages.as_ref().map_or_else(
                  || format_chart_number(value / percentage_total, Some(format)),
                  |percentages| format!("{}%", percentages[point_index] as i32),
                )
              }
            },
          )
        });
      let range_text = if point_settings.show_data_labels_range {
        data_labels_range
          .get(point_index)
          .map(String::as_str)
          .filter(|value| !value.is_empty())
      } else {
        None
      };
      let point_context = DataLabelPointContext {
        series_name,
        category_name: categories.get(point_index).map(String::as_str),
        value,
        value_format_code: point_settings.value_format_code,
        bubble_size: bubble_sizes
          .and_then(|sizes| sizes.get(point_index))
          .copied()
          .flatten(),
        data_labels_range: range_text,
      };
      let custom_text = custom_chart_text.map(|chart_text| {
        data_label_chart_text(
          chart_text,
          point_labels[point_index],
          point_context,
          percentage_text.as_deref(),
        )
      });
      let value_component_index = (custom_text.is_none() && point_settings.show_value).then(|| {
        usize::from(range_text.is_some())
          + usize::from(point_settings.show_series_name && !series_name.is_empty())
          + usize::from(
            point_settings.show_category_name
              && categories
                .get(point_index)
                .is_some_and(|category| !category.is_empty()),
          )
      });
      let (text, text_components, separator, rich_text_runs) = match custom_text {
        Some(text) if !text.text.is_empty() => {
          (text.text, text.lines, "\n", text.rich_text_runs)
        }
        Some(_) => return None,
        None => {
          let (text, components, separator) = compose_clustered_column_data_label(
            point_settings,
            point_context,
            point_settings
              .show_percent
              .then_some(percentage_text.clone())
              .flatten(),
          )?;
          (text, components, separator, Vec::new())
        }
      };
      Some(ClusteredColumnDataLabel {
        point_index,
        text,
        text_components,
        value_component_index,
        rich_text_runs,
        value_format_code: point_settings.value_format_code,
        separator,
        position: point_settings.position,
        layout: point_layout,
        text_frame_layout,
        text_properties,
        text_body_properties,
        shape_properties: point_settings.shape_properties,
      })
    })
    .collect()
}

fn percentage_format_has_fractional_placeholder(format_code: &str) -> bool {
  let mut quoted = false;
  let mut bracketed = false;
  let mut escaped = false;
  let mut after_decimal = false;
  let mut chars = format_code.chars();
  while let Some(character) = chars.next() {
    if escaped {
      escaped = false;
      continue;
    }
    if quoted {
      if character == '"' {
        quoted = false;
      }
      continue;
    }
    if bracketed {
      if character == ']' {
        bracketed = false;
      }
      continue;
    }
    match character {
      '"' => quoted = true,
      '[' => bracketed = true,
      '\\' => escaped = true,
      '_' | '*' => {
        let _ = chars.next();
      }
      ';' | '%' => break,
      '.' => after_decimal = true,
      '0' | '#' | '?' if after_decimal => return true,
      _ => {}
    }
  }
  false
}

#[derive(Clone, Copy, Debug)]
struct ClusteredColumnDataLabelSettings<'a> {
  deleted: bool,
  show_value: bool,
  show_category_name: bool,
  show_series_name: bool,
  show_percent: bool,
  show_bubble_size: bool,
  show_data_labels_range: bool,
  separator: &'a str,
  separator_explicit: bool,
  use_pie_separator_default: bool,
  value_format_code: Option<&'a str>,
  percentage_format_code: Option<&'a str>,
  position: c::DataLabelPositionValues,
  shape_properties: Option<&'a c::ChartShapeProperties>,
  text_frame_layout: Option<ChartManualLayout>,
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
      show_data_labels_range: false,
      separator: ", ",
      separator_explicit: false,
      use_pie_separator_default: false,
      value_format_code: None,
      percentage_format_code: None,
      shape_properties: None,
      text_frame_layout: None,
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
  if let Some(layout) = data_labels_extension_layout(labels) {
    settings.text_frame_layout = layout
      .manual_layout
      .as_deref()
      .and_then(chart_manual_layout);
  }
  if let Some(show) = data_labels_show_range(labels) {
    settings.show_data_labels_range = show;
  }
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

pub(crate) fn has_indexed_scatter_multicomponent_data_labels(chart_space: &c::ChartSpace) -> bool {
  chart_space
    .chart
    .plot_area
    .plot_area_choice1
    .iter()
    .filter_map(|choice| match choice {
      c::PlotAreaChoice::ScatterChart(chart) => Some(chart),
      _ => None,
    })
    .any(|chart| {
      chart.scatter_chart_series.iter().any(|series| {
        let has_indexed_x_values = series.x_values.as_deref().is_some_and(|values| {
          matches!(
            values.x_values_choice.as_ref(),
            Some(
              c::XValuesChoice::MultiLevelStringReference(_)
                | c::XValuesChoice::StringReference(_)
                | c::XValuesChoice::StringLiteral(_)
            )
          )
        });
        if !has_indexed_x_values {
          return false;
        }

        let mut settings = ClusteredColumnDataLabelSettings::default();
        apply_data_labels_settings(&mut settings, chart.data_labels.as_deref());
        apply_data_labels_settings(&mut settings, series.data_labels.as_deref());
        !settings.deleted
          && [
            settings.show_value,
            settings.show_category_name,
            settings.show_series_name,
            settings.show_percent,
            settings.show_bubble_size,
            settings.show_data_labels_range,
          ]
          .into_iter()
          .filter(|show| *show)
          .count()
            > 1
      })
    })
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
    settings.separator_explicit = true;
  }
  if let Some(format) = sequence.numbering_format.as_ref() {
    let format = format.format_code.as_str();
    if settings.use_pie_separator_default && settings.show_percent {
      settings.percentage_format_code = Some(if format.eq_ignore_ascii_case("General") {
        "0%"
      } else {
        format
      });
    } else {
      settings.value_format_code = Some(format);
    }
  }
  if let Some(position) = sequence.data_label_position.as_ref() {
    settings.position = position.val;
  }
  if let Some(properties) = sequence.chart_shape_properties.as_deref() {
    settings.shape_properties = Some(properties);
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
    settings.separator_explicit = true;
  }
  apply_data_label_sequence_presentation_settings(settings, sequence);
}

fn apply_data_label_sequence_presentation_settings<'a>(
  settings: &mut ClusteredColumnDataLabelSettings<'a>,
  sequence: &'a c::DataLabelChoiceSequence,
) {
  if let Some(format) = sequence.numbering_format.as_ref() {
    let format = format.format_code.as_str();
    if settings.use_pie_separator_default && settings.show_percent {
      settings.percentage_format_code = Some(if format.eq_ignore_ascii_case("General") {
        "0%"
      } else {
        format
      });
    } else {
      settings.value_format_code = Some(format);
    }
  }
  if let Some(position) = sequence.data_label_position.as_ref() {
    settings.position = position.val;
  }
  if let Some(properties) = sequence.chart_shape_properties.as_deref() {
    settings.shape_properties = Some(properties);
  }
}

#[derive(Clone, Copy)]
struct DataLabelPointContext<'a> {
  series_name: &'a str,
  category_name: Option<&'a str>,
  value: f64,
  value_format_code: Option<&'a str>,
  bubble_size: Option<f64>,
  data_labels_range: Option<&'a str>,
}

fn compose_clustered_column_data_label<'a>(
  settings: ClusteredColumnDataLabelSettings<'a>,
  context: DataLabelPointContext<'_>,
  percentage: Option<String>,
) -> Option<(String, Vec<String>, &'a str)> {
  let DataLabelPointContext {
    series_name,
    category_name: category,
    value,
    value_format_code,
    bubble_size,
    data_labels_range,
  } = context;
  let mut components = Vec::with_capacity(5);
  // [MS-ODRAWXML] §2.3.58 requires the value-from-cells field to be the
  // first field in the visible label, ahead of the classic series/category/
  // value/percentage components.
  if let Some(value) = data_labels_range.filter(|value| !value.is_empty()) {
    components.push(value.to_string());
  }
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
  let separator = if !settings.separator_explicit
    && settings.use_pie_separator_default
    && settings.show_category_name
    && settings.show_percent
    && !settings.show_series_name
    && !settings.show_value
    && !settings.show_bubble_size
    && !settings.show_data_labels_range
  {
    "\n"
  } else {
    settings.separator
  };
  (!components.is_empty()).then(|| {
    let text = components.join(separator);
    (text, components, separator)
  })
}

fn largest_remainder_percentages(values: &[Option<f64>], total: f64) -> Vec<f64> {
  if !total.is_finite()
    || total <= f64::EPSILON
    || values
      .iter()
      .flatten()
      .any(|value| !value.is_finite() || *value < 0.0)
  {
    return vec![0.0; values.len()];
  }
  let mut percentages = values
    .iter()
    .map(|value| {
      value.map_or((0.0, 0.0), |value| {
        let scaled = value * 100.0;
        let floor = (scaled / total).floor();
        // Compare equivalent unnormalised remainders. Computing `fract()`
        // after division can erase the source distinction between OOXML
        // decimals such as 8.1999999999999993 and 1.2 when their sum rounds
        // to 14.0. Multiplication/subtraction retains that evidence while an
        // exact tie (for example three equal values) still uses point order.
        (floor, scaled - floor * total)
      })
    })
    .collect::<Vec<_>>();
  let mut remaining = 100_i64
    - percentages
      .iter()
      .map(|(floor, _)| *floor as i64)
      .sum::<i64>();
  let mut order = (0..percentages.len()).collect::<Vec<_>>();
  order.sort_by(|left, right| {
    percentages[*right]
      .1
      .total_cmp(&percentages[*left].1)
      .then_with(|| left.cmp(right))
  });
  for index in order {
    percentages[index].0 += f64::from(remaining > 0);
    remaining -= i64::from(remaining > 0);
  }
  percentages.into_iter().map(|(value, _)| value).collect()
}

struct ResolvedDataLabelChartText<'a> {
  text: String,
  lines: Vec<String>,
  rich_text_runs: Vec<ChartDataLabelTextRun<'a>>,
}

fn data_label_chart_text<'a>(
  chart_text: &'a c::ChartText,
  data_label: Option<&'a c::DataLabel>,
  context: DataLabelPointContext<'_>,
  percentage: Option<&str>,
) -> ResolvedDataLabelChartText<'a> {
  let DataLabelPointContext {
    series_name,
    category_name,
    value,
    value_format_code,
    bubble_size,
    data_labels_range,
  } = context;
  let Some(c::ChartTextChoice::RichText(rich)) = chart_text.chart_text_choice.as_ref() else {
    let mut values = Vec::new();
    push_chart_text(&mut values, chart_text);
    let text = values.join(" ");
    return ResolvedDataLabelChartText {
      lines: vec![text.clone()],
      text,
      rich_text_runs: Vec::new(),
    };
  };
  let mut rich_text_runs = Vec::new();
  let mut line_index = 0;
  for (paragraph_index, paragraph) in rich.paragraph.iter().enumerate() {
    if paragraph_index > 0 {
      line_index += 1;
    }
    let paragraph_default_run_properties = paragraph
      .paragraph_properties
      .as_deref()
      .and_then(|properties| properties.default_run_properties.as_deref());
    for choice in &paragraph.paragraph_choice {
      match choice {
        a::ParagraphChoice::Run(run) => push_data_label_rich_text_run(
          &mut rich_text_runs,
          &mut line_index,
          &run.text,
          paragraph_default_run_properties,
          run.run_properties.as_deref(),
        ),
        a::ParagraphChoice::Field(field) => {
          let cached_cell_reference =
            data_label.and_then(|label| data_label_field_cache_value(label, &field.id));
          let resolved = if let Some(value) = cached_cell_reference {
            Cow::Borrowed(value)
          } else {
            match field
              .r#type
              .as_deref()
              .map(str::to_ascii_uppercase)
              .as_deref()
            {
              Some("VALUE") => Cow::Owned(format_chart_number(value, value_format_code)),
              Some("SERIESNAME") => Cow::Borrowed(series_name),
              Some("CATEGORYNAME") => Cow::Borrowed(category_name.unwrap_or_default()),
              Some("PERCENTAGE") => Cow::Borrowed(percentage.unwrap_or_default()),
              Some("BUBBLESIZE") => bubble_size.map_or_else(
                || Cow::Borrowed(""),
                |value| Cow::Owned(general_chart_number(value)),
              ),
              Some("CELLRANGE") => Cow::Borrowed(data_labels_range.unwrap_or_default()),
              _ => Cow::Borrowed(field.text.as_deref().unwrap_or_default()),
            }
          };
          push_data_label_rich_text_run(
            &mut rich_text_runs,
            &mut line_index,
            &resolved,
            paragraph_default_run_properties,
            field.run_properties.as_deref(),
          );
        }
        a::ParagraphChoice::Break(line_break) => {
          line_index += 1;
          if line_break.run_properties.is_some() {
            // The break carries the following line's character properties in
            // DrawingML. Retain no empty paint run; the next authored run owns
            // visible text and resolves its own direct properties.
          }
        }
        a::ParagraphChoice::TextMath(math) => push_data_label_rich_text_run(
          &mut rich_text_runs,
          &mut line_index,
          &text_math_text(math),
          paragraph_default_run_properties,
          None,
        ),
        a::ParagraphChoice::AlternateContent(_) => {}
      }
    }
  }
  let line_count = rich_text_runs
    .iter()
    .map(|run| run.line_index)
    .max()
    .map_or(1, |index| index + 1);
  let mut lines = vec![String::new(); line_count];
  for run in &rich_text_runs {
    lines[run.line_index].push_str(&run.text);
  }
  let text = lines.join("\n").trim().to_string();
  ResolvedDataLabelChartText {
    text,
    lines,
    rich_text_runs,
  }
}

fn push_data_label_rich_text_run<'a>(
  runs: &mut Vec<ChartDataLabelTextRun<'a>>,
  line_index: &mut usize,
  text: &str,
  paragraph_default_run_properties: Option<&'a a::DefaultRunProperties>,
  run_properties: Option<&'a a::RunProperties>,
) {
  let mut segment = String::new();
  let mut characters = text.chars().peekable();
  while let Some(character) = characters.next() {
    if character == '\r' || character == '\n' {
      if !segment.is_empty() {
        runs.push(ChartDataLabelTextRun {
          text: std::mem::take(&mut segment),
          line_index: *line_index,
          paragraph_default_run_properties,
          run_properties,
        });
      }
      if character == '\r' && characters.peek() == Some(&'\n') {
        characters.next();
      }
      *line_index += 1;
    } else {
      segment.push(character);
    }
  }
  if !segment.is_empty() {
    runs.push(ChartDataLabelTextRun {
      text: segment,
      line_index: *line_index,
      paragraph_default_run_properties,
      run_properties,
    });
  }
}

fn data_label_field_cache_value<'a>(label: &'a c::DataLabel, field_id: &str) -> Option<&'a str> {
  label
    .d_lbl_extension_list
    .as_ref()
    .into_iter()
    .flat_map(|list| &list.d_lbl_extension)
    .flat_map(|extension| &extension.d_lbl_extension_choice)
    .find_map(|choice| match choice {
      c::DLblExtensionChoice::DataLabelFieldTable(table) => table
        .data_label_field_table_entry
        .iter()
        .find(|entry| entry.text_field_guid.eq_ignore_ascii_case(field_id))
        .and_then(|entry| entry.data_label_field_table_cache.as_deref())
        .and_then(|cache| {
          cache
            .string_point
            .iter()
            .find(|point| point.index == 0)
            .or_else(|| cache.string_point.first())
        })
        .map(|point| point.numeric_value.as_str()),
      _ => None,
    })
}

fn general_chart_number(value: f64) -> String {
  // ECMA-376 Part 1, 18.8.30 gives General a display budget of 11
  // characters (excluding a negative sign, including the decimal point).
  // Office also switches twelve-digit integers and sufficiently small
  // fractions to exponential notation. Chart labels use that same number
  // format language even though they are not worksheet cells.
  const DISPLAY_CHARACTERS: usize = 11;

  if value == 0.0 {
    return "0".to_string();
  }
  if !value.is_finite() {
    return value.to_string();
  }

  let absolute = value.abs();
  if !(1.0e-4..1.0e11).contains(&absolute) {
    return format_general_chart_scientific(value, DISPLAY_CHARACTERS);
  }

  let integer_digits = absolute.log10().floor().max(0.0) as usize + 1;
  let decimals = DISPLAY_CHARACTERS.saturating_sub(integer_digits + 1);
  let formatted = trim_general_chart_fraction(format!("{value:.decimals$}"));
  if formatted.trim_start_matches('-').len() <= DISPLAY_CHARACTERS {
    formatted
  } else {
    // Rounding can carry an eleven-digit fixed value into a twelfth digit.
    format_general_chart_scientific(value, DISPLAY_CHARACTERS)
  }
}

fn format_general_chart_scientific(value: f64, display_characters: usize) -> String {
  let estimated_exponent = value.abs().log10().floor() as i32;
  let exponent_digits = estimated_exponent.unsigned_abs().to_string().len().max(2);
  // One mantissa digit, decimal point, E, exponent sign and exponent digits
  // consume the rest of the General-format display budget.
  let decimals = display_characters.saturating_sub(4 + exponent_digits);
  let scientific = format!("{value:.decimals$e}");
  let Some((mantissa, exponent)) = scientific.split_once('e') else {
    return scientific;
  };
  let mantissa = trim_general_chart_fraction(mantissa.to_string());
  let exponent = exponent.parse::<i32>().unwrap_or(estimated_exponent);
  format!(
    "{mantissa}E{}{absolute:02}",
    if exponent < 0 { '-' } else { '+' },
    absolute = exponent.unsigned_abs()
  )
}

fn trim_general_chart_fraction(mut text: String) -> String {
  if text.contains('.') {
    while text.ends_with('0') {
      text.pop();
    }
    if text.ends_with('.') {
      text.pop();
    }
  }
  if text == "-0" { "0".to_string() } else { text }
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
/// either border receives one interval of breathing room.
pub fn linear_axis_scale(
  values: impl IntoIterator<Item = f64>,
  axis: Option<&c::ValueAxis>,
  maximum_auto_increment_count: usize,
) -> Option<LinearAxisScale> {
  linear_axis_scale_with_options(
    values,
    axis,
    maximum_auto_increment_count,
    LinearAxisScaleOptions::default(),
  )
}

pub(crate) fn linear_axis_scale_with_options(
  values: impl IntoIterator<Item = f64>,
  axis: Option<&c::ValueAxis>,
  maximum_auto_increment_count: usize,
  options: LinearAxisScaleOptions,
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

  let mut explicit_minimum =
    axis.and_then(|axis| axis.scaling.min_axis_value.as_ref().map(|v| v.val));
  let mut explicit_maximum =
    axis.and_then(|axis| axis.scaling.max_axis_value.as_ref().map(|v| v.val));
  let explicit_unit = axis.map(|axis| axis.major_unit.as_ref().map(|unit| unit.val));
  let logarithmic_base = axis
    .and_then(|axis| axis.scaling.log_base.as_ref())
    .map(|base| base.val)
    .filter(|base| *base > 1.0);
  // LibreOffice handles an all-negative range by negating and swapping it,
  // running the ordinary positive-range algorithm, then restoring the sign.
  // This gives [-5,-2] the same zero-expansion rhythm as [2,5].
  let swap_and_negate = logarithmic_base.is_none() && source_minimum < 0.0 && source_maximum <= 0.0;
  if swap_and_negate {
    (source_minimum, source_maximum) = (-source_maximum, -source_minimum);
    (explicit_minimum, explicit_maximum) = (
      explicit_maximum.map(|value| -value),
      explicit_minimum.map(|value| -value),
    );
  }
  let restore_scale = |mut scale: LinearAxisScale| {
    if swap_and_negate {
      (scale.minimum, scale.maximum) = (-scale.maximum, -scale.minimum);
    }
    scale
  };
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
    return Some(restore_scale(LinearAxisScale {
      minimum: explicit_minimum.unwrap_or_else(|| base.powf(temporary_minimum.log(base).floor())),
      maximum: explicit_maximum.unwrap_or_else(|| base.powf(temporary_maximum.log(base).ceil())),
      major_unit: explicit_unit.flatten().unwrap_or(1.0),
      logarithmic_base: Some(base),
      reversed: axis
        .and_then(|axis| axis.scaling.orientation.as_ref())
        .and_then(|orientation| orientation.val)
        == Some(c::OrientationValues::MaxMin),
    }));
  }

  let max_increments = maximum_auto_increment_count.clamp(2, 10);
  let mut major_unit = explicit_unit
    .flatten()
    .filter(|unit| unit.is_finite() && *unit > 0.0)
    .unwrap_or_else(|| {
      let automatic =
        nice_increment((temporary_maximum - temporary_minimum) / max_increments as f64);
      options
        .minimum_automatic_major_unit
        .filter(|unit| unit.is_finite() && *unit > 0.0)
        .map_or(automatic, |minimum| automatic.max(minimum))
    });
  let automatic_unit = explicit_unit.flatten().is_none();
  loop {
    let mut minimum =
      explicit_minimum.unwrap_or_else(|| increment_floor(temporary_minimum, major_unit));
    let mut maximum =
      explicit_maximum.unwrap_or_else(|| increment_ceil(temporary_maximum, major_unit));
    if options.expand_if_values_close_to_border
      && explicit_minimum.is_none()
      && minimum != 0.0
      && (maximum - source_minimum) / (maximum - minimum) > 20.0 / 21.0
    {
      minimum -= major_unit;
    }
    if options.expand_if_values_close_to_border
      && explicit_maximum.is_none()
      && maximum != 0.0
      && (source_maximum - minimum) / (maximum - minimum) > 20.0 / 21.0
    {
      maximum += major_unit;
    }
    let increment_count = axis_interval_count(minimum, maximum, major_unit, usize::MAX);
    // ScaleAutomatism performs this check after both automatic borders have
    // been aligned and expanded. Boundary breathing space therefore consumes
    // the same interval budget as the data-bearing range.
    if increment_count <= max_increments || !automatic_unit {
      return Some(restore_scale(LinearAxisScale {
        minimum,
        maximum,
        major_unit,
        logarithmic_base,
        reversed: axis
          .and_then(|axis| axis.scaling.orientation.as_ref())
          .and_then(|orientation| orientation.val)
          == Some(c::OrientationValues::MaxMin),
      }));
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
  fixed_output_texts_for_host_ui_language(
    chart_space,
    ChartHostApplication::Spreadsheet,
    ui_language,
  )
}

pub fn fixed_output_texts_for_host_ui_language(
  chart_space: &c::ChartSpace,
  host: ChartHostApplication,
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
      linear_axis_scale_with_options(
        scale_values,
        Some(axis),
        10,
        LinearAxisScaleOptions {
          expand_if_values_close_to_border: !value_axis_is_used_by_3d_group(
            chart_space,
            axis.axis_id.val,
          ),
          minimum_automatic_major_unit: None,
        },
      )
    };
    let Some(scale) = scale else {
      continue;
    };
    let display_unit = value_axis_display_unit(axis);
    let uses_x_sequence = value_axis_uses_x_sequence(chart_space, axis.axis_id.val);
    let format_code = effective_axis_number_format_code(
      Some(axis),
      mode == ChartValueMode::PercentStacked,
      axis_series.iter().copied().map(|series| {
        if uses_x_sequence {
          series_x_number_format_code(series)
        } else {
          series_number_format_code(series)
        }
      }),
    );
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

  push_fixed_axis_titles(&mut texts, chart_space, host, ui_language);

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
    let cartesian = cartesian_chart_for_host_locales(chart_space, host, ui_language, ui_language);
    if let Some(chart) = cartesian
      .as_ref()
      .filter(|chart| chart.vary_colors_by_point)
    {
      for category in chart
        .visible_legend_indices
        .iter()
        .filter_map(|index| chart.categories.get(*index))
      {
        push_unique_text(&mut texts, category);
      }
    } else {
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
  host: ChartHostApplication,
  ui_language: Option<&str>,
) {
  for choice in &chart_space.chart.plot_area.plot_area_choice2 {
    let title = match choice {
      c::PlotAreaChoice2::ValueAxis(axis) => axis.title.as_deref(),
      _ => None,
    };
    push_fixed_axis_title(texts, title, host, ui_language);
  }
  for choice in &chart_space.chart.plot_area.plot_area_choice2 {
    let title = match choice {
      c::PlotAreaChoice2::CategoryAxis(axis) => axis.title.as_deref(),
      c::PlotAreaChoice2::DateAxis(axis) => axis.title.as_deref(),
      c::PlotAreaChoice2::SeriesAxis(axis) => axis.title.as_deref(),
      c::PlotAreaChoice2::ValueAxis(_) => None,
    };
    push_fixed_axis_title(texts, title, host, ui_language);
  }
}

fn push_fixed_axis_title(
  texts: &mut Vec<String>,
  title: Option<&c::Title>,
  host: ChartHostApplication,
  ui_language: Option<&str>,
) {
  let Some(title) = title else {
    return;
  };
  let before = texts.len();
  push_title_texts(texts, title);
  if texts.len() == before
    && let Some(text) = title_text_or_automatic(title, host, ui_language)
  {
    push_unique_text(texts, &text);
  }
}

fn automatic_axis_title(ui_language: Option<&str>) -> &'static str {
  OfficeStringCatalog::for_ui_language(ui_language).chart_axis_title()
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
    let separator = settings.separator.as_deref().unwrap_or(", ");
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

fn series_x_number_format_code(series: ChartSeriesRef<'_>) -> Option<&str> {
  let values = series.x_values?;
  match values.x_values_choice.as_ref() {
    Some(c::XValuesChoice::NumberReference(reference)) => reference
      .numbering_cache
      .as_deref()
      .and_then(|cache| cache.format_code.as_ref())
      .and_then(|format| format.xml_content.as_deref()),
    Some(c::XValuesChoice::NumberLiteral(literal)) => literal
      .format_code
      .as_ref()
      .and_then(|format| format.xml_content.as_deref()),
    Some(
      c::XValuesChoice::StringReference(_)
      | c::XValuesChoice::StringLiteral(_)
      | c::XValuesChoice::MultiLevelStringReference(_),
    )
    | None => None,
  }
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

fn value_axis_is_used_by_3d_group(chart_space: &c::ChartSpace, axis_id: i32) -> bool {
  chart_space
    .chart
    .plot_area
    .plot_area_choice1
    .iter()
    .any(|choice| {
      let axes = match choice {
        c::PlotAreaChoice::Area3DChart(chart) => Some(&chart.axis_id),
        c::PlotAreaChoice::Line3DChart(chart) => Some(&chart.axis_id),
        c::PlotAreaChoice::Bar3DChart(chart) => Some(&chart.axis_id),
        c::PlotAreaChoice::Surface3DChart(chart) => Some(&chart.axis_id),
        _ => None,
      };
      axes.is_some_and(|axes| axes.iter().any(|axis| axis.val == axis_id))
    })
}

fn axis_tick_values(scale: LinearAxisScale) -> Vec<f64> {
  if !scale.minimum.is_finite()
    || !scale.maximum.is_finite()
    || !scale.major_unit.is_finite()
    || scale.major_unit <= 0.0
  {
    return Vec::new();
  }
  let count = axis_interval_count(scale.minimum, scale.maximum, scale.major_unit, 1_000);
  (0..=count)
    .map(|index| scale.minimum + scale.major_unit * index as f64)
    .collect()
}

/// Counts complete axis intervals while tolerating the binary representation
/// error of decimal OOXML units such as 0.005. Without the near-integer snap,
/// a nominal 0.00%-5.00% axis can lose its final 5.00% tick because the raw
/// quotient is 9.999999999999998.
pub(crate) fn axis_interval_count(
  minimum: f64,
  maximum: f64,
  unit: f64,
  maximum_count: usize,
) -> usize {
  if !minimum.is_finite()
    || !maximum.is_finite()
    || !unit.is_finite()
    || unit <= 0.0
    || maximum <= minimum
  {
    return 0;
  }
  let raw = (maximum - minimum) / unit;
  let nearest = raw.round();
  let stable = if (raw - nearest).abs() <= 1.0e-10 * raw.abs().max(1.0) {
    nearest
  } else {
    raw.floor()
  };
  stable.clamp(0.0, maximum_count as f64) as usize
}

pub(crate) fn value_axis_display_unit(axis: &c::ValueAxis) -> f64 {
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

/// Resolves the visible number format of a vertical numeric axis.
///
/// `c:numFmt/@sourceLinked` defaults to true in the Office/LibreOffice import
/// model. In that state the axis format comes from the attached value-y data
/// sequence, not from the fallback `formatCode` carried by the axis itself.
/// LibreOffice `AxisHelper::getExplicitNumberFormatKeyForAxis` selects the
/// most frequent attached sequence format and has a separate percent-scale
/// branch; keeping that decision here makes every OOXML host use one rule.
pub(crate) fn vertical_axis_number_format_code<'data>(
  chart: &ClusteredColumnChart<'data>,
  axis_set_index: usize,
) -> Option<&'data str> {
  let axis = chart
    .axis_sets
    .get(axis_set_index)
    .and_then(|set| set.vertical_value_axis)
    .or_else(|| (axis_set_index == 0).then_some(chart.value_axis).flatten());
  let percent_stacked = chart
    .series
    .iter()
    .filter(|series| series.axis_set_index == axis_set_index)
    .all(|series| series.grouping == ChartSeriesGrouping::PercentStacked)
    && chart
      .series
      .iter()
      .any(|series| series.axis_set_index == axis_set_index);
  effective_axis_number_format_code(
    axis,
    percent_stacked,
    chart
      .series
      .iter()
      .filter(move |series| series.axis_set_index == axis_set_index)
      .map(|series| series.number_format_code),
  )
}

/// Resolves the source-linked format for a scatter/bubble X value axis.
pub(crate) fn horizontal_axis_number_format_code<'data>(
  chart: &ClusteredColumnChart<'data>,
  axis_set_index: usize,
) -> Option<&'data str> {
  let axis = chart
    .axis_sets
    .get(axis_set_index)
    .and_then(|set| set.horizontal_value_axis)
    .or_else(|| {
      (axis_set_index == 0)
        .then_some(chart.horizontal_value_axis)
        .flatten()
    });
  effective_axis_number_format_code(
    axis,
    false,
    chart
      .series
      .iter()
      .filter(move |series| series.axis_set_index == axis_set_index)
      .map(|series| series.x_number_format_code),
  )
}

fn effective_axis_number_format_code<'a>(
  axis: Option<&'a c::ValueAxis>,
  percent_scale: bool,
  source_formats: impl Iterator<Item = Option<&'a str>>,
) -> Option<&'a str> {
  let numbering_format = axis.and_then(|axis| axis.numbering_format.as_ref());
  let authored = numbering_format
    .map(|format| format.format_code.as_str())
    .filter(|format| !format.is_empty());
  let source_linked = numbering_format.is_none_or(|format| {
    format
      .source_linked
      .is_none_or(|source_linked| source_linked.as_bool())
  });
  if !source_linked {
    return authored;
  }
  if percent_scale {
    return Some("0%");
  }
  most_frequent_number_format(source_formats).or(authored)
}

fn most_frequent_number_format<'a>(
  formats: impl Iterator<Item = Option<&'a str>>,
) -> Option<&'a str> {
  let mut counts = Vec::<(&'a str, usize)>::new();
  let mut best = None;
  let mut best_count = 0usize;
  for format in formats.flatten().filter(|format| !format.is_empty()) {
    let count = if let Some((_, count)) = counts
      .iter_mut()
      .find(|(candidate, _)| *candidate == format)
    {
      *count += 1;
      *count
    } else {
      counts.push((format, 1));
      1
    };
    if count > best_count {
      best = Some(format);
      best_count = count;
    }
  }
  best
}

pub(crate) fn value_axis_display_unit_label_text(
  axis: &c::ValueAxis,
  ui_language: Option<&str>,
) -> Option<String> {
  let units = axis.display_units.as_deref()?;
  if let Some(label) = units.display_units_label.as_deref()
    && let Some(text) = label.chart_text.as_deref()
  {
    let mut values = Vec::new();
    push_chart_text(&mut values, text);
    return (!values.is_empty()).then(|| values.join(" "));
  }
  let c::DisplayUnitsChoice::BuiltInUnit(unit) = units.display_units_choice.as_ref()? else {
    return None;
  };
  let strings = OfficeStringCatalog::for_ui_language(ui_language);
  Some(
    strings
      .chart_display_unit(match unit.val.unwrap_or_default() {
        c::BuiltInUnitValues::Hundreds => ChartDisplayUnit::Hundreds,
        c::BuiltInUnitValues::Thousands => ChartDisplayUnit::Thousands,
        c::BuiltInUnitValues::TenThousands => ChartDisplayUnit::TenThousands,
        c::BuiltInUnitValues::HundredThousands => ChartDisplayUnit::HundredThousands,
        c::BuiltInUnitValues::Millions => ChartDisplayUnit::Millions,
        c::BuiltInUnitValues::TenMillions => ChartDisplayUnit::TenMillions,
        c::BuiltInUnitValues::HundredMillions => ChartDisplayUnit::HundredMillions,
        c::BuiltInUnitValues::Billions => ChartDisplayUnit::Billions,
        c::BuiltInUnitValues::Trillions => ChartDisplayUnit::Trillions,
      })
      .to_string(),
  )
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
  let _ = unit;
  if let Some(label) = value_axis_display_unit_label_text(axis, ui_language) {
    push_unique_text(texts, &label);
  }
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

fn value_axis_uses_x_sequence(chart_space: &c::ChartSpace, axis_id: i32) -> bool {
  chart_space
    .chart
    .plot_area
    .plot_area_choice1
    .iter()
    .any(|choice| match choice {
      c::PlotAreaChoice::ScatterChart(chart) => chart
        .axis_id
        .first()
        .is_some_and(|axis| axis.val == axis_id),
      c::PlotAreaChoice::BubbleChart(chart) => chart
        .axis_id
        .first()
        .is_some_and(|axis| axis.val == axis_id),
      _ => false,
    })
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
  if !is_general_chart_number_format(code) {
    let uppercase_code = code.to_ascii_uppercase();
    if uppercase_code.contains("E+") || uppercase_code.contains("E-") {
      return format_chart_scientific(value, format_decimal_places(code));
    }
    let code = chart_number_format_without_cell_alignment(code);
    return crate::xlsx::format_spreadsheet_number(value, &code);
  }
  general_chart_number(value)
}

fn is_general_chart_number_format(code: &str) -> bool {
  // Spreadsheet producers may persist the localized name of built-in format
  // zero. LibreOffice's complete locale-data set currently resolves that
  // keyword to this finite set; its scanner accepts the localized name and
  // English `General` interchangeably. Leading NatNum/locale modifiers do
  // not change the underlying General format.
  let mut keyword = code.split(';').next().unwrap_or(code).trim();
  while let Some(rest) = keyword.strip_prefix('[')
    && let Some(end) = rest.find(']')
  {
    keyword = rest[end + 1..].trim_start();
  }
  const GENERAL_KEYWORDS: &[&str] = &[
    "General",
    "Standard",
    "Standaard",
    "Yleinen",
    "Geral",
    "Estandar",
    "Estandarra",
    "Bendras",
    "Kadaywan",
    "Skoueriek",
    "Општо",
    "عادی",
    "常规",
  ];
  GENERAL_KEYWORDS
    .iter()
    .any(|candidate| keyword.eq_ignore_ascii_case(candidate))
}

fn chart_number_format_without_cell_alignment(code: &str) -> std::borrow::Cow<'_, str> {
  if !code.contains(['_', '*']) {
    return std::borrow::Cow::Borrowed(code);
  }
  let mut normalized = String::with_capacity(code.len());
  let mut chars = code.chars();
  let mut in_quotes = false;
  while let Some(ch) = chars.next() {
    match ch {
      '"' => {
        in_quotes = !in_quotes;
        normalized.push(ch);
      }
      '\\' => {
        normalized.push(ch);
        if let Some(literal) = chars.next() {
          normalized.push(literal);
        }
      }
      '_' | '*' if !in_quotes => {
        // These directives reserve/repeat a following character to align a
        // worksheet cell. A chart label is an independently sized text shape,
        // so Office drops both the directive and its fill character.
        chars.next();
      }
      _ => normalized.push(ch),
    }
  }
  std::borrow::Cow::Owned(normalized)
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
  let strings = OfficeStringCatalog::for_ui_language(ui_language);
  if strings.resource_locale().is_chinese() {
    return automatic_series_title(ui_language, series_index);
  }
  if let Some(formula) = series_value_formula(series)
    && let Some(range) = parse_chart_a1_range(formula)
    && range.start_col == range.end_col
    && range.start_row != range.end_row
  {
    return strings.chart_column_title(series_index);
  }
  strings.chart_row_title(series_index)
}

fn series_name_formula(series: ChartSeriesRef<'_>) -> Option<&str> {
  series
    .series_text
    .and_then(|text| match text.series_text_choice.as_ref() {
      Some(c::SeriesTextChoice::StringReference(reference)) => {
        reference.formula.xml_content.as_deref()
      }
      Some(c::SeriesTextChoice::NumericValue(_)) | None => None,
    })
}

fn series_category_formula(series: ChartSeriesRef<'_>) -> Option<&str> {
  series
    .category_axis_data
    .and_then(|data| match data.category_axis_data_choice.as_ref() {
      Some(c::CategoryAxisDataChoice::StringReference(reference)) => {
        reference.formula.xml_content.as_deref()
      }
      Some(c::CategoryAxisDataChoice::NumberReference(reference)) => {
        reference.formula.xml_content.as_deref()
      }
      Some(c::CategoryAxisDataChoice::MultiLevelStringReference(reference)) => {
        reference.formula.xml_content.as_deref()
      }
      Some(
        c::CategoryAxisDataChoice::StringLiteral(_) | c::CategoryAxisDataChoice::NumberLiteral(_),
      )
      | None => None,
    })
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

fn series_x_value_formula(series: ChartSeriesRef<'_>) -> Option<&str> {
  series
    .x_values
    .and_then(|values| match values.x_values_choice.as_ref() {
      Some(c::XValuesChoice::NumberReference(reference)) => {
        reference.formula.xml_content.as_deref()
      }
      // String and multi-level string x-values are category/index axes, not
      // numeric scatter coordinates. Their authored caches must stay intact;
      // reading the backing cells through the numeric resolver would turn a
      // valid string cache into an all-missing numeric sequence.
      Some(
        c::XValuesChoice::StringReference(_)
        | c::XValuesChoice::MultiLevelStringReference(_)
        | c::XValuesChoice::NumberLiteral(_)
        | c::XValuesChoice::StringLiteral(_),
      )
      | None => None,
    })
}

fn series_bubble_size_formula(series: ChartSeriesRef<'_>) -> Option<&str> {
  series
    .bubble_size
    .and_then(|values| match values.bubble_size_choice.as_ref() {
      Some(c::BubbleSizeChoice::NumberReference(reference)) => {
        reference.formula.xml_content.as_deref()
      }
      Some(c::BubbleSizeChoice::NumberLiteral(_)) | None => None,
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

pub fn shape_properties_has_no_outline(properties: &c::ShapeProperties) -> bool {
  matches!(
    properties
      .outline
      .as_deref()
      .and_then(|outline| outline.outline_choice1.as_ref()),
    Some(a::OutlineChoice::NoFill(_))
  )
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

pub(crate) fn chart_shape_properties_has_no_outline(properties: &c::ChartShapeProperties) -> bool {
  matches!(
    properties
      .outline
      .as_deref()
      .and_then(|outline| outline.outline_choice1.as_ref()),
    Some(a::OutlineChoice::NoFill(_))
  )
}

fn chart_title_text(chart: &c::Chart) -> Option<ChartTitleText> {
  if let Some(title) = chart.title.as_deref() {
    return explicit_title_text(title).map(ChartTitleText::Explicit);
  }

  chart_automatic_title_is_visible(chart).then_some(ChartTitleText::Automatic)
}

fn chart_title_vertical_anchor(chart: &c::Chart) -> Option<a::TextAnchoringTypeValues> {
  let chart_text = chart.title.as_deref()?.chart_text.as_deref()?;
  let c::ChartTextChoice::RichText(rich) = chart_text.chart_text_choice.as_ref()? else {
    return None;
  };
  rich.body_properties.anchor
}

fn chart_title_layout(chart: &c::Chart) -> Option<ChartManualLayout> {
  chart
    .title
    .as_deref()
    .and_then(|title| chart_text_layout(title.layout.as_deref()))
}

fn chart_title_rotation_degrees(chart: &c::Chart) -> f32 {
  chart
    .title
    .as_deref()
    .and_then(title_rotation_degrees)
    .unwrap_or(0.0)
}

pub(crate) fn title_rotation_degrees(title: &c::Title) -> Option<f32> {
  // LibreOffice's TitleConverter selects c:title/c:txPr when present and
  // otherwise falls back to the body properties on c:tx/c:rich. This matters
  // for linked/string titles, which do not own a rich text body at all.
  let rotation = title
    .text_properties
    .as_deref()
    .and_then(|properties| properties.body_properties.rotation)
    .or_else(|| {
      title
        .chart_text
        .as_deref()
        .and_then(|text| match text.chart_text_choice.as_ref()? {
          c::ChartTextChoice::RichText(rich) => rich.body_properties.rotation,
          c::ChartTextChoice::StringReference(_) | c::ChartTextChoice::StringLiteral(_) => None,
        })
    })?;
  let normalized = rotation.rem_euclid(21_600_000);
  let normalized = if normalized > 10_800_000 {
    normalized - 21_600_000
  } else {
    normalized
  };
  Some(normalized as f32 / 60_000.0)
}

pub(crate) fn category_axis_title_source<'a>(
  chart: &ClusteredColumnChart<'a>,
) -> Option<(&'a c::Title, f32)> {
  let axis_set_index = visible_category_axis_set_index(&chart.axis_sets);
  let horizontal_bar = axis_set_uses_horizontal_bars(&chart.series, axis_set_index);
  chart
    .category_axis
    .and_then(|axis| {
      axis.title.as_deref().map(|title| {
        (
          title,
          automatic_axis_title_rotation_for_dimension(
            axis.axis_position.val,
            ChartAxisDimension::X,
            horizontal_bar,
          ),
        )
      })
    })
    .or_else(|| {
      chart.date_axis.and_then(|axis| {
        axis.title.as_deref().map(|title| {
          (
            title,
            automatic_axis_title_rotation_for_dimension(
              axis.axis_position.val,
              ChartAxisDimension::X,
              horizontal_bar,
            ),
          )
        })
      })
    })
    .or_else(|| {
      chart.horizontal_value_axis.and_then(|axis| {
        axis.title.as_deref().map(|title| {
          (
            title,
            automatic_axis_title_rotation_for_dimension(
              axis.axis_position.val,
              ChartAxisDimension::X,
              horizontal_bar,
            ),
          )
        })
      })
    })
}

pub(crate) fn value_axis_title_source<'a>(
  chart: &ClusteredColumnChart<'a>,
) -> Option<(&'a c::Title, f32)> {
  let horizontal_bar = axis_set_uses_horizontal_bars(&chart.series, 0);
  chart.value_axis.and_then(|axis| {
    axis.title.as_deref().map(|title| {
      (
        title,
        automatic_axis_title_rotation_for_dimension(
          axis.axis_position.val,
          ChartAxisDimension::Y,
          horizontal_bar,
        ),
      )
    })
  })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ChartAxisDimension {
  X,
  Y,
}

fn axis_set_uses_horizontal_bars(
  series: &[ClusteredColumnSeries<'_>],
  axis_set_index: usize,
) -> bool {
  let mut attached = series
    .iter()
    .filter(|series| series.axis_set_index == axis_set_index);
  let Some(first) = attached.next() else {
    return false;
  };
  first.kind == ChartSeriesKind::Bar && attached.all(|series| series.kind == ChartSeriesKind::Bar)
}

fn automatic_axis_title_rotation_for_dimension(
  position: c::AxisPositionValues,
  dimension: ChartAxisDimension,
  horizontal_bar: bool,
) -> f32 {
  // LibreOffice AxisConverter::convertFromModel starts with the c:axPos
  // default imported by AxisContext, then makes the logical X dimension
  // horizontal for ordinary charts and the logical Y dimension horizontal
  // for horizontal bar charts. This distinction is observable when an X
  // axis is authored on the left or right: its title remains horizontal.
  let horizontal_dimension = if horizontal_bar {
    ChartAxisDimension::Y
  } else {
    ChartAxisDimension::X
  };
  if dimension == horizontal_dimension {
    0.0
  } else {
    automatic_axis_title_rotation_degrees(position)
  }
}

pub(crate) fn automatic_axis_title_rotation_degrees(position: c::AxisPositionValues) -> f32 {
  // AxisContext's initial title default follows c:axPos. AxisConverter later
  // applies the logical-axis/chart-type correction above.
  match position {
    c::AxisPositionValues::Left | c::AxisPositionValues::Right => -90.0,
    c::AxisPositionValues::Top | c::AxisPositionValues::Bottom => 0.0,
  }
}

fn explicit_title_text(title: &c::Title) -> Option<String> {
  let mut values = Vec::new();
  push_chart_text(&mut values, title.chart_text.as_deref()?);
  let value = values.join(" ");
  (!value.is_empty()).then_some(value)
}

fn title_text_or_automatic(
  title: &c::Title,
  host: ChartHostApplication,
  ui_language: Option<&str>,
) -> Option<String> {
  if let Some(text) = explicit_title_text(title) {
    return Some(text);
  }
  // MS-OI29500 §21.2.2.210 gives an omitted c:tx a localized default specific
  // to the title's parent. Word fixed output materializes that default from a
  // bare axis c:title and from a formatting-only title. Excel preserves a
  // formatting-only title whose defRPr is empty as invisible, as demonstrated
  // by axis_title_rotated.xlsx, while axis_title_rotation.xlsx has authored
  // character properties and displays the localized default. Keep this host
  // distinction explicit instead of weakening either counterexample.
  if title.chart_text.is_some() {
    return None;
  }
  if host != ChartHostApplication::Wordprocessing
    && !title
      .text_properties
      .as_deref()
      .is_some_and(text_properties_have_character_properties)
  {
    return None;
  }
  Some(automatic_axis_title(ui_language).to_string())
}

fn text_properties_have_character_properties(properties: &c::TextProperties) -> bool {
  let default_properties = a::DefaultRunProperties::default();
  let direct_properties = a::RunProperties::default();
  properties.paragraph.iter().any(|paragraph| {
    paragraph
      .paragraph_properties
      .as_deref()
      .and_then(|properties| properties.default_run_properties.as_deref())
      .is_some_and(|properties| properties != &default_properties)
      || paragraph
        .paragraph_choice
        .iter()
        .any(|choice| match choice {
          a::ParagraphChoice::Run(run) => run
            .run_properties
            .as_deref()
            .is_some_and(|properties| properties != &direct_properties),
          a::ParagraphChoice::Break(line_break) => line_break
            .run_properties
            .as_deref()
            .is_some_and(|properties| properties != &direct_properties),
          a::ParagraphChoice::Field(field) => field
            .run_properties
            .as_deref()
            .is_some_and(|properties| properties != &direct_properties),
          a::ParagraphChoice::TextMath(_) | a::ParagraphChoice::AlternateContent(_) => false,
        })
  }) || properties
    .list_style
    .as_deref()
    .and_then(|style| style.default_paragraph_properties.as_deref())
    .and_then(|properties| properties.default_run_properties.as_deref())
    .is_some_and(|properties| properties != &default_properties)
}

pub fn has_powerpoint_automatic_title_placeholder(chart: &c::Chart) -> bool {
  // PowerPoint distinguishes a bare empty title from its generated insertion
  // placeholder. Newer producers retain title text properties even though
  // c:tx has not been populated yet, while older POI/Office chart parts retain
  // only an empty c:layout. LibreOffice ChartSpaceConverter::convertFromModel
  // resolves either authored title container from the sole series title.
  // Word and Excel do not share every editing-placeholder rule, so keep this
  // policy host-specific.
  chart.title.as_deref().is_some_and(|title| {
    title.chart_text.is_none() && (title.text_properties.is_some() || title.layout.is_some())
  }) && chart_automatic_title_is_visible(chart)
}

pub fn has_powerpoint_generic_title_placeholder(chart: &c::Chart) -> bool {
  chart.title.as_deref().is_some_and(|title| {
    title.chart_text.is_none()
      && title.text_properties.is_some()
      && title.chart_shape_properties.is_none()
      && title.overlay.is_none()
  }) && chart_automatic_title_is_visible(chart)
}

pub fn has_word_automatic_title_placeholder(chart: &c::Chart) -> bool {
  // MS-OI29500 §2.1.1431 applies autoTitleDeleted only when c:title is absent.
  // Word fixed output materializes an authored empty c:title from the sole
  // series title, whereas Excel preserves the same markup as empty. Keep this
  // host policy outside the shared title parser.
  chart
    .title
    .as_deref()
    .is_some_and(|title| explicit_title_text(title).is_none())
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

fn chart_numeric_category_values(chart_space: &c::ChartSpace) -> Vec<Option<f64>> {
  series(chart_space)
    .into_iter()
    .filter_map(|series| series.category_axis_data)
    .find_map(indexed_category_axis_numeric_values)
    .unwrap_or_default()
}

fn chart_numeric_category_format_code(chart_space: &c::ChartSpace) -> Option<String> {
  series(chart_space)
    .into_iter()
    .filter_map(|series| series.category_axis_data)
    .find_map(indexed_category_axis_number_format_code)
    .map(ToOwned::to_owned)
}

fn indexed_category_axis_number_format_code(data: &c::CategoryAxisData) -> Option<&str> {
  match data.category_axis_data_choice.as_ref()? {
    c::CategoryAxisDataChoice::NumberReference(reference) => reference
      .numbering_cache
      .as_deref()?
      .format_code
      .as_ref()?
      .xml_content
      .as_deref(),
    c::CategoryAxisDataChoice::NumberLiteral(literal) => {
      literal.format_code.as_ref()?.xml_content.as_deref()
    }
    c::CategoryAxisDataChoice::StringReference(_)
    | c::CategoryAxisDataChoice::StringLiteral(_)
    | c::CategoryAxisDataChoice::MultiLevelStringReference(_) => None,
  }
}

fn indexed_category_axis_numeric_values(data: &c::CategoryAxisData) -> Option<Vec<Option<f64>>> {
  let points =
    match data.category_axis_data_choice.as_ref()? {
      c::CategoryAxisDataChoice::NumberReference(reference) => reference
        .numbering_cache
        .as_deref()
        .map(|cache| cache.numeric_point.as_slice())?,
      c::CategoryAxisDataChoice::NumberLiteral(literal) => literal.numeric_point.as_slice(),
      _ => return None,
    };
  let length = points
    .iter()
    .filter_map(|point| usize::try_from(point.index).ok())
    .max()
    .map_or(0, |index| index + 1);
  let mut values = vec![None; length];
  for point in points {
    let Ok(index) = usize::try_from(point.index) else {
      continue;
    };
    values[index] = point.numeric_value.trim().parse::<f64>().ok();
  }
  Some(values)
}

fn chart_uses_1904_date_system(chart_space: &c::ChartSpace) -> bool {
  chart_space
    .date1904
    .as_ref()
    .and_then(|value| value.val)
    .is_some_and(|value| value.as_bool())
}

/// Builds the explicit major labels for an OOXML date axis. Category caches
/// contain data points, not the date-axis ticks: Office advances the latter
/// in calendar days, months, or years according to c:majorTimeUnit.
pub fn date_axis_ticks(chart: &ClusteredColumnChart<'_>) -> Option<Vec<ChartCategoryTick>> {
  date_axis_ticks_with_maximum_auto_increment_count(chart, 500)
}

pub fn date_axis_ticks_with_maximum_auto_increment_count(
  chart: &ClusteredColumnChart<'_>,
  maximum_auto_increment_count: usize,
) -> Option<Vec<ChartCategoryTick>> {
  let axis = chart.date_axis?;
  let scale = date_axis_scale(chart)?;
  let interval = date_axis_major_interval(axis, scale, maximum_auto_increment_count);
  let date_1904 = chart.date_1904;
  let (mut year, mut month, mut day) = chart_date_from_serial(scale.minimum as i64, date_1904)?;
  let (format_code, source_linked) = effective_date_axis_number_format(chart, axis);
  let mut ticks = Vec::new();
  for _ in 0..10_000 {
    let serial = chart_date_serial(year, month, day, date_1904)?;
    if serial > scale.maximum + f64::EPSILON
      || (scale.shifted && serial >= scale.maximum - f64::EPSILON)
    {
      break;
    }
    if serial >= scale.minimum - f64::EPSILON {
      ticks.push(ChartCategoryTick {
        position: date_axis_position(chart, scale, year, month, day, true)?,
        gridline_position: date_axis_position(chart, scale, year, month, day, false)?,
        text: format_chart_date(
          year,
          month as u32,
          day as u32,
          format_code,
          source_linked,
          chart.format_locale.as_deref(),
        ),
      });
    }
    (year, month, day) = advance_chart_date(year, month, day, interval, date_1904)?;
  }
  (!ticks.is_empty()).then_some(ticks)
}

/// Builds the minor gridline positions for an OOXML date axis. Like the
/// major labels, these are calendar ticks rather than cached category points.
pub fn date_axis_minor_tick_positions(chart: &ClusteredColumnChart<'_>) -> Option<Vec<f64>> {
  date_axis_minor_tick_positions_with_maximum_auto_increment_count(chart, 500)
}

pub fn date_axis_minor_tick_positions_with_maximum_auto_increment_count(
  chart: &ClusteredColumnChart<'_>,
  maximum_auto_increment_count: usize,
) -> Option<Vec<f64>> {
  let axis = chart.date_axis?;
  let scale = date_axis_scale(chart)?;
  let interval = date_axis_minor_interval(axis, scale, maximum_auto_increment_count);
  let date_1904 = chart.date_1904;
  let (mut year, mut month, mut day) = chart_date_from_serial(scale.minimum as i64, date_1904)?;
  let mut positions = Vec::new();
  for _ in 0..10_000 {
    let serial = chart_date_serial(year, month, day, date_1904)?;
    if serial > scale.maximum + f64::EPSILON
      || (scale.shifted && serial >= scale.maximum - f64::EPSILON)
    {
      break;
    }
    if serial >= scale.minimum - f64::EPSILON {
      positions.push(date_axis_position(chart, scale, year, month, day, false)?);
    }
    (year, month, day) = advance_chart_date(year, month, day, interval, date_1904)?;
  }
  (!positions.is_empty()).then_some(positions)
}

/// Maps a cached date-category serial to the normalized plot coordinate used
/// by the series geometry. LibreOffice keeps the authored date scaling
/// unshifted for data points; `crossBetween="between"` only substitutes a
/// shifted scaling while it builds axis labels (`Tickmarks_Dates.cxx`).
///
/// The returned coordinate is intentionally not clamped. Chart plotters clip
/// the resulting series geometry to the visible scale rectangle, so segments
/// that enter or leave an explicitly narrowed date window retain their exact
/// boundary intersection.
pub fn date_axis_data_position(chart: &ClusteredColumnChart<'_>, serial: f64) -> Option<f64> {
  if !serial.is_finite() {
    return None;
  }
  let scale = date_axis_scale(chart)?;
  let minimum = date_axis_scaled_serial_value(chart, scale, scale.minimum, false)?;
  let maximum = date_axis_scaled_serial_value(chart, scale, scale.maximum, false)?;
  let value = date_axis_scaled_serial_value(chart, scale, serial, false)?;
  (maximum > minimum).then_some((value - minimum) / (maximum - minimum))
}

#[derive(Clone, Copy)]
struct DateAxisScale {
  minimum: f64,
  maximum: f64,
  resolution: c::TimeUnitValues,
  shifted: bool,
}

#[derive(Clone, Copy)]
struct DateAxisInterval {
  count: i32,
  unit: c::TimeUnitValues,
}

fn date_axis_scale(chart: &ClusteredColumnChart<'_>) -> Option<DateAxisScale> {
  let axis = chart.date_axis?;
  let mut source_minimum = f64::INFINITY;
  let mut source_maximum = f64::NEG_INFINITY;
  for value in chart
    .category_axis_values
    .iter()
    .flatten()
    .copied()
    .filter(|value| value.is_finite())
  {
    source_minimum = source_minimum.min(value);
    source_maximum = source_maximum.max(value);
  }
  if !source_minimum.is_finite() || !source_maximum.is_finite() {
    return None;
  }
  let mut minimum = axis
    .scaling
    .min_axis_value
    .as_ref()
    .map_or(source_minimum, |value| value.val)
    .floor();
  let mut maximum = axis
    .scaling
    .max_axis_value
    .as_ref()
    .map_or(source_maximum, |value| value.val)
    .floor();
  if !minimum.is_finite() || !maximum.is_finite() {
    return None;
  }
  if minimum > maximum {
    std::mem::swap(&mut minimum, &mut maximum);
  }
  let resolution = axis
    .base_time_unit
    .as_ref()
    .and_then(|unit| unit.val)
    .unwrap_or(c::TimeUnitValues::Days);
  let date_1904 = chart.date_1904;
  let (mut minimum_year, mut minimum_month, mut minimum_day) =
    chart_date_from_serial(minimum as i64, date_1904)?;
  let (mut maximum_year, mut maximum_month, mut maximum_day) =
    chart_date_from_serial(maximum as i64, date_1904)?;
  align_date_to_resolution_start(
    &mut minimum_year,
    &mut minimum_month,
    &mut minimum_day,
    resolution,
  );
  align_date_to_resolution_start(
    &mut maximum_year,
    &mut maximum_month,
    &mut maximum_day,
    resolution,
  );
  minimum = chart_date_serial(minimum_year, minimum_month, minimum_day, date_1904)?;
  maximum = chart_date_serial(maximum_year, maximum_month, maximum_day, date_1904)?;
  let shifted = chart.category_axis_shifted;
  if shifted || maximum <= minimum {
    (maximum_year, maximum_month, maximum_day) = advance_chart_date(
      maximum_year,
      maximum_month,
      maximum_day,
      DateAxisInterval {
        count: 1,
        unit: resolution,
      },
      date_1904,
    )?;
    maximum = chart_date_serial(maximum_year, maximum_month, maximum_day, date_1904)?;
  }
  (maximum > minimum).then_some(DateAxisScale {
    minimum,
    maximum,
    resolution,
    shifted,
  })
}

fn align_date_to_resolution_start(
  year: &mut i32,
  month: &mut i32,
  day: &mut i32,
  resolution: c::TimeUnitValues,
) {
  match resolution {
    c::TimeUnitValues::Days => {}
    c::TimeUnitValues::Months => *day = 1,
    c::TimeUnitValues::Years => {
      *month = 1;
      *day = 1;
    }
  }
  let _ = year;
}

fn date_axis_major_interval(
  axis: &c::DateAxis,
  scale: DateAxisScale,
  maximum_auto_increment_count: usize,
) -> DateAxisInterval {
  if let Some(count) = axis
    .major_unit
    .as_ref()
    .map(|unit| unit.val)
    .filter(|count| count.is_finite() && *count >= 1.0)
  {
    let unit = axis
      .major_time_unit
      .as_ref()
      .and_then(|unit| unit.val)
      .unwrap_or(c::TimeUnitValues::Days);
    return DateAxisInterval {
      count: count.trunc().min(f64::from(i32::MAX)) as i32,
      unit: coarser_time_unit(unit, scale.resolution),
    };
  }

  let maximum_count = maximum_auto_increment_count.max(2).saturating_sub(1) as i64;
  let day_count = (scale.maximum - scale.minimum).floor().max(1.0) as i64;
  let interval_days = day_count / maximum_count.max(1);
  let (mut unit, days_per_unit) =
    if interval_days > 365 || scale.resolution == c::TimeUnitValues::Years {
      (c::TimeUnitValues::Years, 365.0)
    } else if interval_days > 31 || scale.resolution == c::TimeUnitValues::Months {
      (c::TimeUnitValues::Months, 31.0)
    } else {
      (c::TimeUnitValues::Days, 1.0)
    };
  let mut count = ((interval_days as f64 / days_per_unit).floor() as i32).max(1);
  if unit == c::TimeUnitValues::Days {
    if (3..7).contains(&count) {
      count = 7;
    } else if count > 7 {
      unit = c::TimeUnitValues::Months;
      count = ((interval_days as f64 / 31.0).floor() as i32).max(1);
    }
  }
  DateAxisInterval { count, unit }
}

fn date_axis_minor_interval(
  axis: &c::DateAxis,
  scale: DateAxisScale,
  maximum_auto_increment_count: usize,
) -> DateAxisInterval {
  let major = date_axis_major_interval(axis, scale, maximum_auto_increment_count);
  if let Some(count) = axis
    .minor_unit
    .as_ref()
    .map(|unit| unit.val)
    .filter(|count| count.is_finite() && *count >= 1.0)
  {
    let unit = axis
      .minor_time_unit
      .as_ref()
      .and_then(|unit| unit.val)
      .unwrap_or(c::TimeUnitValues::Days);
    return DateAxisInterval {
      count: count.trunc().min(f64::from(i32::MAX)) as i32,
      unit: finer_time_unit(unit, major.unit),
    };
  }

  let mut minor = DateAxisInterval {
    count: 1,
    unit: major.unit,
  };
  if major.count >= 2 {
    minor.count = if major.count % 2 == 0 {
      major.count / 2
    } else if major.count % 3 == 0 {
      major.count / 3
    } else if major.count % 5 == 0 {
      major.count / 5
    } else if major.count > 50 {
      major.count
    } else {
      1
    };
  } else if major.unit == c::TimeUnitValues::Months && scale.resolution == c::TimeUnitValues::Days {
    minor.unit = c::TimeUnitValues::Days;
  } else if major.unit == c::TimeUnitValues::Years && scale.resolution != c::TimeUnitValues::Years {
    minor.unit = c::TimeUnitValues::Months;
  }
  minor
}

fn time_unit_rank(unit: c::TimeUnitValues) -> u8 {
  match unit {
    c::TimeUnitValues::Days => 0,
    c::TimeUnitValues::Months => 1,
    c::TimeUnitValues::Years => 2,
  }
}

fn coarser_time_unit(left: c::TimeUnitValues, right: c::TimeUnitValues) -> c::TimeUnitValues {
  if time_unit_rank(left) >= time_unit_rank(right) {
    left
  } else {
    right
  }
}

fn finer_time_unit(left: c::TimeUnitValues, right: c::TimeUnitValues) -> c::TimeUnitValues {
  if time_unit_rank(left) <= time_unit_rank(right) {
    left
  } else {
    right
  }
}

fn advance_chart_date(
  year: i32,
  month: i32,
  day: i32,
  interval: DateAxisInterval,
  date_1904: bool,
) -> Option<(i32, i32, i32)> {
  match interval.unit {
    c::TimeUnitValues::Days => {
      let serial = chart_date_serial(year, month, day, date_1904)? as i64;
      chart_date_from_serial(serial + i64::from(interval.count), date_1904)
    }
    c::TimeUnitValues::Months => {
      let month_index = year
        .checked_mul(12)?
        .checked_add(month - 1)?
        .checked_add(interval.count)?;
      let year = month_index.div_euclid(12);
      let month = month_index.rem_euclid(12) + 1;
      Some((year, month, day.min(gregorian_days_in_month(year, month)?)))
    }
    c::TimeUnitValues::Years => {
      let year = year.checked_add(interval.count)?;
      Some((year, month, day.min(gregorian_days_in_month(year, month)?)))
    }
  }
}

fn date_axis_position(
  chart: &ClusteredColumnChart<'_>,
  scale: DateAxisScale,
  year: i32,
  month: i32,
  day: i32,
  shifted_label: bool,
) -> Option<f64> {
  let (minimum_year, minimum_month, minimum_day) =
    chart_date_from_serial(scale.minimum as i64, chart.date_1904)?;
  let (maximum_year, maximum_month, maximum_day) =
    chart_date_from_serial(scale.maximum as i64, chart.date_1904)?;
  let minimum = date_axis_scaled_value(
    chart,
    scale.resolution,
    minimum_year,
    minimum_month,
    minimum_day,
    false,
  )?;
  let maximum = date_axis_scaled_value(
    chart,
    scale.resolution,
    maximum_year,
    maximum_month,
    maximum_day,
    false,
  )?;
  let value = date_axis_scaled_value(
    chart,
    scale.resolution,
    year,
    month,
    day,
    scale.shifted && shifted_label,
  )?;
  (maximum > minimum).then_some((value - minimum) / (maximum - minimum))
}

fn date_axis_scaled_serial_value(
  chart: &ClusteredColumnChart<'_>,
  scale: DateAxisScale,
  serial: f64,
  shifted: bool,
) -> Option<f64> {
  let whole_days = serial.floor();
  if whole_days < i64::MIN as f64 || whole_days > i64::MAX as f64 {
    return None;
  }
  let day_fraction = serial - whole_days;
  let (year, month, day) = chart_date_from_serial(whole_days as i64, chart.date_1904)?;
  let mut value = date_axis_scaled_value(chart, scale.resolution, year, month, day, shifted)?;
  value += if scale.resolution == c::TimeUnitValues::Days {
    day_fraction
  } else {
    day_fraction / f64::from(gregorian_days_in_month(year, month)?)
  };
  Some(value)
}

fn date_axis_scaled_value(
  chart: &ClusteredColumnChart<'_>,
  resolution: c::TimeUnitValues,
  year: i32,
  month: i32,
  day: i32,
  shifted: bool,
) -> Option<f64> {
  if resolution == c::TimeUnitValues::Days {
    return Some(
      chart_date_serial(year, month, day, chart.date_1904)? + if shifted { 0.5 } else { 0.0 },
    );
  }
  let days_in_month = f64::from(gregorian_days_in_month(year, month)?);
  let mut value = f64::from(year) * 12.0 + f64::from(month) + f64::from(day - 1) / days_in_month;
  if shifted {
    value += if resolution == c::TimeUnitValues::Years {
      6.0
    } else {
      0.5
    };
  }
  Some(value)
}

fn effective_date_axis_number_format<'a>(
  chart: &'a ClusteredColumnChart<'_>,
  axis: &'a c::DateAxis,
) -> (Option<&'a str>, bool) {
  let numbering_format = axis.numbering_format.as_ref();
  let authored = numbering_format
    .map(|format| format.format_code.as_str())
    .filter(|format| !format.is_empty());
  let source_linked = numbering_format.is_none_or(|format| {
    format
      .source_linked
      .is_none_or(|source_linked| source_linked.as_bool())
  });
  if source_linked {
    (
      chart.category_number_format_code.as_deref().or(authored),
      true,
    )
  } else {
    (authored, false)
  }
}

fn chart_date_from_serial(serial: i64, date_1904: bool) -> Option<(i32, i32, i32)> {
  let unix_days = serial
    - if date_1904 {
      24_107
    } else if serial < 60 {
      25_568
    } else {
      25_569
    };
  let (year, month, day) = civil_from_days(unix_days);
  Some((i32::try_from(year).ok()?, month as i32, day as i32))
}

fn chart_date_serial(year: i32, month: i32, day: i32, date_1904: bool) -> Option<f64> {
  let unix_days = days_from_civil(i64::from(year), month, day)?;
  let offset = if date_1904 {
    24_107
  } else if (year, month, day) < (1900, 3, 1) {
    25_568
  } else {
    25_569
  };
  Some((unix_days + offset) as f64)
}

fn days_from_civil(year: i64, month: i32, day: i32) -> Option<i64> {
  if !(1..=12).contains(&month)
    || !(1..=gregorian_days_in_month(i32::try_from(year).ok()?, month)?).contains(&day)
  {
    return None;
  }
  let year = year - i64::from(month <= 2);
  let era = year.div_euclid(400);
  let year_of_era = year - era * 400;
  let month_prime = i64::from(month) + if month > 2 { -3 } else { 9 };
  let day_of_year = (153 * month_prime + 2) / 5 + i64::from(day) - 1;
  let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
  Some(era * 146_097 + day_of_era - 719_468)
}

fn gregorian_days_in_month(year: i32, month: i32) -> Option<i32> {
  let days = match month {
    1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
    4 | 6 | 9 | 11 => 30,
    2 if year.rem_euclid(4) == 0 && (year.rem_euclid(100) != 0 || year.rem_euclid(400) == 0) => 29,
    2 => 28,
    _ => return None,
  };
  Some(days)
}

fn format_chart_date(
  year: i32,
  month: u32,
  day: u32,
  format_code: Option<&str>,
  source_linked: bool,
  format_locale: Option<&str>,
) -> String {
  let Some(value) = u16::try_from(year).ok().map(|year| FieldUpdateDateTime {
    year,
    month: month as u8,
    day: day as u8,
    hour: 0,
    minute: 0,
    second: 0,
  }) else {
    return format!("{year:04}/{month}/{day}");
  };
  if source_linked
    && format_code.is_none_or(is_source_linked_short_date_format)
    && let Some(text) = field_datetime::format_office_short_date(format_locale, value)
  {
    return text;
  }
  if let Some(code) = format_code {
    // A source-linked chart cache without an LCID keeps Office's invariant
    // package month-name vocabulary; it does not inherit the machine UI or
    // format culture. Explicit axis formats use the configured format locale,
    // and an embedded [$-LCID] marker overrides either fallback. This split is
    // visible in tdf134118.xlsx on a zh-CN Office host: d-mmm remains 10-May.
    let fallback_language = if source_linked { None } else { format_locale };
    if let Some(text) =
      field_datetime::format_spreadsheet_date_picture(code, fallback_language, value)
    {
      return text;
    }
  }
  field_datetime::format_office_short_date(format_locale, value)
    .unwrap_or_else(|| format!("{year:04}/{month}/{day}"))
}

fn is_source_linked_short_date_format(code: &str) -> bool {
  let normalized = code
    .split(';')
    .next()
    .unwrap_or(code)
    .chars()
    .filter(|ch| !ch.is_ascii_whitespace())
    .flat_map(char::to_lowercase)
    .collect::<String>();
  matches!(
    normalized.as_str(),
    "m/d/yy" | "m/d/yyyy" | "mm/dd/yy" | "mm/dd/yyyy"
  )
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

fn data_labels_range_values(range: Option<&c15::DataLabelsRange>) -> Vec<String> {
  let Some(cache) = range.and_then(|range| range.data_labels_range_chache.as_deref()) else {
    return Vec::new();
  };
  let length = cache
    .string_point
    .iter()
    .filter_map(|point| usize::try_from(point.index).ok())
    .max()
    .map_or(0, |index| index + 1);
  let mut values = vec![String::new(); length];
  for point in &cache.string_point {
    let Ok(index) = usize::try_from(point.index) else {
      continue;
    };
    values[index] = point.numeric_value.clone();
  }
  values
}

fn line_series_data_labels_range(series: &c::LineChartSeries) -> Option<&c15::DataLabelsRange> {
  series
    .line_ser_extension_list
    .as_ref()?
    .line_ser_extension
    .iter()
    .find_map(
      |extension| match extension.line_ser_extension_choice.as_ref()? {
        c::LineSerExtensionChoice::DataLabelsRange(range) => Some(range.as_ref()),
        _ => None,
      },
    )
}

fn scatter_series_data_labels_range(
  series: &c::ScatterChartSeries,
) -> Option<&c15::DataLabelsRange> {
  series
    .scatter_ser_extension_list
    .as_ref()?
    .scatter_ser_extension
    .iter()
    .find_map(
      |extension| match extension.scatter_ser_extension_choice.as_ref()? {
        c::ScatterSerExtensionChoice::DataLabelsRange(range) => Some(range.as_ref()),
        _ => None,
      },
    )
}

fn radar_series_data_labels_range(series: &c::RadarChartSeries) -> Option<&c15::DataLabelsRange> {
  series
    .radar_ser_extension_list
    .as_ref()?
    .radar_ser_extension
    .iter()
    .find_map(
      |extension| match extension.radar_ser_extension_choice.as_ref()? {
        c::RadarSerExtensionChoice::DataLabelsRange(range) => Some(range.as_ref()),
        _ => None,
      },
    )
}

fn bar_series_data_labels_range(series: &c::BarChartSeries) -> Option<&c15::DataLabelsRange> {
  series
    .bar_ser_extension_list
    .as_ref()?
    .bar_ser_extension
    .iter()
    .find_map(
      |extension| match extension.bar_ser_extension_choice.as_ref()? {
        c::BarSerExtensionChoice::DataLabelsRange(range) => Some(range.as_ref()),
        _ => None,
      },
    )
}

fn area_series_data_labels_range(series: &c::AreaChartSeries) -> Option<&c15::DataLabelsRange> {
  series
    .area_ser_extension_list
    .as_ref()?
    .area_ser_extension
    .iter()
    .find_map(
      |extension| match extension.area_ser_extension_choice.as_ref()? {
        c::AreaSerExtensionChoice::DataLabelsRange(range) => Some(range.as_ref()),
        _ => None,
      },
    )
}

fn pie_series_data_labels_range(series: &c::PieChartSeries) -> Option<&c15::DataLabelsRange> {
  series
    .pie_ser_extension_list
    .as_ref()?
    .pie_ser_extension
    .iter()
    .find_map(
      |extension| match extension.pie_ser_extension_choice.as_ref()? {
        c::PieSerExtensionChoice::DataLabelsRange(range) => Some(range.as_ref()),
        _ => None,
      },
    )
}

fn bubble_series_data_labels_range(series: &c::BubbleChartSeries) -> Option<&c15::DataLabelsRange> {
  series
    .bubble_ser_extension_list
    .as_ref()?
    .bubble_ser_extension
    .iter()
    .find_map(
      |extension| match extension.bubble_ser_extension_choice.as_ref()? {
        c::BubbleSerExtensionChoice::DataLabelsRange(range) => Some(range.as_ref()),
        _ => None,
      },
    )
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
    formatting_index: ser.index.val as usize,
    series_text: ser.series_text.as_deref(),
    category_axis_data: ser.category_axis_data.as_deref(),
    values: ser.values.as_deref(),
    y_values: None,
    x_values: None,
    bubble_size: None,
    data_labels: ser.data_labels.as_deref(),
    data_labels_range: area_series_data_labels_range(ser),
    chart_shape_properties: ser.chart_shape_properties.as_deref(),
    data_points: &ser.data_point,
    marker: None,
    smooth: None,
    trendlines: &ser.trendline,
    error_bars: [ser.error_bars.first(), ser.error_bars.get(1)],
  }
}

fn line_series_ref(ser: &c::LineChartSeries) -> ChartSeriesRef<'_> {
  ChartSeriesRef {
    formatting_index: ser.index.val as usize,
    series_text: ser.series_text.as_deref(),
    category_axis_data: ser.category_axis_data.as_deref(),
    values: ser.values.as_deref(),
    y_values: None,
    x_values: None,
    bubble_size: None,
    data_labels: ser.data_labels.as_deref(),
    data_labels_range: line_series_data_labels_range(ser),
    chart_shape_properties: ser.chart_shape_properties.as_deref(),
    data_points: &ser.data_point,
    marker: ser.marker.as_deref(),
    smooth: ser.smooth.as_ref(),
    trendlines: &ser.trendline,
    error_bars: [ser.error_bars.as_deref(), None],
  }
}

fn radar_series_ref(ser: &c::RadarChartSeries) -> ChartSeriesRef<'_> {
  ChartSeriesRef {
    formatting_index: ser.index.val as usize,
    series_text: ser.series_text.as_deref(),
    category_axis_data: ser.category_axis_data.as_deref(),
    values: ser.values.as_deref(),
    y_values: None,
    x_values: None,
    bubble_size: None,
    data_labels: ser.data_labels.as_deref(),
    data_labels_range: radar_series_data_labels_range(ser),
    chart_shape_properties: ser.chart_shape_properties.as_deref(),
    data_points: &ser.data_point,
    marker: ser.marker.as_deref(),
    smooth: None,
    trendlines: &[],
    error_bars: [None, None],
  }
}

fn scatter_series_ref(ser: &c::ScatterChartSeries) -> ChartSeriesRef<'_> {
  ChartSeriesRef {
    formatting_index: ser.index.val as usize,
    series_text: ser.series_text.as_deref(),
    category_axis_data: None,
    values: None,
    y_values: ser.y_values.as_deref(),
    x_values: ser.x_values.as_deref(),
    bubble_size: None,
    data_labels: ser.data_labels.as_deref(),
    data_labels_range: scatter_series_data_labels_range(ser),
    chart_shape_properties: ser.chart_shape_properties.as_deref(),
    data_points: &ser.data_point,
    marker: ser.marker.as_deref(),
    smooth: ser.smooth.as_ref(),
    trendlines: &ser.trendline,
    error_bars: [ser.error_bars.first(), ser.error_bars.get(1)],
  }
}

fn pie_series_ref(ser: &c::PieChartSeries) -> ChartSeriesRef<'_> {
  ChartSeriesRef {
    formatting_index: ser.index.as_ref().map_or(0, |index| index.val as usize),
    series_text: ser.series_text.as_deref(),
    category_axis_data: ser.category_axis_data.as_deref(),
    values: ser.values.as_deref(),
    y_values: None,
    x_values: None,
    bubble_size: None,
    data_labels: ser.data_labels.as_deref(),
    data_labels_range: pie_series_data_labels_range(ser),
    chart_shape_properties: ser.chart_shape_properties.as_deref(),
    data_points: &ser.data_point,
    marker: None,
    smooth: None,
    trendlines: &[],
    error_bars: [None, None],
  }
}

fn bar_series_ref(ser: &c::BarChartSeries) -> ChartSeriesRef<'_> {
  ChartSeriesRef {
    formatting_index: ser.index.val as usize,
    series_text: ser.series_text.as_deref(),
    category_axis_data: ser.category_axis_data.as_deref(),
    values: ser.values.as_deref(),
    y_values: None,
    x_values: None,
    bubble_size: None,
    data_labels: ser.data_labels.as_deref(),
    data_labels_range: bar_series_data_labels_range(ser),
    chart_shape_properties: ser.chart_shape_properties.as_deref(),
    data_points: &ser.data_point,
    marker: None,
    smooth: None,
    trendlines: &ser.trendline,
    error_bars: [ser.error_bars.as_deref(), None],
  }
}

fn surface_series_ref(ser: &c::SurfaceChartSeries) -> ChartSeriesRef<'_> {
  ChartSeriesRef {
    formatting_index: ser.index.val as usize,
    series_text: ser.series_text.as_deref(),
    category_axis_data: ser.category_axis_data.as_deref(),
    values: ser.values.as_deref(),
    y_values: None,
    x_values: None,
    bubble_size: None,
    data_labels: None,
    data_labels_range: None,
    chart_shape_properties: ser.chart_shape_properties.as_deref(),
    data_points: &[],
    marker: None,
    smooth: None,
    trendlines: &[],
    error_bars: [None, None],
  }
}

fn bubble_series_ref(ser: &c::BubbleChartSeries) -> ChartSeriesRef<'_> {
  ChartSeriesRef {
    formatting_index: ser.index.val as usize,
    series_text: ser.series_text.as_deref(),
    category_axis_data: None,
    values: None,
    y_values: ser.y_values.as_deref(),
    x_values: ser.x_values.as_deref(),
    bubble_size: ser.bubble_size.as_deref(),
    data_labels: ser.data_labels.as_deref(),
    data_labels_range: bubble_series_data_labels_range(ser),
    chart_shape_properties: ser.chart_shape_properties.as_deref(),
    data_points: &ser.data_point,
    marker: None,
    smooth: None,
    trendlines: &ser.trendline,
    error_bars: [ser.error_bars.first(), ser.error_bars.get(1)],
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
      .unwrap_or(", ");
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
    ChartTitleText, LinearAxisScaleOptions, automatic_chart_title, automatic_series_title,
    cartesian_chart_for_ui_language, chart_title_text, clustered_column_chart,
    clustered_column_slot, fixed_output_latin_font_family, fixed_output_texts_for_ui_language,
    format_chart_date, format_chart_number, has_indexed_scatter_multicomponent_data_labels,
    largest_remainder_percentages, linear_axis_scale, linear_axis_scale_with_options,
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
  fn largest_remainder_percentages_rejects_mixed_sign_and_invalid_totals() {
    assert_eq!(
      largest_remainder_percentages(&[Some(1.0), Some(-1.0)], f64::EPSILON * 2.0),
      vec![0.0, 0.0]
    );
    assert_eq!(
      largest_remainder_percentages(&[Some(1.0), Some(2.0)], f64::INFINITY),
      vec![0.0, 0.0]
    );
  }

  #[test]
  fn indexed_scatter_multicomponent_labels_are_detected_before_value_resolution() {
    let chart_space = c::ChartSpace::from_bytes(
      br#"<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"><c:chart><c:plotArea><c:scatterChart><c:scatterStyle val="lineMarker"/><c:ser><c:idx val="0"/><c:order val="0"/><c:dLbls><c:showVal val="1"/><c:showSerName val="1"/></c:dLbls><c:xVal><c:strRef><c:f>Sheet1!$C$6:$C$9</c:f></c:strRef></c:xVal><c:yVal><c:numRef><c:f>Sheet1!$D$6:$D$9</c:f></c:numRef></c:yVal></c:ser><c:axId val="1"/><c:axId val="2"/></c:scatterChart></c:plotArea></c:chart></c:chartSpace>"#,
    )
    .expect("chart space");

    assert!(has_indexed_scatter_multicomponent_data_labels(&chart_space));
  }

  #[test]
  fn three_dimensional_cartesian_groups_preserve_depth_and_surface_semantics() {
    let chart_space = c::ChartSpace::from_bytes(
      br#"<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><c:chart><c:plotArea><c:area3DChart><c:grouping val="standard"/><c:ser><c:idx val="0"/><c:order val="0"/><c:val><c:numLit><c:pt idx="0"><c:v>1</c:v></c:pt></c:numLit></c:val></c:ser><c:gapDepth val="275"/><c:axId val="1"/><c:axId val="2"/><c:axId val="3"/></c:area3DChart><c:line3DChart><c:grouping val="standard"/><c:ser><c:idx val="1"/><c:order val="1"/><c:val><c:numLit><c:pt idx="0"><c:v>2</c:v></c:pt></c:numLit></c:val></c:ser><c:gapDepth val="80"/><c:axId val="1"/><c:axId val="2"/><c:axId val="3"/></c:line3DChart><c:surface3DChart><c:wireframe/><c:ser><c:idx val="2"/><c:order val="2"/><c:val><c:numLit><c:pt idx="0"><c:v>3</c:v></c:pt></c:numLit></c:val></c:ser><c:bandFmts><c:bandFmt><c:idx val="4"/><c:spPr><a:solidFill><a:srgbClr val="FF0000"/></a:solidFill></c:spPr></c:bandFmt></c:bandFmts><c:axId val="1"/><c:axId val="2"/><c:axId val="3"/></c:surface3DChart></c:plotArea></c:chart></c:chartSpace>"#,
    )
    .expect("chart space");

    let chart = cartesian_chart_for_ui_language(&chart_space, None).expect("cartesian chart");
    assert_eq!(chart.series.len(), 3);
    assert_eq!(chart.series[0].gap_depth_percent, 275.0);
    assert_eq!(chart.series[1].gap_depth_percent, 80.0);
    assert_eq!(chart.surface_groups.len(), 1);
    assert!(chart.surface_groups[0].is_3d);
    assert!(chart.surface_groups[0].wireframe);
    assert_eq!(chart.surface_groups[0].first_series_index, 2);
    assert_eq!(chart.surface_groups[0].series_count, 1);
    assert_eq!(chart.surface_groups[0].band_fills[0].index, 4);
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

    let labels = fixed_output_texts_for_ui_language(&chart_space, None)
      .into_iter()
      .map(|label| label.split_whitespace().collect::<Vec<_>>().join(" "))
      .collect::<Vec<_>>();
    assert_eq!(labels, ["A 34%", "B 33%", "C 33%"]);
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
  fn pie_automatic_title_uses_the_effective_first_series() {
    let chart_space = c::ChartSpace::from_bytes(
      br#"<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"><c:chart><c:title/><c:autoTitleDeleted val="0"/><c:plotArea><c:pieChart><c:ser><c:idx val="0"/><c:order val="0"/><c:tx><c:v>col1</c:v></c:tx><c:val><c:numLit><c:pt idx="0"><c:v>1</c:v></c:pt></c:numLit></c:val></c:ser><c:ser><c:idx val="1"/><c:order val="1"/><c:tx><c:v>col2</c:v></c:tx><c:val><c:numLit><c:pt idx="0"><c:v>2</c:v></c:pt></c:numLit></c:val></c:ser></c:pieChart></c:plotArea></c:chart></c:chartSpace>"#,
    )
    .expect("chart space");

    let pie = pie_chart_model(&chart_space).expect("pie chart");
    assert_eq!(
      pie.title,
      Some(ChartTitleText::Explicit("col1".to_string()))
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
  fn pie_model_preserves_legend_text_vertical_anchor() {
    let chart_space = c::ChartSpace::from_bytes(
      br#"<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><c:chart><c:plotArea><c:pieChart><c:ser><c:idx val="0"/><c:order val="0"/><c:val><c:numLit><c:pt idx="0"><c:v>1</c:v></c:pt></c:numLit></c:val></c:ser></c:pieChart></c:plotArea><c:legend><c:legendPos val="b"/><c:txPr><a:bodyPr anchor="ctr"/><a:lstStyle/><a:p/></c:txPr></c:legend></c:chart></c:chartSpace>"#,
    )
    .expect("chart space");

    let pie = pie_chart_model(&chart_space).expect("pie chart");
    assert_eq!(
      pie.legend_vertical_anchor,
      Some(
        ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextAnchoringTypeValues::Center
      )
    );
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
      br#"<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><c:chart><c:title><c:txPr><a:bodyPr/><a:lstStyle/><a:p/></c:txPr></c:title><c:autoTitleDeleted val="0"/><c:plotArea/></c:chart></c:chartSpace>"#,
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

    let word_empty_deleted_title = c::ChartSpace::from_bytes(
      br#"<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"><c:chart><c:title/><c:autoTitleDeleted val="1"/><c:plotArea/></c:chart></c:chartSpace>"#,
    )
    .expect("chart space");
    assert!(super::has_word_automatic_title_placeholder(
      &word_empty_deleted_title.chart
    ));
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
  fn single_series_vary_colors_uses_point_legend_entries() {
    let chart_space = c::ChartSpace::from_bytes(
      br#"<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"><c:chart><c:plotArea><c:barChart><c:barDir val="col"/><c:varyColors val="1"/><c:ser><c:idx val="0"/><c:order val="0"/><c:tx><c:v>Revenue</c:v></c:tx><c:cat><c:strLit><c:pt idx="0"><c:v>North</c:v></c:pt><c:pt idx="1"><c:v>South</c:v></c:pt></c:strLit></c:cat><c:val><c:numLit><c:pt idx="0"><c:v>1</c:v></c:pt><c:pt idx="1"><c:v>2</c:v></c:pt></c:numLit></c:val></c:ser></c:barChart></c:plotArea><c:legend><c:legendEntry><c:idx val="1"/><c:delete val="1"/></c:legendEntry></c:legend></c:chart></c:chartSpace>"#,
    )
    .expect("chart space");

    let chart = clustered_column_chart(&chart_space).expect("clustered chart");
    assert!(chart.vary_colors_by_point);
    assert_eq!(chart.visible_legend_indices, [0]);
    assert_eq!(
      fixed_output_texts_for_ui_language(&chart_space, None),
      ["North", "South"]
    );
  }

  #[test]
  fn multiple_series_ignore_vary_colors_for_legend_entries() {
    let chart_space = c::ChartSpace::from_bytes(
      br#"<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"><c:chart><c:plotArea><c:barChart><c:barDir val="col"/><c:varyColors val="1"/><c:ser><c:idx val="0"/><c:order val="0"/><c:tx><c:v>Revenue</c:v></c:tx><c:cat><c:strLit><c:pt idx="0"><c:v>North</c:v></c:pt></c:strLit></c:cat><c:val><c:numLit><c:pt idx="0"><c:v>1</c:v></c:pt></c:numLit></c:val></c:ser><c:ser><c:idx val="1"/><c:order val="1"/><c:tx><c:v>Costs</c:v></c:tx><c:cat><c:strLit><c:pt idx="0"><c:v>North</c:v></c:pt></c:strLit></c:cat><c:val><c:numLit><c:pt idx="0"><c:v>2</c:v></c:pt></c:numLit></c:val></c:ser></c:barChart></c:plotArea><c:legend/></c:chart></c:chartSpace>"#,
    )
    .expect("chart space");

    let chart = clustered_column_chart(&chart_space).expect("clustered chart");
    assert!(!chart.vary_colors_by_point);
    assert_eq!(chart.visible_legend_indices, [0, 1]);
    assert_eq!(
      fixed_output_texts_for_ui_language(&chart_space, None),
      ["North", "Revenue", "Costs"]
    );
  }

  #[test]
  fn scatter_axes_follow_group_axis_ids_instead_of_xml_axis_order() {
    let chart_space = c::ChartSpace::from_bytes(
      br#"<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"><c:chart><c:plotArea><c:scatterChart><c:scatterStyle val="marker"/><c:ser><c:idx val="0"/><c:order val="0"/><c:xVal><c:numLit><c:pt idx="0"><c:v>1</c:v></c:pt></c:numLit></c:xVal><c:yVal><c:numLit><c:pt idx="0"><c:v>2</c:v></c:pt></c:numLit></c:yVal></c:ser><c:axId val="11"/><c:axId val="22"/></c:scatterChart><c:valAx><c:axId val="22"/><c:scaling/><c:axPos val="l"/><c:crossAx val="11"/></c:valAx><c:valAx><c:axId val="11"/><c:scaling/><c:axPos val="b"/><c:crossAx val="22"/></c:valAx></c:plotArea></c:chart></c:chartSpace>"#,
    )
    .expect("chart space");

    let chart = cartesian_chart_for_ui_language(&chart_space, None).expect("scatter chart");

    assert_eq!(
      chart.horizontal_value_axis.map(|axis| axis.axis_id.val),
      Some(11)
    );
    assert_eq!(chart.value_axis.map(|axis| axis.axis_id.val), Some(22));
    assert_eq!(chart.axis_sets.len(), 1);
    assert_eq!(chart.axis_sets[0].axis_ids, [11, 22]);
    assert_eq!(chart.series[0].axis_set_index, 0);
  }

  #[test]
  fn combined_chart_preserves_primary_and_secondary_axis_sets() {
    let chart_space = c::ChartSpace::from_bytes(
      br#"<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"><c:chart><c:plotArea><c:barChart><c:barDir val="col"/><c:ser><c:idx val="0"/><c:order val="0"/><c:cat><c:strLit><c:pt idx="0"><c:v>A</c:v></c:pt></c:strLit></c:cat><c:val><c:numLit><c:pt idx="0"><c:v>2</c:v></c:pt></c:numLit></c:val></c:ser><c:axId val="1"/><c:axId val="2"/></c:barChart><c:lineChart><c:grouping val="standard"/><c:ser><c:idx val="1"/><c:order val="1"/><c:cat><c:strLit><c:pt idx="0"><c:v>A</c:v></c:pt></c:strLit></c:cat><c:val><c:numLit><c:pt idx="0"><c:v>20</c:v></c:pt></c:numLit></c:val></c:ser><c:axId val="3"/><c:axId val="4"/></c:lineChart><c:catAx><c:axId val="1"/><c:scaling/><c:axPos val="b"/><c:crossAx val="2"/></c:catAx><c:valAx><c:axId val="2"/><c:scaling/><c:axPos val="l"/><c:crossAx val="1"/></c:valAx><c:catAx><c:axId val="3"/><c:scaling/><c:axPos val="t"/><c:crossAx val="4"/></c:catAx><c:valAx><c:axId val="4"/><c:scaling/><c:axPos val="r"/><c:crossAx val="3"/></c:valAx></c:plotArea></c:chart></c:chartSpace>"#,
    )
    .expect("chart space");

    let chart = cartesian_chart_for_ui_language(&chart_space, None).expect("combined chart");

    assert_eq!(chart.axis_sets.len(), 2);
    assert_eq!(chart.axis_sets[0].axis_ids, [1, 2]);
    assert_eq!(chart.axis_sets[1].axis_ids, [3, 4]);
    assert_eq!(
      chart
        .series
        .iter()
        .map(|series| series.axis_set_index)
        .collect::<Vec<_>>(),
      [0, 1]
    );
  }

  #[test]
  fn combined_chart_uses_later_visible_category_axis_without_reordering_value_axes() {
    let chart_space = c::ChartSpace::from_bytes(
      br#"<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"><c:chart><c:plotArea><c:barChart><c:barDir val="col"/><c:ser><c:idx val="0"/><c:order val="0"/><c:cat><c:strLit><c:pt idx="0"><c:v>Category 1</c:v></c:pt></c:strLit></c:cat><c:val><c:numLit><c:pt idx="0"><c:v>6</c:v></c:pt></c:numLit></c:val></c:ser><c:axId val="1"/><c:axId val="2"/></c:barChart><c:lineChart><c:grouping val="standard"/><c:ser><c:idx val="1"/><c:order val="1"/><c:cat><c:strLit><c:pt idx="0"><c:v>Category 1</c:v></c:pt></c:strLit></c:cat><c:val><c:numLit><c:pt idx="0"><c:v>5</c:v></c:pt></c:numLit></c:val></c:ser><c:axId val="3"/><c:axId val="4"/></c:lineChart><c:catAx><c:axId val="1"/><c:scaling/><c:delete val="1"/><c:axPos val="b"/><c:crossAx val="2"/></c:catAx><c:valAx><c:axId val="2"/><c:scaling/><c:axPos val="r"/><c:crossAx val="1"/></c:valAx><c:catAx><c:axId val="3"/><c:scaling/><c:axPos val="b"/><c:crossAx val="4"/></c:catAx><c:valAx><c:axId val="4"/><c:scaling/><c:axPos val="l"/><c:crossBetween val="midCat"/><c:crossAx val="3"/></c:valAx></c:plotArea></c:chart></c:chartSpace>"#,
    )
    .expect("chart space");

    let chart = cartesian_chart_for_ui_language(&chart_space, None).expect("combined chart");

    assert_eq!(chart.category_axis.map(|axis| axis.axis_id.val), Some(3));
    assert_eq!(chart.value_axis.map(|axis| axis.axis_id.val), Some(2));
    assert!(!chart.category_axis_shifted);
    assert_eq!(
      chart
        .series
        .iter()
        .map(|series| series.axis_set_index)
        .collect::<Vec<_>>(),
      [0, 1]
    );
  }

  #[test]
  fn display_units_scale_value_labels_and_localize_the_axis_unit() {
    let chart_space = c::ChartSpace::from_bytes(
      br#"<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"><c:chart><c:plotArea><c:barChart><c:barDir val="col"/><c:ser><c:idx val="0"/><c:order val="0"/><c:dLbls><c:showVal val="1"/></c:dLbls><c:cat><c:strLit><c:pt idx="0"><c:v>A</c:v></c:pt></c:strLit></c:cat><c:val><c:numLit><c:formatCode>0.0E+00</c:formatCode><c:pt idx="0"><c:v>4.3</c:v></c:pt></c:numLit></c:val></c:ser><c:axId val="1"/><c:axId val="2"/></c:barChart><c:catAx><c:axId val="1"/><c:scaling/><c:axPos val="b"/><c:crossAx val="2"/></c:catAx><c:valAx><c:axId val="2"/><c:scaling/><c:axPos val="l"/><c:dispUnits><c:builtInUnit val="billions"/></c:dispUnits><c:crossAx val="1"/></c:valAx></c:plotArea></c:chart></c:chartSpace>"#,
    )
    .expect("chart space");

    let chart = cartesian_chart_for_ui_language(&chart_space, Some("zh-CN")).expect("column chart");

    assert_eq!(chart.series[0].data_labels[0].text, "4.3E-09");
    assert_eq!(
      super::value_axis_display_unit_label_text(
        chart.value_axis.expect("value axis"),
        chart.ui_language.as_deref(),
      )
      .as_deref(),
      Some("十亿")
    );
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
  fn three_dimensional_linear_scale_keeps_a_value_on_the_increment_border() {
    let scale = linear_axis_scale_with_options(
      [2.0, 2.0, 3.0, 5.0],
      None,
      10,
      LinearAxisScaleOptions {
        expand_if_values_close_to_border: false,
        minimum_automatic_major_unit: None,
      },
    )
    .expect("finite values produce a scale");

    assert_eq!(scale.minimum, 0.0);
    assert_eq!(scale.maximum, 5.0);
    assert_eq!(scale.major_unit, 0.5);
  }

  #[test]
  fn three_dimensional_bar_model_preserves_view_shape_and_gap_depth() {
    let chart_space = c::ChartSpace::from_bytes(
      br#"<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"><c:chart><c:view3D><c:rotX val="25"/><c:rotY val="35"/><c:hPercent val="120"/><c:depthPercent val="180"/><c:rAngAx val="0"/><c:perspective val="60"/></c:view3D><c:plotArea><c:bar3DChart><c:barDir val="col"/><c:grouping val="clustered"/><c:ser><c:idx val="0"/><c:order val="0"/><c:cat><c:strLit><c:pt idx="0"><c:v>A</c:v></c:pt></c:strLit></c:cat><c:val><c:numLit><c:pt idx="0"><c:v>5</c:v></c:pt></c:numLit></c:val></c:ser><c:gapDepth val="225"/><c:shape val="pyramidToMax"/><c:axId val="1"/><c:axId val="2"/><c:axId val="3"/></c:bar3DChart><c:catAx><c:axId val="1"/><c:scaling/><c:axPos val="b"/><c:crossAx val="2"/></c:catAx><c:valAx><c:axId val="2"/><c:scaling/><c:axPos val="l"/><c:crossAx val="1"/></c:valAx><c:serAx><c:axId val="3"/><c:scaling/><c:axPos val="b"/><c:crossAx val="2"/></c:serAx></c:plotArea></c:chart></c:chartSpace>"#,
    )
    .expect("chart space");

    let chart = cartesian_chart_for_ui_language(&chart_space, None).expect("3-D chart");
    let view = chart.view_3d.expect("3-D view");

    assert_eq!(view.rotate_x_deg, 25.0);
    assert_eq!(view.rotate_y_deg, 35.0);
    assert_eq!(view.height_percent, 120.0);
    assert_eq!(view.depth_percent, 180.0);
    assert!(!view.right_angle_axes);
    assert_eq!(view.perspective_half_degrees, 60.0);
    assert_eq!(chart.series[0].shape_3d, c::ShapeValues::PyramidToMaximum);
    assert_eq!(chart.series[0].gap_depth_percent, 225.0);
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
  fn linear_scale_expands_values_sitting_on_both_nonzero_borders() {
    let scale = linear_axis_scale([-2.0, 3.0], None, 10).expect("finite values produce a scale");

    assert_eq!(scale.minimum, -3.0);
    assert_eq!(scale.maximum, 4.0);
    assert_eq!(scale.major_unit, 1.0);
  }

  #[test]
  fn linear_scale_mirrors_the_positive_algorithm_for_negative_only_values() {
    let scale = linear_axis_scale([-5.0, -2.0], None, 10).expect("finite values produce a scale");

    assert_eq!(scale.minimum, -6.0);
    assert_eq!(scale.maximum, 0.0);
    assert_eq!(scale.major_unit, 1.0);
  }

  #[test]
  fn date_axis_ticks_follow_calendar_months_and_explicit_bounds() {
    let chart_space = c::ChartSpace::from_bytes(
      br#"<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"><c:date1904 val="0"/><c:chart><c:plotArea><c:lineChart><c:grouping val="standard"/><c:ser><c:idx val="0"/><c:order val="0"/><c:cat><c:numLit><c:formatCode>yyyy\-mm\-dd</c:formatCode><c:pt idx="0"><c:v>43466</c:v></c:pt><c:pt idx="1"><c:v>43646</c:v></c:pt></c:numLit></c:cat><c:val><c:numLit><c:pt idx="0"><c:v>1</c:v></c:pt><c:pt idx="1"><c:v>2</c:v></c:pt></c:numLit></c:val></c:ser><c:axId val="1"/><c:axId val="2"/></c:lineChart><c:dateAx><c:axId val="1"/><c:scaling><c:min val="43466"/><c:max val="43586"/></c:scaling><c:axPos val="b"/><c:numFmt formatCode="yyyy\-mm\-dd" sourceLinked="1"/><c:crossAx val="2"/><c:baseTimeUnit val="days"/><c:majorUnit val="1"/><c:majorTimeUnit val="months"/><c:minorUnit val="7"/><c:minorTimeUnit val="days"/></c:dateAx><c:valAx><c:axId val="2"/><c:scaling/><c:axPos val="l"/><c:crossAx val="1"/></c:valAx></c:plotArea></c:chart></c:chartSpace>"#,
    )
    .expect("chart space");

    let chart = super::cartesian_chart_for_ui_language(&chart_space, None).expect("line chart");
    let ticks = super::date_axis_ticks(&chart).expect("date ticks");
    assert_eq!(
      ticks
        .iter()
        .map(|tick| tick.text.as_str())
        .collect::<Vec<_>>(),
      [
        "2019-01-01",
        "2019-02-01",
        "2019-03-01",
        "2019-04-01",
        "2019-05-01",
      ]
    );
    assert_eq!(ticks.first().map(|tick| tick.position), Some(0.5 / 121.0));
    assert_eq!(ticks.last().map(|tick| tick.position), Some(120.5 / 121.0));
    let minor = super::date_axis_minor_tick_positions(&chart).expect("minor date ticks");
    assert_eq!(minor.len(), 18);
    assert!((minor[1] - 7.0 / 121.0).abs() < f64::EPSILON);
  }

  #[test]
  fn source_linked_chart_cache_month_names_do_not_follow_the_host_locale() {
    assert_eq!(
      format_chart_date(2020, 5, 10, Some(r"d\-mmm"), true, Some("zh-CN")),
      "10-May"
    );
    assert_eq!(
      format_chart_date(2020, 5, 10, Some(r"d\-mmm"), false, Some("zh-CN")),
      "10-5月"
    );
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
