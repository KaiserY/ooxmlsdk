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

pub(crate) const TOP_LEGEND_LEFT_INSET_RATIO: f32 = 0.004;
pub(crate) const UNTITLED_TOP_LEGEND_TOP_RATIO: f32 = 0.018;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct CartesianLayoutAdjustment {
  pub title_top_ratio: f32,
  pub category_top_ratio: f32,
  pub plot_top_ratio: f32,
  pub plot_bottom_ratio: f32,
  pub tick_left_ratio: f32,
  pub plot_left_ratio: f32,
  pub plot_right_ratio: f32,
}

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

/// Excel side-legend charts with an explicit title.
pub(crate) const EXCEL_EXPLICIT_TITLE_SIDE_LEGEND: CartesianLayoutAdjustment =
  CartesianLayoutAdjustment {
    category_top_ratio: -0.013_298,
    plot_top_ratio: 0.003_864,
    plot_left_ratio: -0.011_323,
    plot_right_ratio: 0.007_094,
    ..ZERO_ADJUSTMENT
  };

pub(crate) const EXCEL_TITLE_ONLY: CartesianLayoutAdjustment = CartesianLayoutAdjustment {
  category_top_ratio: 0.020_87,
  plot_top_ratio: 0.018_42,
  plot_left_ratio: 0.021_49,
  plot_right_ratio: -0.027_7,
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
pub(crate) const POWERPOINT_TITLED_BOTTOM_RIGHT_MARGIN_RATIO: f32 = 0.030_1;
pub(crate) const WORD_HIDDEN_VALUE_RIGHT_MARGIN_RATIO: f32 = 0.041;
pub(crate) const DEFAULT_RIGHT_MARGIN_RATIO: f32 = 0.026;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct HorizontalCartesianLegendProfile {
  pub entry_gap_em: f32,
  pub x_offset_height_ratio: f32,
  pub y_offset_height_ratio: f32,
}

pub(crate) const DEFAULT_HORIZONTAL_CARTESIAN_LEGEND: HorizontalCartesianLegendProfile =
  HorizontalCartesianLegendProfile {
    entry_gap_em: 0.94,
    x_offset_height_ratio: 0.0,
    y_offset_height_ratio: 0.0,
  };
pub(crate) const EXCEL_TITLED_INDEXED_SCATTER_LEGEND: HorizontalCartesianLegendProfile =
  HorizontalCartesianLegendProfile {
    entry_gap_em: 2.43,
    x_offset_height_ratio: -0.008_5,
    y_offset_height_ratio: 0.004_16,
  };
pub(crate) const EXCEL_EXPLICIT_BOTTOM_COLUMN_LEGEND: HorizontalCartesianLegendProfile =
  HorizontalCartesianLegendProfile {
    entry_gap_em: 0.681_4,
    x_offset_height_ratio: -0.010_35,
    y_offset_height_ratio: -0.009_93,
  };
pub(crate) const EXCEL_UNTITLED_BOTTOM_COLUMN_LEGEND: HorizontalCartesianLegendProfile =
  HorizontalCartesianLegendProfile {
    entry_gap_em: 0.681_4,
    x_offset_height_ratio: -0.006_09,
    y_offset_height_ratio: -0.005_91,
  };
pub(crate) const EXCEL_UNTITLED_BOTTOM_LINE_LEGEND: HorizontalCartesianLegendProfile =
  HorizontalCartesianLegendProfile {
    entry_gap_em: 0.94,
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
  compact_radius_scale: 0.346,
  explosion_scale: 0.24,
  legend_marker_em: 0.55,
  legend_marker_gap_em: 0.3,
  horizontal_legend_entry_gap_em: 1.0,
  horizontal_legend_center_offset_em: 0.0,
  side_legend_entry_step: 1.285_65,
  side_legend_center_offset_em: 0.923_3,
};

pub(crate) const WORD_RADIAL_DEFAULTS: RadialHostDefaults = RadialHostDefaults {
  title_height_scale: 1.5,
  side_legend_width_em: 2.2,
  radius_height_basis_scale: 1.35,
  radius_scale: 0.42,
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
