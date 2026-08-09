use emfsdk::{EmfMetafile, EmfRecordData, EmrExtTextOut};
use olecfsdk::{
  ograph::{
    OgraphAxis, OgraphCellValue, OgraphChart, OgraphChartGroup, OgraphChartGroupKind,
    OgraphDataFormat, OgraphSeries,
  },
  xls::{
    Chart3DFlags, ChartAreaFlags, ChartAreaFormatRecord, ChartAreaRecordFlags,
    ChartAttachedLabelFlags, ChartBarFlags, ChartFormatFlags, ChartLabelRangeFlags,
    ChartLegendFlags, ChartLineFlags, ChartLineFormatRecord, ChartLineGroupFlags, ChartMarkerFlags,
    ChartScatterFlags, ChartSeriesFormatFlags, ChartSurfFlags, ChartTickFlags,
    ChartValueRangeFlags,
  },
};
use ooxmlsdk::schemas::{
  schemas_openxmlformats_org_drawingml_2006_chart as c,
  schemas_openxmlformats_org_drawingml_2006_main as a,
};

struct CachedPreviewTextRecord {
  record_index: usize,
  text: String,
  value: EmrExtTextOut,
  run: emfsdk::render::MetafileTextRun,
}

/// Refreshes an authored Graph EMF preview with label placement resolved from
/// the editable `/Workbook` chart.
///
/// MS-OGRAPH `Tick.fAutoRot` leaves category-label placement to the running
/// Graph server. An embedded EMF is only a cached presentation and can retain
/// an older two-line break after the native view fits the same words on one
/// line for the print device. Match the complete Graph categories against the
/// cached text records, apply the same neighbor-collision decision to their
/// authored advances, and move only records whose complete label fits.
/// Unrecognized previews return `None` so the caller can use the complete
/// native chart renderer instead of partially rewriting a metafile.
pub(super) fn refresh_cached_preview(
  source: &OgraphChart,
  data: &[u8],
  content_type: Option<&str>,
) -> Option<Vec<u8>> {
  let category_tick = source
    .axes
    .iter()
    .find(|axis| axis.source.axis_type == 0)?
    .tick?;
  if !category_tick.flags.contains(ChartTickFlags::AUTO_ROTATION)
    || category_tick.flags.bits() & ChartTickFlags::ROTATION.bits() != 0
  {
    return None;
  }

  let category_sets = source
    .series
    .iter()
    .filter(|series| series.included && !series.categories.is_empty())
    .map(|series| {
      series
        .categories
        .iter()
        .map(ograph_cell_text)
        .collect::<Option<Vec<_>>>()
    })
    .collect::<Option<Vec<_>>>()?;
  let categories = category_sets.first()?.clone();
  if categories.len() < 2 || category_sets.iter().any(|values| values != &categories) {
    return None;
  }

  let runs = emfsdk::render::extract_metafile_text_runs(data, content_type);
  let mut metafile = EmfMetafile::from_bytes(data).ok()?;
  let mut parsed = Vec::new();
  for (record_index, record) in metafile.records.iter().enumerate() {
    let EmfRecordData::ExtTextOutW(value) = EmfRecordData::from_record(record).ok()? else {
      continue;
    };
    let text = value.text.text.as_str().ok()?.into_owned();
    if !text.trim().is_empty() {
      parsed.push((record_index, text, value));
    }
  }
  if parsed.len() != runs.len() {
    return None;
  }
  let mut records = parsed
    .into_iter()
    .zip(runs)
    .map(|((record_index, text, value), run)| {
      (run.text == text).then_some(CachedPreviewTextRecord {
        record_index,
        text,
        value,
        run,
      })
    })
    .collect::<Option<Vec<_>>>()?;

  let mut groups = Vec::with_capacity(categories.len());
  let mut cursor = 0usize;
  for category in &categories {
    let tokens = category.split_whitespace().collect::<Vec<_>>();
    let start = (cursor..records.len()).find(|start| {
      records[*start].text == *category
        || (!tokens.is_empty()
          && records
            .get(*start..start.saturating_add(tokens.len()))
            .is_some_and(|candidate| {
              candidate
                .iter()
                .zip(&tokens)
                .all(|(record, token)| record.text == *token)
            }))
    })?;
    let indices = if records[start].text == *category {
      vec![start]
    } else {
      (start..start + tokens.len()).collect()
    };
    cursor = indices.last().copied()?.saturating_add(1);
    groups.push(indices);
  }

  let centers = groups
    .iter()
    .map(|indices| {
      let run = &records[*indices.first()?].run;
      Some(run.x + run.width? * 0.5)
    })
    .collect::<Option<Vec<_>>>()?;
  let raw_centers = groups
    .iter()
    .map(|indices| Some(records[*indices.first()?].value.text.reference.x))
    .collect::<Option<Vec<_>>>()?;

  for (group_index, indices) in groups.iter().enumerate() {
    if indices.len() <= 1 {
      continue;
    }
    let baseline_y = records[indices[0]].value.text.reference.y;
    if indices
      .iter()
      .all(|index| records[*index].value.text.reference.y == baseline_y)
    {
      continue;
    }
    let anchor_x = raw_centers[group_index];
    if indices.iter().any(|index| {
      records[*index].value.text.reference.x != anchor_x
        || (records[*index].run.x + records[*index].run.width.unwrap_or_default() * 0.5
          - centers[group_index])
          .abs()
          > 0.001
    }) {
      continue;
    }

    let neighbor_index = if group_index + 1 < groups.len() {
      group_index + 1
    } else {
      group_index.checked_sub(1)?
    };
    let pitch = (centers[neighbor_index] - centers[group_index]).abs();
    let raw_pitch = raw_centers[neighbor_index] - raw_centers[group_index];
    if pitch <= f32::EPSILON || raw_pitch == 0 {
      continue;
    }
    let widths = indices
      .iter()
      .map(|index| records[*index].run.width)
      .collect::<Option<Vec<_>>>()?;
    let ink_width = widths.iter().sum::<f32>();
    // Touching the neighboring category cell is still an overlap after GDI
    // rounds the generated text shape to device coordinates. Only accept a
    // strictly smaller ink extent; this distinguishes narrow ordinal labels
    // from equal-width cells without a fixture-specific threshold.
    if ink_width >= pitch {
      continue;
    }

    let source_width = source.width_points() as f32;
    let source_height = source.height_points() as f32;
    if source_width <= f32::EPSILON || source_height <= f32::EPSILON {
      return None;
    }
    let font_height = records[indices[0]].run.font_size?;
    let word_gap = font_height * source_height / source_width * 0.25;
    let complete_width = ink_width + word_gap * (indices.len() - 1) as f32;
    let scale = (centers[neighbor_index] - centers[group_index]) / raw_pitch as f32;
    if scale.abs() <= f32::EPSILON {
      continue;
    }
    let mut left = centers[group_index] - complete_width * 0.5;
    for (position, index) in indices.iter().enumerate() {
      let desired_center = left + widths[position] * 0.5;
      let current_center = records[*index].run.x + widths[position] * 0.5;
      let delta_x = ((desired_center - current_center) / scale).round() as i32;
      let delta_y = baseline_y - records[*index].value.text.reference.y;
      shift_cached_text_record(&mut records[*index].value, delta_x, delta_y)?;
      left += widths[position];
      if position + 1 < indices.len() {
        left += word_gap;
      }
    }
  }

  for record in records {
    metafile.records[record.record_index] =
      EmfRecordData::ExtTextOutW(record.value).to_record().ok()?;
  }
  metafile.to_bytes().ok()
}

fn ograph_cell_text(value: &Option<OgraphCellValue>) -> Option<String> {
  match value.as_ref()? {
    OgraphCellValue::Text(value) => Some(value.clone()),
    OgraphCellValue::Number(value) => Some(value.to_string()),
    OgraphCellValue::Blank => None,
  }
}

fn shift_cached_text_record(value: &mut EmrExtTextOut, delta_x: i32, delta_y: i32) -> Option<()> {
  value.text.reference.x = value.text.reference.x.checked_add(delta_x)?;
  value.text.reference.y = value.text.reference.y.checked_add(delta_y)?;
  shift_cached_text_rect(&mut value.bounds, delta_x, delta_y)?;
  if let Some(rectangle) = &mut value.text.rectangle {
    shift_cached_text_rect(rectangle, delta_x, delta_y)?;
  }
  Some(())
}

fn shift_cached_text_rect(rectangle: &mut emfsdk::RectL, delta_x: i32, delta_y: i32) -> Option<()> {
  rectangle.left = rectangle.left.checked_add(delta_x)?;
  rectangle.right = rectangle.right.checked_add(delta_x)?;
  rectangle.top = rectangle.top.checked_add(delta_y)?;
  rectangle.bottom = rectangle.bottom.checked_add(delta_y)?;
  Some(())
}

pub(super) fn chart_space(source: &OgraphChart) -> c::ChartSpace {
  let plot_area_choice1 = source
    .groups
    .iter()
    .map(|group| chart_group(source, group))
    .collect();
  let wall = source
    .axes
    .iter()
    .find(|axis| axis.source.axis_type == 0)
    .and_then(|axis| chart_plane_shape_properties(source, axis, 0x00ff_ffff));
  let floor = source
    .axes
    .iter()
    .find(|axis| axis.source.axis_type == 1)
    .and_then(|axis| chart_plane_shape_properties(source, axis, indexed_color(source, 23, 0)));
  c::ChartSpace {
    date1904: Some(c::Date1904 {
      val: Some(source.date_1904.into()),
    }),
    chart: Box::new(c::Chart {
      auto_title_deleted: Some(c::AutoTitleDeleted {
        val: Some(true.into()),
      }),
      view3_d: source
        .groups
        .iter()
        .find_map(|group| group.view_3d)
        .map(|view| Box::new(view_3d(view))),
      floor: floor.clone().map(|shape_properties| {
        Box::new(c::Floor {
          shape_properties: Some(Box::new(shape_properties)),
          ..Default::default()
        })
      }),
      side_wall: wall.clone().map(|shape_properties| {
        Box::new(c::SideWall {
          shape_properties: Some(Box::new(shape_properties)),
          ..Default::default()
        })
      }),
      back_wall: wall.map(|shape_properties| {
        Box::new(c::BackWall {
          shape_properties: Some(Box::new(shape_properties)),
          ..Default::default()
        })
      }),
      plot_area: Box::new(c::PlotArea {
        plot_area_choice1,
        plot_area_choice2: source
          .axes
          .iter()
          .map(|axis| plot_axis(source, axis))
          .collect(),
        shape_properties: Some(Box::new(shape_properties(
          Some(c::ShapePropertiesChoice2::NoFill(a::NoFill::default())),
          Some(no_fill_outline()),
        ))),
        ..Default::default()
      }),
      legend: legend(source),
      plot_visible_only: Some(c::PlotVisibleOnly {
        val: Some(true.into()),
      }),
      ..Default::default()
    }),
    shape_properties: Some(Box::new(shape_properties(
      Some(c::ShapePropertiesChoice2::SolidFill(Box::new(solid_fill(
        0x00ff_ffff,
      )))),
      Some(no_fill_outline()),
    ))),
    ..Default::default()
  }
}

fn chart_group(source: &OgraphChart, group: &OgraphChartGroup) -> c::PlotAreaChoice {
  let series = source
    .series
    .iter()
    .filter(|series| series.included && series.group_index == group.index)
    .collect::<Vec<_>>();
  let varied = c::VaryColors {
    val: Some(group.format.flags.contains(ChartFormatFlags::VARIED).into()),
  };
  match &group.kind {
    OgraphChartGroupKind::Bar(kind) => {
      let chart_series = series
        .iter()
        .map(|series| bar_series(source, group, series))
        .collect();
      let direction = c::BarDirection {
        val: if kind.flags.contains(ChartBarFlags::HORIZONTAL) {
          c::BarDirectionValues::Bar
        } else {
          c::BarDirectionValues::Column
        },
      };
      let grouping = Some(c::BarGrouping {
        val: Some(bar_grouping(kind.flags)),
      });
      let gap_width = u16::try_from(kind.category_space)
        .ok()
        .map(|val| c::GapWidth { val: Some(val) });
      if group.view_3d.is_some() {
        c::PlotAreaChoice::Bar3DChart(Box::new(c::Bar3DChart {
          bar_direction: direction,
          bar_grouping: grouping,
          vary_colors: Some(varied),
          bar_chart_series: chart_series,
          gap_width,
          gap_depth: group.view_3d.map(|view| c::GapDepth {
            val: Some(view.gap_percent),
          }),
          shape: series_shape(&series),
          axis_id: axis_ids(source, group),
          ..Default::default()
        }))
      } else {
        c::PlotAreaChoice::BarChart(Box::new(c::BarChart {
          bar_direction: direction,
          bar_grouping: grouping,
          vary_colors: Some(varied),
          bar_chart_series: chart_series,
          gap_width,
          overlap: i8::try_from(-kind.bar_space)
            .ok()
            .map(|val| c::Overlap { val: Some(val) }),
          axis_id: axis_ids(source, group),
          ..Default::default()
        }))
      }
    }
    OgraphChartGroupKind::Line(kind) => {
      let chart_series = series
        .iter()
        .map(|series| line_series(source, series))
        .collect();
      let grouping = c::Grouping {
        val: Some(grouping(kind.flags.bits())),
      };
      if group.view_3d.is_some() {
        c::PlotAreaChoice::Line3DChart(Box::new(c::Line3DChart {
          grouping,
          vary_colors: Some(varied),
          line_chart_series: chart_series,
          gap_depth: group.view_3d.map(|view| c::GapDepth {
            val: Some(view.gap_percent),
          }),
          axis_id: axis_ids(source, group),
          ..Default::default()
        }))
      } else {
        c::PlotAreaChoice::LineChart(Box::new(c::LineChart {
          grouping: Some(grouping),
          vary_colors: Some(varied),
          line_chart_series: chart_series,
          axis_id: axis_ids(source, group),
          ..Default::default()
        }))
      }
    }
    OgraphChartGroupKind::Pie(kind) => {
      let chart_series = series
        .iter()
        .map(|series| pie_series(source, series))
        .collect();
      if kind.doughnut_hole_percent > 0 {
        c::PlotAreaChoice::DoughnutChart(Box::new(c::DoughnutChart {
          vary_colors: Some(varied),
          pie_chart_series: chart_series,
          first_slice_angle: Some(c::FirstSliceAngle {
            val: Some(kind.starting_angle),
          }),
          hole_size: c::HoleSize {
            val: u8::try_from(kind.doughnut_hole_percent)
              .expect("validated MS-OGRAPH doughnut hole percentage fits u8"),
          },
          ..Default::default()
        }))
      } else if group.view_3d.is_some() {
        c::PlotAreaChoice::Pie3DChart(Box::new(c::Pie3DChart {
          vary_colors: Some(varied),
          pie_chart_series: chart_series,
          ..Default::default()
        }))
      } else {
        c::PlotAreaChoice::PieChart(Box::new(c::PieChart {
          vary_colors: Some(varied),
          pie_chart_series: chart_series,
          first_slice_angle: Some(c::FirstSliceAngle {
            val: Some(kind.starting_angle),
          }),
          ..Default::default()
        }))
      }
    }
    OgraphChartGroupKind::Area(kind) => {
      let chart_series = series
        .iter()
        .map(|series| area_series(source, series))
        .collect();
      let chart_grouping = Some(c::Grouping {
        val: Some(grouping(kind.flags.bits())),
      });
      if group.view_3d.is_some() {
        c::PlotAreaChoice::Area3DChart(Box::new(c::Area3DChart {
          grouping: chart_grouping,
          vary_colors: Some(varied),
          area_chart_series: chart_series,
          gap_depth: group.view_3d.map(|view| c::GapDepth {
            val: Some(view.gap_percent),
          }),
          axis_id: axis_ids(source, group),
          ..Default::default()
        }))
      } else {
        c::PlotAreaChoice::AreaChart(Box::new(c::AreaChart {
          grouping: chart_grouping,
          vary_colors: Some(varied),
          area_chart_series: chart_series,
          axis_id: axis_ids(source, group),
          ..Default::default()
        }))
      }
    }
    OgraphChartGroupKind::Scatter(kind) if kind.flags.contains(ChartScatterFlags::BUBBLES) => {
      c::PlotAreaChoice::BubbleChart(Box::new(c::BubbleChart {
        vary_colors: Some(varied),
        bubble_chart_series: series
          .iter()
          .map(|series| bubble_series(source, series))
          .collect(),
        bubble_scale: Some(c::BubbleScale {
          val: Some(u32::from(kind.bubble_size_ratio)),
        }),
        show_negative_bubbles: Some(c::ShowNegativeBubbles {
          val: Some(
            kind
              .flags
              .contains(ChartScatterFlags::SHOW_NEGATIVE_BUBBLES)
              .into(),
          ),
        }),
        size_represents: Some(c::SizeRepresents {
          val: Some(if kind.bubble_size_representation == 2 {
            c::SizeRepresentsValues::Width
          } else {
            c::SizeRepresentsValues::Area
          }),
        }),
        axis_id: axis_ids(source, group),
        ..Default::default()
      }))
    }
    OgraphChartGroupKind::Scatter(_) => {
      let smooth = series.iter().any(|series| series_is_smoothed(series));
      c::PlotAreaChoice::ScatterChart(Box::new(c::ScatterChart {
        scatter_style: c::ScatterStyle {
          val: Some(if smooth {
            c::ScatterStyleValues::SmoothMarker
          } else {
            c::ScatterStyleValues::LineMarker
          }),
        },
        vary_colors: Some(varied),
        scatter_chart_series: series
          .iter()
          .map(|series| scatter_series(source, series))
          .collect(),
        axis_id: axis_ids(source, group),
        ..Default::default()
      }))
    }
    OgraphChartGroupKind::Radar(_) | OgraphChartGroupKind::FilledRadar(_) => {
      c::PlotAreaChoice::RadarChart(Box::new(c::RadarChart {
        radar_style: c::RadarStyle {
          val: if matches!(&group.kind, OgraphChartGroupKind::FilledRadar(_)) {
            c::RadarStyleValues::Filled
          } else {
            c::RadarStyleValues::Marker
          },
        },
        vary_colors: Some(varied),
        radar_chart_series: series
          .iter()
          .map(|series| radar_series(source, series))
          .collect(),
        axis_id: axis_ids(source, group),
        ..Default::default()
      }))
    }
    OgraphChartGroupKind::Surface(kind) => {
      let chart_series = series
        .iter()
        .map(|series| surface_series(source, series))
        .collect();
      let wireframe = Some(c::Wireframe {
        val: Some((!kind.flags.contains(ChartSurfFlags::FILL_SURFACE)).into()),
      });
      if group.view_3d.is_some() {
        c::PlotAreaChoice::Surface3DChart(Box::new(c::Surface3DChart {
          wireframe,
          vary_colors: Some(varied),
          surface_chart_series: chart_series,
          axis_id: axis_ids(source, group),
          ..Default::default()
        }))
      } else {
        c::PlotAreaChoice::SurfaceChart(Box::new(c::SurfaceChart {
          wireframe,
          surface_chart_series: chart_series,
          axis_id: axis_ids(source, group),
          ..Default::default()
        }))
      }
    }
    OgraphChartGroupKind::BopPop(kind) => {
      let (split_type, split_position, custom_split) = if kind.automatic_split != 0 {
        (None, None, None)
      } else {
        let split_type = match kind.split_kind {
          0 => c::SplitValues::Position,
          1 => c::SplitValues::Value,
          2 => c::SplitValues::Percent,
          _ => c::SplitValues::Custom,
        };
        let position = match kind.split_kind {
          0 => f64::from(kind.split_position),
          1 => f64::from_bits(kind.split_value_bits),
          2 => f64::from(kind.split_percent),
          _ => 0.0,
        };
        (
          Some(c::SplitType { val: split_type }),
          (kind.split_kind != 3).then_some(c::SplitPosition { val: position }),
          group.bop_pop_custom.as_ref().map(custom_split),
        )
      };
      c::PlotAreaChoice::OfPieChart(Box::new(c::OfPieChart {
        of_pie_type: c::OfPieType {
          val: if kind.pie_kind == 2 {
            c::OfPieValues::Bar
          } else {
            c::OfPieValues::Pie
          },
        },
        vary_colors: Some(varied),
        pie_chart_series: series
          .iter()
          .map(|series| pie_series(source, series))
          .collect(),
        gap_width: u16::try_from(kind.gap_percent)
          .ok()
          .map(|val| c::GapWidth { val: Some(val) }),
        split_type,
        split_position,
        custom_split,
        second_pie_size: u16::try_from(kind.secondary_size_percent)
          .ok()
          .map(|val| c::SecondPieSize { val: Some(val) }),
        ..Default::default()
      }))
    }
  }
}

fn axis_id_value(axis_group: u16, axis_type: u16) -> i32 {
  i32::from(axis_group) * 4 + i32::from(axis_type) + 1
}

fn axis_ids(source: &OgraphChart, group: &OgraphChartGroup) -> Vec<c::AxisId> {
  source
    .axes
    .iter()
    .filter(|axis| axis.axis_group == group.axis_group)
    .map(|axis| c::AxisId {
      val: axis_id_value(axis.axis_group, axis.source.axis_type),
    })
    .collect()
}

fn plot_axis(source: &OgraphChart, axis: &OgraphAxis) -> c::PlotAreaChoice2 {
  let group = source
    .groups
    .iter()
    .find(|group| group.axis_group == axis.axis_group)
    .expect("validated MS-OGRAPH AxisParent contains a chart group");
  let scatter_or_bubble = matches!(group.kind, OgraphChartGroupKind::Scatter(_));
  if axis.source.axis_type == 0 && !scatter_or_bubble {
    c::PlotAreaChoice2::CategoryAxis(Box::new(category_axis(source, group, axis)))
  } else if axis.source.axis_type == 2 {
    c::PlotAreaChoice2::SeriesAxis(Box::new(series_axis(source, group, axis)))
  } else {
    c::PlotAreaChoice2::ValueAxis(Box::new(value_axis(source, group, axis)))
  }
}

fn crossing_axis<'a>(source: &'a OgraphChart, axis: &OgraphAxis) -> &'a OgraphAxis {
  let crossing_type = match axis.source.axis_type {
    0 => 1,
    1 => 0,
    _ => 1,
  };
  source
    .axes
    .iter()
    .find(|candidate| {
      candidate.axis_group == axis.axis_group && candidate.source.axis_type == crossing_type
    })
    .expect("validated MS-OGRAPH axis has its crossing axis")
}

fn axis_position(group: &OgraphChartGroup, axis: &OgraphAxis) -> c::AxisPosition {
  let horizontal_bar = matches!(
    group.kind,
    OgraphChartGroupKind::Bar(bar) if bar.flags.contains(ChartBarFlags::HORIZONTAL)
  );
  let horizontal = match axis.source.axis_type {
    0 => !horizontal_bar,
    1 => horizontal_bar,
    _ => true,
  };
  let secondary = axis.axis_group != 0;
  c::AxisPosition {
    val: match (horizontal, secondary) {
      (true, false) => c::AxisPositionValues::Bottom,
      (true, true) => c::AxisPositionValues::Top,
      (false, false) => c::AxisPositionValues::Left,
      (false, true) => c::AxisPositionValues::Right,
    },
  }
}

fn axis_scaling(axis: &OgraphAxis) -> Box<c::Scaling> {
  let reversed = axis
    .value_range
    .is_some_and(|range| range.flags.contains(ChartValueRangeFlags::REVERSED))
    || axis
      .label_range
      .is_some_and(|range| range.flags.contains(ChartLabelRangeFlags::REVERSED));
  let value_range = axis.value_range;
  Box::new(c::Scaling {
    log_base: value_range
      .filter(|range| range.flags.contains(ChartValueRangeFlags::LOGARITHMIC))
      .map(|_| c::LogBase { val: 10.0 }),
    orientation: Some(c::Orientation {
      val: Some(if reversed {
        c::OrientationValues::MaxMin
      } else {
        c::OrientationValues::MinMax
      }),
    }),
    max_axis_value: value_range
      .filter(|range| !range.flags.contains(ChartValueRangeFlags::AUTO_MAXIMUM))
      .map(|range| c::MaxAxisValue {
        val: f64::from_bits(range.maximum_bits),
      }),
    min_axis_value: value_range
      .filter(|range| !range.flags.contains(ChartValueRangeFlags::AUTO_MINIMUM))
      .map(|range| c::MinAxisValue {
        val: f64::from_bits(range.minimum_bits),
      }),
    ..Default::default()
  })
}

fn tick_mark(value: u8) -> c::TickMarkValues {
  match value {
    0 => c::TickMarkValues::None,
    1 => c::TickMarkValues::Inside,
    2 => c::TickMarkValues::Outside,
    _ => c::TickMarkValues::Cross,
  }
}

fn tick_label_position(value: u8) -> c::TickLabelPositionValues {
  match value {
    0 => c::TickLabelPositionValues::None,
    1 => c::TickLabelPositionValues::Low,
    2 => c::TickLabelPositionValues::High,
    _ => c::TickLabelPositionValues::NextTo,
  }
}

#[derive(Clone, Copy)]
enum AutomaticLineRole {
  CommonHairline,
  LinearSeries(u16),
  FilledSeries { three_dimensional: bool },
}

fn indexed_color(chart: &OgraphChart, color_index: u16, fallback: u32) -> u32 {
  match color_index {
    8..=63 => chart
      .palette
      .get(usize::from(color_index - 8))
      .copied()
      .unwrap_or(fallback),
    // BIFF chart-window text/background and automatic series border colors.
    77 | 79 => 0x0000_0000,
    78 => 0x00ff_ffff,
    _ => fallback,
  }
}

fn line_width_emu(weight: i16) -> i32 {
  match weight {
    -1 => 0,
    0 => 12_600,
    1 => 25_200,
    _ => 37_800,
  }
}

fn line_dash(style: u16) -> Option<a::OutlineChoice2> {
  let val = match style {
    1 => a::PresetLineDashValues::Dash,
    2 => a::PresetLineDashValues::Dot,
    3 => a::PresetLineDashValues::DashDot,
    4 => a::PresetLineDashValues::LargeDashDotDot,
    _ => return None,
  };
  Some(a::OutlineChoice2::PresetDash(a::PresetDash {
    val: Some(val),
  }))
}

fn line_outline(
  chart: &OgraphChart,
  line: Option<ChartLineFormatRecord>,
  role: AutomaticLineRole,
) -> a::Outline {
  if line.is_some_and(|line| !line.flags.contains(ChartLineFlags::AUTO) && line.line_style == 5) {
    return no_fill_outline();
  }
  let automatic = line.is_none_or(|line| line.flags.contains(ChartLineFlags::AUTO));
  let (color, weight, style) = if automatic {
    match role {
      AutomaticLineRole::CommonHairline => (0, -1, 0),
      AutomaticLineRole::LinearSeries(format_index) => (
        automatic_color(chart, format_index, false).unwrap_or_default(),
        0,
        0,
      ),
      AutomaticLineRole::FilledSeries { three_dimensional } => {
        (0, if three_dimensional { -1 } else { 0 }, 0)
      }
    }
  } else {
    let line = line.expect("nonautomatic Graph line exists");
    let weight = if matches!(
      role,
      AutomaticLineRole::FilledSeries {
        three_dimensional: true
      }
    ) {
      -1
    } else {
      line.weight
    };
    (
      indexed_color(chart, line.color_index, line.color_rgb),
      weight,
      line.line_style,
    )
  };
  let opacity = match style {
    6 => 75_000,
    7 => 50_000,
    8 => 25_000,
    _ => 100_000,
  };
  a::Outline {
    width: Some(line_width_emu(weight)),
    outline_choice1: Some(a::OutlineChoice::SolidFill(Box::new(
      solid_fill_with_opacity(color, opacity),
    ))),
    outline_choice2: line_dash(style),
    ..Default::default()
  }
}

fn axis_line_shape_properties(
  chart: &OgraphChart,
  line: Option<ChartLineFormatRecord>,
) -> Option<Box<c::ChartShapeProperties>> {
  Some(Box::new(c::ChartShapeProperties {
    outline: Some(Box::new(line_outline(
      chart,
      line,
      AutomaticLineRole::CommonHairline,
    ))),
    ..Default::default()
  }))
}

fn major_gridlines(chart: &OgraphChart, axis: &OgraphAxis) -> Option<Box<c::MajorGridlines>> {
  axis.line_formats[1].and_then(|line| {
    (line.line_style != 5).then(|| {
      Box::new(c::MajorGridlines {
        chart_shape_properties: axis_line_shape_properties(chart, Some(line)),
      })
    })
  })
}

fn minor_gridlines(chart: &OgraphChart, axis: &OgraphAxis) -> Option<Box<c::MinorGridlines>> {
  axis.line_formats[2].and_then(|line| {
    (line.line_style != 5).then(|| {
      Box::new(c::MinorGridlines {
        chart_shape_properties: axis_line_shape_properties(chart, Some(line)),
      })
    })
  })
}

enum AxisCrossing {
  AutoZero,
  Maximum,
  At(f64),
}

fn axis_crossing(crossing: &OgraphAxis) -> AxisCrossing {
  if let Some(range) = crossing.value_range {
    if range.flags.contains(ChartValueRangeFlags::MAXIMUM_CROSS) {
      AxisCrossing::Maximum
    } else if range.flags.contains(ChartValueRangeFlags::AUTO_CROSS) {
      AxisCrossing::AutoZero
    } else {
      AxisCrossing::At(f64::from_bits(range.cross_value_bits))
    }
  } else {
    let range = crossing
      .label_range
      .expect("validated MS-OGRAPH axis has a label or value range");
    if range.flags.contains(ChartLabelRangeFlags::MAXIMUM_CROSS) {
      AxisCrossing::Maximum
    } else {
      AxisCrossing::At(f64::from(range.crossing_point))
    }
  }
}

fn category_axis_choice(source: &OgraphChart, axis: &OgraphAxis) -> c::CategoryAxisChoice {
  match axis_crossing(crossing_axis(source, axis)) {
    AxisCrossing::AutoZero => c::CategoryAxisChoice::Crosses(c::Crosses {
      val: c::CrossesValues::AutoZero,
    }),
    AxisCrossing::Maximum => c::CategoryAxisChoice::Crosses(c::Crosses {
      val: c::CrossesValues::Maximum,
    }),
    AxisCrossing::At(val) => c::CategoryAxisChoice::CrossesAt(c::CrossesAt { val }),
  }
}

fn value_axis_choice(source: &OgraphChart, axis: &OgraphAxis) -> c::ValueAxisChoice {
  match axis_crossing(crossing_axis(source, axis)) {
    AxisCrossing::AutoZero => c::ValueAxisChoice::Crosses(c::Crosses {
      val: c::CrossesValues::AutoZero,
    }),
    AxisCrossing::Maximum => c::ValueAxisChoice::Crosses(c::Crosses {
      val: c::CrossesValues::Maximum,
    }),
    AxisCrossing::At(val) => c::ValueAxisChoice::CrossesAt(c::CrossesAt { val }),
  }
}

fn series_axis_choice(source: &OgraphChart, axis: &OgraphAxis) -> c::SeriesAxisChoice {
  match axis_crossing(crossing_axis(source, axis)) {
    AxisCrossing::AutoZero => c::SeriesAxisChoice::Crosses(c::Crosses {
      val: c::CrossesValues::AutoZero,
    }),
    AxisCrossing::Maximum => c::SeriesAxisChoice::Crosses(c::Crosses {
      val: c::CrossesValues::Maximum,
    }),
    AxisCrossing::At(val) => c::SeriesAxisChoice::CrossesAt(c::CrossesAt { val }),
  }
}

fn category_axis(
  source: &OgraphChart,
  group: &OgraphChartGroup,
  axis: &OgraphAxis,
) -> c::CategoryAxis {
  let tick = axis.tick;
  let label_range = axis
    .label_range
    .expect("validated MS-OGRAPH category axis has LabelRange");
  c::CategoryAxis {
    axis_id: c::AxisId {
      val: axis_id_value(axis.axis_group, axis.source.axis_type),
    },
    scaling: axis_scaling(axis),
    axis_position: axis_position(group, axis),
    major_gridlines: major_gridlines(source, axis),
    minor_gridlines: minor_gridlines(source, axis),
    major_tick_mark: tick.map(|tick| c::MajorTickMark {
      val: Some(tick_mark(tick.major_tick_type)),
    }),
    minor_tick_mark: tick.map(|tick| c::MinorTickMark {
      val: Some(tick_mark(tick.minor_tick_type)),
    }),
    tick_label_position: tick.map(|tick| c::TickLabelPosition {
      val: Some(tick_label_position(tick.label_position)),
    }),
    chart_shape_properties: axis_line_shape_properties(source, axis.line_formats[0]),
    crossing_axis: c::CrossingAxis {
      val: axis_id_value(
        axis.axis_group,
        crossing_axis(source, axis).source.axis_type,
      ),
    },
    category_axis_choice: Some(category_axis_choice(source, axis)),
    auto_labeled: Some(c::AutoLabeled {
      val: Some(true.into()),
    }),
    label_alignment: Some(c::LabelAlignment {
      val: c::LabelAlignmentValues::Center,
    }),
    tick_label_skip: (label_range.label_frequency > 0).then_some(c::TickLabelSkip {
      val: i32::from(label_range.label_frequency),
    }),
    tick_mark_skip: (label_range.tick_frequency > 0).then_some(c::TickMarkSkip {
      val: i32::from(label_range.tick_frequency),
    }),
    ..Default::default()
  }
}

fn value_axis(source: &OgraphChart, group: &OgraphChartGroup, axis: &OgraphAxis) -> c::ValueAxis {
  let tick = axis.tick;
  let value_range = axis
    .value_range
    .expect("validated MS-OGRAPH value axis has ValueRange");
  let crossing = crossing_axis(source, axis);
  c::ValueAxis {
    axis_id: c::AxisId {
      val: axis_id_value(axis.axis_group, axis.source.axis_type),
    },
    scaling: axis_scaling(axis),
    axis_position: axis_position(group, axis),
    major_gridlines: major_gridlines(source, axis),
    minor_gridlines: minor_gridlines(source, axis),
    major_tick_mark: tick.map(|tick| c::MajorTickMark {
      val: Some(tick_mark(tick.major_tick_type)),
    }),
    minor_tick_mark: tick.map(|tick| c::MinorTickMark {
      val: Some(tick_mark(tick.minor_tick_type)),
    }),
    tick_label_position: tick.map(|tick| c::TickLabelPosition {
      val: Some(tick_label_position(tick.label_position)),
    }),
    chart_shape_properties: axis_line_shape_properties(source, axis.line_formats[0]),
    crossing_axis: c::CrossingAxis {
      val: axis_id_value(axis.axis_group, crossing.source.axis_type),
    },
    value_axis_choice: Some(value_axis_choice(source, axis)),
    cross_between: crossing.label_range.map(|range| c::CrossBetween {
      val: if range
        .flags
        .contains(ChartLabelRangeFlags::BETWEEN_CATEGORIES)
      {
        c::CrossBetweenValues::Between
      } else {
        c::CrossBetweenValues::MidpointCategory
      },
    }),
    major_unit: (!value_range.flags.contains(ChartValueRangeFlags::AUTO_MAJOR)).then(|| {
      c::MajorUnit {
        val: f64::from_bits(value_range.major_unit_bits),
      }
    }),
    minor_unit: (!value_range.flags.contains(ChartValueRangeFlags::AUTO_MINOR)).then(|| {
      c::MinorUnit {
        val: f64::from_bits(value_range.minor_unit_bits),
      }
    }),
    ..Default::default()
  }
}

fn series_axis(source: &OgraphChart, group: &OgraphChartGroup, axis: &OgraphAxis) -> c::SeriesAxis {
  let tick = axis.tick;
  let label_range = axis
    .label_range
    .expect("validated MS-OGRAPH series axis has LabelRange");
  c::SeriesAxis {
    axis_id: c::AxisId {
      val: axis_id_value(axis.axis_group, axis.source.axis_type),
    },
    scaling: axis_scaling(axis),
    axis_position: axis_position(group, axis),
    major_gridlines: major_gridlines(source, axis),
    minor_gridlines: minor_gridlines(source, axis),
    major_tick_mark: tick.map(|tick| c::MajorTickMark {
      val: Some(tick_mark(tick.major_tick_type)),
    }),
    minor_tick_mark: tick.map(|tick| c::MinorTickMark {
      val: Some(tick_mark(tick.minor_tick_type)),
    }),
    tick_label_position: tick.map(|tick| c::TickLabelPosition {
      val: Some(tick_label_position(tick.label_position)),
    }),
    chart_shape_properties: axis_line_shape_properties(source, axis.line_formats[0]),
    crossing_axis: c::CrossingAxis {
      val: axis_id_value(
        axis.axis_group,
        crossing_axis(source, axis).source.axis_type,
      ),
    },
    series_axis_choice: Some(series_axis_choice(source, axis)),
    tick_label_skip: (label_range.label_frequency > 0).then_some(c::TickLabelSkip {
      val: i32::from(label_range.label_frequency),
    }),
    tick_mark_skip: (label_range.tick_frequency > 0).then_some(c::TickMarkSkip {
      val: i32::from(label_range.tick_frequency),
    }),
    ..Default::default()
  }
}

fn bar_grouping(flags: ChartBarFlags) -> c::BarGroupingValues {
  if flags.contains(ChartBarFlags::DISPLAY_AS_PERCENTAGE) {
    c::BarGroupingValues::PercentStacked
  } else if flags.contains(ChartBarFlags::STACKED) {
    c::BarGroupingValues::Stacked
  } else {
    c::BarGroupingValues::Clustered
  }
}

fn grouping(flags: u16) -> c::GroupingValues {
  if flags & ChartLineGroupFlags::DISPLAY_AS_PERCENTAGE.bits() != 0
    || flags & ChartAreaRecordFlags::DISPLAY_AS_PERCENTAGE.bits() != 0
  {
    c::GroupingValues::PercentStacked
  } else if flags & ChartLineGroupFlags::STACKED.bits() != 0
    || flags & ChartAreaRecordFlags::STACKED.bits() != 0
  {
    c::GroupingValues::Stacked
  } else {
    c::GroupingValues::Standard
  }
}

fn index(series: &OgraphSeries) -> c::Index {
  c::Index {
    val: u32::try_from(series.index).expect("MS-OGRAPH series index fits u32"),
  }
}

fn order(series: &OgraphSeries) -> c::Order {
  c::Order {
    val: u32::try_from(series.index).expect("MS-OGRAPH series index fits u32"),
  }
}

fn series_text(series: &OgraphSeries) -> Option<Box<c::SeriesText>> {
  (!series.name.is_empty()).then(|| {
    Box::new(c::SeriesText {
      series_text_choice: Some(c::SeriesTextChoice::NumericValue(series.name.clone())),
    })
  })
}

fn values(values: &[Option<f64>]) -> Option<Box<c::Values>> {
  Some(Box::new(c::Values {
    values_choice: Some(c::ValuesChoice::NumberLiteral(Box::new(number_literal(
      values,
    )))),
  }))
}

fn y_values(values: &[Option<f64>]) -> Option<Box<c::YValues>> {
  Some(Box::new(c::YValues {
    y_values_choice: Some(c::YValuesChoice::NumberLiteral(Box::new(number_literal(
      values,
    )))),
  }))
}

fn bubble_size(values: &[Option<f64>]) -> Option<Box<c::BubbleSize>> {
  Some(Box::new(c::BubbleSize {
    bubble_size_choice: Some(c::BubbleSizeChoice::NumberLiteral(Box::new(
      number_literal(values),
    ))),
  }))
}

fn number_literal(values: &[Option<f64>]) -> c::NumberLiteral {
  c::NumberLiteral {
    point_count: Some(c::PointCount {
      val: u32::try_from(values.len()).expect("MS-OGRAPH point count fits u32"),
    }),
    numeric_point: values
      .iter()
      .enumerate()
      .filter_map(|(index, value)| {
        value.map(|value| c::NumericPoint {
          index: u32::try_from(index).expect("MS-OGRAPH point index fits u32"),
          numeric_value: value.to_string(),
          ..Default::default()
        })
      })
      .collect(),
    ..Default::default()
  }
}

fn category_axis_data(values: &[Option<OgraphCellValue>]) -> Option<Box<c::CategoryAxisData>> {
  let contains_text = values
    .iter()
    .any(|value| matches!(value, Some(OgraphCellValue::Text(_))));
  let choice = if contains_text {
    c::CategoryAxisDataChoice::StringLiteral(Box::new(string_literal(values)))
  } else {
    c::CategoryAxisDataChoice::NumberLiteral(Box::new(number_literal(
      &values
        .iter()
        .map(|value| match value {
          Some(OgraphCellValue::Number(value)) => Some(*value),
          _ => None,
        })
        .collect::<Vec<_>>(),
    )))
  };
  Some(Box::new(c::CategoryAxisData {
    category_axis_data_choice: Some(choice),
  }))
}

fn x_values(values: &[Option<OgraphCellValue>]) -> Option<Box<c::XValues>> {
  let contains_text = values
    .iter()
    .any(|value| matches!(value, Some(OgraphCellValue::Text(_))));
  let choice = if contains_text {
    c::XValuesChoice::StringLiteral(Box::new(string_literal(values)))
  } else {
    c::XValuesChoice::NumberLiteral(Box::new(number_literal(
      &values
        .iter()
        .map(|value| match value {
          Some(OgraphCellValue::Number(value)) => Some(*value),
          _ => None,
        })
        .collect::<Vec<_>>(),
    )))
  };
  Some(Box::new(c::XValues {
    x_values_choice: Some(choice),
  }))
}

fn string_literal(values: &[Option<OgraphCellValue>]) -> c::StringLiteral {
  c::StringLiteral {
    point_count: Some(c::PointCount {
      val: u32::try_from(values.len()).expect("MS-OGRAPH point count fits u32"),
    }),
    string_point: values
      .iter()
      .enumerate()
      .filter_map(|(index, value)| {
        let text = match value.as_ref()? {
          OgraphCellValue::Text(value) => value.clone(),
          OgraphCellValue::Number(value) => value.to_string(),
          OgraphCellValue::Blank => return None,
        };
        Some(c::StringPoint {
          index: u32::try_from(index).expect("MS-OGRAPH point index fits u32"),
          numeric_value: text,
        })
      })
      .collect(),
    ..Default::default()
  }
}

fn series_data_format(series: &OgraphSeries) -> Option<&OgraphDataFormat> {
  series
    .data_formats
    .iter()
    .find(|format| format.source.point_index == u16::MAX)
}

fn format_index(series: &OgraphSeries) -> u16 {
  series_data_format(series)
    .map(|format| format.source.series_number)
    .unwrap_or_else(|| {
      u16::try_from(series.index).expect("validated MS-OGRAPH series index fits u16")
    })
}

fn series_is_smoothed(series: &OgraphSeries) -> bool {
  series_data_format(series).is_some_and(|format| {
    format
      .series
      .is_some_and(|format| format.flags.contains(ChartSeriesFormatFlags::SMOOTHED_LINE))
  })
}

fn series_shape_properties(
  chart: &OgraphChart,
  series: &OgraphSeries,
  filled: bool,
) -> Option<Box<c::ChartShapeProperties>> {
  let format = series_data_format(series);
  let format_index = format_index(series);
  let three_dimensional = chart
    .groups
    .get(series.group_index)
    .is_some_and(|group| group.view_3d.is_some());
  let fill = filled.then(|| {
    area_fill_choice(
      chart,
      format.and_then(|format| format.area),
      automatic_color(chart, format_index, true).unwrap_or_default(),
    )
  });
  let line_role = if filled {
    AutomaticLineRole::FilledSeries { three_dimensional }
  } else {
    AutomaticLineRole::LinearSeries(format_index)
  };
  Some(Box::new(c::ChartShapeProperties {
    chart_shape_properties_choice2: fill,
    outline: Some(Box::new(line_outline(
      chart,
      format.and_then(|format| format.line),
      line_role,
    ))),
    ..Default::default()
  }))
}

fn automatic_color(chart: &OgraphChart, format_index: u16, filled: bool) -> Option<u32> {
  // LibreOffice's BIFF/Graph importer uses these two exact Icv sequences.
  // Icv 8..=63 address palette slots 0..=55; the line sequence deliberately
  // puts Icv 63 last instead of wrapping it immediately after Icv 62.
  const LINE_PALETTE_INDICES: [usize; 56] = [
    24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
    48, 49, 50, 51, 52, 53, 54, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
    19, 20, 21, 22, 23, 55,
  ];
  const FILL_PALETTE_INDICES: [usize; 56] = [
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
    40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
    10, 11, 12, 13, 14, 15,
  ];
  const FILL_TRANSPARENCY: [u8; 5] = [0x00, 0x40, 0x20, 0x60, 0x70];

  let sequence_index = usize::from(format_index) % 56;
  let palette_index = if filled {
    FILL_PALETTE_INDICES[sequence_index]
  } else {
    LINE_PALETTE_INDICES[sequence_index]
  };
  let color = chart.palette.get(palette_index).copied()?;
  if !filled {
    return Some(color);
  }
  let transparency = FILL_TRANSPARENCY[(usize::from(format_index) / 56) % 5];
  Some(mix_color(color, 0x00ff_ffff, transparency))
}

fn mix_color(foreground: u32, background: u32, transparency: u8) -> u32 {
  let mix = |shift: u32| {
    let foreground =
      i32::try_from((foreground >> shift) & 0xff_u32).expect("RGB component fits i32");
    let background =
      i32::try_from((background >> shift) & 0xff_u32).expect("RGB component fits i32");
    u32::try_from(((background - foreground) * i32::from(transparency)) / 0x80 + foreground)
      .expect("mixed RGB component remains in byte range")
  };
  mix(0) | (mix(8) << 8) | (mix(16) << 16)
}

fn no_fill_outline() -> a::Outline {
  a::Outline {
    outline_choice1: Some(a::OutlineChoice::NoFill(a::NoFill::default())),
    ..Default::default()
  }
}

fn shape_properties(
  fill: Option<c::ShapePropertiesChoice2>,
  outline: Option<a::Outline>,
) -> c::ShapeProperties {
  c::ShapeProperties {
    shape_properties_choice2: fill,
    outline: outline.map(Box::new),
    ..Default::default()
  }
}

fn area_fill_choice(
  chart: &OgraphChart,
  area: Option<ChartAreaFormatRecord>,
  automatic_fill: u32,
) -> c::ChartShapePropertiesChoice2 {
  let Some(area) = area else {
    return c::ChartShapePropertiesChoice2::SolidFill(Box::new(solid_fill(automatic_fill)));
  };
  if area.flags.contains(ChartAreaFlags::AUTO) {
    return c::ChartShapePropertiesChoice2::SolidFill(Box::new(solid_fill(automatic_fill)));
  }
  if area.fill_pattern == 0 {
    return c::ChartShapePropertiesChoice2::NoFill(a::NoFill::default());
  }
  let foreground = indexed_color(chart, area.foreground_color_index, area.foreground_rgb);
  if area.fill_pattern == 1 {
    return c::ChartShapePropertiesChoice2::SolidFill(Box::new(solid_fill(foreground)));
  }
  let background = indexed_color(chart, area.background_color_index, area.background_rgb);
  // LibreOffice's BIFF chart importer converts legacy hatch fills into their
  // exact foreground/background coverage color before handing them to chart2.
  const PATTERN_TRANSPARENCY: [u8; 19] = [
    0x80, 0x00, 0x40, 0x20, 0x60, 0x40, 0x40, 0x40, 0x40, 0x40, 0x20, 0x60, 0x60, 0x60, 0x60, 0x48,
    0x50, 0x70, 0x78,
  ];
  let transparency = PATTERN_TRANSPARENCY[usize::from(area.fill_pattern)];
  c::ChartShapePropertiesChoice2::SolidFill(Box::new(solid_fill(mix_color(
    foreground,
    background,
    transparency,
  ))))
}

fn chart_plane_shape_properties(
  chart: &OgraphChart,
  axis: &OgraphAxis,
  automatic_fill: u32,
) -> Option<c::ShapeProperties> {
  if axis.line_formats[3].is_none() && axis.area_formats[3].is_none() {
    return None;
  }
  let fill = match area_fill_choice(chart, axis.area_formats[3], automatic_fill) {
    c::ChartShapePropertiesChoice2::NoFill(value) => c::ShapePropertiesChoice2::NoFill(value),
    c::ChartShapePropertiesChoice2::SolidFill(value) => c::ShapePropertiesChoice2::SolidFill(value),
    c::ChartShapePropertiesChoice2::GradientFill(value) => {
      c::ShapePropertiesChoice2::GradientFill(value)
    }
    c::ChartShapePropertiesChoice2::BlipFill(value) => c::ShapePropertiesChoice2::BlipFill(value),
    c::ChartShapePropertiesChoice2::PatternFill(value) => {
      c::ShapePropertiesChoice2::PatternFill(value)
    }
  };
  Some(shape_properties(
    Some(fill),
    Some(line_outline(
      chart,
      axis.line_formats[3],
      AutomaticLineRole::CommonHairline,
    )),
  ))
}

fn solid_fill(color: u32) -> a::SolidFill {
  solid_fill_with_opacity(color, 100_000)
}

fn solid_fill_with_opacity(color: u32, opacity: i32) -> a::SolidFill {
  let red = color & 0xff;
  let green = (color >> 8) & 0xff;
  let blue = (color >> 16) & 0xff;
  a::SolidFill {
    solid_fill_choice: Some(a::SolidFillChoice::RgbColorModelHex(Box::new(
      a::RgbColorModelHex {
        val: format!("{red:02X}{green:02X}{blue:02X}"),
        rgb_color_model_hex_choice: (opacity < 100_000)
          .then_some(a::RgbColorModelHexChoice::Alpha(a::Alpha {
            val: ooxmlsdk::units::DrawingmlPercentageValue::Decimal(opacity),
          }))
          .into_iter()
          .collect(),
        ..Default::default()
      },
    ))),
    ..Default::default()
  }
}

fn data_points(chart: &OgraphChart, series: &OgraphSeries, filled: bool) -> Vec<c::DataPoint> {
  let parent = series_data_format(series);
  let parent_area_is_automatic = parent
    .and_then(|format| format.area)
    .is_none_or(|area| area.flags.contains(ChartAreaFlags::AUTO));
  let parent_line_is_automatic = parent
    .and_then(|format| format.line)
    .is_none_or(|line| line.flags.contains(ChartLineFlags::AUTO));
  let three_dimensional = chart
    .groups
    .get(series.group_index)
    .is_some_and(|group| group.view_3d.is_some());
  series
    .data_formats
    .iter()
    .filter(|format| format.source.point_index != u16::MAX)
    .map(|format| {
      let fill = if filled {
        match format.area {
          Some(area) if !area.flags.contains(ChartAreaFlags::AUTO) || !parent_area_is_automatic => {
            Some(area_fill_choice(
              chart,
              Some(area),
              automatic_color(chart, format.source.series_number, true).unwrap_or_default(),
            ))
          }
          _ => None,
        }
      } else {
        None
      };
      let outline = (filled
        && format.line.is_some_and(|line| {
          !line.flags.contains(ChartLineFlags::AUTO) || !parent_line_is_automatic
        }))
      .then(|| {
        Box::new(line_outline(
          chart,
          format.line,
          AutomaticLineRole::FilledSeries { three_dimensional },
        ))
      });
      let chart_shape_properties = if fill.is_some() || outline.is_some() {
        Some(Box::new(c::ChartShapeProperties {
          chart_shape_properties_choice2: fill,
          outline,
          ..Default::default()
        }))
      } else {
        None
      };
      c::DataPoint {
        index: c::Index {
          val: u32::from(format.source.point_index),
        },
        invert_if_negative: format.area.and_then(|area| {
          area
            .flags
            .contains(ChartAreaFlags::INVERT_NEGATIVE)
            .then_some(c::InvertIfNegative {
              val: Some(true.into()),
            })
        }),
        explosion: format.pie.map(|pie| c::Explosion {
          val: u32::try_from(pie.explode_percent)
            .expect("validated MS-OGRAPH pie explosion is nonnegative"),
        }),
        chart_shape_properties,
        ..Default::default()
      }
    })
    .collect()
}

fn series_data_labels(series: &OgraphSeries) -> Option<Box<c::DataLabels>> {
  let flags = series_data_format(series)?.attached_label?.flags;
  Some(Box::new(c::DataLabels {
    data_labels_choice: Some(c::DataLabelsChoice::Sequence(Box::new(
      c::DataLabelsChoiceSequence {
        show_value: flag(flags, ChartAttachedLabelFlags::SHOW_VALUE).then_some(c::ShowValue {
          val: Some(true.into()),
        }),
        show_percent: (flag(flags, ChartAttachedLabelFlags::SHOW_PERCENT)
          || flag(flags, ChartAttachedLabelFlags::SHOW_LABEL_AND_PERCENT))
        .then_some(c::ShowPercent {
          val: Some(true.into()),
        }),
        show_category_name: (flag(flags, ChartAttachedLabelFlags::SHOW_LABEL)
          || flag(flags, ChartAttachedLabelFlags::SHOW_LABEL_AND_PERCENT))
        .then_some(c::ShowCategoryName {
          val: Some(true.into()),
        }),
        show_series_name: flag(flags, ChartAttachedLabelFlags::SHOW_SERIES_NAME).then_some(
          c::ShowSeriesName {
            val: Some(true.into()),
          },
        ),
        show_bubble_size: flag(flags, ChartAttachedLabelFlags::SHOW_BUBBLE_SIZE).then_some(
          c::ShowBubbleSize {
            val: Some(true.into()),
          },
        ),
        ..Default::default()
      },
    ))),
    ..Default::default()
  }))
}

fn flag(flags: ChartAttachedLabelFlags, flag: ChartAttachedLabelFlags) -> bool {
  flags.contains(flag)
}

fn marker(chart: &OgraphChart, series: &OgraphSeries) -> Option<Box<c::Marker>> {
  let marker = series_data_format(series)?.marker?;
  let marker_type = if marker.flags.contains(ChartMarkerFlags::AUTO) {
    const AUTOMATIC: [c::MarkerStyleValues; 9] = [
      c::MarkerStyleValues::Diamond,
      c::MarkerStyleValues::Square,
      c::MarkerStyleValues::Triangle,
      c::MarkerStyleValues::X,
      c::MarkerStyleValues::Star,
      c::MarkerStyleValues::Circle,
      c::MarkerStyleValues::Plus,
      c::MarkerStyleValues::Dash,
      c::MarkerStyleValues::Dot,
    ];
    AUTOMATIC[usize::from(format_index(series)) % AUTOMATIC.len()]
  } else {
    match marker.marker_type {
      1 => c::MarkerStyleValues::Square,
      2 => c::MarkerStyleValues::Diamond,
      3 => c::MarkerStyleValues::Triangle,
      4 => c::MarkerStyleValues::X,
      5 => c::MarkerStyleValues::Star,
      6 => c::MarkerStyleValues::Dash,
      7 => c::MarkerStyleValues::Dot,
      8 => c::MarkerStyleValues::Circle,
      9 => c::MarkerStyleValues::Plus,
      _ => c::MarkerStyleValues::None,
    }
  };
  let automatic = marker.flags.contains(ChartMarkerFlags::AUTO);
  let automatic_color = automatic_color(chart, format_index(series), false).unwrap_or_default();
  let foreground = if automatic {
    automatic_color
  } else {
    indexed_color(chart, marker.foreground_color_index, marker.foreground_rgb)
  };
  let background = if automatic {
    automatic_color
  } else {
    indexed_color(chart, marker.background_color_index, marker.background_rgb)
  };
  let fill = if marker.flags.contains(ChartMarkerFlags::HIDE_INTERIOR) {
    c::ChartShapePropertiesChoice2::NoFill(a::NoFill::default())
  } else {
    c::ChartShapePropertiesChoice2::SolidFill(Box::new(solid_fill(background)))
  };
  let outline = if marker.flags.contains(ChartMarkerFlags::HIDE_BORDER) {
    no_fill_outline()
  } else {
    a::Outline {
      width: Some(12_600),
      outline_choice1: Some(a::OutlineChoice::SolidFill(Box::new(solid_fill(
        foreground,
      )))),
      ..Default::default()
    }
  };
  Some(Box::new(c::Marker {
    symbol: Some(c::Symbol { val: marker_type }),
    size: u8::try_from((marker.marker_size_twips / 20).clamp(2, 72))
      .ok()
      .map(|val| c::Size { val: Some(val) }),
    chart_shape_properties: Some(Box::new(c::ChartShapeProperties {
      chart_shape_properties_choice2: Some(fill),
      outline: Some(Box::new(outline)),
      ..Default::default()
    })),
    ..Default::default()
  }))
}

fn bar_series(
  chart: &OgraphChart,
  _group: &OgraphChartGroup,
  series: &OgraphSeries,
) -> c::BarChartSeries {
  c::BarChartSeries {
    index: index(series),
    order: order(series),
    series_text: series_text(series),
    chart_shape_properties: series_shape_properties(chart, series, true),
    invert_if_negative: series_data_format(series).and_then(|format| {
      format.area.and_then(|area| {
        area
          .flags
          .contains(ChartAreaFlags::INVERT_NEGATIVE)
          .then_some(c::InvertIfNegative {
            val: Some(true.into()),
          })
      })
    }),
    data_point: data_points(chart, series, true),
    data_labels: series_data_labels(series),
    category_axis_data: category_axis_data(&series.categories),
    values: values(&series.values),
    ..Default::default()
  }
}

fn line_series(chart: &OgraphChart, series: &OgraphSeries) -> c::LineChartSeries {
  c::LineChartSeries {
    index: index(series),
    order: order(series),
    series_text: series_text(series),
    chart_shape_properties: series_shape_properties(chart, series, false),
    marker: marker(chart, series),
    data_point: data_points(chart, series, false),
    data_labels: series_data_labels(series),
    category_axis_data: category_axis_data(&series.categories),
    values: values(&series.values),
    smooth: series_is_smoothed(series).then_some(c::Smooth {
      val: Some(true.into()),
    }),
    ..Default::default()
  }
}

fn area_series(chart: &OgraphChart, series: &OgraphSeries) -> c::AreaChartSeries {
  c::AreaChartSeries {
    index: index(series),
    order: order(series),
    series_text: series_text(series),
    chart_shape_properties: series_shape_properties(chart, series, true),
    data_point: data_points(chart, series, true),
    data_labels: series_data_labels(series),
    category_axis_data: category_axis_data(&series.categories),
    values: values(&series.values),
    ..Default::default()
  }
}

fn pie_series(chart: &OgraphChart, series: &OgraphSeries) -> c::PieChartSeries {
  c::PieChartSeries {
    index: Some(index(series)),
    order: order(series),
    series_text: series_text(series),
    chart_shape_properties: series_shape_properties(chart, series, true),
    explosion: series_data_format(series).and_then(|format| {
      format.pie.map(|pie| c::Explosion {
        val: u32::try_from(pie.explode_percent)
          .expect("validated MS-OGRAPH pie explosion is nonnegative"),
      })
    }),
    data_point: data_points(chart, series, true),
    data_labels: series_data_labels(series),
    category_axis_data: category_axis_data(&series.categories),
    values: values(&series.values),
    ..Default::default()
  }
}

fn radar_series(chart: &OgraphChart, series: &OgraphSeries) -> c::RadarChartSeries {
  c::RadarChartSeries {
    index: index(series),
    order: order(series),
    series_text: series_text(series),
    chart_shape_properties: series_shape_properties(chart, series, false),
    marker: marker(chart, series),
    data_point: data_points(chart, series, false),
    data_labels: series_data_labels(series),
    category_axis_data: category_axis_data(&series.categories),
    values: values(&series.values),
    ..Default::default()
  }
}

fn scatter_series(chart: &OgraphChart, series: &OgraphSeries) -> c::ScatterChartSeries {
  c::ScatterChartSeries {
    index: index(series),
    order: order(series),
    series_text: series_text(series),
    chart_shape_properties: series_shape_properties(chart, series, false),
    marker: marker(chart, series),
    data_point: data_points(chart, series, false),
    data_labels: series_data_labels(series),
    x_values: x_values(&series.categories),
    y_values: y_values(&series.values),
    smooth: series_is_smoothed(series).then_some(c::Smooth {
      val: Some(true.into()),
    }),
    ..Default::default()
  }
}

fn bubble_series(chart: &OgraphChart, series: &OgraphSeries) -> c::BubbleChartSeries {
  let bubble_3d = series_data_format(series).is_some_and(|format| {
    format.series.is_some_and(|format| {
      format
        .flags
        .contains(ChartSeriesFormatFlags::THREE_DIMENSIONAL_BUBBLES)
    })
  });
  c::BubbleChartSeries {
    index: index(series),
    order: order(series),
    series_text: series_text(series),
    chart_shape_properties: series_shape_properties(chart, series, false),
    data_point: data_points(chart, series, false),
    data_labels: series_data_labels(series),
    x_values: x_values(&series.categories),
    y_values: y_values(&series.values),
    bubble_size: bubble_size(&series.bubble_sizes),
    bubble3_d: bubble_3d.then_some(c::Bubble3D {
      val: Some(true.into()),
    }),
    ..Default::default()
  }
}

fn surface_series(chart: &OgraphChart, series: &OgraphSeries) -> c::SurfaceChartSeries {
  c::SurfaceChartSeries {
    index: index(series),
    order: order(series),
    series_text: series_text(series),
    chart_shape_properties: series_shape_properties(chart, series, true),
    category_axis_data: category_axis_data(&series.categories),
    values: values(&series.values),
    ..Default::default()
  }
}

fn series_shape(series: &[&OgraphSeries]) -> Option<c::Shape> {
  let shape = series
    .iter()
    .find_map(|series| series_data_format(series)?.bar_shape)?;
  let val = match (shape.riser, shape.taper) {
    (0, 0) => c::ShapeValues::Box,
    (0, 1) => c::ShapeValues::Pyramid,
    (0, _) => c::ShapeValues::PyramidToMaximum,
    (_, 0) => c::ShapeValues::Cylinder,
    (_, 1) => c::ShapeValues::Cone,
    _ => c::ShapeValues::ConeToMax,
  };
  Some(c::Shape { val: Some(val) })
}

fn view_3d(source: olecfsdk::xls::Chart3DRecord) -> c::View3D {
  let rotation = i32::from(source.rotation).rem_euclid(360) as u16;
  c::View3D {
    rotate_x: i8::try_from(source.elevation)
      .ok()
      .map(|val| c::RotateX { val: Some(val) }),
    height_percent: (!source.flags.contains(Chart3DFlags::AUTO_SCALING)).then_some(
      c::HeightPercent {
        val: Some(source.height_percent),
      },
    ),
    rotate_y: Some(c::RotateY {
      val: Some(rotation),
    }),
    depth_percent: u16::try_from(source.depth_percent)
      .ok()
      .map(|val| c::DepthPercent { val: Some(val) }),
    right_angle_axes: Some(c::RightAngleAxes {
      val: Some(
        (!source
          .flags
          .contains(olecfsdk::xls::Chart3DFlags::PERSPECTIVE))
        .into(),
      ),
    }),
    // MS-OGRAPH stores the field-of-view angle; DrawingML c:perspective
    // stores twice that angle.
    perspective: u8::try_from(source.field_of_view * 2)
      .ok()
      .map(|val| c::Perspective { val: Some(val) }),
    ..Default::default()
  }
}

fn legend(source: &OgraphChart) -> Option<Box<c::Legend>> {
  let legend = source
    .groups
    .iter()
    .find_map(|group| group.legend.as_ref())?;
  if legend.source.flags.contains(ChartLegendFlags::DATA_TABLE)
    || legend.position.source.top_left_mode == 3
  {
    return None;
  }
  // MS-OGRAPH marks the byte interpreted by MS-XLS as `dock mode` unused.
  // Entry direction and the required Pos record are the Graph authorities.
  let position = if legend.source.flags.contains(ChartLegendFlags::VERTICAL) {
    c::LegendPositionValues::Right
  } else {
    c::LegendPositionValues::Bottom
  };
  let width_points = source.width_points();
  let height_points = source.height_points();
  let manual_layout = c::ManualLayout {
    left_mode: Some(c::LeftMode {
      val: Some(c::LayoutModeValues::Edge),
    }),
    top_mode: Some(c::TopMode {
      val: Some(c::LayoutModeValues::Edge),
    }),
    width_mode: (legend.position.source.bottom_right_mode == 1).then_some(c::WidthMode {
      val: Some(c::LayoutModeValues::Factor),
    }),
    height_mode: (legend.position.source.bottom_right_mode == 1).then_some(c::HeightMode {
      val: Some(c::LayoutModeValues::Factor),
    }),
    left: Some(c::Left {
      val: f64::from(legend.position.x1) / 4_000.0,
    }),
    top: Some(c::Top {
      val: f64::from(legend.position.y1) / 4_000.0,
    }),
    width: (legend.position.source.bottom_right_mode == 1 && width_points > 0.0).then_some(
      c::Width {
        val: f64::from(legend.position.x2) / width_points,
      },
    ),
    height: (legend.position.source.bottom_right_mode == 1 && height_points > 0.0).then_some(
      c::Height {
        val: f64::from(legend.position.y2) / height_points,
      },
    ),
    ..Default::default()
  };
  let frame = legend.frame.as_ref();
  Some(Box::new(c::Legend {
    legend_position: Some(c::LegendPosition {
      val: Some(position),
    }),
    layout: Some(Box::new(c::Layout {
      manual_layout: Some(Box::new(manual_layout)),
      ..Default::default()
    })),
    chart_shape_properties: Some(Box::new(c::ChartShapeProperties {
      chart_shape_properties_choice2: Some(area_fill_choice(
        source,
        frame.and_then(|frame| frame.area),
        0x00ff_ffff,
      )),
      outline: Some(Box::new(line_outline(
        source,
        frame.and_then(|frame| frame.line),
        AutomaticLineRole::CommonHairline,
      ))),
      ..Default::default()
    })),
    ..Default::default()
  }))
}

fn custom_split(source: &olecfsdk::xls::ChartBopPopCustomRecord) -> c::CustomSplit {
  let bit_count = source.membership_bits.len() * 8;
  let meaningful_bits = usize::from(source.data_point_count_plus_one);
  let padding = bit_count.saturating_sub(meaningful_bits);
  let point_count = meaningful_bits.saturating_sub(1);
  let second_pie_point = (0..point_count)
    .filter(|point| {
      let bit = padding + point;
      source
        .membership_bits
        .get(bit / 8)
        .is_some_and(|byte| byte & (1 << (7 - bit % 8)) != 0)
    })
    .map(|point| c::SecondPiePoint {
      val: u32::try_from(point).expect("MS-OGRAPH BopPop point index fits u32"),
    })
    .collect();
  c::CustomSplit { second_pie_point }
}
