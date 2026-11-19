use super::*;

pub(super) struct WordDateLabels {
  pub body: WordHorizontalCategoryLabels,
  pub ticks: Vec<ChartCategoryTick>,
  pub plot_left: f32,
  pub plot_right: f32,
  pub plot_bottom: f32,
}

pub(super) struct LabelSpace {
  pub frame: ChartFrame,
  pub plot_top: f32,
  pub value_label_width: f32,
}

pub(super) fn layout(
  space: LabelSpace,
  chart: &ClusteredColumnChart<'_>,
  style: &ClusteredColumnStyle,
  scale: crate::render::chart::LinearAxisScale,
  metrics: &mut TextMetrics,
) -> Option<WordDateLabels> {
  let LabelSpace {
    frame,
    plot_top,
    value_label_width: maximum_value_width,
  } = space;
  let axis = chart.date_axis?;
  if style.layout_profile != ChartLayoutProfile::Word
    || chart.view_3d.is_some()
    || chart.plot_layout.is_some()
    || chart.title_layout.is_some()
    || chart.data_table.is_some()
    || chart.axis_sets.len() > 1
    || chart.category_axis_shifted
    || chart.category_axis_reversed
    || chart.category_axis_title.is_some()
    || chart.value_axis_title.is_some()
    || !chart.additional_axis_titles.is_empty()
    || chart.legend_layout.is_some()
    || chart.legend_overlay
    || !matches!(
      chart.legend_position,
      None | Some(ChartLegendPosition::Bottom)
    )
    || !chart
      .series
      .iter()
      .all(|series| series.kind == ChartSeriesKind::Area)
    || !date_axis_is_visible(axis)
    || axis.axis_position.val != c::AxisPositionValues::Bottom
    || axis.tick_label_position.as_ref().is_some_and(|position| {
      matches!(
        position.val,
        Some(c::TickLabelPositionValues::None | c::TickLabelPositionValues::High)
      )
    })
    || !chart
      .value_axis
      .is_some_and(|axis| value_axis_is_visible(axis) && !value_axis_is_on_right(axis))
  {
    return None;
  }
  let properties = axis.text_properties.as_deref();
  let automatic = properties.is_none_or(|p| p.body_properties.rotation == Some(-60_000_000));
  let authored = category_axis_text_rotation_degrees(properties);
  if !automatic && (authored + 45.0).abs() > f32::EPSILON {
    return None;
  }
  let ticks = date_axis_ticks(chart)?;
  if ticks.is_empty() {
    return None;
  }
  let device = style.stroke_scale;
  let value_line =
    word_chart_natural_line_height(metrics.vertical_metrics_for_text("0", &style.value_label));
  let plot_left = frame.x_pt + 6.5 * device + maximum_value_width + value_line * 0.76;
  let plot_right = frame.x_pt + frame.width_pt - 11.0 * device;
  let legend_band = if chart.legend_position == Some(ChartLegendPosition::Bottom) {
    let slot =
      word_chart_symbol_line_height(metrics.vertical_metrics_for_text("Mg", &style.legend)) * 0.9;
    word_bottom_legend_reservation_with_slot(frame, chart, style, scale, metrics, slot)
  } else {
    0.0
  };
  let initial_height = frame.y_pt + frame.height_pt - 11.0 * device - legend_band - plot_top;
  let available_width = plot_right - plot_left;
  if initial_height <= 0.0 || available_width <= 0.0 {
    return None;
  }
  let natural_line =
    word_chart_natural_line_height(metrics.vertical_metrics_for_text("0", &style.category_label));
  let maximum_width = ticks
    .iter()
    .map(|tick| word_chart_tick_label_width(&tick.text, &style.category_label, metrics))
    .fold(0.0_f32, f32::max);
  // Word's date-axis measurement inflates both dimensions by 1.2 for
  // admission only (CHART 16.0.20527.20088, 0x533249..0x53328d).
  // The divisor is the number of dates, not the number of intervals. This
  // precedes the endpoint-label overhangs that narrow the painted plot.
  let spacing = available_width / ticks.len() as f32;
  let rotation = if automatic {
    word_automatic_category_label_rotation(maximum_width * 1.2, natural_line * 1.2, spacing)
  } else {
    authored
  };
  // Quarter-turn/multiline date labels also require calendar tick-density
  // selection; leave that owner with the existing date-axis path.
  if rotation.abs() >= 90.0 {
    return None;
  }
  let offset = axis
    .label_offset
    .as_ref()
    .and_then(|v| v.val)
    .unwrap_or(100);
  let label_gap = natural_line * (0.27 + offset as f32 / 400.0);
  let mut lines = ticks
    .iter()
    .map(|tick| vec![tick.text.clone()])
    .collect::<Vec<_>>();
  let mut alignment_widths = vec![None; lines.len()];
  if rotation.abs() > f32::EPSILON {
    // The native date-axis caller passes half the initial axis height into
    // the generated label. CHART 0x101370 inverts its rotated rectangle
    // while retaining one natural line; the axis gap is outside that box.
    let (sin, cos) = rotation.to_radians().sin_cos();
    let maximum_width = ((initial_height * 0.5 - natural_line * cos.abs()) / sin.abs()).max(0.0);
    for (index, text) in ticks.iter().enumerate() {
      if word_chart_tick_label_width(&text.text, &style.category_label, metrics) <= maximum_width {
        continue;
      }
      let overflow = properties.and_then(|p| p.body_properties.vertical_overflow);
      if !matches!(
        overflow,
        Some(a::TextVerticalOverflowValues::Ellipsis | a::TextVerticalOverflowValues::Clip)
      ) {
        return None;
      }
      // A shallow native box retains at least one grapheme. Its generated
      // ellipsis may protrude beyond the requested width, like the ordinary
      // diagonal category-label owner.
      let first_end = icu_segmenter::GraphemeClusterSegmenter::new()
        .segment_str(&text.text)
        .nth(1)
        .unwrap_or(text.text.len());
      let mut minimum = text.text[..first_end].to_string();
      if overflow == Some(a::TextVerticalOverflowValues::Ellipsis) {
        minimum.push('…');
      }
      let maximum_width = maximum_width.max(metrics.measure_text(&minimum, &style.category_label));
      let wrapped =
        word_chart_text_lines(&text.text, &style.category_label, maximum_width, metrics);
      let first = wrapped.first()?;
      let visible = if overflow == Some(a::TextVerticalOverflowValues::Ellipsis) {
        ellipsize_chart_legend_line(first, maximum_width, &style.category_label, metrics)
      } else {
        first.clone()
      };
      if let Some(prefix) = visible.strip_suffix('\u{2026}') {
        alignment_widths[index] = Some(word_chart_tick_label_width(
          prefix,
          &style.category_label,
          metrics,
        ));
      }
      lines[index] = vec![visible];
    }
  }
  let maximum_width = lines
    .iter()
    .enumerate()
    .map(|(i, lines)| {
      alignment_widths[i]
        .unwrap_or_else(|| word_chart_tick_label_width(&lines[0], &style.category_label, metrics))
    })
    .fold(0.0_f32, f32::max);
  let (sin, cos) = rotation.to_radians().sin_cos();
  let band_height = maximum_width * sin.abs() + natural_line * cos.abs() + label_gap;
  Some(WordDateLabels {
    body: WordHorizontalCategoryLabels {
      lines,
      alignment_widths,
      rotation,
      natural_line,
      label_gap,
      band_height,
    },
    ticks,
    plot_left,
    plot_right,
    plot_bottom: frame.y_pt + frame.height_pt - 6.5 * device - legend_band - band_height,
  })
}
