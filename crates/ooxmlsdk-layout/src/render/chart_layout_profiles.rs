//! Office automatic-chart layout profiles shared by the document hosts.
//!
//! ECMA-376 and MS-OI29500 define chart data, defaults, and manual-layout
//! semantics, but do not specify the final coordinates chosen by the Office
//! hosts for automatic layout. Keep ratios measured from immutable Microsoft
//! Office fixed output in this data-only module so lowering code selects a
//! named profile instead of accumulating anonymous fixture-tuned literals.
//!
//! Promotion identities and hashes are recorded in
//! `ooxmlsdk-test-suite/docs/ooxmlsdk-pdf-test/corpus_pdf_conv.md`.

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct CartesianHostDefaults {
  pub title_top_ratio: f32,
  pub category_bottom_ratio: f32,
  pub untitled_side_plot_top_ratio: f32,
  pub untitled_no_side_plot_top_ratio: f32,
  pub titled_top_legend_gap_ratio: f32,
}

pub(crate) const POWERPOINT_CARTESIAN_DEFAULTS: CartesianHostDefaults = CartesianHostDefaults {
  title_top_ratio: 0.024,
  category_bottom_ratio: 0.018,
  untitled_side_plot_top_ratio: 0.0449,
  untitled_no_side_plot_top_ratio: 0.035,
  titled_top_legend_gap_ratio: 0.009,
};

pub(crate) const WORD_CARTESIAN_DEFAULTS: CartesianHostDefaults = CartesianHostDefaults {
  title_top_ratio: 0.0365,
  category_bottom_ratio: 0.022_87,
  untitled_side_plot_top_ratio: 0.0449,
  untitled_no_side_plot_top_ratio: 0.035,
  titled_top_legend_gap_ratio: 0.0375,
};

pub(crate) const EXCEL_CARTESIAN_DEFAULTS: CartesianHostDefaults = CartesianHostDefaults {
  title_top_ratio: 0.024,
  category_bottom_ratio: 0.05,
  untitled_side_plot_top_ratio: 0.032,
  untitled_no_side_plot_top_ratio: 0.025,
  titled_top_legend_gap_ratio: 0.009,
};

/// Additional top inset for a legacy Excel cartesian title that inherits the
/// application's 18pt default instead of carrying an authored run size.
pub(crate) const EXCEL_LEGACY_DEFAULT_TITLE_TOP_ADJUSTMENT_RATIO: f32 = 0.023_15;
pub(crate) const EXCEL_LEGACY_DEFAULT_SINGLE_SERIES_SIDE_TITLE: CartesianLayoutAdjustment =
  CartesianLayoutAdjustment {
    category_top_ratio: 0.002_61,
    plot_top_ratio: -0.001_5,
    plot_bottom_ratio: -0.001_92,
    plot_left_ratio: 0.007_31,
    plot_right_ratio: 0.003_2,
    ..ZERO_ADJUSTMENT
  };
pub(crate) const EXCEL_LEGACY_DEFAULT_SINGLE_SERIES_LEGEND_RESERVATION_EM: f32 = 0.5;

pub(crate) const TOP_LEGEND_LEFT_INSET_RATIO: f32 = 0.004;
pub(crate) const UNTITLED_TOP_LEGEND_TOP_RATIO: f32 = 0.018;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct CartesianLayoutAdjustment {
  pub title_top_ratio: f32,
  pub category_top_ratio: f32,
  pub plot_top_ratio: f32,
  pub plot_bottom_ratio: f32,
  pub tick_left_ratio: f32,
  pub tick_top_ratio: f32,
  pub plot_left_ratio: f32,
  pub plot_right_ratio: f32,
}

/// PowerPoint's legacy empty-title container resolved from the sole series.
///
/// Apache POI's bar/line/radar/scatter fixtures share the same 480x320pt
/// chart frame and immutable Microsoft Office fixed output. The ordinary
/// line and indexed-scatter plots share this residual axis rectangle after
/// the generated series title is reserved.
pub(crate) const POWERPOINT_DERIVED_SERIES_TITLE_SIDE_LEGEND: CartesianLayoutAdjustment =
  CartesianLayoutAdjustment {
    category_top_ratio: -0.003_025,
    plot_top_ratio: -0.005_282,
    plot_bottom_ratio: -0.002_813,
    tick_left_ratio: 0.002_16,
    plot_left_ratio: 0.005_262,
    plot_right_ratio: -0.021_514,
    ..ZERO_ADJUSTMENT
  };

/// PowerPoint's generated-title layout with a bottom legend.
///
/// `tdf148117.pptx` carries an empty title container with no `c:layout`,
/// `autoTitleDeleted=0`, and no manual plot layout. PowerPoint therefore supplies the localized UI
/// title and uses a different title-to-plot reservation from an authored
/// title. These residual frame ratios are measured from its immutable Office
/// fixed output; the category and plot-bottom values remain independent
/// because the lower axis-label band moves by a smaller amount than the plot.
pub(crate) const POWERPOINT_GENERATED_TITLE_BOTTOM_LEGEND: CartesianLayoutAdjustment =
  CartesianLayoutAdjustment {
    title_top_ratio: -0.004_441,
    category_top_ratio: 0.000_502,
    plot_top_ratio: 0.006_816,
    plot_bottom_ratio: -0.000_274,
    ..ZERO_ADJUSTMENT
  };

/// PowerPoint's generated-title layout without a legend.
///
/// A title element without `c:tx`, combined with `autoTitleDeleted="0"`,
/// resolves to the first series name. PowerPoint reserves a balanced inner
/// plot around that generated title even when no legend consumes the right or
/// bottom band. The ratios come from the immutable Microsoft fixed output for
/// LibreOffice `date-categories.pptx`; its explicit date window and 90-degree
/// tick labels expose every edge of the resulting plot rectangle.
pub(crate) const POWERPOINT_GENERATED_TITLE_NO_LEGEND: CartesianLayoutAdjustment =
  CartesianLayoutAdjustment {
    title_top_ratio: 0.010_795,
    category_top_ratio: -0.012_536,
    plot_top_ratio: 0.024_713,
    plot_bottom_ratio: -0.008_722,
    tick_left_ratio: 0.015_232,
    tick_top_ratio: 0.0,
    plot_left_ratio: 0.012_497,
    plot_right_ratio: -0.024_926,
  };

/// The swapped axes of the matching horizontal-bar fixture use a taller plot
/// band and a narrower horizontal value-axis span than line/scatter charts.
pub(crate) const POWERPOINT_DERIVED_SERIES_TITLE_HORIZONTAL_BAR: CartesianLayoutAdjustment =
  CartesianLayoutAdjustment {
    plot_top_ratio: -0.024_28,
    plot_bottom_ratio: 0.014_47,
    plot_left_ratio: 0.009_34,
    plot_right_ratio: -0.036_59,
    ..ZERO_ADJUSTMENT
  };

/// A single-entry side legend is centered in PowerPoint's residual plot band,
/// not in the complete chart frame, for the legacy derived-title profile.
pub(crate) const POWERPOINT_DERIVED_SERIES_TITLE_LEGEND_Y_RATIO: f32 = 0.051_91;

/// Length of an automatic major tick in PowerPoint's fixed chart output.
///
/// The POI chart family emits 5.71pt outward ticks for both category and
/// value axes. This remains a physical distance rather than a plot ratio,
/// matching LibreOffice's `AXIS2D_TICKLENGTH` fixed-distance model.
pub(crate) const POWERPOINT_AUTOMATIC_MAJOR_TICK_LENGTH_PT: f32 = 5.71;

/// Normalized half-aperture used by Office's cartesian 3-D chart camera.
///
/// `c:perspective` stores twice the field-of-view angle (MS-OE376,
/// §21.2.2.152). The projection therefore follows the ordinary pinhole
/// relation `distance = aperture / tan(field_of_view / 2)`. The aperture is
/// expressed in the unit-width chart volume used by the lowering code and is
/// calibrated against Microsoft's immutable O12 PowerPoint fixed output; the
/// same camera is shared by Word, Excel, and PowerPoint chart hosts.
pub(crate) const OFFICE_CARTESIAN_3D_CAMERA_HALF_APERTURE: f32 = 0.414_75;

/// Office's default directional-light response for the receding side of a
/// solid cartesian 3-D box.
///
/// The ratio is stable across the blue, red, and green series in Microsoft's
/// immutable O12 PowerPoint output. LibreOffice likewise delegates cuboid
/// face colors to its 3-D scene lights instead of applying a white tint.
pub(crate) const OFFICE_CARTESIAN_3D_BOX_SIDE_SHADE: f32 = 0.64;

/// Office's default directional-light response for the visible top face of a
/// solid cartesian 3-D box.
pub(crate) const OFFICE_CARTESIAN_3D_BOX_TOP_SHADE: f32 = 0.76;

/// Residual PowerPoint screen transform after the standards-defined camera
/// projection and automatic scene fit.
///
/// The matrix is normalized around the fitted scene center. It is measured
/// from the front horizontal and vertical edges of Microsoft's immutable O12
/// 3-D column output, and keeps those independent vectors separate from the
/// camera's depth convergence.
pub(crate) const POWERPOINT_CARTESIAN_3D_SCREEN_MATRIX: [f32; 4] =
  [1.006_924, -0.009_923, -0.014_393, 1.006_372];

/// A visible projected series axis moves PowerPoint's complete 3-D scene
/// upward within the automatic chart frame before category-label placement.
pub(crate) const POWERPOINT_CARTESIAN_3D_SERIES_AXIS_TOP_RATIO: f32 = -0.031_64;

/// Category text remains below the shifted front floor, using the larger
/// automatic gap observed for PowerPoint's projected series-axis layout.
pub(crate) const POWERPOINT_CARTESIAN_3D_CATEGORY_LABEL_GAP_EM: f32 = 0.93;

/// Position of a PowerPoint shifted series-axis label inside its depth slab.
///
/// LibreOffice's axis pipeline explicitly builds shifted tick information for
/// series labels and a separate unshifted set for tick marks. Microsoft's O12
/// fixed output places the label axis in the leading part of each marker slab,
/// independently of the marker's geometric center.
pub(crate) const POWERPOINT_CARTESIAN_3D_SERIES_AXIS_SLOT_RATIO: f32 = 0.225;

/// Word side-legend charts with an explicit title.
///
/// The ratios are stable across the solid, gradient, and bitmap title-fill
/// Office golden identities promoted together as the Word chart-title family.
pub(crate) const WORD_EXPLICIT_TITLE_SIDE_LEGEND: CartesianLayoutAdjustment =
  CartesianLayoutAdjustment {
    category_top_ratio: 0.010_665,
    plot_top_ratio: 0.012_695,
    ..ZERO_ADJUSTMENT
  };

/// Word's automatic untitled side-legend plot reservation.
///
/// Office's O12 fixed output keeps the category-label band at the generic
/// Word position, but lifts the plot by roughly a quarter point in a 252pt
/// chart frame. Keep that residual automatic-layout adjustment independent
/// from the explicit-title profile above.
pub(crate) const WORD_UNTITLED_SIDE_LEGEND: CartesianLayoutAdjustment = CartesianLayoutAdjustment {
  plot_top_ratio: -0.000_853,
  plot_bottom_ratio: -0.001_166,
  plot_left_ratio: 0.000_1,
  ..ZERO_ADJUSTMENT
};

/// Word's automatic untitled layout when no legend is present.
///
/// The ratios are measured from the Office fixed-output plot rectangle of
/// LibreOffice's tdf139658 regression document. Word reserves a symmetric
/// horizontal axis band and a smaller vertical band than the generic
/// no-side-legend defaults, independently of the enclosing chart size.
pub(crate) const WORD_UNTITLED_NO_LEGEND: CartesianLayoutAdjustment = CartesianLayoutAdjustment {
  plot_top_ratio: 0.008_651,
  plot_bottom_ratio: -0.008_336,
  plot_left_ratio: 0.017_905,
  plot_right_ratio: -0.017_651,
  ..ZERO_ADJUSTMENT
};

/// Word's automatic two-dimensional cartesian layout with a non-overlay title
/// and bottom legend.
///
/// The shared bands are measured from the immutable Office fixed output for
/// `tdf91250.docx`, `tdf131288.docx`, `tdf125337.docx`, and
/// `Chart_Plot_BorderLine_Style.docx`. Those documents cover automatic and
/// explicit titles, Latin and East Asian title faces, one and three series,
/// and line and column plots. Title-script and physical-font-height residuals
/// are added by the lowering code; the remaining plot, axis, category, and
/// legend bands are invariant across that matrix.
pub(crate) const WORD_TITLED_BOTTOM_LEGEND: CartesianLayoutAdjustment = CartesianLayoutAdjustment {
  category_top_ratio: -0.021_274,
  plot_top_ratio: 0.005_444,
  plot_bottom_ratio: -0.007_901,
  tick_left_ratio: 0.010_897,
  plot_left_ratio: 0.007_022,
  plot_right_ratio: -0.017_651,
  ..ZERO_ADJUSTMENT
};

/// DrawingML's automatic East Asian title box reserves additional leading
/// beyond an otherwise equal-height Latin title. Any physical line-height
/// excess of the selected East Asian face is added separately in points.
pub(crate) const WORD_BOTTOM_LEGEND_EAST_ASIAN_TITLE_EXTRA_RATIO: f32 = 0.004_405;

/// Excel side-legend charts with an explicit title.
pub(crate) const EXCEL_EXPLICIT_TITLE_SIDE_LEGEND: CartesianLayoutAdjustment =
  CartesianLayoutAdjustment {
    category_top_ratio: -0.013_298,
    plot_top_ratio: 0.003_864,
    plot_left_ratio: -0.011_323,
    plot_right_ratio: 0.007_094,
    ..ZERO_ADJUSTMENT
  };

/// Excel date-line charts with a title, an overlaid top-right legend, and
/// independent category/value axis titles.
///
/// The immutable Office PDF for LibreOffice `tdf118150.xlsx` exposes all 28
/// monthly ticks across a worksheet page break. Their first/last centers and
/// constant interval independently determine both plot edges; the vertical
/// value grid determines the remaining band offsets. These are residual
/// deltas on top of Excel's generic automatic plot bands, not a source-file
/// special case.
pub(crate) const EXCEL_EXPLICIT_DATE_LINE_TOP_RIGHT_OVERLAY: CartesianLayoutAdjustment =
  CartesianLayoutAdjustment {
    category_top_ratio: 0.010_99,
    plot_top_ratio: -0.003_18,
    plot_bottom_ratio: -0.006_10,
    plot_left_ratio: -0.014_30,
    plot_right_ratio: 0.075_85,
    ..ZERO_ADJUSTMENT
  };

/// Leftward residual for the vertical value-axis title in the matching
/// date-line profile. The title occupies its own band outside the tick-label
/// ink; applying this separately avoids moving the value ticks themselves.

pub(crate) const EXCEL_TITLE_ONLY: CartesianLayoutAdjustment = CartesianLayoutAdjustment {
  category_top_ratio: 0.020_87,
  plot_top_ratio: 0.018_42,
  plot_left_ratio: 0.021_49,
  plot_right_ratio: -0.027_7,
  ..ZERO_ADJUSTMENT
};

/// Excel indexed scatter with an explicit rich title and no legend.
///
/// The normalized plot geometry comes from the immutable Microsoft Excel 365
/// fixed output for LibreOffice `tdf122915.xlsx`. This profile is distinct
/// from the generic title-only cartesian layout: Office expands the scatter
/// plot toward both horizontal edges and keeps its top on the automatic
/// title-band baseline.
pub(crate) const EXCEL_EXPLICIT_TITLE_INDEXED_SCATTER: CartesianLayoutAdjustment =
  CartesianLayoutAdjustment {
    category_top_ratio: 0.050_221,
    plot_top_ratio: 0.000_067,
    tick_top_ratio: 0.000_56,
    plot_left_ratio: -0.003_063,
    plot_right_ratio: -0.017_872,
    ..ZERO_ADJUSTMENT
  };

/// Pre-2007 empty-overlay automatic-title side-legend profile.
pub(crate) const EXCEL_LEGACY_EMPTY_OVERLAY_SIDE_LEGEND: CartesianLayoutAdjustment =
  CartesianLayoutAdjustment {
    category_top_ratio: 0.019_2,
    plot_top_ratio: 0.015,
    plot_left_ratio: -0.003_74,
    ..ZERO_ADJUSTMENT
  };

pub(crate) const EXCEL_UNTITLED_EXPLICIT_CATEGORY_SIDE_LEGEND: CartesianLayoutAdjustment =
  CartesianLayoutAdjustment {
    category_top_ratio: 0.020_37,
    plot_top_ratio: 0.029_15,
    plot_left_ratio: -0.003_74,
    ..ZERO_ADJUSTMENT
  };

pub(crate) const EXCEL_AUTOMATIC_UNTITLED_SIDE_LEGEND: CartesianLayoutAdjustment =
  CartesianLayoutAdjustment {
    category_top_ratio: 0.028_17,
    plot_top_ratio: 0.029_15,
    plot_left_ratio: -0.003_74,
    ..ZERO_ADJUSTMENT
  };

pub(crate) const EXCEL_INDEPENDENT_AXIS_TEXT: CartesianLayoutAdjustment =
  CartesianLayoutAdjustment {
    category_top_ratio: -0.010_35,
    plot_top_ratio: -0.004_6,
    ..ZERO_ADJUSTMENT
  };

pub(crate) const EXCEL_TITLED_INDEXED_SCATTER: CartesianLayoutAdjustment =
  CartesianLayoutAdjustment {
    category_top_ratio: -0.018,
    plot_top_ratio: 0.006_15,
    plot_bottom_ratio: -0.029_8,
    plot_left_ratio: 0.016_9,
    plot_right_ratio: -0.045_46,
    ..ZERO_ADJUSTMENT
  };

/// Automatic indexed-scatter geometry measured from `ser_labels.xlsx`.
///
/// Office places the plot at `(0.1078, 0.0579)-(0.9573, 0.8721)` in normalized
/// chart-frame coordinates. These deltas preserve the shared host defaults
/// while making that measured profile explicit.
pub(crate) const EXCEL_AUTOMATIC_INDEXED_SCATTER: CartesianLayoutAdjustment =
  CartesianLayoutAdjustment {
    category_top_ratio: 0.007_35,
    plot_top_ratio: 0.033_46,
    plot_bottom_ratio: -0.006_33,
    tick_left_ratio: 0.015_34,
    plot_left_ratio: 0.027_69,
    plot_right_ratio: -0.071_92,
    ..ZERO_ADJUSTMENT
  };

pub(crate) const EXCEL_LEGACY_INDEXED_SCATTER: CartesianLayoutAdjustment =
  CartesianLayoutAdjustment {
    category_top_ratio: 0.013_2,
    plot_top_ratio: 0.025_55,
    tick_left_ratio: 0.014_62,
    plot_right_ratio: -0.100_3,
    ..ZERO_ADJUSTMENT
  };

pub(crate) const EXCEL_MODERN_SINGLE_SERIES_TITLE: CartesianLayoutAdjustment =
  CartesianLayoutAdjustment {
    category_top_ratio: -0.006_82,
    plot_top_ratio: -0.006_82,
    ..ZERO_ADJUSTMENT
  };

/// Modern Excel single-series scatter with a derived title and no legend.
///
/// Scatter reserves a four-sided numeric-axis band distinct from the compact
/// column profile above. The normalized inner rectangle is measured from the
/// immutable Office fixed output for LibreOffice
/// `tdf135184RoundLineCap.xlsx`; its explicit 0..4 and 0..10 axes also keep
/// this profile independent of autoscale differences in error-bar charts.
pub(crate) const EXCEL_MODERN_SINGLE_SERIES_SCATTER_TITLE: CartesianLayoutAdjustment =
  CartesianLayoutAdjustment {
    category_top_ratio: 0.009_776,
    plot_top_ratio: -0.008_636,
    plot_bottom_ratio: -0.025_627,
    plot_left_ratio: 0.007_904,
    plot_right_ratio: -0.012_711,
    ..ZERO_ADJUSTMENT
  };

pub(crate) const EXCEL_LEGACY_SINGLE_SERIES_TITLE: CartesianLayoutAdjustment =
  CartesianLayoutAdjustment {
    category_top_ratio: 0.025_8,
    plot_top_ratio: 0.004_72,
    plot_left_ratio: 0.016_77,
    plot_right_ratio: -0.033_85,
    ..ZERO_ADJUSTMENT
  };

pub(crate) const EXCEL_UNTITLED_BOTTOM_COLUMN: CartesianLayoutAdjustment =
  CartesianLayoutAdjustment {
    category_top_ratio: -0.032_45,
    plot_top_ratio: 0.033_7,
    plot_left_ratio: 0.016_9,
    plot_right_ratio: -0.035,
    ..ZERO_ADJUSTMENT
  };

pub(crate) const EXCEL_DERIVED_SINGLE_SERIES_SIDE_TITLE: CartesianLayoutAdjustment =
  CartesianLayoutAdjustment {
    title_top_ratio: 0.027_64,
    category_top_ratio: 0.029_25,
    plot_top_ratio: 0.003_91,
    tick_left_ratio: 0.012_92,
    plot_right_ratio: -0.025_65,
    ..ZERO_ADJUSTMENT
  };

pub(crate) const EXCEL_EXPLICIT_SINGLE_SERIES_SIDE_TITLE: CartesianLayoutAdjustment =
  CartesianLayoutAdjustment {
    title_top_ratio: 0.024_36,
    category_top_ratio: 0.041_57,
    plot_top_ratio: 0.006_41,
    tick_left_ratio: 0.010_34,
    plot_left_ratio: 0.007_37,
    plot_right_ratio: -0.034_82,
    ..ZERO_ADJUSTMENT
  };

pub(crate) const EXCEL_UNTITLED_BOTTOM_LINE_NO_MARKER: CartesianLayoutAdjustment =
  CartesianLayoutAdjustment {
    category_top_ratio: -0.035_58,
    plot_top_ratio: 0.032_24,
    tick_left_ratio: 0.017_65,
    plot_left_ratio: 0.016_68,
    plot_right_ratio: -0.035,
    ..ZERO_ADJUSTMENT
  };

pub(crate) const EXCEL_EXPLICIT_BOTTOM_COLUMN: CartesianLayoutAdjustment =
  CartesianLayoutAdjustment {
    title_top_ratio: 0.020_82,
    category_top_ratio: -0.035_58,
    plot_top_ratio: 0.006_74,
    tick_left_ratio: 0.017_65,
    plot_left_ratio: 0.016_68,
    plot_right_ratio: -0.035,
    ..ZERO_ADJUSTMENT
  };

pub(crate) const EXCEL_UNSHIFTED_SIDE_LINE: CartesianLayoutAdjustment = CartesianLayoutAdjustment {
  category_top_ratio: 0.013,
  plot_top_ratio: 0.004_79,
  plot_bottom_ratio: 0.008_4,
  ..ZERO_ADJUSTMENT
};

/// Explicit-category automatic chart profile measured from
/// `chart-area-style-border.xlsx`.
pub(crate) const EXCEL_EXPLICIT_CATEGORY_AUTOMATIC: CartesianLayoutAdjustment =
  CartesianLayoutAdjustment {
    plot_top_ratio: -0.002_76,
    plot_bottom_ratio: -0.002_80,
    plot_left_ratio: 0.005_54,
    plot_right_ratio: 0.000_63,
    ..ZERO_ADJUSTMENT
  };

/// Excel's legacy shifted-category column layout with an empty side legend.
///
/// LibreOffice `tdf130657.xlsx` preserves a right-legend container but has no
/// series title, and omits `c:crossBetween`. Excel therefore lays out a wider,
/// lower inner plot than its ordinary explicit-category automatic profile.
/// The ratios are measured from the immutable Office fixed output; the local
/// LibreOffice regression independently requires shifted category positions
/// for the missing-crossBetween form.
pub(crate) const EXCEL_SHIFTED_CATEGORY_EMPTY_SIDE_LEGEND: CartesianLayoutAdjustment =
  CartesianLayoutAdjustment {
    category_top_ratio: 0.015_547,
    plot_top_ratio: 0.012_224,
    plot_bottom_ratio: 0.023_379,
    plot_left_ratio: 0.005_286,
    plot_right_ratio: 0.015,
    ..ZERO_ADJUSTMENT
  };

pub(crate) const EXCEL_SHIFTED_CATEGORY_EMPTY_SIDE_LEGEND_TICK_GAP_RATIO: f32 = 0.034_84;

/// Excel's automatic single-series, vary-colors chart with a data table.
///
/// The data table is a chart table shape whose columns stay aligned with the
/// plotted category slots. The immutable Office fixed output for
/// `DataTable-MultipleLegendEntriesForOneDataSeries.xlsx` exposes the complete
/// two-page geometry: value ticks, first/last gridlines, and all six visible
/// table-column centers on each horizontal page.
pub(crate) const EXCEL_VARY_COLORS_DATA_TABLE: CartesianLayoutAdjustment =
  CartesianLayoutAdjustment {
    category_top_ratio: 0.030_10,
    plot_top_ratio: -0.021_77,
    // `plot_bottom` already follows the adjusted category/table boundary.
    plot_bottom_ratio: 0.040_02,
    tick_left_ratio: 0.058_81,
    // `plot_left` is first derived from the adjusted tick band, so this is
    // the residual needed to leave the plotted/table boundary at +0.03050.
    plot_left_ratio: -0.029_57,
    plot_right_ratio: 0.081_08,
    ..ZERO_ADJUSTMENT
  };

/// Residual horizontal placement of the right-side point legend for
/// `EXCEL_VARY_COLORS_DATA_TABLE`.
///
/// This is separate from the plot/table bands because Office keeps the point
/// legend aligned to the right page's chart-frame band rather than the
/// horizontally paginated plot area.
pub(crate) const EXCEL_VARY_COLORS_DATA_TABLE_LEGEND_X_RATIO: f32 = 0.040_74;
pub(crate) const EXCEL_VARY_COLORS_DATA_TABLE_LEGEND_Y_RATIO: f32 = 0.001_26;
pub(crate) const EXCEL_VARY_COLORS_DATA_TABLE_LEGEND_MARKER_EM: f32 = 0.50;
pub(crate) const EXCEL_VARY_COLORS_DATA_TABLE_LEGEND_ENTRY_GAP_EM: f32 = 0.57;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SideLegendBands {
  pub category_gap_ratio: f32,
  pub legend_outer_margin_ratio: f32,
  pub plot_gap_ratio: f32,
  pub tick_left_ratio: f32,
  pub tick_gap_ratio: f32,
}

pub(crate) const POWERPOINT_SIDE_LEGEND_BANDS: SideLegendBands = SideLegendBands {
  category_gap_ratio: 0.033_35,
  legend_outer_margin_ratio: 0.028_25,
  plot_gap_ratio: 0.048,
  tick_left_ratio: 0.018_25,
  tick_gap_ratio: 0.046_85,
};

pub(crate) const WORD_SIDE_LEGEND_BANDS: SideLegendBands = SideLegendBands {
  category_gap_ratio: 0.027_75,
  legend_outer_margin_ratio: 0.039_77,
  plot_gap_ratio: 0.070_91,
  tick_left_ratio: 0.025_81,
  tick_gap_ratio: 0.036_7,
};

pub(crate) const WORD_EXPLICIT_TITLE_SIDE_LEGEND_BANDS: SideLegendBands = SideLegendBands {
  category_gap_ratio: 0.023_376,
  legend_outer_margin_ratio: 0.039_77,
  plot_gap_ratio: 0.065_677,
  tick_left_ratio: 0.022_392,
  tick_gap_ratio: 0.036_7,
};

pub(crate) const EXCEL_SIDE_LEGEND_BANDS: SideLegendBands = SideLegendBands {
  category_gap_ratio: 0.025,
  legend_outer_margin_ratio: 0.044_5,
  plot_gap_ratio: 0.076_6,
  tick_left_ratio: 0.018_25,
  tick_gap_ratio: 0.046_85,
};

pub(crate) const EXCEL_AUTOMATIC_UNTITLED_SIDE_CATEGORY_GAP_RATIO: f32 = 0.036_52;
pub(crate) const EXCEL_DERIVED_TITLE_SIDE_CATEGORY_GAP_RATIO: f32 = 0.035_92;
pub(crate) const EXCEL_EXPLICIT_TITLE_SIDE_CATEGORY_GAP_RATIO: f32 = 0.032_41;
pub(crate) const EXCEL_AUTOMATIC_UNTITLED_SIDE_LEGEND_OUTER_MARGIN_RATIO: f32 = 0.057_5;
pub(crate) const EXCEL_DERIVED_TITLE_SIDE_LEGEND_OUTER_MARGIN_RATIO: f32 = 0.056_24;
pub(crate) const EXCEL_AUTOMATIC_UNTITLED_COMPACT_SIDE_PLOT_GAP_RATIO: f32 = 0.106_4;
pub(crate) const EXCEL_AUTOMATIC_UNTITLED_WIDE_SIDE_PLOT_GAP_RATIO: f32 = 0.176_5;
pub(crate) const EXCEL_UNSHIFTED_LINE_SIDE_PLOT_GAP_RATIO: f32 = 0.146_52;
pub(crate) const EXCEL_AUTOMATIC_UNTITLED_SIDE_TICK_LEFT_RATIO: f32 = 0.031_43;
pub(crate) const EXCEL_UNSHIFTED_LINE_SIDE_TICK_GAP_RATIO: f32 = 0.036_515;
pub(crate) const CARTESIAN_SIDE_PLOT_OUTER_MARGIN_RATIO: f32 = 0.031_8;

pub(crate) const EXCEL_LEGACY_EMPTY_OVERLAY_TITLE_TOP_RATIO: f32 = 0.053_15;
pub(crate) const EXCEL_TITLED_INDEXED_SCATTER_TITLE_TOP_RATIO: f32 = 0.052_7;
pub(crate) const EXCEL_LEGACY_SINGLE_SERIES_TITLE_TOP_RATIO: f32 = 0.048_36;
pub(crate) const DATA_TABLE_BOTTOM_LEGEND_GAP_RATIO: f32 = 0.045;
pub(crate) const POWERPOINT_TITLED_BOTTOM_LEGEND_CATEGORY_GAP_RATIO: f32 = 0.026_1;
pub(crate) const DEFAULT_BOTTOM_LEGEND_CATEGORY_GAP_RATIO: f32 = 0.021;
pub(crate) const WORD_TOP_LEGEND_GAP_RATIO: f32 = 0.031;
pub(crate) const DEFAULT_TOP_LEGEND_GAP_RATIO: f32 = 0.018;

pub(crate) const WORD_MULTILINE_CATEGORY_PLOT_GAP_RATIO: f32 = 0.039;
pub(crate) const EXCEL_TITLE_ONLY_CATEGORY_PLOT_GAP_RATIO: f32 = 0.035_09;
pub(crate) const EXCEL_LEGACY_TITLE_CATEGORY_PLOT_GAP_RATIO: f32 = 0.032_44;
pub(crate) const EXCEL_BOTTOM_CATEGORY_PLOT_GAP_RATIO: f32 = 0.032_2;
pub(crate) const DEFAULT_CATEGORY_PLOT_GAP_RATIO: f32 = 0.018;
pub(crate) const POWERPOINT_TITLED_BOTTOM_PLOT_GAP_RATIO: f32 = 0.022_5;

pub(crate) const EXCEL_TITLED_INDEXED_SCATTER_TICK_LEFT_RATIO: f32 = 0.029_56;
pub(crate) const EXCEL_LEGACY_SINGLE_SERIES_TICK_LEFT_RATIO: f32 = 0.029_58;
pub(crate) const EXCEL_UNTITLED_BOTTOM_COLUMN_TICK_LEFT_RATIO: f32 = 0.032_35;
pub(crate) const WORD_HIDDEN_VALUE_TICK_LEFT_RATIO: f32 = 0.045_5;
pub(crate) const EXCEL_TITLE_ONLY_TICK_LEFT_RATIO: f32 = 0.029_74;
pub(crate) const POWERPOINT_TITLED_BOTTOM_TICK_LEFT_RATIO: f32 = 0.019;
pub(crate) const DEFAULT_TICK_LEFT_RATIO: f32 = 0.015;

pub(crate) const POWERPOINT_TITLED_BOTTOM_TICK_GAP_RATIO: f32 = 0.032_3;
pub(crate) const DEFAULT_TICK_GAP_RATIO: f32 = 0.026;
/// Value-label gap outside an authored modern Excel scatter inner plot.
///
/// `layoutTarget="inner"` excludes the axes and their labels. Office fixed
/// output for LibreOffice `fdo70609.xlsx` keeps an 8.32pt label-to-axis band
/// on a 195.84pt chart frame, independently of the authored inner rectangle.
pub(crate) const EXCEL_MANUAL_INNER_SCATTER_TICK_GAP_RATIO: f32 = 0.042_5;
pub(crate) const POWERPOINT_TITLED_BOTTOM_RIGHT_MARGIN_RATIO: f32 = 0.030_1;
pub(crate) const WORD_HIDDEN_VALUE_RIGHT_MARGIN_RATIO: f32 = 0.041;
pub(crate) const DEFAULT_RIGHT_MARGIN_RATIO: f32 = 0.026;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct HorizontalCartesianLegendProfile {
  pub entry_gap_em: f32,
  pub marker_gap_em: f32,
  pub line_key_width_em: f32,
  pub x_offset_height_ratio: f32,
  pub y_offset_height_ratio: f32,
}

pub(crate) const DEFAULT_HORIZONTAL_CARTESIAN_LEGEND: HorizontalCartesianLegendProfile =
  HorizontalCartesianLegendProfile {
    entry_gap_em: 0.94,
    marker_gap_em: CARTESIAN_LEGEND_MARKER_GAP_EM,
    line_key_width_em: CARTESIAN_LINE_LEGEND_KEY_WIDTH_EM,
    x_offset_height_ratio: 0.0,
    y_offset_height_ratio: 0.0,
  };
pub(crate) const WORD_TITLED_BOTTOM_CARTESIAN_LEGEND: HorizontalCartesianLegendProfile =
  HorizontalCartesianLegendProfile {
    entry_gap_em: 1.318_7,
    marker_gap_em: 0.23,
    line_key_width_em: 2.133_3,
    x_offset_height_ratio: 0.0,
    y_offset_height_ratio: -0.006_944,
  };
pub(crate) const WORD_AUTOMATIC_TITLE_BOTTOM_COLUMN_LEGEND: HorizontalCartesianLegendProfile =
  HorizontalCartesianLegendProfile {
    entry_gap_em: 1.095_4,
    marker_gap_em: 0.237_4,
    line_key_width_em: CARTESIAN_LINE_LEGEND_KEY_WIDTH_EM,
    x_offset_height_ratio: -0.006_016,
    y_offset_height_ratio: -0.006_944,
  };
pub(crate) const EXCEL_TITLED_INDEXED_SCATTER_LEGEND: HorizontalCartesianLegendProfile =
  HorizontalCartesianLegendProfile {
    // Office gives scatter legends a full line key, then uses the ordinary
    // one-em inter-entry band.  The former 2.43em value folded the missing
    // line-key width into the gap; keeping both made the second entry 13.4pt
    // too far right in the immutable `dispBlanksAs_2013.xlsx` fixed output.
    entry_gap_em: 0.94,
    marker_gap_em: CARTESIAN_LEGEND_MARKER_GAP_EM,
    line_key_width_em: CARTESIAN_LINE_LEGEND_KEY_WIDTH_EM,
    x_offset_height_ratio: -0.008_5,
    y_offset_height_ratio: -0.009_49,
  };
pub(crate) const EXCEL_EXPLICIT_BOTTOM_COLUMN_LEGEND: HorizontalCartesianLegendProfile =
  HorizontalCartesianLegendProfile {
    entry_gap_em: 0.681_4,
    marker_gap_em: CARTESIAN_LEGEND_MARKER_GAP_EM,
    line_key_width_em: CARTESIAN_LINE_LEGEND_KEY_WIDTH_EM,
    x_offset_height_ratio: -0.010_35,
    y_offset_height_ratio: -0.009_93,
  };
pub(crate) const EXCEL_UNTITLED_BOTTOM_COLUMN_LEGEND: HorizontalCartesianLegendProfile =
  HorizontalCartesianLegendProfile {
    entry_gap_em: 0.681_4,
    marker_gap_em: CARTESIAN_LEGEND_MARKER_GAP_EM,
    line_key_width_em: CARTESIAN_LINE_LEGEND_KEY_WIDTH_EM,
    x_offset_height_ratio: -0.006_09,
    y_offset_height_ratio: -0.005_91,
  };
pub(crate) const EXCEL_UNTITLED_BOTTOM_LINE_LEGEND: HorizontalCartesianLegendProfile =
  HorizontalCartesianLegendProfile {
    entry_gap_em: 0.94,
    marker_gap_em: CARTESIAN_LEGEND_MARKER_GAP_EM,
    line_key_width_em: CARTESIAN_LINE_LEGEND_KEY_WIDTH_EM,
    x_offset_height_ratio: -0.009_21,
    y_offset_height_ratio: -0.009_93,
  };

pub(crate) const CARTESIAN_LEGEND_MARKER_GAP_EM: f32 = 0.247;
pub(crate) const CARTESIAN_LINE_LEGEND_KEY_WIDTH_EM: f32 = 2.068;
pub(crate) const OFFICE_VERTICAL_LEGEND_ENTRY_GAP_EM: f32 = 0.61;
pub(crate) const POWERPOINT_VERTICAL_LEGEND_ENTRY_GAP_EM: f32 = 0.344;
pub(crate) const EXCEL_EXPLICIT_SINGLE_SERIES_LEGEND_Y_RATIO: f32 = 0.074_94;
pub(crate) const EXCEL_EXPLICIT_TITLE_LEGEND_Y_RATIO: f32 = 0.021_5;
pub(crate) const EXCEL_DERIVED_TITLE_LEGEND_Y_RATIO: f32 = 0.087_5;
pub(crate) const EXCEL_INDEXED_SCATTER_MULTICOMPONENT_LEGEND_Y_RATIO: f32 = -0.002;
pub(crate) const EXCEL_EXPLICIT_CATEGORY_LEGEND_Y_RATIO: f32 = 0.002_44;
pub(crate) const EXCEL_AUTOMATIC_UNTITLED_LEGEND_Y_RATIO: f32 = 0.006_8;
pub(crate) const EXCEL_GENERIC_LEGEND_Y_RATIO: f32 = -0.014_8;
pub(crate) const WORD_EXPLICIT_TITLE_LEGEND_Y_RATIO: f32 = 0.060_738_5;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct RadialPlotProfile {
  pub center_x_width_ratio: f32,
  pub center_y_height_ratio: f32,
  pub radius_x_height_ratio: f32,
  pub radius_y_height_ratio: f32,
}

/// Word automatic pie with a horizontal bottom legend.
pub(crate) const WORD_BOTTOM_LEGEND_PIE: RadialPlotProfile = RadialPlotProfile {
  center_x_width_ratio: 0.5,
  center_y_height_ratio: 0.454_6,
  radius_x_height_ratio: 0.410_5,
  radius_y_height_ratio: 0.410_5,
};

/// Word automatic pie without a legend.
pub(crate) const WORD_NO_LEGEND_PIE: RadialPlotProfile = RadialPlotProfile {
  center_x_width_ratio: 0.5,
  center_y_height_ratio: 0.5,
  radius_x_height_ratio: 0.394_355_15,
  radius_y_height_ratio: 0.394_355_15,
};

/// Word automatic pie with a vertical side legend.
pub(crate) const WORD_SIDE_LEGEND_PIE: RadialPlotProfile = RadialPlotProfile {
  center_x_width_ratio: 0.444_498_36,
  center_y_height_ratio: 0.5,
  radius_x_height_ratio: 0.394_355_15,
  radius_y_height_ratio: 0.394_355_15,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct RadialLabelProfile {
  pub center_y_height_ratio: f32,
  pub plot_radius_x_height_ratio: f32,
  pub radius_x_scale: f32,
  pub radius_y_height_ratio: f32,
}

pub(crate) const WORD_NO_LEGEND_PIE_LABELS: RadialLabelProfile = RadialLabelProfile {
  center_y_height_ratio: 0.504_26,
  plot_radius_x_height_ratio: 0.394_355_15,
  radius_x_scale: 0.84,
  radius_y_height_ratio: 0.348_384,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct BottomLegendProfile {
  pub marker_size_em: f32,
  pub marker_text_gap_em: f32,
  pub item_gap_em: f32,
  pub centered_row_offset_width_ratio: f32,
  pub marker_y_height_ratio: f32,
  pub text_y_height_ratio: f32,
}

pub(crate) const WORD_BOTTOM_PIE_LEGEND: BottomLegendProfile = BottomLegendProfile {
  marker_size_em: 0.55,
  marker_text_gap_em: 0.275,
  item_gap_em: 0.515,
  centered_row_offset_width_ratio: 0.004_6,
  marker_y_height_ratio: 0.932_9,
  text_y_height_ratio: 0.924,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SideLegendProfile {
  pub marker_size_em: f32,
  pub marker_x_width_ratio: f32,
  pub text_x_width_ratio: f32,
  pub first_marker_y_height_ratio: f32,
  pub first_text_y_height_ratio: f32,
  pub row_step_height_ratio: f32,
}

pub(crate) const WORD_SIDE_PIE_LEGEND: SideLegendProfile = SideLegendProfile {
  marker_size_em: 0.502_87,
  marker_x_width_ratio: 0.899_492_86,
  text_x_width_ratio: 0.915_457_55,
  first_marker_y_height_ratio: 0.389_812,
  first_text_y_height_ratio: 0.376_303_4,
  row_step_height_ratio: 0.066_914_94,
};

pub(crate) const WORD_SIDE_PIE_FRAME_Y_OFFSET_RATIO: f32 = 0.013_606_33;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct RadialHostDefaults {
  pub title_height_scale: f32,
  pub side_legend_width_em: f32,
  pub radius_height_basis_scale: f32,
  pub radius_scale: f32,
  pub titled_bottom_legend_radius_scale: f32,
  pub compact_radius_scale: f32,
  pub explosion_scale: f32,
  pub legend_marker_em: f32,
  pub legend_marker_gap_em: f32,
  pub horizontal_legend_entry_gap_em: f32,
  pub horizontal_legend_center_offset_em: f32,
  pub side_legend_entry_step: f32,
  pub side_legend_center_offset_em: f32,
}

pub(crate) const POWERPOINT_RADIAL_DEFAULTS: RadialHostDefaults = RadialHostDefaults {
  title_height_scale: 1.364_2,
  side_legend_width_em: 1.765_4,
  radius_height_basis_scale: 1.35,
  radius_scale: 0.40,
  titled_bottom_legend_radius_scale: 0.40,
  compact_radius_scale: 0.346,
  explosion_scale: 0.24,
  legend_marker_em: 0.55,
  legend_marker_gap_em: 0.3,
  horizontal_legend_entry_gap_em: 1.0,
  horizontal_legend_center_offset_em: 0.0,
  side_legend_entry_step: 1.285_65,
  side_legend_center_offset_em: 0.923_3,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Pie3DProjectionProfile {
  /// Horizontal radius relative to the host's radial plot basis.
  pub radius_x_scale: f32,
  /// Residual multiplier after projecting a circular top by `sin(rotX)`.
  pub vertical_tilt_scale: f32,
  /// Default visible extrusion relative to the automatic plot height at the
  /// canonical 30-degree X rotation.
  pub depth_height_scale: f32,
  /// Vertical residual after centering the complete top-plus-side scene.
  pub center_y_offset_height_ratio: f32,
  /// Series/point explosion displacement relative to the fitted radius.
  /// The scene radius is reduced by the same outward extent so the complete
  /// exploded pie remains inside the automatic plot rectangle.
  pub explosion_scale: f32,
  /// Residual safety inset applied after the explosion envelope is fitted.
  pub exploded_geometry_scale: f32,
  /// Vertical scene-centre correction used only by an exploded pie.
  pub exploded_center_y_offset_height_ratio: f32,
}

/// PowerPoint's automatic 3-D pie scene.
///
/// The strict Open XML `3D Pie-O12-PPT-Charts.pptx` reference has an authored
/// `rotX=30`, `perspective=30` view and no `hPercent`. Its immutable Office
/// fixed output rasterizes only the 3-D plot at 200 ppi while retaining the
/// title and legend as vectors. The plot measures a 180.1pt horizontal radius,
/// 85.3pt projected vertical radius, and 41.6pt visible extrusion inside the
/// same automatic chart bands used by the vector host. These normalized
/// values define the scene profile; they are not PDF-image dimensions.
pub(crate) const POWERPOINT_PIE_3D_PROJECTION: Pie3DProjectionProfile = Pie3DProjectionProfile {
  radius_x_scale: 0.468_8,
  vertical_tilt_scale: 0.947,
  depth_height_scale: 0.146_2,
  center_y_offset_height_ratio: -0.011_86,
  explosion_scale: 0.95,
  exploded_geometry_scale: 0.98,
  exploded_center_y_offset_height_ratio: -0.004_75,
};

pub(crate) const WORD_RADIAL_DEFAULTS: RadialHostDefaults = RadialHostDefaults {
  title_height_scale: 1.5,
  side_legend_width_em: 2.2,
  radius_height_basis_scale: 1.35,
  radius_scale: 0.42,
  titled_bottom_legend_radius_scale: 0.42,
  compact_radius_scale: 0.42,
  explosion_scale: 0.24,
  legend_marker_em: 0.55,
  legend_marker_gap_em: 0.3,
  horizontal_legend_entry_gap_em: 1.0,
  horizontal_legend_center_offset_em: 0.0,
  side_legend_entry_step: 1.0,
  side_legend_center_offset_em: 0.09,
};

pub(crate) const EXCEL_RADIAL_DEFAULTS: RadialHostDefaults = RadialHostDefaults {
  title_height_scale: 1.5,
  side_legend_width_em: 2.12,
  radius_height_basis_scale: 1.0,
  radius_scale: 0.445,
  titled_bottom_legend_radius_scale: 0.420_5,
  compact_radius_scale: 0.347,
  explosion_scale: 0.9,
  legend_marker_em: 0.477_7,
  legend_marker_gap_em: 0.212_3,
  horizontal_legend_entry_gap_em: 0.635,
  horizontal_legend_center_offset_em: 0.052_9,
  side_legend_entry_step: 1.350_8,
  side_legend_center_offset_em: 0.09,
};

pub(crate) const RADIAL_TITLE_TOP_RATIO: f32 = 0.025;
pub(crate) const EXCEL_BOTTOM_LEGEND_TITLE_OFFSET_EM: f32 = 0.317;
pub(crate) const EXCEL_TITLED_BOTTOM_LEGEND_HEIGHT_SCALE: f32 = 0.938_5;
pub(crate) const EXCEL_UNTITLED_BOTTOM_LEGEND_HEIGHT_SCALE: f32 = 1.281_5;

/// Fixed inset used before PowerPoint lays out an automatic pie diagram.
///
/// `chart2/source/view/main/ChartView.cxx` uses 350 hundredths of a
/// millimetre on every side for pie charts instead of its ordinary 2% page
/// inset, establishing that this is a fixed-distance policy. The immutable
/// PowerPoint output for the 3x3 frame-size matrix in
/// `PieChartWithAutomaticLayout_SizeAndPosition.pptx` resolves that host
/// distance to exactly 11pt; the remaining diagram is then reduced to a
/// centred 1:1 rectangle.
pub(crate) const POWERPOINT_AUTOMATIC_PIE_FIXED_INSET_PT: f32 = 11.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct BestFitSectorProfile {
  pub radius_factor: f32,
  pub angle_adjustment_degrees: f32,
}

pub(crate) const EXCEL_REFLEX_BEST_FIT_LABEL: BestFitSectorProfile = BestFitSectorProfile {
  radius_factor: 0.589,
  angle_adjustment_degrees: 25.0,
};
pub(crate) const EXCEL_ORDINARY_BEST_FIT_LABEL: BestFitSectorProfile = BestFitSectorProfile {
  radius_factor: 0.614,
  angle_adjustment_degrees: 9.1,
};
pub(crate) const EXCEL_NARROW_BEST_FIT_LABEL: BestFitSectorProfile = BestFitSectorProfile {
  radius_factor: 0.699,
  angle_adjustment_degrees: -2.1,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct OfPiePlotProfile {
  pub primary_x_ratio: f32,
  pub secondary_x_ratio: f32,
  pub center_y_ratio: f32,
  pub primary_radius_scale: f32,
}

pub(crate) const EXCEL_PIE_OF_PIE_PLOT: OfPiePlotProfile = OfPiePlotProfile {
  primary_x_ratio: 0.268,
  secondary_x_ratio: 0.766_3,
  center_y_ratio: 0.510_5,
  primary_radius_scale: 0.314,
};
pub(crate) const EXCEL_BAR_OF_PIE_PLOT: OfPiePlotProfile = OfPiePlotProfile {
  primary_x_ratio: 0.349,
  secondary_x_ratio: 0.761,
  center_y_ratio: 0.5,
  primary_radius_scale: 0.393,
};
pub(crate) const DEFAULT_OF_PIE_PLOT: OfPiePlotProfile = OfPiePlotProfile {
  primary_x_ratio: 0.32,
  secondary_x_ratio: 0.77,
  center_y_ratio: 0.5,
  primary_radius_scale: 0.29,
};

const ZERO_ADJUSTMENT: CartesianLayoutAdjustment = CartesianLayoutAdjustment {
  title_top_ratio: 0.0,
  category_top_ratio: 0.0,
  plot_top_ratio: 0.0,
  plot_bottom_ratio: 0.0,
  tick_left_ratio: 0.0,
  tick_top_ratio: 0.0,
  plot_left_ratio: 0.0,
  plot_right_ratio: 0.0,
};

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn host_defaults_are_normalized_frame_ratios() {
    for profile in [
      POWERPOINT_CARTESIAN_DEFAULTS,
      WORD_CARTESIAN_DEFAULTS,
      EXCEL_CARTESIAN_DEFAULTS,
    ] {
      assert!((0.0..1.0).contains(&profile.title_top_ratio));
      assert!((0.0..1.0).contains(&profile.category_bottom_ratio));
      assert!((0.0..1.0).contains(&profile.untitled_side_plot_top_ratio));
      assert!((0.0..1.0).contains(&profile.untitled_no_side_plot_top_ratio));
      assert!((0.0..1.0).contains(&profile.titled_top_legend_gap_ratio));
    }
  }

  #[test]
  fn indexed_scatter_profile_reconstructs_promoted_plot_adjustments() {
    let profile = EXCEL_AUTOMATIC_INDEXED_SCATTER;
    assert_eq!(profile.category_top_ratio, 0.007_35);
    assert_eq!(profile.plot_top_ratio, 0.033_46);
    assert_eq!(profile.plot_bottom_ratio, -0.006_33);
    assert_eq!(profile.plot_left_ratio, 0.027_69);
    assert_eq!(profile.plot_right_ratio, -0.071_92);
  }
}
